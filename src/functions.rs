use image;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImageView;
pub struct ImageAscii {
    pub img: DynamicImage,
    ascii: Vec<char>,
    char_basis: Vec<char>,
    char_basis_len: usize,
}
impl ImageAscii {
    pub fn new(filename: &str) -> ImageAscii {
        let img = image::open(filename)
            .unwrap()
            .resize(200, 200, FilterType::Nearest);
        let char_basis = vec![
            '@', '#', '$', '=', '*', '!', ';', ':', '~', '-', ',', '.', ' ',
        ];
        let char_basis_len = char_basis.len() - 1;
        ImageAscii {
            img: img,
            ascii: Vec::<char>::new(),
            char_basis: char_basis,
            char_basis_len: char_basis_len,
        }
    }
    pub fn get_ascii_codes(&mut self) {
        for y in 0..self.img.height() {
            for x in 0..self.img.width() {
                if x == self.img.width() - 1 {
                    self.ascii.push('\n')
                }
                let rgb = [
                    self.img.get_pixel(x, y)[0],
                    self.img.get_pixel(x, y)[1],
                    self.img.get_pixel(x, y)[2],
                ];
                let max_value = *rgb.iter().max().unwrap() as f32 / 255.0;
                let code = self.char_basis[(max_value * (self.char_basis_len as f32)) as usize];
                self.ascii.push(code);
            }
        }
    }
    pub fn print_ascii_codes_str(&self) -> String {
        let mut s = String::new();
        for i in self.ascii.iter() {
            s += &i.to_string();
        }
        return s;
    }

    pub fn print_ascii_codes_html(&self) -> String {
        let mut s = String::new();
        s += "<p style=\"font: bold 8px/5px monospace;color:#000;\">";
        for i in self.ascii.iter() {
            if i == &'\n' {
                s += "<br>"
            } else {
                s += &i.to_string();
            }
        }
        s += "</p>";
        return s;
    }
}
