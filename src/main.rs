mod functions;
use functions::ImageAscii;

fn main() {
    let mut image_ascii = ImageAscii::new("test3.jpg");
    image_ascii.get_ascii_codes();
    println!("{}", image_ascii.print_ascii_codes_html());
}
