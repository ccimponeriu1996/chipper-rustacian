use crate::chip_eight_core::api::Tick;

pub struct Sound {
    sound_timer: u16,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            sound_timer: 0,
        }
    }
}

impl Tick for Sound {
    fn tick(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
