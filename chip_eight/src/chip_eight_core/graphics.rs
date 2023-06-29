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
}
