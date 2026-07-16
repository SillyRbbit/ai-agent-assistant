# Cortexa brand asset usage

## Asset selection

- Use `logo-primary.png` in README files, general documentation, and neutral
  print or digital layouts.
- Use `logo-light.png` in light product interfaces.
- Use `logo-dark.png` in dark product interfaces. It intentionally preserves
  the same protected field as the primary asset.
- Use `favicon.png` only for browser or compact document-favicon contexts.
- Use `app-icon-source.png` only as input to the separately approved and
  verified application-icon rollout. It is not the current production Tauri
  icon set.

Do not copy the owner screenshot into another tracked location. Repository
references must point to the canonical files under `assets/branding/`.

## Repository examples

Markdown:

```markdown
![Cortexa logo](assets/branding/logo-primary.png)
```

HTML with a bounded display size:

```html
<img src="assets/branding/logo-primary.png" alt="Cortexa logo" width="144" />
```

React light/dark selection:

```tsx
<picture aria-hidden="true">
  <source media="(prefers-color-scheme: dark)" srcSet={brandLogoDark} />
  <img alt="" src={brandLogoLight} />
</picture>
```

Always constrain one dimension and preserve the source aspect ratio. Use
`object-fit: contain` when both layout dimensions must be stable.

## Accessibility

- Use `alt="Cortexa logo"` when the standalone logo conveys the product
  identity and no adjacent text does so.
- Use `alt=""` and hide the containing picture from assistive technology when
  adjacent visible text already says `Cortexa`.
- Do not put product meaning only in the image. Keep titles and navigation
  labels as text.
- Follow the text contrast rules in `COLORS.md`; sampled logo colors are not
  automatically approved for small text.

## Naming and compatibility

Use the exact display name `Cortexa`. Do not rename the repository, npm package,
Rust crate, executable, bundle identifier, database, GitHub URL, IPC command,
storage path, or compatibility identifier merely to align branding. D-026 in
`DECISIONS.md` remains authoritative for the display-name boundary.

## Review checklist

- Reference an authoritative asset and verify the path exists.
- Preserve the full field, aspect ratio, orientation, and original colors.
- Check clear space and minimum size at every supported viewport or page size.
- Check light and dark backgrounds.
- Use meaningful or deliberately empty alternative text.
- Confirm no Tauri production icon changed outside its own verified increment.
- Scan the final output for the former product name and placeholder marks.
