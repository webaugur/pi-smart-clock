Pi Smart Clock - Playful Cartoony Icon Set
==========================================

Location: assets/icons/playful/

This replaces the previous custom "vivid" hand-authored SVGs (which were rejected
for visual quality after detail passes).

Source & License
----------------
- Primary weather/status icons: adapted from Meteocons by Bas Milius
  (https://github.com/basmilius/meteocons)
  - MIT licensed
  - Used the "Fill" / rich-color hand-crafted styles ("icons that feel alive")
  - CDN / static packages: svg-static/fill/ etc.
  - Specific glyphs chosen/mapped for our WMO states + night handling.

- Supplemental + base for some icons: Tabler Icons
  (https://github.com/tabler/tabler-icons)
  - MIT licensed
  - 24x24 grid, high-quality SVG.

- Zodiac (12 signs), calendar, starred, help, and all .hires variants:
  Project-authored (or lightly derived) for visual consistency:
  chunky lines, high-saturation playful colors, bold for distance legibility
  on the dark UI panels. These are under the project's dual MIT OR GPL-2.0-or-later
  (asset terms compatible; see debian/copyright for full attribution).

High-res / Low-res SVG variants
-------------------------------
The atlas (src/icons/atlas.rs) supports paired versions:
  foo.svg          (standard - chunky, tuned for small/medium targets)
  foo.hires.svg    (richer detail - extra rays, accents, etc.; auto-selected
                   when target size >= ~60px, e.g. large decorative icons
                   in calendar/holidays/zodiac panels)

This directly addresses prior complaints about icons looking like "ants" at
small sizes or "garbage" when over-detailed at target sizes. Artists can drop
a .hires sibling for any icon that benefits from extra fun at large scale.

Filenames & Usage
-----------------
- status/     : weather + misc (sun, moon, cloud-*, fog, cloud-rain etc., help)
- zodiac/     : zodiac-*-symbolic.svg (and optional .hires)
- apps/       : calendar-symbolic.svg (and optional .hires)

See CUSTOMIZATION.md for how to extend and src/modules/weather/icons.rs for
the WMO -> asset mapping.

Color & Style Goals
-------------------
- High saturation, thick strokes (2+), rounded caps for "cartoony" friendly look.
- No tiny elements or over-busy faces/rays that disappear or look noisy at
  24-48px effective render sizes.
- Consistent palette across weather + zodiac for a cohesive playful feel.
- draw_icon() path (color-preserving) is used; no tinting for these.

Maintenance
-----------
- To update from upstream: pull fresh static Fill SVGs from Meteocons CDN or
  package, adapt names + any local thick/color tweaks, add .hires where wanted.
- Re-run the clock (linux-full) and visually check small vs large icon uses.
- Keep the README.txt + debian/copyright in sync.

Thanks to the icon authors for the excellent permissive SVG work that made a
clean, cartoony, maintainable replacement possible.
