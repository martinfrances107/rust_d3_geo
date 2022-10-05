#[macro_use]
extern crate criterion;
extern crate pretty_assertions;

use std::time::Duration;

use criterion::Criterion;
use geo::Coordinate;
use geo::LineString;
use geo::MultiPolygon;
use geo::Polygon;
use lazy_static::lazy_static;
use pretty_assertions::assert_eq;
use regex::Regex;

use rust_d3_geo::circle::generator::Generator as CircleGenerator;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;

lazy_static! {
    /// Ignore every digit in a number after the decimal.
    static ref ROUND_DOWN: Regex = Regex::new(r"\.\d+").unwrap();
}

/// This benchmark is based on examples/rings
///
/// It uses orthographic projection to generated a SVG path for a complex
/// MultiPolygon.
fn rings() {
    let width = 1000_f64;
    let height = 1000_f64;

    let ortho = Orthographic::builder()
        .scale_set(240_f64)
        .translate_set(&Coordinate {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .build();

    let cg_outer = CircleGenerator::default()
        .radius_set(10_f64)
        .precision_set(10_f64);
    let cg_inner = CircleGenerator::default()
        .radius_set(5_f64)
        .precision_set(5_f64);

    let mut p_vec = vec![];
    for lat in (-30..=30).step_by(30) {
        for long in (-180..=180).step_by(40) {
            let mut inner = cg_inner
                .clone()
                .center_set(&Coordinate {
                    x: long as f64,
                    y: lat as f64,
                })
                .circle()
                .exterior()
                .0
                .clone();
            inner.reverse();
            let inner_ring: LineString<f64> = inner.into();

            let poly = Polygon::new(
                cg_outer
                    .clone()
                    .center_set(&Coordinate {
                        x: long as f64,
                        y: lat as f64,
                    })
                    .circle()
                    .exterior()
                    .clone(),
                vec![inner_ring],
            );

            p_vec.push(poly);
        }
    }

    let object = MultiPolygon(p_vec);

    let mut path = PathBuilder::context_pathstring().build(ortho);
    let s = path.object(&object);

    let rounded = ROUND_DOWN.replace_all(&s, "");

    assert_eq!(rounded, String::from("M283,603L284,605L287,611L291,618L294,624L298,630L302,636L304,638L304,638L290,617ZM305,624L301,618L299,611L296,605L295,600L295,594L295,590L296,586L298,584L301,582L304,582L308,582L312,584L317,586L322,590L327,594L331,600L336,605L340,611L343,618L346,624L348,630L349,636L350,641L349,645L348,649L346,652L344,653L340,654L336,653L332,652L327,649L323,645L318,641L313,636L309,630ZM311,621L311,622L312,624L313,625L315,627L316,628L317,629L318,631L319,632L320,633L322,634L323,635L324,635L325,636L326,637L327,637L328,637L329,637L330,637L331,637L332,637L332,636L333,635L333,635L334,634L334,633L334,632L334,631L334,629L334,628L333,627L333,625L333,624L332,622L331,621L331,619L330,617L329,616L328,614L327,613L326,611L325,610L324,609L322,607L321,606L320,605L319,604L318,603L316,603L315,602L314,602L313,601L312,601L311,601L310,601L309,601L309,602L308,602L308,603L307,603L307,604L307,605L306,606L306,607L306,609L307,610L307,611L307,613L308,614L308,616L309,617L310,619ZM392,624L390,618L390,611L390,605L392,600L395,594L399,590L404,586L409,584L416,582L422,582L429,582L436,584L443,586L449,590L455,594L460,600L464,605L467,611L469,618L469,624L469,630L467,636L464,641L460,645L455,649L450,652L443,653L437,654L430,653L423,652L416,649L410,645L404,641L399,636L395,630ZM409,621L410,622L411,624L411,625L412,627L413,628L415,629L416,631L417,632L419,633L420,634L422,635L424,635L425,636L427,637L429,637L431,637L432,637L434,637L436,637L437,637L439,636L440,635L442,635L443,634L444,633L445,632L446,631L447,629L447,628L448,627L448,625L449,624L449,622L449,621L448,619L448,617L447,616L447,614L446,613L445,611L444,610L443,609L441,607L440,606L439,605L437,604L435,603L434,603L432,602L430,602L429,601L427,601L425,601L423,601L422,601L420,602L419,602L417,603L416,603L414,604L413,605L412,606L411,607L411,609L410,610L409,611L409,613L409,614L409,616L409,617L409,619ZM530,624L530,618L532,611L535,605L539,600L544,594L550,590L556,586L563,584L570,582L577,582L583,582L590,584L595,586L600,590L604,594L607,600L609,605L609,611L609,618L607,624L604,630L600,636L595,641L589,645L583,649L576,652L569,653L562,654L556,653L549,652L544,649L539,645L535,641L532,636L530,630ZM550,621L550,622L550,624L551,625L551,627L552,628L552,629L553,631L554,632L555,633L556,634L557,635L559,635L560,636L562,637L563,637L565,637L567,637L568,637L570,637L572,637L574,636L575,635L577,635L579,634L580,633L582,632L583,631L584,629L586,628L587,627L588,625L588,624L589,622L590,621L590,619L590,617L590,616L590,614L590,613L590,611L589,610L588,609L588,607L587,606L586,605L585,604L583,603L582,603L580,602L579,602L577,601L576,601L574,601L572,601L570,601L569,602L567,602L565,603L564,603L562,604L560,605L559,606L558,607L556,609L555,610L554,611L553,613L552,614L552,616L551,617L551,619ZM653,624L656,618L659,611L663,605L668,600L672,594L677,590L682,586L687,584L691,582L695,582L698,582L701,584L703,586L704,590L704,594L704,600L703,605L700,611L698,618L694,624L690,630L686,636L681,641L676,645L672,649L667,652L663,653L659,654L655,653L653,652L651,649L650,645L649,641L650,636L651,630ZM668,621L667,622L666,624L666,625L666,627L665,628L665,629L665,631L665,632L665,633L665,634L666,635L666,635L667,636L667,637L668,637L669,637L670,637L671,637L672,637L673,637L674,636L675,635L676,635L677,634L679,633L680,632L681,631L682,629L683,628L684,627L686,625L687,624L688,622L688,621L689,619L690,617L691,616L691,614L692,613L692,611L692,610L693,609L693,607L693,606L692,605L692,604L692,603L691,603L691,602L690,602L690,601L689,601L688,601L687,601L686,601L685,602L684,602L683,603L681,603L680,604L679,605L678,606L677,607L675,609L674,610L673,611L672,613L671,614L670,616L669,617L668,619ZM695,638L697,636L701,630L705,624L705,624L708,618L712,611L715,605L716,603L716,603L704,625ZM260,499L260,500L260,500ZM274,507L274,500L274,492L275,485L277,479L279,473L281,468L284,463L288,460L291,458L295,458L298,458L302,460L305,463L308,468L311,473L313,479L314,485L315,492L316,499L315,507L314,514L313,520L311,526L308,531L305,536L302,539L298,541L295,541L291,541L288,539L284,536L281,531L279,526L277,520L275,514ZM282,501L282,503L282,505L283,507L283,508L283,510L284,511L284,513L285,514L286,516L286,517L287,518L288,518L289,519L290,520L291,520L292,520L292,520L293,520L294,520L295,520L296,519L297,518L298,518L298,517L299,516L300,514L300,513L301,511L302,510L302,508L302,507L303,505L303,503L303,501L303,499L303,498L303,496L303,494L302,492L302,491L302,489L301,488L300,486L300,485L299,483L298,482L298,481L297,481L296,480L295,479L294,479L293,479L292,479L292,479L291,479L290,479L289,480L288,481L287,481L286,482L286,483L285,485L284,486L284,488L283,489L283,491L283,492L282,494L282,496L282,498L282,500ZM380,507L380,500L380,492L382,485L385,479L389,473L393,468L399,463L405,460L412,458L419,458L425,458L432,460L438,463L444,468L449,473L453,479L455,485L457,492L458,499L457,507L455,514L453,520L449,526L444,531L438,536L432,539L425,541L419,541L412,541L405,539L399,536L393,531L389,526L385,520L382,514ZM398,501L398,503L399,505L399,507L400,508L401,510L402,511L403,513L404,514L405,516L406,517L408,518L409,518L411,519L413,520L414,520L416,520L418,520L419,520L421,520L423,520L424,519L426,518L428,518L429,517L430,516L432,514L433,513L434,511L435,510L436,508L436,507L437,505L437,503L437,501L437,499L437,498L437,496L437,494L436,492L436,491L435,489L434,488L433,486L432,485L430,483L429,482L428,481L426,481L424,480L423,479L421,479L419,479L418,479L416,479L414,479L413,479L411,480L409,481L408,481L406,482L405,483L404,485L403,486L402,488L401,489L400,491L399,492L399,494L398,496L398,498L398,500ZM542,507L541,500L542,492L544,485L546,479L550,473L555,468L561,463L567,460L574,458L580,458L587,458L594,460L600,463L606,468L610,473L614,479L617,485L619,492L620,499L619,507L617,514L614,520L610,526L606,531L600,536L594,539L587,541L580,541L574,541L567,539L561,536L555,531L550,526L546,520L544,514ZM562,501L562,503L562,505L563,507L563,508L564,510L565,511L566,513L567,514L569,516L570,517L571,518L573,518L575,519L576,520L578,520L580,520L581,520L583,520L585,520L586,520L588,519L590,518L591,518L593,517L594,516L595,514L596,513L597,511L598,510L599,508L600,507L600,505L601,503L601,501L601,499L601,498L601,496L600,494L600,492L599,491L598,489L597,488L596,486L595,485L594,483L593,482L591,481L590,481L588,480L586,479L585,479L583,479L581,479L580,479L578,479L576,479L575,480L573,481L571,481L570,482L569,483L567,485L566,486L565,488L564,489L563,491L563,492L562,494L562,496L562,498L562,500ZM684,507L683,500L684,492L685,485L686,479L688,473L691,468L694,463L697,460L701,458L704,458L708,458L711,460L715,463L718,468L720,473L722,479L724,485L725,492L725,499L725,507L724,514L722,520L720,526L718,531L715,536L711,539L708,541L704,541L701,541L697,539L694,536L691,531L688,526L686,520L685,514ZM696,501L696,503L696,505L697,507L697,508L697,510L698,511L699,513L699,514L700,516L701,517L701,518L702,518L703,519L704,520L705,520L706,520L707,520L707,520L708,520L709,520L710,519L711,518L712,518L713,517L713,516L714,514L715,513L715,511L716,510L716,508L716,507L717,505L717,503L717,501L717,499L717,498L717,496L717,494L716,492L716,491L716,489L715,488L715,486L714,485L713,483L713,482L712,481L711,481L710,480L709,479L708,479L707,479L707,479L706,479L705,479L704,479L703,480L702,481L701,481L701,482L700,483L699,485L699,486L698,488L697,489L697,491L697,492L696,494L696,496L696,498L696,500ZM739,500L739,499L739,499ZM304,361L302,363L298,369L294,375L291,381L287,388L284,394L283,396L283,396L295,374ZM299,388L301,381L305,375L309,369L313,363L318,358L323,354L327,350L332,347L336,346L340,345L344,346L346,347L348,350L349,354L350,358L349,363L348,369L346,375L343,381L340,388L336,394L331,399L327,405L322,409L317,413L312,415L308,417L304,417L301,417L298,415L296,413L295,409L295,405L295,399L296,394ZM309,382L308,383L308,385L307,386L307,388L307,389L306,390L306,392L306,393L307,394L307,395L307,396L308,396L308,397L309,397L309,398L310,398L311,398L312,398L313,398L314,397L315,397L316,396L318,396L319,395L320,394L321,393L322,392L324,390L325,389L326,388L327,386L328,385L329,383L330,382L331,380L331,378L332,377L333,375L333,374L333,372L334,371L334,370L334,368L334,367L334,366L334,365L333,364L333,364L332,363L332,362L331,362L330,362L329,362L328,362L327,362L326,362L325,363L324,364L323,364L322,365L320,366L319,367L318,368L317,370L316,371L315,372L313,374L312,375L311,377L311,378L310,380ZM390,388L390,381L392,375L395,369L399,363L404,358L410,354L416,350L423,347L430,346L437,345L443,346L450,347L455,350L460,354L464,358L467,363L469,369L469,375L469,381L467,388L464,394L460,399L455,405L449,409L443,413L436,415L429,417L422,417L416,417L409,415L404,413L399,409L395,405L392,399L390,394ZM409,382L409,383L409,385L409,386L409,388L410,389L411,390L411,392L412,393L413,394L414,395L416,396L417,396L419,397L420,397L422,398L423,398L425,398L427,398L429,398L430,397L432,397L434,396L435,396L437,395L439,394L440,393L441,392L443,390L444,389L445,388L446,386L447,385L447,383L448,382L448,380L449,378L449,377L449,375L448,374L448,372L447,371L447,370L446,368L445,367L444,366L443,365L442,364L440,364L439,363L437,362L436,362L434,362L432,362L431,362L429,362L427,362L425,363L424,364L422,364L420,365L419,366L417,367L416,368L415,370L413,371L412,372L411,374L411,375L410,377L409,378L409,380ZM532,388L530,381L530,375L530,369L532,363L535,358L539,354L544,350L549,347L556,346L562,345L569,346L576,347L583,350L589,354L595,358L600,363L604,369L607,375L609,381L609,388L609,394L607,399L604,405L600,409L595,413L590,415L583,417L577,417L570,417L563,415L556,413L550,409L544,405L539,399L535,394ZM551,382L552,383L552,385L553,386L554,388L555,389L556,390L558,392L559,393L560,394L562,395L564,396L565,396L567,397L569,397L570,398L572,398L574,398L576,398L577,398L579,397L580,397L582,396L583,396L585,395L586,394L587,393L588,392L588,390L589,389L590,388L590,386L590,385L590,383L590,382L590,380L590,378L589,377L588,375L588,374L587,372L586,371L584,370L583,368L582,367L580,366L579,365L577,364L575,364L574,363L572,362L570,362L568,362L567,362L565,362L563,362L562,362L560,363L559,364L557,364L556,365L555,366L554,367L553,368L552,370L552,371L551,372L551,374L550,375L550,377L550,378L551,380ZM659,388L656,381L653,375L651,369L650,363L649,358L650,354L651,350L653,347L655,346L659,345L663,346L667,347L672,350L676,354L681,358L686,363L690,369L694,375L698,381L700,388L703,394L704,399L704,405L704,409L703,413L701,415L698,417L695,417L691,417L687,415L682,413L677,409L672,405L668,399L663,394ZM669,382L670,383L671,385L672,386L673,388L674,389L675,390L677,392L678,393L679,394L680,395L681,396L683,396L684,397L685,397L686,398L687,398L688,398L689,398L690,398L690,397L691,397L691,396L692,396L692,395L692,394L693,393L693,392L693,390L692,389L692,388L692,386L691,385L691,383L690,382L689,380L688,378L688,377L687,375L686,374L684,372L683,371L682,370L681,368L680,367L679,366L677,365L676,364L675,364L674,363L673,362L672,362L671,362L670,362L669,362L668,362L667,362L667,363L666,364L666,364L665,365L665,366L665,367L665,368L665,370L665,371L666,372L666,374L666,375L667,377L668,378L668,380ZM716,396L715,394L712,388L712,388L708,381L705,375L701,369L697,363L695,361L695,361L709,382Z"));
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("ring");

    // Increased the default run time by 3 seconds after gettings warnings that the task was taking too long.
    g.measurement_time(Duration::from_secs(8));

    g.bench_function("rings", |b| b.iter(|| rings()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
