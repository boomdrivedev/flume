# The no-Markdown requirements cite record 0074

## Why

The bolt `bolt/flumes-checks-read-no-markdown` wrote record 0071, flume's
tasks read no Markdown, and cited it from the-gate's "The root project
checks what spans the tree and reads no Markdown" and herdr-plugin's
"The popup's key is the entry lib/common.sh gives". main landed record
0071, a submission whose landing the forge refused, first, then 0072 and
0073. Nothing fails two records of one number now, so the bolt's record
is renumbered by its writer to 0074, the next free, and the two
requirements still cite 0071, which now names the forge's refusal
(core.19, core.21).

## What Changes

- **The-gate's requirement cites record 0074.** "The root project checks
  what spans the tree and reads no Markdown" ends `(core.21; record
  0074)`; its words and its scenario are as they were.
- **Herdr-plugin's requirement cites record 0074.** "The popup's key is
  the entry lib/common.sh gives" ends `(C.5; core.21, core.29; record
  0074)`; its words and its scenarios are as they were.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: "The root project checks what spans the tree and reads no
  Markdown" cites record 0074.
- `herdr-plugin`: "The popup's key is the entry lib/common.sh gives"
  cites record 0074.
