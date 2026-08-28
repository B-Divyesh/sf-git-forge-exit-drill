# Visual thesis: the evidence lattice

## Direction

Generative geometry turns migration evidence into a visible system. Repositories appear as cyan nodes, preserved artifacts as ivory facets, and gaps as coral breaks. Offset registration marks and numbered coordinates borrow from engineering inspection sheets rather than generic developer dashboards. The irregular lattice makes the product identifiable at thumbnail size and explains its job: trace each source artifact across a boundary before cutover.

This is a deliberately dark, single-mode interface. It matches terminal work, keeps the generated line art crisp, and avoids implying that the CLI is a hosted data service.

## Palette

| Token | Value | Use |
| --- | --- | --- |
| `--ink-950` | `#07110f` | page background |
| `--ink-900` | `#0d1c19` | raised surfaces |
| `--ink-800` | `#18302b` | rules and quiet fills |
| `--paper` | `#f4f1df` | primary text |
| `--paper-muted` | `#b8c4b9` | secondary text |
| `--signal` | `#5cf2c7` | links, focus, passing evidence |
| `--coral` | `#ff7a68` | missing evidence and warnings |
| `--amber` | `#f3c969` | review states |

Text pairings were checked for at least 4.5:1 contrast. Color is always paired with a word, icon, or pattern.

## Type

- Display: system monospace (`ui-monospace`, SFMono-Regular, Menlo, Consolas). Its fixed grid echoes manifests and checksums.
- Body: system humanist sans (`Inter` fallback is not requested or downloaded; `system-ui`, Segoe UI, sans-serif). It keeps instructions readable.
- No font files or third-party font requests are needed.

## Spacing and shape

- An 8 px base unit, with section gaps of 64–112 px.
- Reading measure stays under 68 characters.
- Corners are clipped with CSS polygons, like inspection tags cut from a sheet.
- Cards are reserved for independent outputs: evidence, map, and drill result.
- Hairline lattice rules use 1 px strokes. Primary controls use 2 px borders and 44 px minimum targets.

## Interaction grammar

- The cyan route line draws once as the hero enters, then stops.
- Checklist items reveal from source to target in 60 ms offsets.
- Buttons compress by 1 px on press. Route changes focus the new heading.
- `prefers-reduced-motion: reduce` removes drawing, reveal, smooth scroll, and transforms. All information remains visible.

## Original asset plan and provenance

- `site/public/geometry-exit-drill.webp`: generated on 2026-08-28 with the factory image model through `/opt/fleet/lib/gen-image.sh`, then resized and compressed locally to WebP. Prompt: “Abstract generative geometry for a developer-tool landing page: a dark technical inspection field, one dense repository node lattice on the left crossing a narrow boundary into a clean verified lattice on the right, mint cyan paths, warm ivory nodes, sparse coral gaps, precise plotter lines, screen-print grain, orthographic flat composition, no letters, no numbers, no logos, no UI screenshot, high contrast, ample dark negative space.” Original work created for this product; no third-party source asset.
- `site/public/og-image.webp`: composed locally from the same generated artwork with product typography. It contains no required interface text.
- Logo and interface marks are hand-made CSS/SVG geometry created in this repository under the MIT license.

## Why it fits

Migration readiness is a graph problem disguised as a checklist. The lattice gives every artifact a place, while broken routes make unsupported capabilities obvious without turning failure into alarmist decoration.
