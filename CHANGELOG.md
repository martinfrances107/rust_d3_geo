# Changelog

## BREAKING CHANGE in the following functions

gen_clip_circle()
gen_clip()
generate_mls()
StreamTransformRadians::connect()

## [0.2.0] - "Sat 18th Feb"

commit 5533c7d84ac14527627c244360a103fd49dffcbe
Date:   Wed Feb 15 12:55:41 2023 +0000

BREAKING CHANGES
Cargo clippy got better at checking for unused generics.

commit 5770c6f6e9bd7bb66720cf48e6c6c3e4e702da96
Sun Feb 12 12:16:06 2023 +0000

Cargo machete. Removed 'futures' as a dependency.

commit 7130f29105919266fadb5cb347d73cd8a45e55d8
Fri Feb 10 11:22:58 2023 +0000

Looking at the flamegraph for profile_target.
In path/string.rs the formatting for string was slow.
Now using trim_end_matches('0') to remove trailing zeros from numbers.

Rendering of SVG path is now faster, rendering to canvas is unaffected.
