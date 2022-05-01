use lds_rs::Vdcorput;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::{FromColor, Lch};
// use palette::white_point::D65;

fn main() {
    let mut vdc_l = Vdcorput::new(3);
    let mut vdc_h = Vdcorput::new(2);
    // println!("min_l {}", Lch::<D65, f32>::min_l());
    // println!("max_l {}", Lch::<D65, f32>::max_l());
    // println!("min_chroma {}", Lch::<D65, f32>::min_chroma());
    // println!("max_chroma {}", Lch::<D65, f32>::max_chroma());

    for _ in 0..20 {
        let lch_color = Lch::new(vdc_l.pop() * 100.0, 98.0, vdc_h.pop() * 360.0);
        let new_color = palette::Srgb::from_color(lch_color);
        let test: Rgb<Srgb, u8> = new_color.into_format();
        println!("0x{:>08X}", u32::from(test));
    }
}

/*
Output:
0xFF00674D
0xFFC49F00
0xFF0030B0
0xFFD50400
0xFF00E5FF
0xFF004500
0xFFB852FF
0xFFFF839E
0xFF002E40
0xFF346400
0xFF659FFF
0xFF5F0000
0xFF008CFF
0xFF00EF80
0xFF9B0078
0xFFFF0073
0xFF00FFFF
0xFF2D1900
0xFF0063FF
0xFFFF8700

*/
