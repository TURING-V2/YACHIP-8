use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

use crate::timer::Timer;

const SAMPLE_RATE: i32 = 44100;
const AMPLITUDE: f32 = 0.25;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.phase = (self.phase + self.phase_inc) % (2.0 * std::f32::consts::PI);

        let value = if self.phase < std::f32::consts::PI {
            self.volume
        } else {
            -self.volume
        };

        Some(value)
    }
}

pub struct Sound {
    device: Option<AudioDevice<SquareWave>>,
    frequency: i32,
    duration: u8,
}

impl Sound {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLE_RATE as i32),
            channels: Some(1),
            samples: None,
        };

        Sound {
            device: Some(
                audio_subsystem
                    .open_playback(None, &desired_spec, |spec| {
                        SquareWave {
                            phase_inc: 2.0 * std::f32::consts::PI * 440.0 / spec.freq as f32,
                            phase: 0.0,
                            volume: AMPLITUDE,
                        }
                    })
                    .unwrap(),
            ),
            frequency: 440,
            duration: 0,
        }
    }

    pub fn play_sound(&mut self, timer: &mut Timer) {
        if self.duration > 0 {
            self.frequency = 440;
            self.duration = timer.read_sound_timer();
            self.device.as_mut().unwrap().resume();
        } else if let Some(device) = &self.device {
            device.pause();
        }
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            *sample = self.next().unwrap();
        }
    }
}
