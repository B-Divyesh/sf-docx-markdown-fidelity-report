# Visual thesis: topographic cartography

The product maps conversion risk. Its visual language borrows field survey maps: contour lines show document complexity, coordinate ticks suggest exact source locations, and vermilion marks identify places that need human review. This fits a fidelity ledger better than a generic developer-tool shell because it makes "where did risk occur?" visible at a glance.

## Palette

- `--paper #F4F0E4`: warm survey paper and the primary light background.
- `--paper-high #FFFCF4`: raised reading surfaces.
- `--ink #17211D`: near-black green for 12.7:1 body contrast on paper.
- `--muted #526159`: secondary text, 5.9:1 on paper.
- `--forest #174C3C`: contour ink and primary action; white contrast 9.4:1.
- `--vermilion #B83A2D`: review markers; white contrast 5.7:1.
- `--ochre #9A6715`: warning terrain; dark ink is paired with it.
- `--moss #DCE4D8`: quiet status fill.
- `--night #101A17`, `--night-surface #172620`, `--night-text #F4F0E4`: explicit dark treatment for the terminal and footer.

Color never carries status alone. Every marker includes a label and symbol.
The thesis is intentionally single-mode because paper is the map canvas. Dark instrument panels provide local contrast without turning the survey sheet into a generic app theme.

## Type and spacing

The display face is self-hosted **Fraunces**, used sparingly for geographic, editorial headings. The body and interface face is self-hosted **Atkinson Hyperlegible**, chosen for dense reports and distinct letterforms. Terminal output uses the platform monospace stack. Both font files ship with the site and use `font-display: swap`.

Spacing follows an 8 px field grid, with 4 px only for tight label relationships. Content measures stay below 70 characters. Sections alternate between open paper and inset surveyed bands rather than generic feature cards.

## Shape and interaction grammar

Corners are clipped like map sheets (`clip-path` chamfers) on major panels. Buttons are compact map stamps with a 2 px offset shadow. Status markers use numbered survey pins. Tables use coordinate-style row labels. Focus is a 3 px vermilion ring with 3 px clearance.

The signature motion is a one-time contour draw across the hero when it enters. It lasts 700 ms and stops. Small state changes use 180 ms opacity and transform transitions. With `prefers-reduced-motion: reduce`, all drawing and transforms are disabled and content appears immediately. No motion loops.

## Responsive intent

At 390 px, the terminal moves below the headline, facts become a vertical legend, and map coordinate ornaments disappear. Core copy, demo action, command, and pricing remain. Tap targets stay at least 44 px.

## Original asset plan and provenance

- `site/public/terrain-report.webp`: generated for this product with `/opt/fleet/lib/gen-image.sh`, then converted locally to WebP. It depicts a cream topographic survey containing a DOCX page, extracted images, table cells, and vermilion review pins. No text, logos, people, or copyrighted characters. Used in the hero and to compose social art.
- `site/public/og-card.webp`: locally composed crop of the original art with product-owned typography; 1200×630.
- `site/public/favicon.svg`: hand-authored contour-and-pin mark by the builder; MIT with this repository.

Generation prompt: "Editorial topographic cartography illustration for a software tool that converts Word documents to Markdown and maps fidelity risks. Warm cream survey paper, dark forest-green contour lines, a simplified document sheet as terrain, a table grid, small image frames and footnote marks embedded in the landscape, six precise vermilion survey pins marking review locations, subtle paper grain, screen-print ink texture, asymmetric wide composition with quiet negative space on the left, flat graphic printmaking, no gradients, no readable text, no letters, no logos, no UI screenshot, no people, no watermark."

Generated with the factory image deployment on 2026-08-28. The output is an original project asset.
