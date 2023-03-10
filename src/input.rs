use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Input{
    keys: [u8; 16],
    event_pump: EventPump,
}

impl Input{
    pub fn new(sdl_context: &sdl2::Sdl) -> Input{
        Input{
            keys: [0; 16],
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn input_key(&mut self){
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Num0), .. } => self.keys[0] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => self.keys[1] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => self.keys[2] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => self.keys[3] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => self.keys[4] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num5), .. } => self.keys[5] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num6), .. } => self.keys[6] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num7), .. } => self.keys[7] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num8), .. } => self.keys[8] = 1,
                Event::KeyDown { keycode: Some(Keycode::Num9), .. } => self.keys[9] = 1,
                Event::KeyDown { keycode: Some(Keycode::A), .. } => self.keys[10] = 1,
                Event::KeyDown { keycode: Some(Keycode::B), .. } => self.keys[11] = 1,
                Event::KeyDown { keycode: Some(Keycode::C), .. } => self.keys[12] = 1,
                Event::KeyDown { keycode: Some(Keycode::D), .. } => self.keys[13] = 1,
                Event::KeyDown { keycode: Some(Keycode::E), .. } => self.keys[14] = 1,
                Event::KeyDown { keycode: Some(Keycode::F), .. } => self.keys[15] = 1,
                _ => {}
            }
        }
    }

    pub fn get_keys(&mut self) -> [u8; 16]{
        self.keys = [0; 16];
        self.input_key();
        self.keys
    }
}
