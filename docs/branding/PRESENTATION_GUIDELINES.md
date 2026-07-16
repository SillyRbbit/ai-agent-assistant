# Cortexa presentation and diagram guidelines

## Audience and tone

Design for IT leadership, operations leaders, and business executives who need
to assess value, control, and delivery risk quickly. Use concise claims,
traceable evidence, quiet neutral surfaces, and restrained brand accents.

## Presentation structure

- Use 16:9 widescreen layouts unless the delivery channel requires another
  format.
- Make `Cortexa` and the subject of the presentation visible in the first
  viewport or title slide.
- Use one decision, claim, or operating question per slide.
- Keep section structure consistent: context, evidence, decision, next action.
- Prefer real product screenshots, actual repository evidence, and readable
  diagrams over decorative illustrations.
- Treat external decks and montages as references, not as the source of current
  product facts.

## Visual system

Use Brand Field or white as the dominant light surface, Supporting Ink for
text, Deep Indigo/Core Blue for primary emphasis, Circuit Teal for secondary
pathways, and semantic colors only for their functional meanings. Avoid
gradient backgrounds, ornamental blobs, nested cards, and dense decoration.

Use the approved system typography. Keep charts and tables direct: label data
in place, limit legends, disclose units and dates, and distinguish measured
facts from targets or proposals.

## Architecture diagrams

- Draw current implemented boundaries with solid lines.
- Draw proposed or future boundaries with dashed lines and label them as future
  or proposed.
- Keep user experience, trusted local Rust, gateway, provider/model, platform,
  storage, and enterprise-system boundaries visually distinct.
- Show trust direction and ownership explicitly. The model is an untrusted
  planner; deterministic Rust owns validation, policy, approval, execution, and
  audit boundaries.
- Use red only for a prohibited path, failure, or explicit security warning.
- Do not imply a direct model-to-device execution path.
- Mark mocked, volatile, unaudited, disconnected, or non-authorizing behavior
  where those limitations affect interpretation.
- Reconcile labels against `PROJECT_STATUS.md`, `DECISIONS.md`, and
  `docs/product/ARCHITECTURE_BASELINE.md` before publication.

## Logo placement

Use the complete primary logo with required clear space. Do not put it in every
panel or repeat it as decoration. On a title slide, pair it with the product name
without locking new text into the image. Keep the protected near-white field on
dark slides.

## Publication review

Verify product name, dates, statuses, repository evidence, image paths, aspect
ratios, text fit, light/dark contrast, and source attribution. Scan for stale
architecture modules, the former display name, placeholder logos, unsupported
claims, and future capabilities presented as current.
