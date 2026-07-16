# Cortexa colors

## Approved palette

These colors are sampled from the authoritative logo and its protected field.
They support the visual system; they do not authorize recoloring the logo.

| Token          | Hex       | Primary role                                       |
| -------------- | --------- | -------------------------------------------------- |
| Deep Indigo    | `#252B71` | Primary brand emphasis, light-background headings  |
| Core Blue      | `#4758A2` | Controls, links, selected states on light surfaces |
| Sky Blue       | `#6C89BA` | Large accents, charts, non-text decoration         |
| Circuit Teal   | `#9ABFC6` | Large secondary accents and diagram connectors     |
| Brand Field    | `#FBFBFB` | Protected logo field and quiet light surface       |
| Supporting Ink | `#182033` | Primary text and dark neutral surfaces             |

## Contrast guidance

Measured WCAG contrast ratios for the sampled colors are:

| Color     | Against `#FBFBFB` | Against `#182033` |
| --------- | ----------------: | ----------------: |
| `#252B71` |           12.21:1 |            1.29:1 |
| `#4758A2` |            6.37:1 |            2.46:1 |
| `#6C89BA` |            3.43:1 |            4.58:1 |
| `#9ABFC6` |            1.91:1 |            8.23:1 |
| `#182033` |           15.69:1 |               1:1 |

- Use Deep Indigo, Core Blue, or Supporting Ink for normal text on Brand Field.
- Do not use Sky Blue or Circuit Teal for normal text on Brand Field.
- Circuit Teal can support normal text on Supporting Ink. Sky Blue is suitable
  for large text on Supporting Ink but not normal text without a separate
  contrast check.
- Test the actual foreground/background pair for every new component. A brand
  token does not replace accessibility review.

## Interface use

Keep most surfaces neutral. Use Deep Indigo or Core Blue for primary identity
and active controls, Sky Blue for restrained data emphasis, and Circuit Teal
for secondary pathways or informational diagram connectors. Do not build a
one-hue interface around blue variations alone; retain neutral surfaces and
semantic colors.

Success, warning, danger, and informational states are functional tokens, not
logo colors. Preserve their established meaning and contrast. Never recolor the
logo to communicate application state.
