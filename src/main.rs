use lds_rs::Sphere;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::{FromColor, Lab};
// use palette::white_point::D65;

fn main() {
    let mut sgen = Sphere::new(&[3, 2]);

    for _ in 0..20 {
        let coord = sgen.pop();
        let lab_color = Lab::new((coord[2] + 1.0) * 50.0, coord[0]*127.0, coord[1]*127.0);
        // println!("{:#?}", lab_color);
        let new_color = palette::Srgb::from_color(lab_color);
        // println!("{:#?}", new_color);
        let test: Rgb<Srgb, u8> = new_color.into_format();
        println!("0x{:>08X}", u32::from(test));
    }
}

/*
Output:

0xFF0062FF
0xFFFF00A7
0xFF00321C
0xFFEC0000
0xFF00E7FF
0xFF5700A9
0xFF00A200
0xFFFFC64D
0xFF001B4A
0xFFD400A5
0xFF00D254
0xFF820000
0xFF0097C3
0xFF8EBCFF
0xFF1D4900
0xFFD07B00
0xFF00F8FF
0xFF550029
0xFF007E34
0xFFFF474A

*/
