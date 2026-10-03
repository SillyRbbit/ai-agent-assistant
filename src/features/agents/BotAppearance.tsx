import { useEffect, useRef, useState, type CSSProperties } from "react";
import { BOT_MASCOTS, CONDUCTOR_MASCOT } from "./botMascots";
import { useBotAnimations, useReducedBotMotion } from "./botMotion";
import "./bot-mascots.css";
import { BOT_COLOR_VALUES } from "./botAppearanceValues";
import {
  Bot,
  Compass,
  Sparkles,
  Leaf,
  Shield,
  Star,
  Sun,
  Moon,
  Flame,
  Heart,
  Gem,
  Mountain,
  Waves,
  Rocket,
  Book,
  Code,
  Globe,
  Music,
} from "lucide-react";
import type { AgentId, BotIdentity } from "../../infrastructure/tauri/agent-chat-client";
const icons = {
  bot: Bot,
  compass: Compass,
  spark: Sparkles,
  leaf: Leaf,
  shield: Shield,
  star: Star,
  sun: Sun,
  moon: Moon,
  flame: Flame,
  heart: Heart,
  gem: Gem,
  mountain: Mountain,
  waves: Waves,
  rocket: Rocket,
  book: Book,
  code: Code,
  globe: Globe,
  music: Music,
};
interface AvatarProps {
  readonly identity: BotIdentity;
  readonly agentId?: AgentId | null;
  readonly expressive?: boolean;
  readonly greeting?: boolean;
  readonly success?: number;
}
export function BotAvatar({
  identity,
  agentId,
  expressive = false,
  greeting = false,
  success = 0,
}: AvatarProps) {
  if (identity.avatar === "mascot" && agentId)
    return (
      <Mascot
        key={agentId}
        agentId={agentId}
        expressive={expressive}
        greeting={greeting}
        success={success}
      />
    );
  const Icon = icons[identity.avatar === "mascot" ? "bot" : identity.avatar];
  return (
    <span
      className="bot-appearance"
      style={{
        color: BOT_COLOR_VALUES[identity.color],
        background: "#101722",
        display: "inline-flex",
        borderRadius: 8,
        padding: 4,
      }}
    >
      <Icon size={28} aria-hidden="true" />
    </span>
  );
}
export function CoordinatorAvatar({
  expressive = false,
  greeting = false,
  success = 0,
  working = false,
}: {
  readonly expressive?: boolean;
  readonly greeting?: boolean;
  readonly success?: number;
  readonly working?: boolean;
}) {
  return (
    <Mascot
      agentId="conductor"
      expressive={expressive}
      greeting={greeting}
      success={success}
      working={working}
    />
  );
}
function Mascot({
  agentId,
  expressive,
  greeting,
  success,
  working = false,
}: {
  readonly agentId: AgentId | "conductor";
  readonly expressive: boolean;
  readonly greeting: boolean;
  readonly success: number;
  readonly working?: boolean;
}) {
  const enabled = useBotAnimations(),
    reduced = useReducedBotMotion();
  const host = useRef<HTMLSpanElement>(null);
  const [visible, setVisible] = useState(false);
  const [foreground, setForeground] = useState(!document.hidden);
  const allowed = expressive && enabled && !reduced && visible && foreground;
  const [motion, setMotion] = useState({ kind: "idle", success, greeted: false, allowed, working });
  // Adjust only this component's derived presentation state. No deferred effect,
  // frame loop or queued reaction; consumed hidden events cannot replay later.
  if (
    motion.allowed !== allowed ||
    motion.working !== working ||
    motion.success !== success ||
    (greeting && !motion.greeted && (allowed || !enabled || reduced || !foreground))
  ) {
    setMotion({
      allowed,
      working,
      success,
      greeted: motion.greeted || (greeting && (allowed || !enabled || reduced || !foreground)),
      kind: !allowed
        ? "idle"
        : working
          ? "working"
          : motion.success !== success
            ? "success"
            : greeting && !motion.greeted
              ? "greeting"
              : "idle",
    });
  }
  useEffect(() => {
    if (!expressive) return;
    const visibility = () => {
      setForeground(!document.hidden);
      if (document.hidden) setMotion((value) => ({ ...value, kind: "idle" }));
    };
    document.addEventListener("visibilitychange", visibility);
    const observer =
      typeof IntersectionObserver === "undefined"
        ? null
        : new IntersectionObserver(([entry]) => {
            const next = entry?.isIntersecting ?? false;
            setVisible(next);
            if (!next) setMotion((value) => ({ ...value, kind: "idle" }));
          });
    if (host.current) observer?.observe(host.current);
    return () => {
      observer?.disconnect();
      document.removeEventListener("visibilitychange", visibility);
    };
  }, [expressive]);
  const asset = agentId === "conductor" ? CONDUCTOR_MASCOT : BOT_MASCOTS[agentId];
  return (
    <span
      className={`bot-appearance bot-mascot-presentation${expressive ? " bot-mascot-presentation--large" : ""}`}
    >
      <span
        ref={host}
        className="bot-mascot-space"
        aria-hidden="true"
        data-agent={agentId}
        data-motion={allowed ? motion.kind : "static"}
        style={
          {
            "--mascot-src": `url("${asset.src}")`,
            "--blink-duration": `${String(asset.blinkSeconds)}s`,
            ...(agentId === "conductor"
              ? {
                  "--atlas-size": "200% 200%",
                  "--blink-frame": "100% 0%",
                  "--wave-frame": "0% 100%",
                }
              : {}),
          } as CSSProperties
        }
      >
        <span
          className="bot-mascot-body"
          onAnimationEnd={(event) => {
            if (
              event.animationName ===
              (
                {
                  greeting: "mascot-wave",
                  success: "mascot-jump",
                  playful: "mascot-spin",
                } as Record<string, string>
              )[motion.kind]
            )
              setMotion((value) => ({ ...value, kind: "idle" }));
          }}
        >
          <span className="bot-mascot-sprite" />
        </span>
      </span>
      {expressive && (
        <>
          <button
            type="button"
            className="bot-mascot-play"
            disabled={!allowed || motion.kind !== "idle"}
            onClick={() => {
              setMotion((value) => ({ ...value, kind: "playful" }));
            }}
          >
            Playful spin
          </button>
          <small className="bot-mascot-disclosure">
            {agentId === "conductor"
              ? "Workflow status is shown in text · no added execution authority"
              : "Decorative avatar · not connection or work status"}
            {!enabled || reduced ? " · motion disabled" : ""}
          </small>
        </>
      )}
    </span>
  );
}
