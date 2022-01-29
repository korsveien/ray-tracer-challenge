use crate::color::Color;

use std::fs::File;
use std::io::Write;

pub struct Canvas {
    width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    fn with_color(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[self.get_index(x, y)]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_index(x, y);
        if index >= self.pixels.len() {
            return;
        }
        self.pixels[index] = color;
    }

    pub fn to_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    pub fn write_to_file(&self) -> std::io::Result<()> {
        let mut buffer = File::create("foo.ppm")?;
        write!(buffer, "{}", self.to_ppm())
    }

    pub fn to_ppm_body(&self) -> String {
        let mut data = self
            .pixels
            .iter()
            .map(Color::to_true_color)
            .flat_map(|pixel| pixel.iter().map(u8::to_string).collect::<Vec<String>>())
            .collect::<Vec<String>>();

        let mut body = String::with_capacity(self.width * self.height); // can we find out the exact size cheaply?
        let mut char_count = 0;
        let mut pixel_count = 0;

        for (index, s) in data.iter_mut().enumerate() {
            s.push(' ');

            char_count += s.chars().count();

            if char_count > 69 {
                body.pop();
                body.push('\n');
                char_count = 0;
            }

            if (index + 1) % 3 == 0 {
                pixel_count += 1;
            }

            body.push_str(s);

            if pixel_count == self.width {
                body.pop();
                body.push('\n');
                pixel_count = 0;
                char_count = 0;
            }
        }
        body
    }

    fn to_ppm(&self) -> String {
        let header = self.to_ppm_header();
        let body = self.to_ppm_body();
        format!("{}{}", header, body)
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }
}

#[test]
fn should_initalize_a_black_canvas() {
    let canvas = Canvas::new(3, 3);
    let is_all_black = canvas.pixels.iter().all(|x| x.is_black());
    assert_eq!(is_all_black, true);
}

#[test]
fn should_write_pixel_to_canvas() {
    let mut canvas = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    canvas.set_pixel(2, 3, red);

    assert_eq!(canvas.get_pixel(2, 3), red);
}

#[test]
fn should_create_proper_ppm_header() {
    let canvas = Canvas::new(5, 3);

    let expected = "P3\n5 3\n255\n";
    let actual = canvas.to_ppm_header();

    assert_eq!(expected, actual);
}

#[test]
fn should_create_ppm_pixel_data() {
    let mut canvas = Canvas::new(5, 3);
    let color1 = Color::new(1.5, 0.0, 0.0);
    let color2 = Color::new(0.0, 0.5, 0.0);
    let color3 = Color::new(-0.5, 0.0, 1.0);

    canvas.set_pixel(0, 0, color1);
    canvas.set_pixel(2, 1, color2);
    canvas.set_pixel(4, 2, color3);

    let expected = r"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 127 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
";
    let actual = canvas.to_ppm();

    assert_eq!(expected, actual);
}

#[test]
fn should_split_long_lines() {
    let canvas = Canvas::with_color(10, 2, Color::new(1.0, 0.8, 0.6));

    let expected = r"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
";
    let actual = canvas.to_ppm_body();

    assert_eq!(expected, actual);
}
