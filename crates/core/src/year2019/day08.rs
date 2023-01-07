use crate::common::character_recognition::{recognize, CHAR_HEIGHT, CHAR_WIDTH};
use crate::input::Input;

const NUM_LETTERS: usize = 5;
const PIXELS_WIDE: usize = NUM_LETTERS * CHAR_WIDTH;
const LAYER_SIZE: usize = PIXELS_WIDE * CHAR_HEIGHT;

pub fn solve(input: &Input) -> Result<String, String> {
    if input.text.len() % LAYER_SIZE != 0 {
        return Err(format!(
            "Invalid input - expected to be multiple of layer size ({})",
            LAYER_SIZE
        ));
    }

    if input.is_part_one() {
        fn count(slice: &[u8], needle: u8) -> usize {
            slice
                .iter()
                .fold(0, |acc, &b| acc + usize::from(b == needle))
        }

        input
            .text
            .as_bytes()
            .chunks(LAYER_SIZE)
            .map(|layer| (layer, count(layer, b'0')))
            .min_by_key(|(_, num_zeros)| *num_zeros)
            .map(|(layer, _)| count(layer, b'1') * count(layer, b'2'))
            .map(|value| value.to_string())
            .ok_or_else(|| "Internal error: No layer".to_string())
    } else {
        let mut image = vec![b'2'; LAYER_SIZE];

        input.text.as_bytes().chunks(LAYER_SIZE).for_each(|layer| {
            image
                .iter_mut()
                .zip(layer.iter())
                .for_each(|(image_pixel, &layer_pixel)| {
                    if *image_pixel == b'2' {
                        *image_pixel = layer_pixel;
                    }
                });
        });

        let image_bytes = image.iter_mut().map(|b| *b == b'1').collect::<Vec<_>>();
        recognize(&image_bytes)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_two!("021222211201202220222222222222222222221022222222122222222222122222222222222222021222222022202121222220222222022222220212222222222222222221222212222122021222212212222221222222222222222222220222222222022222222222122222222222222222021222222222202120222220222222122222222212222222222222222211222222222022022222210211222220222222222222220222220222222222122222222222122222222222222222022222222222202020222220222222122222222222222222222022222212222202222022221222222222212222222222222222221222021222222222122222222222022222222222222222020222222122212220222221222222021222220222222222222222222202222212222222021222222211212222222222222222220222020022222222022222222222022222222222222222221222222122202221222220222222020222221222222222222022222222222212222022122222212200212220222222222222220222222222222222022222222222022222222222222222021222222122202121222222222222022222221202222222222022222222222202222022122222220221212222222222222222221222222022222222222222222222122222222222222122221222222222222221222222222222221222220212222222222022222200222202222022121222220200212222222222222222222222221122222222022222222222222222222222222222020222222222212122222220222222220222221202222222222222220202222212222022021222222201202221222222220222222222120222222222022222222222122222222222222222022222222222222021222222222222220222220212222222222122220212222222222222220222200211222222222222222222221222220222222222022222222222022222222222222222220222222022202120222222222222121222221202202222222222222221222202222022022222201220212222222222222222221222022022222222122222222222022222222222222122122222222222212222222220222222120222222212212222222122221211222202222222121222211202212222222222222222221222120022222222022222222222022222222222222122221222222122202220222220222222221222222202202222222122202222222202222222220222201220202222222222220222222222121122222222122222222222222222222222222122222222222122202022222222222222220222220202202222222222212220222222222122222222210211222222222222220222221222121122222222022222222222122222222222222122222222222222202221222221222222221222221212202222222222221220222212222022022222211202202221222222220222222222122122222222222222222222222221222222222122221222222022202221222220222222020222221222202222222222202212222212222122220222221201222222222220220222221222021222222222222222222222122220222222222022021222222222222222222212222222021222220212202222222022211210222202222122121222221220212222222222222222221222020122222222122222222222022220222222220022120222222022212220222200222222121222220202202222222122210201222202222122021222211200202221222222222222222222121122222222022222222222022221222222222122220222222222222021222201222222220222221212222222222222222221222222222022221222222201222221222221220022222222220222222222022222222222222221222222220122120222222122222221222210222222020222221222212222222222200221222212222022121222222210222221220222221222222222121122222222222222222222222222222222221022021222222122222221222201222222122222222212212222222022210212222202222022221222212200202222220222222022222222221222222222222222222222222222222222220122121222222122202022222210222222221222222202212222222022221211222212222222221222210201212221222221222022221222122022222222222222222222122221222222221222122222222222222022222202222222020222220202222222222122202200222202222022022222202220222221221221222122222222020222222222222222222222122222222222220122021022222022222222222212222222121222221222222222222122210210222202222022222222201200212220220222221222221222120222222222122222222222122222222222222022220022222122202120222211222222220222222202212222222022210210222222222022220222220201212222222222222022220222222022222222022222222222022222222222222022020122222022202021222200222222120222222222212222222022211200222202222022221222210221222221220221220122222222220022222222222222222222122220222222220122020222222222202021222221222222121222220212202222222222202202222222222112020222221210212222222221220022222222221222022222222222222222022222222222220022020022222122212222222221222222121222220202222222222222201220222222222012022222211211222222222220222022221222121122222222022222222222022222222222222222021122222122202020222200222222122222220222212222222220221211222222222222022222202221202220220220222122221222021022222222022222221222022221222222221222021122222222222021222211222222021222221222222122222221210200222222222002021222200221202222222222221022221222022122022222022222222222122220222222220022120022222122212200222201222222122222220212222122222120220220222212222202221222222201202222220220222122221222220222022222222222221222222221222222220022021122222122202001222210222122122222221222202122222221220222222202222212021222222220212220221221220122222222221222122222122222222222222222222222221222022022022222202122222202222022120222222212202122222122201200222202222202121022201220202220222221221122221222120222122222022222220222122220222222220022022122222222212221222212222222120222221222222122222221221200222222222022022022212200222220220220221122222222022222222222122222222222222222222222222222020022122022202000222201222022122222220202212122222020202221222212222112022122211202212221222221220122220222022222022222122222222222122222222222220122021222022122202112212222222222020222021212202222222220221211222222222012220222210200212220222220220022222222021022222220002222021222122220222022220222120022222022222210002221222122020222222212202122222120211222222222222012120222201221202220222222221122222222121022122222002222020222222222222022220022221222222022212111102221222022121221022212202122222220211221222220222122120122200222222222222222220222221222120022022221122222221222022222222022221222021122122222202001222220222022220220121212222122222020222211222202222202021022222211202220221220222022222222222222222222122222221222122220222222220022122222222122222101002220222022122220122212222222222020200220222211202102020022222212202220221220221122222222220022122220202222121222022220222222222122221122122122222212212200222022222220120212212020222121200212222201212112021022212202222222220221221222222222121222022220202222122222122022222022221022120022022022222010102202222222222221120212202021222122211221222202202202021222221221212221220222220022221202121022022221012022120222222022222222220122222222122222202112122202222222220222120202222021222222212221222212212112020022221211212222221221221022221222020022122220122222120222122122222222221022120222122222222221122201222022022221120202202220222120210210222211212122022022212001222222201220222122221212222022222220212022121222022121222122220022220022022022212012102200222022121222220202212122222222222210222202222012120022220212222221221221222122221212221122022222212122121222122122222222221122122022022022222222222202222122220222221202212020222120222222222200222112022222212212202222210221222222220212222122222222022122021222222222222222220122120222022222212102202201222222220222120212222021222122221202222222212212122122210112212220212220221122221222022122122222112222221222022122222222220122020122222022212212222212222122221221022202222221222022221222222202202012022122100221222220220221220222220212121222122220102022221222022121222022222022222022222022212112002202222022122221121202222222222021211211222201202122220222001212202220200221220022221202222122022222222122022222222021222222221022121122122122212212102222222222021222021212222121222121202210222211212122122222022120202222211221221122201212022122022221112022220222222121222022222222120122222022212021222222222222022222022212212020222021200221222210222102022122201100212220212220221022200201022122222220212022020222022222222122221222120022122022212100202210222022121222120202212022222222200221222220212202122122100201222221220221221122202211222222022220222222022222022020222222220222220100122222212202112222222222221220122222222222222221210202222201212202021222100120202221210220220022210200120022022221002122020222122021222122221222221122122122222222112201222022222222022212212120222220210222222202202222021022112011222220220221222122210200221022122220122222122222222020222222221022222202022222222221212212222122221220120222212020222122220201222220222102121202210112212120210220221022210211022222012220012222222222222222222222222122121020022222212020202210222122122222221202202221222221210222222211222112120012111221222020212221222022212222021222222220202122121222022121222122220122022202022222212112102200222222221221021212222222222022221221222201202222022202000202202220211222221020222202120222022222202022221222122021222022220022020220022222202022012220222122120222020222202220222122222210222220212002121222200100202121220221221120220200020022122221212022221222222220222122220122220100222222222200112220222022022220112222212021222122221200222202212212121022120021222020212220222120212001121222202220022022222222022122222022221022221002222220202220012200222022121221201222222220122020202211222222202112021022112100202120222222222220221210220222012222012222222222022122222222222022222002122220212102202220222022122222211212202120122221222202222221221002020202011010222122202220221122222001122122012220202022122222022121222122220122120011022020222102222200222122121222000212202220122220220221222201201012120002221022202122220221220220211112121222112221002022220222022221222222222222221202022121212102002202222222122222012202102122222121212212222220201012122222110002222020210222220021210101020022102222022222121222102122221222221222022000022020202112202221222222222222120202102021222122202211222212201202222012211101202120220220220022211101022022122220022022220222122120222222202122122222222220212012022202212122121220121222012121122221210211222221211102120012221111212120211220221120221101121222010220002122021222202022220122010022120120022020202102202220202222222220111222002220222120221202222200200102121212021000202022200220220020222121220122102221212122222222022220221120121122020010222222202020112210202122222220100202222220222220211211202201201222220012222020112120220222220120202221021022102221002122121222122022222022002122222110122020212010122210222222022222010212222021222221220222222211212122020121010000022122202222220222222000121122201221202122120222022020220121001022120100022120202112202210202022120220212202002020022221200201202220222102120221200021122000221221220121211211020022211220112122020222010121221021210022121210222220222001102222222022221222200202212221022210220212212200212012221112110102002112211221222222222001121022102221212022220202221121221022211122220201122122222022002222212022122222100202122222122020220221222211010002021021101100212210221221222221212202020222112122002022022212002122221022020222122112022021212000222202212122021220200222102121122101210212222212220122021202011021022022221221221010201211020022111022002122121222102120220121101022222101022022202020202210202022022222122222022022022022200200212202002222120200011110002000222222221010220121000122000120122022120222122122222022001222222110122221212011212202212222121021022222102220122000211202202202010002120001100021212221211222221102222200001122021221222022022212221121222021002022020200022222202112002202212222022021120212202222022200201212202210212200221021201001122020210221220000221021211122101022202122120222012122221022202022220122122122222000012202220022221021200212102021022002210211202202112101120122021022202112222222222122220111200122100220122022021222111120220222222122122111122122212121122222222222120022211202122121222111202210222211020001222211102002212110212220220111222222111122002121122222021202212222021121120122221112122222212221002200200222220020110202202221022002202212202200111200120222110200222202221220222221210220112222001021222222020202202022020020011222121001222222212202022212200222022122101202202021222021202222202210102222121122002112222010211222212220210111201022212121201222221222212020220121112122120220122222212000002221220222020021212222112220122022222221222222121220221200000102212012202220220202212111010222200121210022021212022221022222010122022200122021222111002220220222020020220002212020222001220222222211120212122222021000002100220221212120212212210022022121011222022212122022022021122022022010222021222222202201220222120121122202002120022220211221212220222212120000100101012020220221211011200021222012102220021222021212001020020020211122122100122220212022102222200122121020001022122121022102201200222220210222020001020010002111210122210001200011120112121021101122222222020020122122111102220221222121222201202212202222020120201022222222222011220202012212202221021121021120002102200221212000210111001022122020201222220202001020120120201222022210222022202211001220200122220022202202012121022012220200022210101101220212201220112021221021210221220111000012201220120122021202121121221220020002121221222121212221212201212122121222000022222020222112200221022102002000220102021121102222200220211100201200102112101120111122021212011222220022001002120011122022222000101221020122121222001102202221122102222212012120112121121222010201002110211220220101201112212002110221120022110212100222021221221102022220122101202220102201210222222222012202112201122012222210002211011221020212110102002011221121212121201100211002222120210222201222210222122121211022120210122010212121220222022222220220002210112211112201202220222202222202222211210100000210201121220021201222110212122221012222221212001120222122011112120200222000212000002212012222122121112120212200112211212200002112202111020110111010010212201021220210202100212022001221122122021202101122121020212112021002022121212012101201220222120121222001202222222222210200222210221220120220101110212021222021212000200211211222120221202122201212012122120121012222121100022222222120211212210122220122102100222002112100210210012211022222022211220210202211200120220000211002101122021021122022121222202120021022121112120211022021212210201222121022021121011010012120112201201211112121121201021110102221112020221020222110201021212212022220222222112222101122121121111112120020222212202122020212212022222021121120022222202121220220110001222202121201110221020122222020202121212100200222111222220022110202221120221221211122122010222211222102222210122222221222221112202011002200222221200120101210021111011001202012210221221000200022101222111120020222011222102122120222222012022112222101212122021202111022222122100211222210212202222202022002112211222222022020022121211020220211201000010122121021200102120222221220120021122122122100022000222101202220011222021122210122212110212002220201200021100011122000010021212120212122211200220212101212220221120212012222000022222122210012021000222112202122112211020022022221120010222011220220201222122221120020022110011110122000221221212102200122122122012022110022222222000021222120001102021210022110222201100202220122221020220121222002221022220220022002210001120121022200001201220022200202222100100112122220020212211212222222220021202112222010022101112022010221000122121222211222212211201011211222222201112010112112011112002212102012010212120212212020221001210210120120122202002111011110102210010110010100222010012100100202102100020100020211010011202110011200" => "CFCUG".into());
    test_part_two!("222221202212222122222211222222222222222222222202222022222222222002221222222222220222202222202122222020222222021020220022122222222220222222202222222222222221202202222122222222222222222222222222222202222022222222222022220222222222220222212222212222222220222222221121222022022222222222222222212222222222222220202202222122222210222222222222222222222202222122222222222212222222222022222222212022212122222020222222122021221222122222222221222222222222222222222221202202222022222221222222222222222222222212222022222222222022220222222122222222202122222022222021222222220220220022222222222220222222202222222222222221202222222122222220222212222222222222222202222222222222222022222212222022220222202022202022222120222222121020222122222222222221222222202222222222222221202212222022222221222222222222222222222202222022222222222212220222202022220222202222202022222121222222020021222122022222222220222222212222222220222220212222222122222211222212222222222222222222222122222222222102222202222022222222202222202022222021222222120221220222122222222020222222212222222222222222202212222022222222222222222222222222222212222122222222222012222222202022222222212222222222222120222222122021222122222222222120222222202222222221222121202222222222222201222202222222222222222222222222222222222112221212202222222222202122212122222121222222122020221122122222222220222222212222222222222120212212222222222210222212222222222222222212222122222222222102221212202022222222212022222022222122222222221221222222022222222021222222222222222221222222222212222022222211222202222222222222222212202222222222222112221202222022220222202222202021222121222222220022220122022222222222222222212022222220222020202202222122222211222202222222202222222222222022222222222022222202222222222222202222222120222121222222222120222122122222222221222222202222222222222122202222222122222211220222222222212222022212202122222222222212222222222122222222212022212020222022222222021022221222222222222220222222202022222222222120222222222022222222222222222222202222022202202122222222222012222202212022221222202122212220222120222222222022222022222222222021222221222122222220022220212202222122222200220202222222222222022202200122222222222002221212202122221222202122212222222121222222221120222222122222222221222222212222222220222020222212222022222222220212222222202222122202221122222222222202222222222122222022222122212022222120222222222121220222022222222021222222212022222220122022212202222122222212220212222222212222122222212022222222222222220202212122220122202022212122222022222222021122221122122222222021222221202222222220122100222202122122222200221202222222202222122212201022222222222122220202122022222122212122212020222120222222120121220122222222222220222222212122222220122122222202122022222201222202222222222222022212201122222222222102220212202122222122222022222020222022222222221021222022022222222022222222222122222220222011202222122222222200221222222222212212222202220222222222222002221212022122221022222222212021222220222222122220221222022222222122222221222222222220022201222202122222222212221212222222212002022222201122222222222112222212022022221122222222202120222221222222220020222022122222222122222222212022222220222001202212122122222222222212222222212222022222222222022222222212222222212122221122212122202122222222222222222021220222122222222022222222202222222222222101212212222122222201222222222222212202122212210122122222222002222202202122220222202022212221222122212222020220221022222222222101222220202022222220122110222212122222222222220202222222202122122212202022122222222102221212022222222222202122202022222122222222020221221122022222222122222221202022222221022202202222022122222221220222222222222022222202202222221222222222222202202122220222202122222220222020212222021122220122222222222121222220222022222221122120212222122122222222221202222222202122222222211022222222222222222222102122220122202222212120222122222222122222221022122222222022222222202122222222022021222222222122222210221212222222202022122222202022222222222112222222122222222022222222212220222120202222220021221122022222222121222220202222222221022200212212222022222220221202222222212212022222221122022222222222220222022022221122212222212022222221222222122122221022222222222001222221202022222222022001222212222122222200220212222222212212022222200122220222222212221202022222221222222022222122222020222222020022222222222222222010222222202122222222222122202222022022122221222202222222212012022212200122120222222022220202102222221022212122222122222220212222220222222222022222222001222222202122222222022202222212222022222220221202222222202202222222221022222222222212222212122022222122202122222020222222222222121222222022122222222121222222212022222220222212222202222122022212222202222222212102022202221022222222022222220202002022221022202222202120222020212222220121221222122222222002222220222022222220222021222221222222022200222222222222202002022212220022222222222112221202212222222022212022212222222221212222120220221022222222222201222222221222222222122200202200222222122211221212222222222222222222220222222222122112221202112122221022212022212120222220212222121020222222122222222021222222222022222221222121212210222222122222221202222222202102012212221022022222122002220212122222221222222222202221222021202222121221220022222222222211222221221022222220022021202212022122122220220222222222212022112212220022220222222102220212122222220022222222202020222021222222020221221022222222222102222221200222222221022212202210222022122200220202222222222002222222212122222222222022220212112222220222212222202121222121212222221021222122122222222121222220211222222221222120222210022222222220220202222222212122102202202022020222022212220222002022222222202022202221222222212222121021020222022222222102222220210022222222222010202222122022222212221212222222222202122202201222120222002022220202022222221022222122222120222021222222222121122022222222222022222220211202222200022221212212120022022201220202222222222002102222212222121222102112220222102222220122202022212120222222202222121120120022022222222210222221220212222202222202212200220222222200222212222222202022022222212222221222012102220212102022222022012222212021222121222222021120120122022222222202222221222022222202022012222222122022122220221212222222212122002212220221021222002222222222002022222222202122212221222021222222120020221122022222222202222221212222222202222022212200021122222221220202222222202212202222201020120222222012222202202122221222002122112220222021212222120122021122122222222122222222200102222202022212012221022122122212222212222222012012122222212022022222122012221212002122222122112222102022222022212222222220121022022222222011222221200202222220122021002200022022022201222222222222122212002202212120022222012212222202002122222022002222212021222022202221221020022022122222222120222220212112222222222210102222120222022200220202222222102112012222221222120222002112220212112122220222202122212220222121222220020101022122222222222110222220202202222201122011122200220222222222220222222222122102202222212120120222212002221212112022222022002022002120222020222220121112221122222222222210222221200202222212022112222220022222122220221212222222112212022222222020020222012022221212112122221022202022202221222120212200220021221022222222222221222220221022222220022010222212020022222222220202222222102112102202210020221022022112222202102122222222112222012222222120212221121001020222222222222221222221211222222212122122122200220222122211221202222022002222212222000221121022122012220202122122221022122222012221222120212210221110022022222222222022222222220012222221222200212202120222122220220212222122212002202222021222122222102212121212112022222022102002102020222020212211220001022022222222222121222222222112222202022121002202121122222221220202122222002022022222110222222222222222020212222122220122002222022022222022212211020122121122022222222221222221210002222212022101122200020222122221220222122122112122002222020022222122202012021212222122222222002002122220222021212222121202122022222222222210222221211022222221122122022212220122222200222202122122022012112212112021122022112202222212022022221122222102222122222022222222020210220222122222222122222220211002222201122202022221222122022202220222122122222022222222221120020222222212121212102122222022022212122120222122222221120210122222222022222100222221210122222202122112222211222022222201222202220122102002222222111221201222101212020222122222221222022002222220222022202221120011022122022122222202222220221202222002122011122201120122022220222202021022012112102202000121201222210212020212122022221222112112112020222222222222220211110122022122222012222222202202222202022022222211122022122201220212221122002002112212221120121022000212220202102122222222112122022220222021202220220020010022122122222210222220221112222200122201012200122222122200222212122220121212222202000120110122101212220212202122220122222022022020222120222220122201112022122122222010222220220012222001222211112222022122122202220222121220110222202222001220101122100112122212002122221122122212202020222020212210022212111022022222222121222222221112222112222202102222222222122200222222120222012102122202221220102022100022120222122222220122122120102122222222212212020200201122022022222221222221220002222211022111012212221222222220221212120120112222012202120121101222101002220202120120122222222111112221222121202210221001000222122022222000222222211222222001222110202212022222222212221212221120100012102202210020211022101122021212122222022122112221102120222022212201022120111222022222222210222222211112222120122100012201020122022212221222220120200012122202200222121122101112221221211022120222122222022221222021222210120211112022122222222112222221210112222022222000212201020222222211221210221121000102102212020121120022102022120202201220220222222010012222222121202202000002222222022022222010222222202202222020222002122212122222022222221010121221212112202212111021111222022022020221000120122222202012212120222012222210012221102022222022222021222220201102222110022110002220020222122212220021222120002212202202121222200022200112021202110020221122122222122220222110222211211022111222022122222011222221210012222210022100212212222222002122220202222222102222222112110022111222020022221211110121221022112020012220222011212210220101100222122122222000222221210112222100122001222222120222202021222000120221221122102112121020210222010222122210212120122222012211102122222021212201211202220222022122222221222220201202222221222222212200220122202110222202120020122022222112010021000222210122022220011122022222222022222021220002222020011212102122122222222002222222221112222112222011002212122222112002201200022120221002102012220022000122220002222200010021121122002002122221222121202101121101012022222222222121222222200002222112022020102202122022112210200101021221100222202212021021111022120222020200110222220222122220222222220000222110211022120122222222222000222221210112222020122012202212122222002001210210120122220212202002100021220022212122121200102021021222202120122021222220202220001001221022122122222122222220212122222112022210122212121022012011201010020220222022112112211222011122212112022212022221022022002012112221221221202022022200210122222222222222222221212112222002222122102222020222102222222020220220120212112202111020212222012102122221222122121222212222212222221210022021222212101222022122222211122222201021222121002112202210220122022020221212121122110222212102222020210022122022022222102021122222212002012122200000022102111201200222122022222000122222220000222110102200012220022222122022221220220221011122102212102220002022101102222220121120022122212200102122200102022102200202122022122022222122122221222200222120222002122202022022112220211122021022111222002022120121220022122022222211211021122022122102022222220201112112112002002122122022222020222222200111220102222020122221221222202122202210221220121122022212220120011022011002121211212121022222022022212221212221102212022022101122122222222110222222220021220202202100012212221122202022220211021222012112222002100120010022022212221201121222221222212001122020200212002202110101121122022122222002022222120011222001002210112202022222002022212121121120021102012202002221120022221002022002211021120222222201020221220110012120201202111022222222222202022220212020221002022202002202020022112222221022121020221102212022121221220022121012122000101020020222212111202020210200102121101001210122022022222021122221002222222111112002012221220022102201220012122222101022212122200220021122220112120220202020020222022120002122210121002121001211222022122022222101122221211121220202022120112212021222022100201100020120112002212002001022021222121102222101001120220222212200121122210112122011211010220022022022222100122222010200222222212100222222020122022202211121020122002102222112020022200222122022222002001121120222102001001020221220222220021121202022222022222011022221120212222111222100122202222222211220210211020120212102112212021020020022112012122202110220120122102221011121211000012112201221211012122022222120222222000021222212222211012220022222102021201222022220022022222002201122100022201212022011000021000222002200100222220212022212111002222012122122222201222220020100221120022102212200120222102002201101221120001122022022010121121222220222222210202120101022122121011020020121022222102012010202122122222212022221002101222101012100222220120022202111211202221221201002202222022121110222200202022021100020010122012102011222220122022201212020200222022022222010122222021222021001122210002221120122111220221121220220110002002112001122000022012122121000212022212122212121100122122201110221211112102002122022222020022220001112221112202020102221022022022012201221120221111022212112112121111122022202221110101021122122222210111020111221200011001021020002222122222201222220202101121201112010122211122222011112201102221022200122022012121121200122010212021111022021112122122000021122210100000021000220122222022122222121122220020012222020202112122220122022122112202211220222202212202000120022200222201012022201220020200122112022010221011022120220102201222202022122222220020222201010022010122222012222022222102100212200221121221022102110021220010222201211120121120120020022212110002120220112122111220111221202022222222211121220110021020102012100102220222222211202220222021021002202212110222120012022121222121212202122220022222112020022122112121112111202212122022122222101221221001210122220202102222202220022012222210021120221211012212022222021011122012021222022110220020022112000022022200122012220212102100022122022222000121120211020120100212200012200122022112211210212122021212202022121011022000120022012221220220020121122122222120120201000121220210002210002122222220201121121101101022110122010112202021222122120222021020020001022102120202021212121210011121110021021012122012111212021121201200021010001200012122222220201021120000011020212002210222201220022220222211122022120111012022101201221220212002122120112112221001122101021212121020100202201020202020012222222222010021222201002121221201202010111012000000120021022102012211120100001100000220102221021011011002200112010111012101102112201021120220001010120200100111202002112122101210121" => "JYZHF".into());

    let input = include_str!("day08_input.txt");
    test_part_one!(input => "2413".into());
    test_part_two!(input => "BCPZB".into());
}
