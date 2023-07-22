# Changelog


## [2.0.0] - 22 July 2023

A summary of breaking changes.

 * ClipAngleGet - The method clip_angle() can no longer be called when using the
    ClipAntemeridian strategy.

 * Breaking change related to a misspelt namespaces - no change in behavior

commit fd17b1bb61b85a2af11ba5fb81591303cf582e93
Author: Martin <martinfrances107@hotmail.com>
Date:   Tue Jul 11 15:41:05 2023 +0100

    Breaking change related to misspelt namespaces

    in general commom becomes common.

    ```rustlang
    -use crate::projection::projector_commom::
    +use crate::projection::projector_common::
    ```

commit e043a1c8ae75f8e5ef54ac820ec64ddb5f2371fa
Author: Martin <martinfrances107@hotmail.com>
Date:   Wed Jun 7 20:31:43 2023 +0100

    Restrict clip_angle_get to the ClipCircle stratergy.

## [0.10.0] - Sun 28 May 2023

Removed PV as generic type

commit 991faec4a4c3116cac9ecd6db3ce0a1fd6963c0b
Author: Martin <martinfrances107@hotmail.com>
Date:   Fri May 26 22:41:00 2023 +0100

    Simplification: related to PointVisible.

    Removed two module

    lib/src/clip/antimeridian/pv.rs
    lib/src/clip/circle/pv.rs

    Clipper is no loner generic over PV.

    -pub struct Clipper<I, LU, PV, RC, STATE, T>
    +pub struct Clipper<I, LU, RC, STATE, T>

    The trait PointVisible  is now implmented on Line.


## [0.9.0] - Wed 24 May 2023

MAJOR: Lots of function calls removed from the public interface

commit 92b3a5700e591d411ed60c001e595468d6c5c0cf
Author: Martin <martinfrances107@hotmail.com>
Date:   Wed May 24 08:07:16 2023 +0100

    breaking change enum REFLECT becomes Reflect.


## [0.8.0] - Sat 20 May 2023

commit 32f8ab99295d48a28a5ff82a7c1a591ac9e16a6e
Author: Martin <martinfrances107@hotmail.com>
Date:   Fri May 19 07:40:11 2023 +0100

    Simplification: path/Builder is not longer generic over PCNC, PCNU

    -pub struct Builder<CS, PCNC, PCNU, T>
    +pub struct Builder<CS, T>

commit 02019af6517282e7a94ee3da104d69c056c50b56
Author: Martin <martinfrances107@hotmail.com>
Date:   Thu May 18 12:19:14 2023 +0100

    Major Refactor: Builder is no longer generic over DRAIN

    -pub struct Builder<CLIPC, CLIPU, DRAIN, PCNU, PR, RU, T>
    +pub struct Builder<CLIPC, CLIPU, PCNU, PR, RU, T>

## [0.7.0] - Wed 17th May 2023

Got a performance boost by writing data to Path2d and
the call stroke_with_path().

commit 3afbe268de6094aea118b3f8e44f7e51090d3cd7
Author: Martin <martinfrances107@hotmail.com>
Date:   Mon May 15 09:52:36 2023 +0100

Now writing to Path2d rather the CanvasRenderingContext2d

This broke draw_with_zoom, which has never worked flawlessly.

## [0.6.0] - Sat 13th May 2023

commit 8dec90f81398356ab53a25168e58eae54b4a7222
Author: Martin <martinfrances107@hotmail.com>
Date:   Wed May 10 12:54:39 2023 +0100

    Performance: examples/rotating_50m - Reduced render time by 12%

commit ab616b395c84ed0705e04732183dce89031387f0
Author: Martin <martinfrances107@hotmail.com>
Date:   Thu May 11 12:39:36 2023 +0100

    Major Refactor: All conventional PR are no longer GENERIC over DRAIN.

## [0.5.0] - Tuesday 9 May 2023

   Simplificiation:
     Removed the concept of MultiTransformer, Multiplex.
     used to render AlbersUSA

## [0.4.0] - Monday 27th March 2023

commit e6c8045f7fb989357dbfa00340dbf3f52be0c430
Author: Martin <martinfrances107@hotmail.com>
Date:   Sun Mar 26 08:32:17 2023 +0100

    Simplified PathBuilder, so I could drop the frequent over constraints such as....

    -    let builder: PathBuilder<_, _, _, NoPCNC<PathString<f64>>, _, _, _, _, _> =
    -        PathBuilder::context_pathstring();
    +    let builder = PathBuilder::context_pathstring();

commit 5a182759bd268e94c92227fad157e987c92e23fa
Author: Martin <martinfrances107@hotmail.com>
Date:   Wed Mar 22 14:48:16 2023 +0000

    dropped CC from  Projector<CC, DRAIN, MULTIPLEX>.

## [0.3.1] - Friday 10th March 2023

Bugfixes to two projections, albers, equidistant
Removed duplicate, and unused projection  conic_equal_area_raw.
Made builder_conic trait implementations more generic.
Bumped crate "geo" from 0.23

[0.3.0] BREAKING CHANGE

builder_conic/Builder<BASE, PRConic, T>

simplified to

builder_conic/Builder<BASE, T>

## [0.3.0] - Sat 4th March 2023

Simplified private traits RecenterWithResampling and RecenterWithResampling,
both are replaced by a single trait Recenter.

Removed update_pr() as it was identical to recenter()

## [0.2.0] BREAKING CHANGE in the following functions

gen_clip_circle()
gen_clip()
generate_mls()
StreamTransformRadians::connect()

## [0.2.0] - 2Sat 18th Feb 2023

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
