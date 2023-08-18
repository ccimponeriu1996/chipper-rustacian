// Graphics
const PIXELS: usize = 64 * 32;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PixelState { ON, OFF }

pub struct Graphics {
    display: [PixelState; PIXELS]
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            display: [PixelState::ON; PIXELS]
        }
    }
    pub fn clear(&mut self) {
        self.display = [PixelState::OFF; PIXELS];
    }
    pub fn draw(&mut self, bytes: Vec<u8>, x_coordinate: usize, y_coordinate: usize) -> bool {
        let pixels_erased: bool = false;
        for (i, byte) in bytes.iter().enumerate() {
            self.display[x_coordinate + (y_coordinate * 64)] = PixelState::ON; //  XOR
        }
        println!("{:?}, {}, {}", bytes, x_coordinate, y_coordinate);
        for (i, pixel) in self.display.iter().enumerate() {
            print!("{}", if *pixel == PixelState::ON {"◒"} else {" "});
            if i % 64 == 0 { println!(); }
        }
        println!("\n\n");
        return pixels_erased;
    }
}
