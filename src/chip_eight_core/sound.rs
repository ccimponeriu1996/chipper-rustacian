use crate::chip_eight_core::api::Tick;

pub struct Sound {
    sound_timer: u8,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            sound_timer: 0,
        }
    }
    pub fn set_sound_timer(&mut self, time: u8) {
        self.sound_timer = time;
    }
}

impl Tick for Sound {
    fn tick(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
