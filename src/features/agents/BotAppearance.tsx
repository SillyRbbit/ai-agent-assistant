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
import type { BotIdentity } from "../../infrastructure/tauri/agent-chat-client";
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
export function BotAvatar({ identity }: { readonly identity: BotIdentity }) {
  const Icon = icons[identity.avatar];
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
