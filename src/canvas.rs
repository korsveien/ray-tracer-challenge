use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[self.get_index(x, y)]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_index(x, y);
        self.pixels[index] = color;
    }

    pub(crate) fn to_ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
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
    canvas.write_pixel(2, 3, red);

    assert_eq!(canvas.get_pixel(2, 3), red);
}

#[test]
fn should_create_proper_ppm_header() {
    let canvas = Canvas::new(5, 3);

    let expected = "P3\n5 3\n255\n";
    let result = canvas.to_ppm_header();

    assert_eq!(expected, result);
}
