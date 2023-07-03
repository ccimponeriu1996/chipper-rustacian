// Graphics
const PIXELS: usize = 64 * 32;

pub struct Graphics {
    pixels: [u8; PIXELS]
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            pixels: [0; PIXELS]
        }
    }
    pub fn clear(&mut self) {
        self.pixels = [0; PIXELS];
    }
    pub fn draw(&mut self, bytes: Vec<u8>, x_coordinate: usize, y_coordinate: usize) {
        println!("{:?}, {}, {}", bytes, x_coordinate, y_coordinate);
    }
}
