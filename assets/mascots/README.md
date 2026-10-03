# Cortexa bot mascot sources

The owner-approved concept is retained byte-identically as `concept.png`. Each
canonical role has a transparent three-cell PNG master and a lossless WebP runtime
atlas. Frames are neutral, closed eyelids, and raised arm/wing. The image tool
prepared the artwork; format encoding preserves visible RGB and alpha exactly; fully transparent RGB
may be normalized by the lossless encoder. These are bot
illustrations, not the application logo.

Editable timing/frame data is in `animation-source.json`; runtime state and CSS
keyframes are in `src/features/agents/botMotion.ts` and `bot-mascots.css`.
No animation grants capabilities, represents connection readiness or replaces text
status. Saved legacy avatar choices remain available.
