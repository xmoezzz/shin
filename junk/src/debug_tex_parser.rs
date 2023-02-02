use image::{Rgba, RgbaImage};
use once_cell::sync::Lazy;
use regex::Regex;

// this decodes a texture embedded in the executable used for a debug overlay
// It is in fact a simple font with 64 characters
const DEBUG_TEXTURE: &str = r#"
            710016f1ea [0]                0h,  0h,  2h, 40h,
            710016f1ee [4]                9h, 90h,  9h, 90h,
            710016f1f2 [8]                2h, 40h, 24h,  9h,
            710016f1f6 [12]              2Ah, 40h,  0h, 90h,
            710016f1fa [16]               0h,  0h,  2h, 40h,
            710016f1fe [20]               9h, 90h,  9h, 90h,
            710016f202 [24]              2Ah, A9h, 99h, 25h,
            710016f206 [28]              95h, 90h,  0h, 90h,
            710016f20a [32]               0h,  0h,  2h, 40h,
            710016f20e [36]               5h, 50h, 2Ah, A4h,
            710016f212 [40]              96h, 55h, 65h, 94h,
            710016f216 [44]              66h, 50h,  2h, 50h,
            710016f21a [48]               0h,  0h,  2h, 40h,
            710016f21e [52]               0h,  0h, 19h, 94h,
            710016f222 [56]              6Ah, A4h, 16h, 50h,
            710016f226 [60]              99h, 49h,  1h, 40h,
            710016f22a [64]               0h,  0h,  2h, 40h,
            710016f22e [68]               0h,  0h, 2Ah, A4h,
            710016f232 [72]              16h, 59h,  9h, 64h,
            710016f236 [76]              96h, 25h,  0h,  0h,
            710016f23a [80]               0h,  0h,  1h, 40h,
            710016f23e [84]               0h,  0h, 19h, 94h,
            710016f242 [88]              AAh, A5h, 25h, 99h,
            710016f246 [92]              91h, 94h,  0h,  0h,
            710016f24a [96]               0h,  0h,  2h, 40h,
            710016f24e [100]              0h,  0h,  9h, 90h,
            710016f252 [104]             56h, 54h, 94h, 65h,
            710016f256 [108]             6Ah, 69h,  0h,  0h,
            710016f25a [112]              0h,  0h,  1h, 40h,
            710016f25e [116]              0h,  0h,  5h, 50h,
            710016f262 [120]              1h, 40h, 50h, 14h,
            710016f266 [124]             15h, 55h,  0h,  0h,
            710016f26a [128]              0h, 80h,  8h,  0h,
            710016f26e [132]              2h,  0h,  0h,  0h,
            710016f272 [136]              0h,  0h,  0h,  0h,
            710016f276 [140]              0h,  0h,  0h,  9h,
            710016f27a [144]              2h, 50h,  2h, 40h,
            710016f27e [148]             26h, 64h,  2h, 40h,
            710016f282 [152]              0h,  0h,  0h,  0h,
            710016f286 [156]              0h,  0h,  0h, 25h,
            710016f28a [160]              9h, 40h,  1h, 90h,
            710016f28e [164]             1Ah, 94h,  2h, 40h,
            710016f292 [168]              0h,  0h,  0h,  0h,
            710016f296 [172]              0h,  0h,  0h, 94h,
            710016f29a [176]              9h,  0h,  0h, 90h,
            710016f29e [180]              6h, 50h, 2Ah, A4h,
            710016f2a2 [184]              0h,  0h, 2Ah, A4h,
            710016f2a6 [188]              0h,  0h,  2h, 50h,
            710016f2aa [192]              9h,  0h,  0h, 90h,
            710016f2ae [196]              Ah, 90h, 16h, 54h,
            710016f2b2 [200]              0h, 90h, 15h, 54h,
            710016f2b6 [204]              0h,  0h,  9h, 40h,
            710016f2ba [208]              6h, 40h,  2h, 50h,
            710016f2be [212]             26h, 64h,  2h, 40h,
            710016f2c2 [216]              0h, 90h,  0h,  0h,
            710016f2c6 [220]              0h,  0h, 25h,  0h,
            710016f2ca [224]              1h, 90h,  9h, 40h,
            710016f2ce [228]             16h, 54h,  1h, 40h,
            710016f2d2 [232]              2h, 50h,  0h,  0h,
            710016f2d6 [236]              9h,  0h, 94h,  0h,
            710016f2da [240]              0h, 50h,  5h,  0h,
            710016f2de [244]              1h, 40h,  0h,  0h,
            710016f2e2 [248]              1h, 40h,  0h,  0h,
            710016f2e6 [252]              5h,  0h, 50h,  0h,
            710016f2ea [256]             2Ah, A4h,  Ah, 40h,
            710016f2ee [260]             AAh, A4h, AAh, A4h,
            710016f2f2 [264]              0h, A4h, AAh, A9h,
            710016f2f6 [268]             2Ah, A4h, AAh, A9h,
            710016f2fa [272]             95h, 59h,  6h, 40h,
            710016f2fe [276]             55h, 59h, 55h, 59h,
            710016f302 [280]              2h, 64h, 95h, 55h,
            710016f306 [284]             95h, 54h, 55h, 59h,
            710016f30a [288]             90h,  9h,  2h, 40h,
            710016f30e [292]              0h,  9h,  0h,  9h,
            710016f312 [296]              9h, 64h, 90h,  0h,
            710016f316 [300]             90h,  0h,  0h, 25h,
            710016f31a [304]             90h,  9h,  2h, 40h,
            710016f31e [308]             2Ah, A5h, 2Ah, A5h,
            710016f322 [312]             25h, 24h, AAh, A4h,
            710016f326 [316]             AAh, A4h,  0h, 94h,
            710016f32a [320]             90h,  9h,  2h, 40h,
            710016f32e [324]             95h, 54h, 15h, 59h,
            710016f332 [328]             94h, 24h, 55h, 59h,
            710016f336 [332]             95h, 59h,  2h, 50h,
            710016f33a [336]             90h,  9h,  2h, 40h,
            710016f33e [340]             90h,  0h,  0h,  9h,
            710016f342 [344]             AAh, A9h,  0h,  9h,
            710016f346 [348]             90h,  9h,  9h, 40h,
            710016f34a [352]             6Ah, A5h,  Ah, 90h,
            710016f34e [356]             AAh, A9h, AAh, A5h,
            710016f352 [360]             55h, 65h, AAh, A5h,
            710016f356 [364]             6Ah, A5h, 25h,  0h,
            710016f35a [368]             15h, 54h,  5h, 50h,
            710016f35e [372]             55h, 55h, 55h, 54h,
            710016f362 [376]              0h, 14h, 55h, 54h,
            710016f366 [380]             15h, 54h, 14h,  0h,
            710016f36a [384]             2Ah, A4h, 2Ah, A4h,
            710016f36e [388]              0h,  0h,  0h,  0h,
            710016f372 [392]              0h, 24h,  0h,  0h,
            710016f376 [396]              9h,  0h, 2Ah, A4h,
            710016f37a [400]             95h, 59h, 95h, 59h,
            710016f37e [404]              0h,  0h,  0h,  0h,
            710016f382 [408]              0h, 94h,  0h,  0h,
            710016f386 [412]              6h, 40h, 95h, 59h,
            710016f38a [416]             90h,  9h, 90h,  9h,
            710016f38e [420]              2h, 40h,  2h, 40h,
            710016f392 [424]              2h, 50h, 2Ah, A4h,
            710016f396 [428]              1h, 90h, 50h,  9h,
            710016f39a [432]             6Ah, A5h, 6Ah, A9h,
            710016f39e [436]              1h, 40h,  1h, 40h,
            710016f3a2 [440]              9h, 40h, 15h, 54h,
            710016f3a6 [444]              0h, 64h,  0h, A5h,
            710016f3aa [448]             95h, 59h, 15h, 59h,
            710016f3ae [452]              0h,  0h,  2h, 40h,
            710016f3b2 [456]              6h, 40h, 2Ah, A4h,
            710016f3b6 [460]              0h, 94h,  2h, 54h,
            710016f3ba [464]             90h,  9h,  0h,  9h,
            710016f3be [468]              2h, 40h,  2h, 40h,
            710016f3c2 [472]              1h, 90h, 15h, 54h,
            710016f3c6 [476]              2h, 50h,  1h, 40h,
            710016f3ca [480]             6Ah, A5h, 2Ah, A5h,
            710016f3ce [484]              1h, 40h,  9h, 40h,
            710016f3d2 [488]              0h, 64h,  0h,  0h,
            710016f3d6 [492]              9h, 40h,  2h, 40h,
            710016f3da [496]             15h, 54h, 15h, 54h,
            710016f3de [500]              0h,  0h,  5h,  0h,
            710016f3e2 [504]              0h, 14h,  0h,  0h,
            710016f3e6 [508]              5h,  0h,  1h, 40h,
            710016f3ea [512]              9h,  0h, 2Ah, A4h,
            710016f3ee [516]             AAh, A4h, 2Ah, A9h,
            710016f3f2 [520]             AAh, A4h, AAh, A9h,
            710016f3f6 [524]             AAh, A9h, 2Ah, A9h,
            710016f3fa [528]              Ah, 40h, 95h, 59h,
            710016f3fe [532]             95h, 59h, 95h, 55h,
            710016f402 [536]             95h, 59h, 95h, 55h,
            710016f406 [540]             95h, 55h, 95h, 55h,
            710016f40a [544]              Ah, 90h, 90h,  9h,
            710016f40e [548]             90h,  9h, 90h,  0h,
            710016f412 [552]             90h,  9h, 90h,  0h,
            710016f416 [556]             90h,  0h, 90h,  0h,
            710016f41a [560]              Ah, A4h, AAh, A9h,
            710016f41e [564]             AAh, A5h, 90h,  0h,
            710016f422 [568]             90h,  9h, AAh, A4h,
            710016f426 [572]             AAh, A4h, 92h, A9h,
            710016f42a [576]              Ah, 94h, 95h, 59h,
            710016f42e [580]             95h, 59h, 90h,  0h,
            710016f432 [584]             90h,  9h, 95h, 54h,
            710016f436 [588]             95h, 54h, 91h, 59h,
            710016f43a [592]              Ah, 50h, 90h,  9h,
            710016f43e [596]             90h,  9h, 90h,  0h,
            710016f442 [600]             90h,  9h, 90h,  0h,
            710016f446 [604]             90h,  0h, 90h,  9h,
            710016f44a [608]              9h, 40h, 90h,  9h,
            710016f44e [612]             AAh, A5h, 6Ah, A9h,
            710016f452 [616]             AAh, A5h, AAh, A9h,
            710016f456 [620]             90h,  0h, 6Ah, A9h,
            710016f45a [624]              5h,  0h, 50h,  5h,
            710016f45e [628]             55h, 54h, 15h, 55h,
            710016f462 [632]             55h, 54h, 55h, 55h,
            710016f466 [636]             50h,  0h, 15h, 55h,
            710016f46a [640]             90h,  9h,  Ah, 90h,
            710016f46e [644]              2h, A9h, 90h, 29h,
            710016f472 [648]             90h,  0h, 90h,  9h,
            710016f476 [652]             90h,  9h, 2Ah, A4h,
            710016f47a [656]             90h,  9h,  6h, 50h,
            710016f47e [660]              1h, 65h, 92h, 95h,
            710016f482 [664]             90h,  0h, A4h, 29h,
            710016f486 [668]             A4h,  9h, 95h, 59h,
            710016f48a [672]             90h,  9h,  2h, 40h,
            710016f48e [676]              0h, 24h, A9h, 50h,
            710016f492 [680]             90h,  0h, 99h, 99h,
            710016f496 [684]             99h,  9h, 90h,  9h,
            710016f49a [688]             AAh, A9h,  2h, 40h,
            710016f49e [692]              0h, 24h, 96h, 40h,
            710016f4a2 [696]             90h,  0h, 96h, 59h,
            710016f4a6 [700]             96h, 49h, 90h,  9h,
            710016f4aa [704]             95h, 59h,  2h, 40h,
            710016f4ae [708]              0h, 24h, 91h, 90h,
            710016f4b2 [712]             90h,  0h, 91h, 49h,
            710016f4b6 [716]             91h, 99h, 90h,  9h,
            710016f4ba [720]             90h,  9h,  2h, 40h,
            710016f4be [724]              0h, 24h, 90h, 64h,
            710016f4c2 [728]             90h,  0h, 90h,  9h,
            710016f4c6 [732]             90h, 69h, 90h,  9h,
            710016f4ca [736]             90h,  9h,  Ah, 90h,
            710016f4ce [740]             AAh, 94h, 90h, 19h,
            710016f4d2 [744]             AAh, A9h, 90h,  9h,
            710016f4d6 [748]             90h, 19h, 6Ah, A5h,
            710016f4da [752]             50h,  5h,  5h, 50h,
            710016f4de [756]             55h, 50h, 50h,  5h,
            710016f4e2 [760]             55h, 55h, 50h,  5h,
            710016f4e6 [764]             50h,  5h, 15h, 54h,
            710016f4ea [768]             AAh, A4h, 2Ah, A4h,
            710016f4ee [772]             AAh, A4h, 2Ah, A9h,
            710016f4f2 [776]             AAh, A9h, 90h,  9h,
            710016f4f6 [780]             90h,  9h, 90h,  9h,
            710016f4fa [784]             95h, 59h, 95h, 59h,
            710016f4fe [788]             95h, 59h, 95h, 55h,
            710016f502 [792]             56h, 55h, 90h,  9h,
            710016f506 [796]             90h,  9h, 90h,  9h,
            710016f50a [800]             90h,  9h, 90h,  9h,
            710016f50e [804]             90h,  9h, 90h,  0h,
            710016f512 [808]              2h, 40h, 90h,  9h,
            710016f516 [812]             90h,  9h, 90h,  9h,
            710016f51a [816]             AAh, A5h, 92h, 49h,
            710016f51e [820]             AAh, A5h, 6Ah, A4h,
            710016f522 [824]              2h, 40h, 90h,  9h,
            710016f526 [828]             90h,  9h, 92h, 49h,
            710016f52a [832]             95h, 54h, 91h, 99h,
            710016f52e [836]             95h, 59h, 15h, 59h,
            710016f532 [840]              2h, 40h, 90h,  9h,
            710016f536 [844]             64h, 25h, 99h, 99h,
            710016f53a [848]             90h,  0h, 90h, 65h,
            710016f53e [852]             90h,  9h,  0h,  9h,
            710016f542 [856]              2h, 40h, 90h,  9h,
            710016f546 [860]             19h, 94h, A5h, 69h,
            710016f54a [864]             90h,  0h, 6Ah, 99h,
            710016f54e [868]             90h,  9h, AAh, A5h,
            710016f552 [872]              2h, 40h, 6Ah, A5h,
            710016f556 [876]              6h, 50h, 94h, 19h,
            710016f55a [880]             50h,  0h, 15h, 55h,
            710016f55e [884]             50h,  5h, 55h, 54h,
            710016f562 [888]              1h, 40h, 15h, 54h,
            710016f566 [892]              1h, 40h, 50h,  5h,
            710016f56a [896]             90h,  9h, 90h,  9h,
            710016f56e [900]             AAh, A9h,  Ah, 90h,
            710016f572 [904]             90h,  9h,  Ah, 90h,
            710016f576 [908]              2h, 40h,  0h,  0h,
            710016f57a [912]             64h, 25h, 64h, 25h,
            710016f57e [916]             55h, 65h,  9h, 50h,
            710016f582 [920]             64h, 25h,  5h, 90h,
            710016f586 [924]              9h, 90h,  0h,  0h,
            710016f58a [928]             19h, 94h, 19h, 94h,
            710016f58e [932]              0h, 94h,  9h,  0h,
            710016f592 [936]             19h, 94h,  0h, 90h,
            710016f596 [940]             25h, 64h,  0h,  0h,
            710016f59a [944]              6h, 50h,  6h, 50h,
            710016f59e [948]              2h, 50h,  9h,  0h,
            710016f5a2 [952]             2Ah, A4h,  0h, 90h,
            710016f5a6 [956]             14h, 14h,  0h,  0h,
            710016f5aa [960]              9h, 90h,  2h, 40h,
            710016f5ae [964]              9h, 40h,  9h,  0h,
            710016f5b2 [968]             16h, 54h,  0h, 90h,
            710016f5b6 [972]              0h,  0h,  0h,  0h,
            710016f5ba [976]             25h, 64h,  2h, 40h,
            710016f5be [980]             25h,  0h,  9h,  0h,
            710016f5c2 [984]             2Ah, A4h,  0h, 90h,
            710016f5c6 [988]              0h,  0h,  0h,  0h,
            710016f5ca [992]             94h, 19h,  2h, 40h,
            710016f5ce [996]             AAh, A8h,  Ah, 90h,
            710016f5d2 [1000]            16h, 54h,  Ah, 90h,
            710016f5d6 [1004]             0h,  0h, AAh, A9h,
            710016f5da [1008]            50h,  5h,  1h, 40h,
            710016f5de [1012]            55h, 55h,  5h, 50h,
            710016f5e2 [1016]             1h, 40h,  5h, 50h,
            710016f5e6 [1020]             0h,  0h, 55h, 55h

"#;

const DICTIONARY: [Rgba<u8>; 4] = [
    Rgba([0, 0, 0, 0]),
    Rgba([1, 1, 1, 1]),
    Rgba([255, 255, 255, 255]),
    Rgba([0, 0, 0, 0]),
];

// extract the values from a line
static TEX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[0-9a-f]+\s+\[[0-9]+\]\s+((?:[0-9A-F ]+h,?)+)").unwrap());
static INNER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9A-F]+)h").unwrap());

pub fn main() {
    let mut pixels = Vec::new();

    for line in DEBUG_TEXTURE.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let caps = TEX_REGEX.captures(line).unwrap();
        let inner = caps.get(1).unwrap().as_str();
        for cap in INNER_REGEX.captures_iter(inner) {
            let byte = u8::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
            pixels.push(DICTIONARY[((byte >> 6) & 0x3) as usize]);
            pixels.push(DICTIONARY[((byte >> 4) & 0x3) as usize]);
            pixels.push(DICTIONARY[((byte >> 2) & 0x3) as usize]);
            pixels.push(DICTIONARY[(byte & 0x3) as usize]);
        }
    }

    let mut image = RgbaImage::new(0x40, 0x40);
    for ((_, _, pix), val) in image.enumerate_pixels_mut().zip(pixels) {
        *pix = val;
    }

    image.save("debug.png").unwrap();
}