# Changelog

## [0.1.7] - "Upcomming date"

Looking at the flamegraph for profile_target.
In path/string.rs the formatting for string was slow.
Now using trim_end_matches('0') to remove trailing zeros from numbers.

Rendering of SVG path is now faster, rendering to canvas is unaffected.
