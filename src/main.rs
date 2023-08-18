mod common;
mod chip_eight_core;

use chip_eight_core::chip_eight::ChipEight;
use common::file_utils::read_in_rom;


fn main() {
    let rom: Vec<u8> =
        read_in_rom("/Users/christophercimponeriu/repos/chip8-test-suite/bin/1-chip8-logo.ch8")
            .expect("rom failed to load from file system.");

    let chip_eight: ChipEight = ChipEight::new();
    // TODO: pass in display type so that can be played in GUI, Terminal, or Browser.
    //  Pass in keyboard settings. Create a config.
    chip_eight.run(&rom);
}
