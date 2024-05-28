
use std::env;
use std::i64;

#[derive(Debug)]
enum Color {
    RGB(u8, u8, u8),
    CMYK(f32, f32, f32, f32),
    Hex(String),
}

#[allow(dead_code)]
impl Color {
    fn convert_to_cmyk(&self) {
        match *self {
            Self::RGB(r, g, b) => {
                let r1 = r as f32 / 255.0;
                let g1 = g as f32 / 255.0;
                let b1 = b as f32 / 255.0;

                let k = 1.0 - (r1.max(g1)).max(b1);
                let c = (1.0 - r1 - k) / (1.0 - k);
                let m = (1.0 - g1 - k) / (1.0 - k);
                let y = (1.0 - b1 - k) / (1.0 - k);

                println!("{:.2}, {:.2}, {:.2}, {:.2}", c, m, y, k);
            }
            Self::Hex(ref hex) => {
                let rgb = Self::Hex(hex.to_string()).convert_to_rgb();
                rgb.convert_to_cmyk()
            }
            _ => panic!("Is already CMKY!")
        }
    }
    fn convert_to_rgb(&self) -> Self{
        match *self {
            Self::CMYK(c, m, y, k) => {
                // added round()
                let r = (255.0 * (1.0 - c) * (1.0 - k)).round() as u8;
                let g = (255.0 * (1.0 - m) * (1.0 - k)).round() as u8;
                let b = (255.0 * (1.0 - y) * (1.0 - k)).round() as u8;

                println!("({}, {}, {})", &r, &g, &b);
                Self::RGB(r, g, b)
            }
            Self::Hex(ref hex) => {
                let mut hex_vec: Vec<char> = hex.chars().collect();
                hex_vec.remove(0);
                let r = u8::from_str_radix(&hex_vec[0].to_string(), 16).unwrap() * 16 + u8::from_str_radix(&hex_vec[1].to_string(), 16).unwrap();
                let g = u8::from_str_radix(&hex_vec[2].to_string(), 16).unwrap() * 16 + u8::from_str_radix(&hex_vec[3].to_string(), 16).unwrap();
                let b = u8::from_str_radix(&hex_vec[4].to_string(), 16).unwrap() * 16 + u8::from_str_radix(&hex_vec[5].to_string(), 16).unwrap();
                println!("({}, {}, {})", &r, &g, &b);
                Self::RGB(r, g, b)
            }
            _ => panic!("Is already RGB!")
        }
    }
    fn convert_to_hex(&self) -> Self{
        match *self {
            Self::RGB(r, g, b) => {
                let r1 = r as f64 / 16.0;
                let g1 = g as f64 / 16.0;
                let b1 = b as f64 / 16.0;

                let h1 = format!("{:X}", r1.floor() as i64);
                let h2 = format!("{:X}", (r1.fract() * 16.0) as i64);
                let h3 = format!("{:X}", g1.floor() as i64);
                let h4 = format!("{:X}", (g1.fract() * 16.0) as i64);
                let h5 = format!("{:X}", b1.floor() as i64);
                let h6 = format!("{:X}", (b1.fract() * 16.0) as i64);

                println!("#{}{}{}{}{}{}", h1, h2, h3, h4, h5, h6);
                let hex = "#".to_string() + &h1 + &h2 + &h3 + &h4 + &h5 + &h6;
                Self::Hex(hex)
            }
            Self::CMYK(c, m, k, y) => {
                let cmky = Self::CMYK(c, m, k, y);
                let rgb = cmky.convert_to_rgb();
                rgb.convert_to_hex()
            }
            _ => panic!("Is already Hex!"),
        }
    }
}

fn read_envargs()-> Color{
    let args: Vec<_> = env::args().collect();
    let r: u8 = args[1].parse().unwrap();
    let g: u8 = args[2].parse().unwrap();
    let b: u8 = args[3].parse().unwrap();
    Color::RGB(r, g, b)
}

#[allow(unused_variables)]
fn main() {
    let my_rgb = Color::RGB(240, 236, 18);
    let my_cmyk = Color::CMYK(0.0, 0.02, 0.93, 0.06);

    let env_rgb = read_envargs();
    let env_hex = env_rgb.convert_to_hex();
    let converted_env_hex = env_hex.convert_to_cmyk();

    
    // println!("{:?}", my_cmyk.convert_to_hex())
}