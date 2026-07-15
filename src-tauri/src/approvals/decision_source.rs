use std::fmt;
use std::sync::Arc;

use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

use super::manager::{
    ApprovalManagerInstanceMarker, ApprovalPresentation, ApprovalPresentationParts,
};
use super::types::{
    ApprovalAction, ApprovalAuthenticationEvidence, ApprovalId, ApprovalInteractionSource,
    ApprovalNativeButton, ApprovalRecipients, ApprovalReversibility, ApprovalRisk,
    ApprovalSchedule, ApprovalSourceFailure, ApprovalTarget,
};

pub const NATIVE_APPROVAL_DIALOG_TITLE: &str = "AI Agent Assistant approval";
pub const MAX_NATIVE_APPROVAL_MESSAGE_CHARACTERS: usize = 1_024;

const REJECT_BUTTON: &str = "Reject";
const APPROVE_BUTTON: &str = "Approve";
const EDIT_BUTTON: &str = "Edit";

pub struct MacOsNativeApprovalDecisionSource;

impl MacOsNativeApprovalDecisionSource {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn request_decision(
        &self,
        presentation: ApprovalPresentation,
    ) -> TrustedApprovalSourceOutcome {
        let parts = presentation.into_source_parts();
        let message = match build_native_message(&parts) {
            Ok(message) => message,
            Err(failure) => {
                return TrustedApprovalSourceOutcome::from_presentation(
                    parts,
                    TrustedSourceDecision::SourceFailed(failure),
                );
            }
        };

        let result = MessageDialog::new()
            .set_level(MessageLevel::Info)
            .set_title(NATIVE_APPROVAL_DIALOG_TITLE)
            .set_description(message)
            .set_buttons(native_message_buttons())
            .show();

        outcome_from_dialog_result(parts, result)
    }
}

impl Default for MacOsNativeApprovalDecisionSource {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TrustedApprovalSourceOutcome {
    id: ApprovalId,
    manager_instance: Arc<ApprovalManagerInstanceMarker>,
    run_id: String,
    gateway_request_id: String,
    call_id: String,
    decision: TrustedSourceDecision,
    source: ApprovalInteractionSource,
    authentication: ApprovalAuthenticationEvidence,
}

impl TrustedApprovalSourceOutcome {
    fn from_presentation(
        presentation: ApprovalPresentationParts,
        decision: TrustedSourceDecision,
    ) -> Self {
        let ApprovalPresentationParts {
            id,
            manager_instance,
            run_id,
            gateway_request_id,
            call_id,
            tool_name,
            tool_contract_version,
            policy_outcome,
            policy_reason,
            risk_class,
            required_permission,
            action,
            target,
            schedule,
            recipients,
            reversibility,
            risk,
            title,
            remaining,
        } = presentation;

        let _ = (
            tool_name,
            tool_contract_version,
            policy_outcome,
            policy_reason,
            risk_class,
            required_permission,
            action,
            target,
            schedule,
            recipients,
            reversibility,
            risk,
            remaining,
        );
        drop(title);

        Self {
            id,
            manager_instance,
            run_id,
            gateway_request_id,
            call_id,
            decision,
            source: ApprovalInteractionSource::MacOsNativeDialog,
            authentication: ApprovalAuthenticationEvidence::NotEvaluated,
        }
    }

    pub(super) fn into_parts(self) -> TrustedApprovalSourceOutcomeParts {
        TrustedApprovalSourceOutcomeParts {
            id: self.id,
            manager_instance: self.manager_instance,
            run_id: self.run_id,
            gateway_request_id: self.gateway_request_id,
            call_id: self.call_id,
            decision: self.decision,
            source: self.source,
            authentication: self.authentication,
        }
    }
}

impl fmt::Debug for TrustedApprovalSourceOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TrustedApprovalSourceOutcome")
            .field("id", &self.id)
            .field("identity", &"[REDACTED]")
            .field("decision", &self.decision)
            .field("source", &self.source)
            .field("authentication", &self.authentication)
            .finish()
    }
}

pub(super) struct TrustedApprovalSourceOutcomeParts {
    pub(super) id: ApprovalId,
    pub(super) manager_instance: Arc<ApprovalManagerInstanceMarker>,
    pub(super) run_id: String,
    pub(super) gateway_request_id: String,
    pub(super) call_id: String,
    pub(super) decision: TrustedSourceDecision,
    pub(super) source: ApprovalInteractionSource,
    pub(super) authentication: ApprovalAuthenticationEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TrustedSourceDecision {
    RecognizedButton(ApprovalNativeButton),
    NativeNoDecision,
    SourceFailed(ApprovalSourceFailure),
}

fn outcome_from_dialog_result(
    presentation: ApprovalPresentationParts,
    result: MessageDialogResult,
) -> TrustedApprovalSourceOutcome {
    let decision = match result {
        MessageDialogResult::Custom(label) if label == APPROVE_BUTTON => {
            TrustedSourceDecision::RecognizedButton(ApprovalNativeButton::Approve)
        }
        MessageDialogResult::Custom(label) if label == REJECT_BUTTON => {
            TrustedSourceDecision::RecognizedButton(ApprovalNativeButton::Reject)
        }
        MessageDialogResult::Custom(label) if label == EDIT_BUTTON => {
            TrustedSourceDecision::RecognizedButton(ApprovalNativeButton::Edit)
        }
        MessageDialogResult::Cancel => TrustedSourceDecision::NativeNoDecision,
        MessageDialogResult::Yes
        | MessageDialogResult::No
        | MessageDialogResult::Ok
        | MessageDialogResult::Custom(_) => {
            TrustedSourceDecision::SourceFailed(ApprovalSourceFailure::UnexpectedDialogResult)
        }
    };

    TrustedApprovalSourceOutcome::from_presentation(presentation, decision)
}

fn native_message_buttons() -> MessageButtons {
    MessageButtons::YesNoCancelCustom(
        REJECT_BUTTON.to_owned(),
        APPROVE_BUTTON.to_owned(),
        EDIT_BUTTON.to_owned(),
    )
}

#[cfg(test)]
pub(super) fn test_outcome_from_dialog_result(
    presentation: ApprovalPresentation,
    result: MessageDialogResult,
) -> TrustedApprovalSourceOutcome {
    let parts = presentation.into_source_parts();
    match build_native_message(&parts) {
        Ok(_) => outcome_from_dialog_result(parts, result),
        Err(failure) => TrustedApprovalSourceOutcome::from_presentation(
            parts,
            TrustedSourceDecision::SourceFailed(failure),
        ),
    }
}

fn build_native_message(
    presentation: &ApprovalPresentationParts,
) -> Result<String, ApprovalSourceFailure> {
    build_native_message_from_facts(
        presentation.action,
        presentation.target,
        presentation.schedule,
        presentation.recipients,
        presentation.reversibility,
        presentation.required_permission,
        presentation.risk,
        &presentation.title,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_native_message_from_facts(
    action: ApprovalAction,
    target: ApprovalTarget,
    schedule: ApprovalSchedule,
    recipients: ApprovalRecipients,
    reversibility: ApprovalReversibility,
    required_permission: crate::tools::types::PermissionKind,
    risk: ApprovalRisk,
    title: &str,
) -> Result<String, ApprovalSourceFailure> {
    if title.chars().any(is_unsafe_presentation_character) {
        return Err(ApprovalSourceFailure::UnsafePresentationFormatting);
    }

    let message = format!(
        "Action: {}\nTarget: {}\nSchedule/date-time: {}\nRecipients: {}\nReversibility: {}\nRequired permission: {}\nMain risk: {}\nChoices: Approve, Reject, Edit\nAffected task title: {}",
        action_label(action),
        target_label(target),
        schedule_label(schedule),
        recipients_label(recipients),
        reversibility_label(reversibility),
        permission_label(required_permission),
        risk_label(risk),
        title,
    );

    if message.chars().count() > MAX_NATIVE_APPROVAL_MESSAGE_CHARACTERS {
        return Err(ApprovalSourceFailure::MessageLimitExceeded);
    }

    Ok(message)
}

fn action_label(action: ApprovalAction) -> &'static str {
    match action {
        ApprovalAction::CreateLocalTask => "Create one local task",
    }
}

fn target_label(target: ApprovalTarget) -> &'static str {
    match target {
        ApprovalTarget::LocalTaskList => "Local task list",
    }
}

fn schedule_label(schedule: ApprovalSchedule) -> &'static str {
    match schedule {
        ApprovalSchedule::NotScheduled => "None",
    }
}

fn recipients_label(recipients: ApprovalRecipients) -> &'static str {
    match recipients {
        ApprovalRecipients::None => "None",
    }
}

fn reversibility_label(reversibility: ApprovalReversibility) -> &'static str {
    match reversibility {
        ApprovalReversibility::Reversible => "Reversible",
    }
}

fn permission_label(permission: crate::tools::types::PermissionKind) -> &'static str {
    match permission {
        crate::tools::types::PermissionKind::None => "None",
        crate::tools::types::PermissionKind::Calendar
        | crate::tools::types::PermissionKind::Reminders
        | crate::tools::types::PermissionKind::Contacts
        | crate::tools::types::PermissionKind::Notifications
        | crate::tools::types::PermissionKind::Files
        | crate::tools::types::PermissionKind::Accessibility
        | crate::tools::types::PermissionKind::ScreenRecording
        | crate::tools::types::PermissionKind::Automation
        | crate::tools::types::PermissionKind::Microphone => "Unsupported",
    }
}

fn risk_label(risk: ApprovalRisk) -> &'static str {
    match risk {
        ApprovalRisk::CreatesLocalTask => "Creates a local task",
    }
}

fn is_unsafe_presentation_character(character: char) -> bool {
    matches!(
        u32::from(character),
        0x00AD
            | 0x034F
            | 0x061C
            | 0x115F..=0x1160
            | 0x17B4..=0x17B5
            | 0x180B..=0x180F
            | 0x200B..=0x200F
            | 0x2028..=0x202E
            | 0x2060..=0x206F
            | 0x3164
            | 0xFE00..=0xFE0F
            | 0xFEFF
            | 0xFFA0
            | 0xFFF9..=0xFFFB
            | 0x1BCA0..=0x1BCA3
            | 0x1D173..=0x1D17A
            | 0xE0000..=0xE0001
            | 0xE0020..=0xE007F
            | 0xE0100..=0xE01EF
    )
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io;
    use std::sync::Arc;

    use rfd::{MessageButtons, MessageDialogResult};
    use serde_json::json;

    use super::{
        build_native_message, build_native_message_from_facts, native_message_buttons,
        outcome_from_dialog_result, test_outcome_from_dialog_result, TrustedApprovalSourceOutcome,
        MAX_NATIVE_APPROVAL_MESSAGE_CHARACTERS, NATIVE_APPROVAL_DIALOG_TITLE,
    };
    use crate::agent::function_call_validation::validate_function_call;
    use crate::agent::gateway_protocol::{
        GatewayProtocolError, GatewayStreamValidator, ValidatedGatewayEvent,
        GATEWAY_PROTOCOL_VERSION,
    };
    use crate::approvals::manager::{
        ApprovalError, ApprovalManager, ApprovalPresentation, ApprovalPresentationParts,
        InMemoryApprovalManager,
    };
    use crate::approvals::types::{
        ApprovalAction, ApprovalAuthenticationEvidence, ApprovalCancellationReason,
        ApprovalDisposition, ApprovalInteractionSource, ApprovalNativeButton, ApprovalRecipients,
        ApprovalReversibility, ApprovalRisk, ApprovalSchedule, ApprovalSourceFailure,
        ApprovalTarget,
    };
    use crate::policy::engine::{DeterministicPolicyEngine, PolicyEngine};
    use crate::policy::types::{PolicyDecision, PolicyInput};
    use crate::tools::registry::{InMemoryToolRegistry, ToolRegistry, ToolRegistryResult};
    use crate::tools::types::{PermissionKind, ToolDefinition, ToolSchema};

    const TOOL_CONTRACT_VERSION: u16 = 1;

    fn registry() -> ToolRegistryResult<InMemoryToolRegistry> {
        let mut registry = InMemoryToolRegistry::new();
        registry.register(ToolDefinition::from_schema(
            ToolSchema::GetCurrentDatetimeV1,
        ))?;
        registry.register(ToolDefinition::from_schema(ToolSchema::CreateLocalTaskV1))?;
        Ok(registry)
    }

    fn local_task_decision(
        run_id: &str,
        gateway_request_id: &str,
        call_id: &str,
        title: &str,
    ) -> Result<PolicyDecision, Box<dyn Error>> {
        let mut validator = GatewayStreamValidator::new(
            run_id,
            gateway_request_id,
            ["create_local_task".to_owned()],
            TOOL_CONTRACT_VERSION,
        )?;
        let start_frame = json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": run_id,
            "gateway_request_id": gateway_request_id,
            "sequence": 0,
            "event": {
                "type": "response_started",
                "provider_response_id": "provider-response-native-approval-test-1",
            },
        });
        let function_frame = json!({
            "protocol_version": GATEWAY_PROTOCOL_VERSION,
            "run_id": run_id,
            "gateway_request_id": gateway_request_id,
            "sequence": 1,
            "event": {
                "type": "function_call_completed",
                "call_id": call_id,
                "name": "create_local_task",
                "tool_contract_version": TOOL_CONTRACT_VERSION,
                "arguments_json": json!({ "title": title }).to_string(),
            },
        });

        let _ = validator.accept_frame(&serde_json::to_vec(&start_frame)?)?;
        let call = match validator.accept_frame(&serde_json::to_vec(&function_frame)?)? {
            ValidatedGatewayEvent::FunctionCallCompleted { call } => call,
            _ => return Err(GatewayProtocolError::MalformedEvent.into()),
        };
        let validated = validate_function_call(call, &registry()?)?;
        Ok(DeterministicPolicyEngine::new().evaluate(PolicyInput::from_validated_call(validated)))
    }

    fn manager_and_presentation(
        run_id: &str,
        gateway_request_id: &str,
        call_id: &str,
        title: &str,
    ) -> Result<(InMemoryApprovalManager, ApprovalPresentation), Box<dyn Error>> {
        let mut manager = InMemoryApprovalManager::new();
        let id = manager.create_request(local_task_decision(
            run_id,
            gateway_request_id,
            call_id,
            title,
        )?)?;
        let presentation = manager.issue_presentation(id)?;
        Ok((manager, presentation))
    }

    fn message_for_title(title: &str) -> Result<String, ApprovalSourceFailure> {
        build_native_message_from_facts(
            ApprovalAction::CreateLocalTask,
            ApprovalTarget::LocalTaskList,
            ApprovalSchedule::NotScheduled,
            ApprovalRecipients::None,
            ApprovalReversibility::Reversible,
            PermissionKind::None,
            ApprovalRisk::CreatesLocalTask,
            title,
        )
    }

    fn duplicate_parts(parts: &ApprovalPresentationParts) -> ApprovalPresentationParts {
        ApprovalPresentationParts {
            id: parts.id,
            manager_instance: Arc::clone(&parts.manager_instance),
            run_id: parts.run_id.clone(),
            gateway_request_id: parts.gateway_request_id.clone(),
            call_id: parts.call_id.clone(),
            tool_name: parts.tool_name.clone(),
            tool_contract_version: parts.tool_contract_version,
            policy_outcome: parts.policy_outcome,
            policy_reason: parts.policy_reason,
            risk_class: parts.risk_class,
            required_permission: parts.required_permission,
            action: parts.action,
            target: parts.target,
            schedule: parts.schedule,
            recipients: parts.recipients,
            reversibility: parts.reversibility,
            risk: parts.risk,
            title: parts.title.clone(),
            remaining: parts.remaining,
        }
    }

    #[test]
    fn fixes_title_button_order_and_preview_layout() -> Result<(), Box<dyn Error>> {
        assert_eq!(NATIVE_APPROVAL_DIALOG_TITLE, "AI Agent Assistant approval");
        let MessageButtons::YesNoCancelCustom(first, second, third) = native_message_buttons()
        else {
            return Err(io::Error::other("native button configuration changed").into());
        };
        assert_eq!(first, "Reject");
        assert_eq!(second, "Approve");
        assert_eq!(third, "Edit");

        let (_, presentation) = manager_and_presentation(
            "run-native-layout-1",
            "gateway-request-native-layout-1",
            "call-native-layout-1",
            "Review native approval",
        )?;
        let parts = presentation.into_source_parts();
        let message = build_native_message(&parts)?;
        assert_eq!(
            message,
            "Action: Create one local task\nTarget: Local task list\nSchedule/date-time: None\nRecipients: None\nReversibility: Reversible\nRequired permission: None\nMain risk: Creates a local task\nChoices: Approve, Reject, Edit\nAffected task title: Review native approval"
        );
        let Some(choices_index) = message.find("Choices: Approve, Reject, Edit") else {
            return Err(io::Error::other("fixed choices missing").into());
        };
        let Some(title_index) = message.find("Affected task title:") else {
            return Err(io::Error::other("affected title label missing").into());
        };
        assert!(choices_index < title_index);
        assert!(message.chars().count() <= MAX_NATIVE_APPROVAL_MESSAGE_CHARACTERS);
        Ok(())
    }

    #[test]
    fn enforces_message_limit_and_exact_unsafe_character_set() -> Result<(), Box<dyn Error>> {
        let ordinary_non_ascii = message_for_title("Préparer le résumé 東京")?;
        assert!(ordinary_non_ascii.contains("Préparer le résumé 東京"));
        assert!(message_for_title(&"x".repeat(200)).is_ok());
        assert_eq!(
            message_for_title(&"x".repeat(MAX_NATIVE_APPROVAL_MESSAGE_CHARACTERS)),
            Err(ApprovalSourceFailure::MessageLimitExceeded)
        );

        let mut denied = vec![0x00AD, 0x034F, 0x061C, 0x3164, 0xFEFF, 0xFFA0];
        for (start, end) in [
            (0x115F, 0x1160),
            (0x17B4, 0x17B5),
            (0x180B, 0x180F),
            (0x200B, 0x200F),
            (0x2028, 0x202E),
            (0x2060, 0x206F),
            (0xFE00, 0xFE0F),
            (0xFFF9, 0xFFFB),
            (0x1BCA0, 0x1BCA3),
            (0x1D173, 0x1D17A),
            (0xE0000, 0xE0001),
            (0xE0020, 0xE007F),
            (0xE0100, 0xE01EF),
        ] {
            denied.extend(start..=end);
        }

        for code_point in denied {
            let Some(character) = char::from_u32(code_point) else {
                return Err(io::Error::other("invalid denied code point").into());
            };
            assert_eq!(
                message_for_title(&format!("before{character}after")),
                Err(ApprovalSourceFailure::UnsafePresentationFormatting),
                "U+{code_point:04X} must fail before display"
            );
        }
        Ok(())
    }

    #[test]
    fn maps_every_dialog_result_to_closed_resolution_evidence() -> Result<(), Box<dyn Error>> {
        let cases = [
            (
                MessageDialogResult::Custom("Approve".to_owned()),
                ApprovalDisposition::Approved,
                Some(ApprovalNativeButton::Approve),
                None,
            ),
            (
                MessageDialogResult::Custom("Reject".to_owned()),
                ApprovalDisposition::Rejected,
                Some(ApprovalNativeButton::Reject),
                None,
            ),
            (
                MessageDialogResult::Custom("Edit".to_owned()),
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::EditRequested),
                Some(ApprovalNativeButton::Edit),
                None,
            ),
            (
                MessageDialogResult::Cancel,
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::NativeNoDecision),
                None,
                None,
            ),
            (
                MessageDialogResult::Yes,
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed),
                None,
                Some(ApprovalSourceFailure::UnexpectedDialogResult),
            ),
            (
                MessageDialogResult::No,
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed),
                None,
                Some(ApprovalSourceFailure::UnexpectedDialogResult),
            ),
            (
                MessageDialogResult::Ok,
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed),
                None,
                Some(ApprovalSourceFailure::UnexpectedDialogResult),
            ),
            (
                MessageDialogResult::Custom("unexpected".to_owned()),
                ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed),
                None,
                Some(ApprovalSourceFailure::UnexpectedDialogResult),
            ),
        ];

        for (index, (dialog_result, disposition, button, failure)) in cases.into_iter().enumerate()
        {
            let run_id = format!("run-native-map-{index}");
            let request_id = format!("gateway-request-native-map-{index}");
            let call_id = format!("call-native-map-{index}");
            let (mut manager, presentation) =
                manager_and_presentation(&run_id, &request_id, &call_id, "Map native result")?;
            let id = presentation.id();
            let outcome = test_outcome_from_dialog_result(presentation, dialog_result);
            let resolution = manager.resolve_source_outcome(outcome)?;
            assert_eq!(resolution.id(), id);
            assert_eq!(resolution.disposition(), disposition);
            let Some(evidence) = resolution.interaction_evidence() else {
                return Err(io::Error::other("source evidence missing").into());
            };
            assert_eq!(
                evidence.source(),
                ApprovalInteractionSource::MacOsNativeDialog
            );
            assert_eq!(evidence.native_button(), button);
            assert_eq!(
                evidence.authentication(),
                ApprovalAuthenticationEvidence::NotEvaluated
            );
            assert_eq!(evidence.source_failure(), failure);
        }
        Ok(())
    }

    #[test]
    fn source_failure_precedes_an_approve_result_and_consumes_subject() -> Result<(), Box<dyn Error>>
    {
        let run_id = "run-native-unsafe-1";
        let request_id = "gateway-request-native-unsafe-1";
        let call_id = "call-native-unsafe-1";
        let (mut manager, presentation) =
            manager_and_presentation(run_id, request_id, call_id, "unsafe\u{200b}title")?;
        let id = presentation.id();
        let outcome = test_outcome_from_dialog_result(
            presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );
        let resolution = manager.resolve_source_outcome(outcome)?;
        assert_eq!(
            resolution.disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed)
        );
        let Some(evidence) = resolution.interaction_evidence() else {
            return Err(io::Error::other("source failure evidence missing").into());
        };
        assert_eq!(evidence.native_button(), None);
        assert_eq!(
            evidence.source_failure(),
            Some(ApprovalSourceFailure::UnsafePresentationFormatting)
        );
        assert_eq!(
            manager.create_request(local_task_decision(
                run_id,
                request_id,
                call_id,
                "safe replacement title",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );
        assert_eq!(
            manager.cancel_for_run_termination(id),
            Err(ApprovalError::AlreadyConsumed(id.value()))
        );
        Ok(())
    }

    #[test]
    fn message_limit_failure_is_sealed_and_terminal() -> Result<(), Box<dyn Error>> {
        let (mut manager, presentation) = manager_and_presentation(
            "run-native-limit-1",
            "gateway-request-native-limit-1",
            "call-native-limit-1",
            "Initially bounded title",
        )?;
        let id = presentation.id();
        let mut parts = presentation.into_source_parts();
        parts.title = "x".repeat(MAX_NATIVE_APPROVAL_MESSAGE_CHARACTERS);
        assert_eq!(
            build_native_message(&parts).err(),
            Some(ApprovalSourceFailure::MessageLimitExceeded)
        );
        let outcome = TrustedApprovalSourceOutcome::from_presentation(
            parts,
            super::TrustedSourceDecision::SourceFailed(ApprovalSourceFailure::MessageLimitExceeded),
        );
        let resolution = manager.resolve_source_outcome(outcome)?;
        assert_eq!(
            resolution.disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::SourceFailed)
        );
        let Some(evidence) = resolution.interaction_evidence() else {
            return Err(io::Error::other("message limit evidence missing").into());
        };
        assert_eq!(evidence.native_button(), None);
        assert_eq!(
            evidence.source_failure(),
            Some(ApprovalSourceFailure::MessageLimitExceeded)
        );
        assert_eq!(
            manager.cancel_for_run_termination(id),
            Err(ApprovalError::AlreadyConsumed(id.value()))
        );
        Ok(())
    }

    #[test]
    fn rejects_cross_manager_and_identity_substitution_without_mutating_pending(
    ) -> Result<(), Box<dyn Error>> {
        let run_id = "run-native-cross-manager-1";
        let request_id = "gateway-request-native-cross-manager-1";
        let call_id = "call-native-cross-manager-1";
        let (mut first_manager, first_presentation) =
            manager_and_presentation(run_id, request_id, call_id, "First manager title")?;
        let first_id = first_presentation.id();
        let first_outcome = test_outcome_from_dialog_result(
            first_presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );

        let (mut second_manager, second_presentation) =
            manager_and_presentation(run_id, request_id, call_id, "Different retained title")?;
        let second_id = second_presentation.id();
        assert_eq!(first_id, second_id);
        drop(second_presentation);
        assert_eq!(
            second_manager.resolve_source_outcome(first_outcome),
            Err(ApprovalError::ManagerInstanceMismatch)
        );
        assert!(second_manager.pending()?.is_some());
        assert_eq!(
            second_manager
                .cancel_for_run_termination(second_id)?
                .disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );
        assert_eq!(
            first_manager
                .cancel_for_run_termination(first_id)?
                .disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );

        let (mut identity_manager, identity_presentation) = manager_and_presentation(
            "run-native-identity-1",
            "gateway-request-native-identity-1",
            "call-native-identity-1",
            "Identity-bound title",
        )?;
        let identity_id = identity_presentation.id();
        let mut identity_outcome = test_outcome_from_dialog_result(
            identity_presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );
        identity_outcome.call_id = "call-native-substituted".to_owned();
        assert_eq!(
            identity_manager.resolve_source_outcome(identity_outcome),
            Err(ApprovalError::SourceOutcomeIdentityMismatch)
        );
        assert!(identity_manager.pending()?.is_some());
        assert_eq!(
            identity_manager.issue_presentation(identity_id).err(),
            Some(ApprovalError::PresentationAlreadyIssued(
                identity_id.value()
            ))
        );
        assert_eq!(
            identity_manager
                .cancel_for_run_termination(identity_id)?
                .disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );
        Ok(())
    }

    #[test]
    fn replay_and_late_outcomes_fail_after_terminal_transition() -> Result<(), Box<dyn Error>> {
        let run_id = "run-native-replay-1";
        let request_id = "gateway-request-native-replay-1";
        let call_id = "call-native-replay-1";
        let (mut manager, presentation) =
            manager_and_presentation(run_id, request_id, call_id, "Replay-bound title")?;
        let id = presentation.id();
        let parts = presentation.into_source_parts();
        let duplicate = duplicate_parts(&parts);
        let first =
            outcome_from_dialog_result(parts, MessageDialogResult::Custom("Approve".to_owned()));
        let replay = outcome_from_dialog_result(
            duplicate,
            MessageDialogResult::Custom("Approve".to_owned()),
        );
        assert_eq!(
            manager.resolve_source_outcome(first)?.disposition(),
            ApprovalDisposition::Approved
        );
        assert_eq!(
            manager.resolve_source_outcome(replay),
            Err(ApprovalError::AlreadyConsumed(id.value()))
        );
        assert_eq!(
            manager.create_request(local_task_decision(
                run_id,
                request_id,
                call_id,
                "Changed replay title",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );

        let (mut cancelled_manager, cancelled_presentation) = manager_and_presentation(
            "run-native-late-1",
            "gateway-request-native-late-1",
            "call-native-late-1",
            "Late outcome title",
        )?;
        let cancelled_id = cancelled_presentation.id();
        let late_outcome = test_outcome_from_dialog_result(
            cancelled_presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );
        assert_eq!(
            cancelled_manager
                .cancel_for_run_termination(cancelled_id)?
                .disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::RunTerminated)
        );
        assert_eq!(
            cancelled_manager.resolve_source_outcome(late_outcome),
            Err(ApprovalError::AlreadyConsumed(cancelled_id.value()))
        );
        Ok(())
    }

    #[test]
    fn edit_is_terminal_and_requires_a_fresh_subject() -> Result<(), Box<dyn Error>> {
        let run_id = "run-native-edit-1";
        let request_id = "gateway-request-native-edit-1";
        let call_id = "call-native-edit-1";
        let (mut manager, presentation) =
            manager_and_presentation(run_id, request_id, call_id, "Original title")?;
        let outcome = test_outcome_from_dialog_result(
            presentation,
            MessageDialogResult::Custom("Edit".to_owned()),
        );
        assert_eq!(
            manager.resolve_source_outcome(outcome)?.disposition(),
            ApprovalDisposition::Cancelled(ApprovalCancellationReason::EditRequested)
        );
        assert_eq!(
            manager.create_request(local_task_decision(
                run_id,
                request_id,
                call_id,
                "Edited title",
            )?),
            Err(ApprovalError::DuplicateSubject)
        );

        let fresh_id = manager.create_request(local_task_decision(
            "run-native-edit-2",
            "gateway-request-native-edit-2",
            "call-native-edit-2",
            "Edited title",
        )?)?;
        assert_eq!(manager.issue_presentation(fresh_id)?.id(), fresh_id);
        Ok(())
    }

    #[test]
    fn outcome_debug_redacts_title_and_exact_identity() -> Result<(), Box<dyn Error>> {
        let title = "private-native-title";
        let run_id = "run-native-private-1";
        let request_id = "gateway-request-native-private-1";
        let call_id = "call-native-private-1";
        let (_, presentation) = manager_and_presentation(run_id, request_id, call_id, title)?;
        let outcome = test_outcome_from_dialog_result(
            presentation,
            MessageDialogResult::Custom("Approve".to_owned()),
        );
        let debug = format!("{outcome:?}");
        for private_value in [title, run_id, request_id, call_id] {
            assert!(!debug.contains(private_value));
        }
        assert!(debug.contains("MacOsNativeDialog"));
        assert!(debug.contains("NotEvaluated"));
        drop(outcome);
        Ok(())
    }

    #[test]
    fn trusted_outcome_has_no_public_construction_data() -> Result<(), Box<dyn Error>> {
        let (_, presentation) = manager_and_presentation(
            "run-native-sealed-1",
            "gateway-request-native-sealed-1",
            "call-native-sealed-1",
            "Sealed outcome title",
        )?;
        let outcome: TrustedApprovalSourceOutcome =
            test_outcome_from_dialog_result(presentation, MessageDialogResult::Cancel);
        let parts = outcome.into_parts();
        assert_eq!(parts.source, ApprovalInteractionSource::MacOsNativeDialog);
        assert_eq!(
            parts.authentication,
            ApprovalAuthenticationEvidence::NotEvaluated
        );
        Ok(())
    }
}
