pub struct Timer{
    delay_timer: u8,
    sound_timer: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn read_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn read_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

}
