import Foundation

// Pure arbitration shared with the deterministic self-test build.
struct VoiceDecision {
    enum Phase { case starting, recording, finishing, terminal }
    private(set) var phase: Phase = .starting
    private(set) var discard = false
    mutating func recording() -> Bool {
        guard phase == .starting else { return false }
        phase = .recording
        return true
    }
    mutating func finish() -> Bool {
        guard phase == .recording else { return false }
        phase = .finishing
        return true
    }
    mutating func cancel() -> Bool {
        guard phase != .terminal else { return false }
        discard = true
        phase = .terminal
        return true
    }
    mutating func complete() -> Bool {
        guard phase == .recording || phase == .finishing else { return false }
        phase = .terminal
        return !discard
    }
}

// Require the delegate witness and the stopped predicate before the deadline.
// Do not assume callback delivery itself guarantees the current speaking flag.
struct SpeechSettlement {
    enum Outcome { case waiting, stopped, failed }
    static func observe(callback: Bool, speaking: Bool, now: TimeInterval,
                        deadline: TimeInterval) -> Outcome {
        if now >= deadline { return .failed }
        return callback && !speaking ? .stopped : .waiting
    }
}

// Closed categories from the installed SDK SFSpeechRecognitionTask.error table.
// Never forward NSError descriptions, userInfo, arbitrary domains or codes.
enum RecognitionFailure {
    static func classify(domain: String, code: Int) -> String {
        switch (domain, code) {
        case ("kLSRErrorDomain", 102): return "recognition_assets_missing"
        case ("kLSRErrorDomain", 201): return "recognition_disabled"
        case ("kLSRErrorDomain", 300), ("kAFAssistantErrorDomain", 1101),
             ("kAFAssistantErrorDomain", 1107): return "recognition_service"
        case ("kAFAssistantErrorDomain", 1100): return "recognition_busy"
        case ("kAFAssistantErrorDomain", 1110): return "recognition_no_speech"
        case ("kAFAssistantErrorDomain", 1700): return "permission_denied"
        default: return "recognition_failed"
        }
    }
}

#if VOICE_SELF_TEST
@main
struct VoiceTests {
    static func main() {
        var early = VoiceDecision()
        precondition(!early.finish())
        precondition(early.cancel() && !early.recording() && !early.complete())
        var stopped = VoiceDecision()
        precondition(stopped.recording() && stopped.cancel() && !stopped.complete())
        precondition(!stopped.cancel())
        var finished = VoiceDecision()
        precondition(finished.recording() && finished.finish() && finished.complete())
        precondition(!finished.cancel() && !finished.complete())
        var completed = VoiceDecision()
        precondition(completed.recording() && completed.complete() && !completed.cancel())
        precondition(!completed.finish())
        // Controlled callback/state order, not timing-dependent sleeps.
        precondition(SpeechSettlement.observe(callback: false, speaking: false, now: 0, deadline: 5) == .waiting)
        precondition(SpeechSettlement.observe(callback: true, speaking: true, now: 1, deadline: 5) == .waiting)
        precondition(SpeechSettlement.observe(callback: true, speaking: false, now: 2, deadline: 5) == .stopped)
        precondition(SpeechSettlement.observe(callback: true, speaking: true, now: 5, deadline: 5) == .failed)
        precondition(SpeechSettlement.observe(callback: false, speaking: false, now: 5, deadline: 5) == .failed)
        precondition(SpeechSettlement.observe(callback: true, speaking: false, now: 5, deadline: 5) == .failed)
        let recognized: [(String, Int, String)] = [
            ("kLSRErrorDomain", 102, "recognition_assets_missing"),
            ("kLSRErrorDomain", 201, "recognition_disabled"),
            ("kLSRErrorDomain", 300, "recognition_service"),
            ("kAFAssistantErrorDomain", 1100, "recognition_busy"),
            ("kAFAssistantErrorDomain", 1101, "recognition_service"),
            ("kAFAssistantErrorDomain", 1107, "recognition_service"),
            ("kAFAssistantErrorDomain", 1110, "recognition_no_speech"),
            ("kAFAssistantErrorDomain", 1700, "permission_denied")
        ]
        for (domain, code, expected) in recognized {
            precondition(RecognitionFailure.classify(domain: domain, code: code) == expected)
            precondition(RecognitionFailure.classify(domain: "other", code: code) == "recognition_failed")
        }
        // Cancellation, unknown codes/domains and arbitrary detail never become success.
        for (domain, code) in [("kLSRErrorDomain", 301), ("kAFAssistantErrorDomain", 203),
                               ("kLSRErrorDomain", 1110), ("private-domain", 1110),
                               ("kAFAssistantErrorDomain", Int.max)] {
            precondition(RecognitionFailure.classify(domain: domain, code: code) == "recognition_failed")
        }
        print("VOICE_DECISION_TESTS_PASSED")
    }
}
#else
import AVFAudio
import AVFoundation
import Speech

private struct Input: Decodable, Sendable {
    let version: Int
    let session: String
    let operation: String
    let text: String?
}

// AVAudioEngine's real-time callback is off-main. All access to the request and
// capture ingress lifetime is serialized by this lock; UI/task ownership is MainActor.
private final class AudioIngress: @unchecked Sendable {
    private let lock = NSLock()
    private var request: SFSpeechAudioBufferRecognitionRequest?
    private let overflow: @Sendable () -> Void
    init(_ request: SFSpeechAudioBufferRecognitionRequest, overflow: @escaping @Sendable () -> Void) {
        self.request = request
        self.overflow = overflow
    }
    func append(_ buffer: AVAudioPCMBuffer) {
        lock.lock()
        defer { lock.unlock() }
        guard let request else { return }
        let bytes = UInt64(buffer.frameLength) * UInt64(buffer.format.streamDescription.pointee.mBytesPerFrame)
        guard bytes <= 8 * 1024 * 1024 else {
            self.request = nil
            request.endAudio()
            overflow()
            return
        }
        // No application queue or audio copy is retained after append returns.
        request.append(buffer)
    }
    func close() {
        lock.lock()
        defer { lock.unlock() }
        request?.endAudio()
        request = nil
    }
}

@MainActor
private final class VoiceRunner: NSObject, AVSpeechSynthesizerDelegate {
    private var session: String?
    private var operation: String?
    private var sequence = 0
    private var decision = VoiceDecision()
    private var engine: AVAudioEngine?
    private var ingress: AudioIngress?
    private var recognition: SFSpeechRecognitionTask?
    private var synthesizer: AVSpeechSynthesizer?
    private var timer: Timer?
    private var tapped = false
    private var terminal = false
    private var speechStopRequested = false
    private var speechStarted = false
    private var speechCompletionObserved = false
    private var speechStopDeadline: TimeInterval?
    private var utteranceIdentity: ObjectIdentifier?

    private func emit(_ event: String, text: String? = nil, error: String? = nil) {
        guard let session else { exit(2) }
        sequence += 1
        let value: [String: Any] = ["version": 1, "session": session, "sequence": sequence,
            "event": event, "text": text as Any? ?? NSNull(), "error": error as Any? ?? NSNull()]
        guard JSONSerialization.isValidJSONObject(value),
              let data = try? JSONSerialization.data(withJSONObject: value, options: [.sortedKeys]),
              data.count <= 32768 else { exit(2) }
        FileHandle.standardOutput.write(data)
        FileHandle.standardOutput.write(Data([10]))
    }
    private func releaseAudio(cancel: Bool) -> Bool {
        timer?.invalidate()
        timer = nil
        if tapped { engine?.inputNode.removeTap(onBus: 0); tapped = false }
        engine?.stop()
        ingress?.close()
        ingress = nil
        if cancel { recognition?.cancel() } else { recognition?.finish() }
        recognition = nil
        let stopped = engine?.isRunning != true
        engine = nil
        return stopped
    }
    private func end(_ event: String, text: String? = nil, error: String? = nil) {
        guard !terminal else { return }
        terminal = true
        let captureStopped = releaseAudio(cancel: true)
        if let synthesizer { _ = synthesizer.stopSpeaking(at: .immediate) }
        synthesizer = nil
        guard captureStopped else { emit("error", error: "cleanup_failed"); exit(2) }
        emit(event, text: text, error: error)
        exit(error == nil ? 0 : 2)
    }
    private func fail(_ code: String) { end("error", error: code) }
    func receive(_ data: Data) {
        guard !terminal else { return }
        guard data.count <= 32768,
              let object = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
              Set(object.keys).isSubset(of: ["version", "session", "operation", "text"]),
              let input = try? JSONDecoder().decode(Input.self, from: data), input.version == 1,
              input.session.count <= 64,
              input.session.unicodeScalars.allSatisfy({ CharacterSet(charactersIn: "0123456789abcdef-").contains($0) }) else {
            if session != nil { fail("protocol") } else { exit(2) }
            return
        }
        if let session {
            guard session == input.session, input.text == nil else { fail("protocol"); return }
            switch input.operation {
            case "finish":
                guard operation == "record", decision.finish() else { fail("protocol"); return }
                // Stop capture immediately; allow only bounded final processing.
                if tapped { engine?.inputNode.removeTap(onBus: 0); tapped = false }
                engine?.stop()
                ingress?.close()
                recognition?.finish()
                timer?.invalidate()
                timer = Timer.scheduledTimer(withTimeInterval: 10, repeats: false) { [weak self] _ in
                    Task { @MainActor in self?.fail("timeout") }
                }
            case "cancel", "stop":
                _ = decision.cancel()
                if operation == "speak", let synthesizer, synthesizer.isSpeaking {
                    speechStopRequested = true
                    speechStopDeadline = ProcessInfo.processInfo.systemUptime + 5
                    timer?.invalidate()
                    guard synthesizer.stopSpeaking(at: .immediate) else { fail("cleanup_failed"); return }
                    timer = Timer.scheduledTimer(withTimeInterval: 5, repeats: false) { [weak self] _ in
                        Task { @MainActor in self?.fail("cleanup_failed") }
                    }
                } else { end("cancelled") }
            default: fail("protocol")
            }
            return
        }
        guard ["authorize", "record", "speak"].contains(input.operation),
              (input.operation == "speak") == (input.text != nil) else { exit(2) }
        session = input.session
        operation = input.operation
        switch input.operation {
        case "authorize": authorize()
        case "record": record()
        case "speak": speak(input.text ?? "")
        default: fail("protocol")
        }
    }
    // Legacy Objective-C callbacks do not promise MainActor execution. Explicit
    // Sendable boundaries prevent inherited isolation before the existing hop.
    private func authorize() {
        emit("checking_microphone")
        switch AVCaptureDevice.authorizationStatus(for: .audio) {
        case .authorized: authorizeSpeech()
        case .denied, .restricted: fail("permission_denied")
        case .notDetermined:
            AVCaptureDevice.requestAccess(for: .audio) { @Sendable [weak self] allowed in
                Task { @MainActor in
                    guard let self, !self.terminal else { return }
                    if allowed { self.authorizeSpeech() }
                    else { self.fail("permission_denied") }
                }
            }
        @unknown default: fail("unavailable")
        }
    }
    private func authorizeSpeech() {
        emit("checking_speech")
        switch SFSpeechRecognizer.authorizationStatus() {
        case .authorized: end("authorized")
        case .denied, .restricted: fail("permission_denied")
        case .notDetermined:
            SFSpeechRecognizer.requestAuthorization { @Sendable [weak self] status in
                Task { @MainActor in
                    guard let self, !self.terminal else { return }
                    if status == .authorized { self.end("authorized") }
                    else { self.fail("permission_denied") }
                }
            }
        @unknown default: fail("unavailable")
        }
    }
    private func record() {
        guard AVCaptureDevice.authorizationStatus(for: .audio) == .authorized,
              SFSpeechRecognizer.authorizationStatus() == .authorized else {
            fail("permission_denied"); return
        }
        guard let recognizer = SFSpeechRecognizer(locale: Locale(identifier: "en-US")),
              recognizer.supportsOnDeviceRecognition, recognizer.isAvailable else {
            fail("unavailable"); return
        }
        let request = SFSpeechAudioBufferRecognitionRequest()
        request.requiresOnDeviceRecognition = true
        request.shouldReportPartialResults = false
        let engine = AVAudioEngine()
        let format = engine.inputNode.outputFormat(forBus: 0)
        guard format.channelCount > 0, format.sampleRate > 0 else { fail("unavailable"); return }
        let ingress = AudioIngress(request) { [weak self] in
            Task { @MainActor in self?.fail("limit") }
        }
        self.engine = engine
        self.ingress = ingress
        if #available(macOS 27.0, *) {
            do {
                try engine.inputNode.__installTap(onBus: 0, bufferSize: 1024, format: format, error: ()) { @Sendable buffer, _ in
                    ingress.append(buffer)
                }
            } catch { fail("capture_failed"); return }
        } else {
            engine.inputNode.installTap(onBus: 0, bufferSize: 1024, format: format) { @Sendable buffer, _ in
                ingress.append(buffer)
            }
        }
        tapped = true
        recognition = recognizer.recognitionTask(with: request) { @Sendable [weak self] result, error in
            // Only copy bounded text; never log Apple's error or raw content.
            let text = result?.isFinal == true ? result?.bestTranscription.formattedString : nil
            let failure = error.map { value in
                let native = value as NSError
                return RecognitionFailure.classify(domain: native.domain, code: native.code)
            }
            Task { @MainActor in
                guard let self, !self.terminal else { return }
                if let failure { self.fail(failure); return }
                guard let text else { return }
                guard !text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty,
                      text.count <= 4096,
                      !text.unicodeScalars.contains(where: { $0.value == 0 }) else {
                    self.fail("limit"); return
                }
                guard self.decision.complete() else { self.fail("protocol"); return }
                self.end("transcript", text: text)
            }
        }
        do {
            engine.prepare()
            try engine.start()
            guard decision.recording() else { fail("protocol"); return }
            emit("recording")
            timer = Timer.scheduledTimer(withTimeInterval: 30, repeats: false) { [weak self] _ in
                Task { @MainActor in
                    guard let self, let session = self.session,
                          let data = try? JSONSerialization.data(withJSONObject:
                            ["version": 1, "session": session, "operation": "finish"]) else { return }
                    self.receive(data)
                }
            }
        } catch { fail("capture_failed") }
    }
    private func speak(_ text: String) {
        guard !text.isEmpty, text.count <= 4096, !text.contains("\0"),
              let voice = AVSpeechSynthesisVoice.speechVoices().first(where: {
                  $0.language == "en-US" && $0.quality == .default && $0.voiceTraits.isEmpty
              }) else { fail("unavailable"); return }
        let synthesizer = AVSpeechSynthesizer()
        self.synthesizer = synthesizer
        synthesizer.delegate = self
        let utterance = AVSpeechUtterance(string: text)
        utterance.voice = voice
        utteranceIdentity = ObjectIdentifier(utterance)
        synthesizer.speak(utterance)
        timer = Timer.scheduledTimer(withTimeInterval: 60, repeats: false) { [weak self] _ in
            Task { @MainActor in self?.fail("timeout") }
        }
    }
    private func speechCallbackMatches(_ synthesizer: ObjectIdentifier,
                                       _ utterance: ObjectIdentifier) -> Bool {
        guard let current = self.synthesizer else { return false }
        return !terminal && operation == "speak" && ObjectIdentifier(current) == synthesizer
            && utteranceIdentity == utterance
    }
    nonisolated func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didStart utterance: AVSpeechUtterance) {
        let synth = ObjectIdentifier(synthesizer), speech = ObjectIdentifier(utterance)
        Task { @MainActor in
            guard self.speechCallbackMatches(synth, speech) else { return }
            guard !self.speechStarted, !self.speechCompletionObserved else { self.fail("protocol"); return }
            self.speechStarted = true
            self.emit("speaking")
        }
    }
    private func settleSpeech(deadline: TimeInterval) {
        guard !terminal, let synthesizer else { return }
        switch SpeechSettlement.observe(callback: speechCompletionObserved,
                                        speaking: synthesizer.isSpeaking,
                                        now: ProcessInfo.processInfo.systemUptime,
                                        deadline: deadline) {
        case .stopped: end(speechStopRequested ? "cancelled" : "spoken")
        case .failed: fail("cleanup_failed")
        case .waiting: break
        }
    }
    private func speechCompleted(cancelled: Bool) {
        guard !speechCompletionObserved, cancelled ? speechStopRequested : speechStarted else {
            fail("protocol"); return
        }
        speechCompletionObserved = true
        let deadline = min(ProcessInfo.processInfo.systemUptime + 5,
                           speechStopDeadline ?? .greatestFiniteMagnitude)
        timer?.invalidate()
        settleSpeech(deadline: deadline)
        guard !terminal else { return }
        timer = Timer.scheduledTimer(withTimeInterval: 0.025, repeats: true) { [weak self] _ in
            Task { @MainActor in self?.settleSpeech(deadline: deadline) }
        }
    }
    nonisolated func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didFinish utterance: AVSpeechUtterance) {
        let synth = ObjectIdentifier(synthesizer), speech = ObjectIdentifier(utterance)
        Task { @MainActor in
            guard self.speechCallbackMatches(synth, speech) else { return }
            self.speechCompleted(cancelled: false)
        }
    }
    nonisolated func speechSynthesizer(_ synthesizer: AVSpeechSynthesizer, didCancel utterance: AVSpeechUtterance) {
        let synth = ObjectIdentifier(synthesizer), speech = ObjectIdentifier(utterance)
        Task { @MainActor in
            guard self.speechCallbackMatches(synth, speech) else { return }
            self.speechCompleted(cancelled: true)
        }
    }
    func disconnected() { if session != nil { end("cancelled") } else { exit(2) } }
}

@main
struct VoiceMain {
    @MainActor static func main() {
        let runner = VoiceRunner()
        // Independent of MainActor: a stuck native callback cannot leave an
        // orphan listening indefinitely if the owning app disappears.
        DispatchQueue.global().asyncAfter(deadline: .now() + 130) { exit(2) }
        Thread.detachNewThread {
            var bytes = Data()
            while true {
                let byte = FileHandle.standardInput.readData(ofLength: 1)
                if byte.isEmpty {
                    DispatchQueue.global().asyncAfter(deadline: .now() + 5) { exit(2) }
                    DispatchQueue.main.async { runner.disconnected() }
                    break
                }
                if byte[0] == 10 {
                    let complete = bytes
                    bytes = Data()
                    DispatchQueue.main.async { runner.receive(complete) }
                } else {
                    bytes.append(byte)
                    if bytes.count > 32768 { exit(2) }
                }
            }
        }
        RunLoop.main.run()
    }
}
#endif
