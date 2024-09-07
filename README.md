# nasin sitelen tan anpa nanpa

nasin nanpa uses [this specification](https://www.kreativekorp.com/ucsur/charts/sitelen.html).
o lukin a e [lipu Releases](https://github.com/ETBCOR/nasin-nanpa/releases)!

![nasin nanpa 4.0.1](/renders/nasin-nanpa-4.0.0.png)

## Font Versions
There are currently three parallel versions of nasin nanpa:
- `nasin-nanpa-4.x.x.otf` - the "main" version; uses UCSUR and ligatures from latin characters
- `nasin-nanpa-4.x.x-UCSUR.otf` - the "UCSUR only" version; doesn't have latin ligatures
- `nasin-nanpa-4.x.x-Helvetica.otf` - the "Discord" version; makes UCSUR visible in vanilla Discord

## Glyph Combos
The best way to combine glyphs in nasin nanpa is to put the `ZERO WIDTH JOINER` character (`&` with ligatures enabled) between them. If the first glyph has enough whitespace to contain another glyph, the scaling combination will be used. Otherwise, the stacking combination will be used.

Alternatively, one can use `STACKING JOINER` or `SCALING JOINER` (`-` and `+` respectively) to force a specific combination style.

Glyph combos can also be used inside cartouches and long glyphs.

## Glyph Variations
The following alternate glyph forms can be accessed by adding `VAR01` (or `1` with ligatures enabled) directly after the base glyph:
- `a` with two stems (can also be accessed with `a a`)
- `akesi` with three lines / six legs
- `kala` with eyes
- `meli` that's a circle with plus below
- `mije` that's a circle with arrow to upper right
- `mu` - sideways emitters
- `mute` - four `luka`s
- `olin` - heart with emitters
- `pana` - just emitters
- `sewi` - secular sewi (matches other directional glyphs)
- `tenpo` - hourglass
- `uta` without dot
- `wile` - upside-down `pilin`
- `namako` - looks like crosshairs
- `lanpan` - upside-down `pana`
- `misikeke` - mortar and pestle
- `linluwi` - looks like `len`

There are a few other alternate glyph forms that can be accessed with the other variation selectors:
- 8 `jaki`s (which are also used to randomize the base glyph wherever supported/enabled)
- 8 `ko`s (also used for the `rand` feature like `jaki`)
- 8 directional `ni`s: 1 ←, 2 ↑, 3 →, 4 ↓, 5 ↖, 6 ↗, 7 ↘, 8 ↙ (can also be accessed with `ni` + `ZWJ` + [an arrow character / a sequence like `v<`, if ligatures are enabled])
- `a` with three stems (can also be accessed with `a a a`)

The following glyphs have a "long glyph" variation too (accessed by puting `START OF LONG GLYPH` / `(` after it): `a`, `alasa`, `anu`, `awen`, `kama`, `ken`, `kepeken`, `la` (reversed; needs `END OF REVERSE LONG GLYPH` / `}` *before* it), `lon`, `nanpa`, `open`, `pi`, `pini`, `sona`, `tawa`, `wile`, `wile` alt, `n`, and `wa`.

## Ligatures
Ligatures are a font feature that allow nasin nanpa (and many other sitelen pona fonts) to display strings of existing Unicode characters as sitelen pona glyphs. However, not every text rendering context (web browser, text editing program, etc.) supports this font feature by default, and some may not at all (so see the **AHK Script Guide Section** at the end of this README)!

This table describes both the ligatures in nasin nanpa and the AutoHotKey scripts:
| Codepoint | Latin Character(s) | Resulting Codepoint / Glyph |
| --------- | ------------------ | --------------------------- |
| **U+F1900** -<br>**U+F1988** | `a`, `akesi` ... `wile` \| `namako` ... `ku` | _A_, `AKESI` ... `WILE` \| `NAMAKO` ... `KU` |
| **U+3000** | `  ` / `zz`| `IDEOGRAPHIC SPACE` |
| **U+F1990** | `[` | `START OF CARTOUCHE` |
| **U+F1991** | `]` | `END OF CARTOUCHE` |
| **U+F1992** | `=` | `COMBINING CARTOUCHE EXTENSION` |
| **U+F1993** | (none) | `START OF LONG PI` |
| **U+F1994** | (none) | `COMBINING LONG PI EXTENSION` |
| **U+F1995** | `-` | `STACKING JOINER` |
| **U+F1996** | `+` | `SCALING JOINER` |
| **U+F1997** | `(` | `START OF LONG GLYPH` |
| **U+F1998** | `)` | `END OF LONG GLYPH` |
| **U+F1999** | `_` | `COMBINING LONG GLYPH EXTENSION` |
| **U+F199A** | `{` | `START OF REVERSE LONG GLYPH` |
| **U+F199B** | `}` | `END OF REVERSE LONG GLYPH` |
| **U+F199C** | `.` | `MIDDLE DOT` |
| **U+F199D** | `:` | `COLON` |
| **U+FE00** -<br>**U+FE07** | `1` - `8` | `VARIATION SELECTOR 1` (`VAR01`) - `VARIATION SELECTOR 8` (`VAR08`) |
| **U+200C** | `\|` | `ZERO WIDTH NON JOINER` (`ZWNJ`) |
| **U+200D** | `&` | `ZERO WIDTH JOINER` (`ZWJ`) |
| (none) | `itan` | jan Itan's personal glyph |
| (none) | `lepeka` | jan Lepeka's personal glyph |
| (none) | `lipamanka` | lipamanka's personal glyph |

## AHK Scripts
See [ahk-script/README.md](https://github.com/ETBCOR/nasin-nanpa/tree/main/ahk-script#readme).
