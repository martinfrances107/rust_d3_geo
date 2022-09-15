#[macro_use]
extern crate criterion;
extern crate pretty_assertions;

use std::time::Duration;

use criterion::Criterion;
use geo::{Coordinate, Geometry, LineString, MultiLineString};
use lazy_static::lazy_static;
use pretty_assertions::assert_eq;
use regex::Regex;

use rust_d3_geo::graticule::generate as generate_graticule;
use rust_d3_geo::path::builder::Builder as PathBuilder;
use rust_d3_geo::projection::orthographic::Orthographic;
use rust_d3_geo::projection::Build;
use rust_d3_geo::projection::ProjectionRawBase;
use rust_d3_geo::projection::RotateSet;
use rust_d3_geo::projection::ScaleSet;
use rust_d3_geo::projection::TranslateSet;

lazy_static! {
    /// Ignore every digit in a number after the decimal.
    static ref ROUND_DOWN: Regex = Regex::new(r"\.\d+").unwrap();
}

/// This benchmark is based on examples/graticule
///
/// It uses orthographic projection to generated a SVG path
/// for a complex MultiLineString
fn graticule() {
    let width = 1000_f64;
    let height = 1000_f64;
    let center = Coordinate {
        x: width / 2_f64,
        y: height / 2_f64,
    };

    let ortho = Orthographic::builder()
        .scale_set(240_f64)
        .translate_set(&center)
        .rotate_set(&[0_f64, -20_f64, 0_f64])
        .build();

    let mut pb = PathBuilder::context_pathstring().build(ortho);

    let lines: Vec<LineString<f64>> = generate_graticule::<f64>().lines().collect();

    let mls = Geometry::MultiLineString(MultiLineString(lines));

    let s = pb.object(&mls);

    let rounded = ROUND_DOWN.replace_all(&s, "");

    assert_eq!(rounded, String::from("M500,260L500,274M260,500L261,477L264,456L270,434L278,413L288,393L300,374L314,356L330,340L347,325L366,312L386,301L408,291L430,284L453,278L476,275L499,274M500,740L500,728L500,696L500,646L500,582L500,489L500,398L500,323L500,274M739,500L738,477L735,456L729,434L721,413L711,393L699,374L685,356L669,340L652,325L633,312L613,301L591,291L569,284L546,278L523,275L500,274M260,499L260,503L260,507L262,510L263,514L265,517L268,521L271,524L274,528L278,531L282,534L287,537L292,541L297,544L303,547L309,549L316,552L323,555L330,558L337,560L345,562L353,565L362,567L371,569L380,571L389,572L398,574L408,575L417,577L427,578L437,579L448,580L458,580L468,581L479,581L489,582L500,582L510,582L520,581L531,581L541,580L551,580L562,579L572,578L582,577L591,575L601,574L610,572L620,571L628,569L637,567L646,565L654,562L662,560L669,558L676,555L683,552L690,549L696,547L702,544L707,541L712,537L717,534L721,531L725,528L728,524L731,521L734,517L736,514L737,510L739,507L739,503L740,500L740,499M485,260L489,261L492,263M470,261L477,262L485,264M453,264L466,263L479,265M433,269L453,266L473,266M409,277L423,273L437,269L452,268L468,268M377,293L397,283L418,276L440,272L463,270M335,325L348,314L362,304L377,295L392,288L409,282L425,277L443,274L460,273M286,390L300,367L316,346L335,328L356,311L380,298L405,287L431,279L458,275M286,609L279,594L273,578L268,561L265,544L264,527L263,509L267,474L276,441L290,408L309,378L332,350L360,326L390,306L424,291L458,280M335,674L318,656L304,636L292,614L283,591L277,566L274,541L274,514L277,488L286,454L299,420L318,389L340,360L366,334L395,312L427,295L460,282M377,706L355,690L336,671L319,648L307,622L298,594L293,564L292,533L295,501L303,466L315,432L332,399L353,369L377,342L404,318L433,299L463,285M409,722L385,709L365,692L347,669L333,643L323,614L317,582L316,548L318,512L325,477L336,442L351,409L369,377L391,349L415,324L440,303L468,287M433,730L422,726L412,721L402,714L392,705L376,684L362,659L353,629L347,596L345,560L348,522L353,487L363,451L375,417L390,384L408,355L428,328L450,306L473,288M453,735L444,732L435,728L427,722L419,714L405,695L394,670L386,640L381,606L380,569L381,530L386,494L393,458L403,423L415,390L429,360L444,332L461,309L479,290M470,738L463,736L457,733L451,727L446,720L436,702L428,677L422,648L419,614L417,576L419,536L422,500L427,464L433,428L441,395L451,363L462,335L473,311L485,291M485,739L482,738L479,735L473,724L467,706L463,682L460,652L458,618L458,540L463,467L470,397L480,337L486,312L492,291M514,739L517,738L520,735L526,724L532,706L536,682L539,652L541,618L541,540L536,467L529,397L519,337L513,312L507,291M529,738L536,736L542,733L548,727L553,720L563,702L571,677L577,648L580,614L582,576L580,536L577,500L572,464L566,428L558,395L548,363L537,335L526,311L514,291M546,735L555,732L564,728L572,722L580,714L594,695L605,670L613,640L618,606L619,569L618,530L613,494L606,458L596,423L584,390L570,360L555,332L538,309L520,290M566,730L577,726L587,721L597,714L607,705L623,684L637,659L646,629L652,596L654,560L651,522L646,487L636,451L624,417L609,384L591,355L571,328L549,306L526,288M590,722L614,709L634,692L652,669L666,643L676,614L682,582L683,548L681,512L674,477L663,442L648,409L630,377L608,349L584,324L559,303L531,287M622,706L644,690L663,671L680,648L692,622L701,594L706,564L707,533L704,501L696,466L684,432L667,399L646,369L622,342L595,318L566,299L536,285M664,674L681,656L695,636L707,614L716,591L722,566L725,541L725,514L722,488L713,454L700,420L681,389L659,360L633,334L604,312L572,295L539,282M713,609L720,594L726,578L731,561L734,544L735,527L736,509L732,474L723,441L709,408L690,378L667,350L639,326L609,306L575,291L541,280M713,390L699,367L683,346L664,328L643,311L619,298L594,287L568,279L541,275M664,325L651,314L637,304L622,295L607,288L590,282L574,277L556,274L539,273M622,293L602,283L581,276L559,272L536,270M590,277L576,273L562,269L547,268L531,268M566,269L546,266L526,266M546,264L533,263L520,265M529,261L522,262L514,264M514,260L510,261L507,263M499,739L500,740L500,739M406,721L408,721L411,723L415,724L418,725L422,726L426,727L431,728L435,729L440,730L444,731L449,732L454,733L458,733L463,734L468,734L474,735L479,735L484,736L489,736L494,736L500,736L505,736L510,736L515,736L520,735L525,735L531,734L536,734L541,733L545,733L550,732L555,731L560,730L564,729L568,728L573,727L577,726L581,725L584,724L588,723L591,721L593,721M361,695L363,697L366,699L369,701L373,703L377,704L381,706L386,708L390,710L395,711L400,713L406,714L411,715L417,717L422,718L428,719L434,720L440,721L447,722L453,723L460,723L466,724L473,724L479,725L486,725L493,725L500,725L506,725L513,725L520,725L526,724L533,724L539,723L546,723L552,722L559,721L565,720L571,719L577,718L582,717L588,715L593,714L599,713L604,711L609,710L613,708L618,706L622,704L626,703L630,701L633,699L636,697L638,695M324,664L327,666L330,669L333,671L336,674L340,676L344,678L349,681L354,683L359,685L364,687L369,689L375,691L381,693L388,694L394,696L401,697L408,699L415,700L422,701L429,703L437,704L444,704L452,705L460,706L468,706L476,707L483,707L491,707L500,707L508,707L516,707L523,707L531,706L539,706L547,705L555,704L562,704L570,703L577,701L584,700L591,699L598,697L605,696L611,694L618,693L624,691L630,689L635,687L640,685L645,683L650,681L655,678L659,676L663,674L666,671L669,669L672,666L675,664M296,627L297,628L299,631L301,634L304,637L307,639L311,642L315,645L320,648L324,650L329,653L335,656L340,658L346,660L353,663L359,665L366,667L373,669L380,670L388,672L396,674L404,675L412,677L420,678L428,679L437,680L446,681L455,682L463,682L472,683L481,683L490,683L500,683L509,683L518,683L527,683L536,682L544,682L553,681L562,680L571,679L579,678L587,677L595,675L603,674L611,672L619,670L626,669L633,667L640,665L646,663L653,660L659,658L664,656L670,653L675,650L680,648L684,645L688,642L692,639L695,637L698,634L700,631L702,628L703,627M276,587L277,590L279,593L282,597L284,600L288,603L291,606L295,609L299,612L304,615L309,618L315,621L321,624L327,626L333,629L340,631L347,634L355,636L362,638L370,640L378,642L387,643L395,645L404,647L413,648L422,649L432,650L441,651L451,652L460,653L470,653L480,653L490,654L500,654L509,654L519,653L529,653L539,653L548,652L558,651L567,650L577,649L586,648L595,647L604,645L612,643L621,642L629,640L637,638L644,636L652,634L659,631L666,629L672,626L678,624L684,621L690,618L695,615L700,612L704,609L708,606L711,603L715,600L717,597L720,593L722,590L723,587M264,544L264,546L265,549L267,553L269,556L271,560L274,563L277,566L281,570L285,573L290,576L295,579L300,582L306,585L312,588L318,591L325,593L332,596L340,598L348,601L356,603L364,605L373,607L381,609L390,610L400,612L409,613L419,615L428,616L438,617L448,618L458,618L469,619L479,619L489,619L500,620L510,619L520,619L530,619L541,618L551,618L561,617L571,616L580,615L590,613L599,612L609,610L618,609L626,607L635,605L643,603L651,601L659,598L667,596L674,593L681,591L687,588L693,585L699,582L704,579L709,576L714,573L718,570L722,566L725,563L728,560L730,556L732,553L734,549L735,546L735,544M264,455L263,457L263,460L263,464L264,467L265,471L267,474L269,478L271,481L274,485L277,488L281,491L285,495L290,498L295,501L300,504L306,507L312,510L318,512L325,515L332,517L340,520L348,522L356,524L364,527L373,529L381,530L390,532L400,534L409,535L419,536L428,537L438,538L448,539L458,540L469,540L479,541L489,541L500,541L510,541L520,541L530,540L541,540L551,539L561,538L571,537L580,536L590,535L599,534L609,532L618,530L626,529L635,527L643,524L651,522L659,520L667,517L674,515L681,512L687,510L693,507L699,504L704,501L709,498L714,495L718,491L722,488L725,485L728,481L730,478L732,474L734,471L735,467L736,464L736,460L736,457L735,455M276,412L276,412L275,416L274,419L274,422L274,426L275,429L276,432L277,436L279,439L282,442L284,446L288,449L291,452L295,455L299,458L304,461L309,464L315,467L321,469L327,472L333,474L340,477L347,479L355,481L362,484L370,486L378,487L387,489L395,491L404,492L413,494L422,495L432,496L441,497L451,498L460,498L470,499L480,499L490,499L500,500L509,499L519,499L529,499L539,498L548,498L558,497L567,496L577,495L586,494L595,492L604,491L612,489L621,487L629,486L637,484L644,481L652,479L659,477L666,474L672,472L678,469L684,467L690,464L695,461L700,458L704,455L708,452L711,449L715,446L717,442L720,439L722,436L723,432L724,429L725,426L725,422L725,419L724,416L723,412L723,412M296,372L295,374L293,377L292,381L292,384L292,387L292,390L292,393L293,396L295,399L297,402L299,405L301,408L304,411L307,414L311,417L315,420L320,422L324,425L329,428L335,430L340,432L346,435L353,437L359,439L366,441L373,443L380,445L388,447L396,448L404,450L412,451L420,452L428,454L437,455L446,455L455,456L463,457L472,457L481,458L490,458L500,458L509,458L518,458L527,457L536,457L544,456L553,455L562,455L571,454L579,452L587,451L595,450L603,448L611,447L619,445L626,443L633,441L640,439L646,437L653,435L659,432L664,430L670,428L675,425L680,422L684,420L688,417L692,414L695,411L698,408L700,405L702,402L704,399L706,396L707,393L707,390L707,387L707,384L707,381L706,377L704,374L703,372M324,335L324,336L322,338L320,341L318,344L317,346L316,349L316,352L316,355L316,357L316,360L317,363L318,365L320,368L322,371L324,373L327,376L330,379L333,381L336,384L340,386L344,388L349,391L354,393L359,395L364,397L369,399L375,401L381,403L388,404L394,406L401,408L408,409L415,410L422,412L429,413L437,414L444,415L452,415L460,416L468,416L476,417L483,417L491,417L500,417L508,417L516,417L523,417L531,416L539,416L547,415L555,415L562,414L570,413L577,412L584,410L591,409L598,408L605,406L611,404L618,403L624,401L630,399L635,397L640,395L645,393L650,391L655,388L659,386L663,384L666,381L669,379L672,376L675,373L677,371L679,368L681,365L682,363L683,360L683,357L683,355L683,352L683,349L682,346L681,344L679,341L677,338L675,336L675,335M361,304L360,304L357,307L355,309L352,311L350,313L349,315L348,318L347,320L346,322L345,324L345,327L345,329L346,331L347,334L348,336L349,338L350,340L352,343L355,345L357,347L360,349L363,351L366,353L369,355L373,357L377,359L381,361L386,362L390,364L395,366L400,367L406,369L411,370L417,371L422,372L428,374L434,375L440,375L447,376L453,377L460,378L466,378L473,379L479,379L486,379L493,379L500,380L506,379L513,379L520,379L526,379L533,378L539,378L546,377L552,376L559,375L565,375L571,374L577,372L582,371L588,370L593,369L599,367L604,366L609,364L613,362L618,361L622,359L626,357L630,355L633,353L636,351L639,349L642,347L644,345L647,343L649,340L650,338L651,336L652,334L653,331L654,329L654,327L654,324L653,322L652,320L651,318L650,315L649,313L647,311L644,309L642,307L639,304L638,304M406,278L404,279L401,281L398,282L396,284L393,285L391,287L389,288L387,290L385,292L384,294L382,295L381,297L381,299L380,301L380,302L380,304L380,306L380,308L381,310L381,311L382,313L384,315L385,317L387,318L389,320L391,322L393,323L396,325L398,326L401,328L404,329L408,331L411,332L415,333L418,334L422,336L426,337L431,338L435,339L440,340L444,341L449,341L454,342L458,343L463,343L468,344L474,344L479,345L484,345L489,345L494,345L500,345L505,345L510,345L515,345L520,345L525,344L531,344L536,343L541,343L545,342L550,341L555,341L560,340L564,339L568,338L573,337L577,336L581,334L584,333L588,332L591,331L595,329L598,328L601,326L603,325L606,323L608,322L610,320L612,318L614,317L615,315L617,313L618,311L618,310L619,308L619,306L620,304L619,302L619,301L618,299L618,297L617,295L615,294L614,292L612,290L610,288L608,287L606,285L603,284L601,282L598,281L595,279L593,278M500,260L496,260L492,260L489,260L485,260L482,260L478,260L475,261L471,261L468,262L465,262L462,263L458,263L455,264L452,265L450,265L447,266L444,267L441,268L439,269L437,270L434,270L432,271L430,272L428,274L427,275L425,276L424,277L422,278L421,279L420,280L419,281L419,283L418,284L418,285L417,286L417,288L417,289L418,290L418,291L419,292L419,294L420,295L421,296L422,297L424,298L425,299L427,301L428,302L430,303L432,304L434,305L437,306L439,307L441,307L444,308L447,309L450,310L452,311L455,311L458,312L462,312L465,313L468,314L471,314L475,314L478,315L482,315L485,315L489,315L492,316L496,316L500,316L503,316L507,316L510,315L514,315L517,315L521,315L524,314L528,314L531,314L534,313L537,312L541,312L544,311L547,311L549,310L552,309L555,308L558,307L560,307L562,306L565,305L567,304L569,303L571,302L572,301L574,299L575,298L577,297L578,296L579,295L580,294L580,292L581,291L581,290L582,289L582,288L582,286L581,285L581,284L580,283L580,281L579,280L578,279L577,278L575,277L574,276L572,275L571,274L569,272L567,271L565,270L562,270L560,269L558,268L555,267L552,266L549,265L547,265L544,264L541,263L537,263L534,262L531,262L528,261L524,261L521,260L517,260L514,260L510,260L507,260L503,260L500,260M500,263L498,263L496,263L494,263L492,263L490,263L489,264L487,264L485,264L484,264L482,264L480,265L479,265L477,265L476,266L474,266L473,266L471,267L470,267L469,268L468,268L466,269L465,269L464,270L463,270L463,271L462,271L461,272L460,273L460,273L459,274L459,274L458,275L458,276L458,276L458,277L458,277L458,278L458,279L458,279L458,280L459,280L459,281L460,282L460,282L461,283L462,283L463,284L463,285L464,285L465,286L466,286L468,287L469,287L470,287L471,288L473,288L474,289L476,289L477,289L479,290L480,290L482,290L484,291L485,291L487,291L489,291L490,291L492,291L494,292L496,292L498,292L500,292L501,292L503,292L505,292L507,291L509,291L510,291L512,291L514,291L515,291L517,290L519,290L520,290L522,289L523,289L525,289L526,288L528,288L529,287L530,287L531,287L533,286L534,286L535,285L536,285L536,284L537,283L538,283L539,282L539,282L540,281L540,280L541,280L541,279L541,279L541,278L541,277L541,277L541,276L541,276L541,275L540,274L540,274L539,273L539,273L538,272L537,271L536,271L536,270L535,270L534,269L533,269L531,268L530,268L529,267L528,267L526,266L525,266L523,266L522,265L520,265L519,265L517,264L515,264L514,264L512,264L510,264L509,263L507,263L505,263L503,263L501,263L500,263"));
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("graticule");

    // Increased the default run time by 3 seconds after gettings warnings that the task was taking too long.
    g.measurement_time(Duration::from_secs(8));
    g.bench_function("graticule", |b| b.iter(|| graticule()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
