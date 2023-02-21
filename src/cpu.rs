use rand::random;

use crate::ram::RAM;
use crate::display::Display;
use crate::input::Input;
use crate::timer::Timer;
use crate::sound::Sound;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::REGISTER_COUNT;
use crate::STACK_SIZE;

pub struct CPU{
    registers: [u8; REGISTER_COUNT],
    index_register: u16,
    program_counter: u16,
    stack: [u16; STACK_SIZE],
    stack_pointer: u8,
    draw_flag: bool,
}

impl CPU{
    pub fn new() -> CPU{
        CPU{
            registers: [0; REGISTER_COUNT],
            index_register: 0,
            program_counter: 0x200,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            draw_flag: false,
        }
    }

    pub fn cycle(&mut self, ram: &mut RAM, display: &mut Display, input: &mut Input, timer: &mut Timer, sound: &mut Sound, canvas: &mut Canvas<Window>){
        let opcode = self.fetch_opcode(ram);
        self.execute_opcode(opcode, ram, display, input, timer, sound, canvas);
    }

    fn fetch_opcode(&self, ram: &RAM) -> u16{
        let opcode = (ram.read(self.program_counter) as u16) << 8 | ram.read(self.program_counter + 1) as u16;
        opcode
    }

    fn execute_opcode(&mut self, opcode: u16, ram: &mut RAM, display: &mut Display, input: &mut Input, timer: &mut Timer, sound: &mut Sound, canvas: &mut Canvas<Window>){
        match opcode & 0xF000{
            0x0000 => match opcode & 0x000F{
                0x0000 => self.clear_screen(display),
                0x000E => self.return_from_subroutine(),
                _ => println!("Unknown opcode: {:x}", opcode),
            },
            0x1000 => self.jump_to_address(opcode),
            0x2000 => self.call_subroutine(opcode),
            0x3000 => self.skip_if_equal(opcode),
            0x4000 => self.skip_if_not_equal(opcode),
            0x5000 => self.skip_if_equal_register(opcode),
            0x6000 => self.set_register(opcode),
            0x7000 => self.add_to_register(opcode),
            0x8000 => match opcode & 0x000F{
                0x0000 => self.set_register_register(opcode),
                0x0001 => self.set_register_or(opcode),
                0x0002 => self.set_register_and(opcode),
                0x0003 => self.set_register_xor(opcode),
                0x0004 => self.add_register_register(opcode),
                0x0005 => self.sub_register_register(opcode),
                0x0006 => self.shift_right(opcode),
                0x0007 => self.sub_register_register_reverse(opcode),
                0x000E => self.shift_left(opcode),
                _ => println!("Unknown opcode: {:x}", opcode),
            },
            0x9000 => self.skip_if_not_equal_register(opcode),
            0xA000 => self.set_index_register(opcode),
            0xB000 => self.jump_to_address_plus_register(opcode),
            0xC000 => self.set_register_random(opcode),
            0xD000 => self.draw_sprite(opcode, display, ram, canvas),
            0xE000 => match opcode & 0x000F{
                0x000E => self.skip_if_key_pressed(opcode, input),
                0x0001 => self.skip_if_key_not_pressed(opcode, input),
                _ => println!("Unknown opcode: {:x}", opcode),
            },
            0xF000 => match opcode & 0x00FF{
                0x0007 => self.set_register_delay_timer(opcode, timer),
                0x000A => self.wait_for_key_press(opcode, input),
                0x0015 => self.set_delay_timer_register(opcode, timer),
                0x0018 => self.set_sound_timer_register(opcode, timer, sound),
                0x001E => self.add_index_register_register(opcode),
                0x0029 => self.set_index_register_sprite(opcode),
                0x0033 => self.store_bcd(opcode, ram),
                0x0055 => self.store_registers(opcode, ram),
                0x0065 => self.load_registers(opcode, ram),
                _ => println!("Unknown opcode: {:x}", opcode),
            },
            _ => println!("Unknown opcode: {:x}", opcode),
        }
    }

    fn clear_screen(&mut self, display: &mut Display){
        display.clear();
        self.program_counter += 2;
    }

    fn return_from_subroutine(&mut self){
        self.program_counter = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
        self.program_counter += 2;
    }

    fn jump_to_address(&mut self, opcode: u16){
        self.program_counter = opcode & 0x0FFF;
    }

    fn call_subroutine(&mut self, opcode: u16){
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.program_counter = opcode & 0x0FFF;
    }

    fn skip_if_equal(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        let value = (opcode & 0x00FF) as u8;
        if self.registers[register] == value{
            self.program_counter += 4;
        }else{
            self.program_counter += 2;
        }
    }

    fn skip_if_not_equal(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        let value = (opcode & 0x00FF) as u8;
        if self.registers[register] != value{
            self.program_counter += 4;
        }else{
            self.program_counter += 2;
        }
    }

    fn skip_if_equal_register(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        if self.registers[register1] == self.registers[register2]{
            self.program_counter += 4;
        }else{
            self.program_counter += 2;
        }
    }

    fn set_register(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        let value = (opcode & 0x00FF) as u8;
        self.registers[register] = value;
        self.program_counter += 2;
    }

    fn add_to_register(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        let value = (opcode & 0x00FF) as u8;
        self.registers[register] = self.registers[register].wrapping_add(value);
        self.program_counter += 2;
    }

    fn set_register_register(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[register1] = self.registers[register2];
        self.program_counter += 2;
    }

    fn set_register_or(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[register1] = self.registers[register1] | self.registers[register2];
        self.program_counter += 2;
    }

    fn set_register_and(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[register1] = self.registers[register1] & self.registers[register2];
        self.program_counter += 2;
    }

    fn set_register_xor(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        self.registers[register1] = self.registers[register1] ^ self.registers[register2];
        self.program_counter += 2;
    }

    fn add_register_register(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        let (result, overflow) = self.registers[register1].overflowing_add(self.registers[register2]);
        self.registers[register1] = result;
        self.registers[0xF] = if overflow {1} else {0};
        self.program_counter += 2;
    }

    fn sub_register_register(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        let (result, overflow) = self.registers[register1].overflowing_sub(self.registers[register2]);
        self.registers[register1] = result;
        self.registers[0xF] = if overflow {0} else {1};
        self.program_counter += 2;
    }

    fn shift_right(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        self.registers[0xF] = self.registers[register] & 0x1;
        self.registers[register] >>= 1;
        self.program_counter += 2;
    }

    fn sub_register_register_reverse(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        let (result, overflow) = self.registers[register2].overflowing_sub(self.registers[register1]);
        self.registers[register1] = result;
        self.registers[0xF] = if overflow {0} else {1};
        self.program_counter += 2;
    }

    fn shift_left(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        self.registers[0xF] = self.registers[register] >> 7;
        self.registers[register] <<= 1;
        self.program_counter += 2;
    }

    fn skip_if_not_equal_register(&mut self, opcode: u16){
        let register1 = ((opcode & 0x0F00) >> 8) as usize;
        let register2 = ((opcode & 0x00F0) >> 4) as usize;
        if self.registers[register1] != self.registers[register2]{
            self.program_counter += 4;
        }else{
            self.program_counter += 2;
        }
    }

    fn set_index_register(&mut self, opcode: u16){
        self.index_register = opcode & 0x0FFF;
        self.program_counter += 2;
    }

    fn jump_to_address_plus_register(&mut self, opcode: u16){
        let address = opcode & 0x0FFF;
        self.program_counter = address + self.registers[0] as u16;
    }

   fn set_register_random(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        let value = (opcode & 0x00FF) as u8;
        let random = random::<u8>();
        self.registers[register] = random & value;
        self.program_counter += 2;
    }

   fn draw_sprite(&mut self, opcode: u16, display: &mut Display, ram: &RAM, canvas: &mut Canvas<Window>){
       let x = self.registers[((opcode & 0x0F00) >> 8) as usize] as usize;
       let y = self.registers[((opcode & 0x00F0) >> 4) as usize] as usize;
       let height = (opcode & 0x000F) as usize;
       self.registers[0xF] = 0;
       let mut sprite = vec![0; height];
       for yline in 0..height {
           sprite[yline] = ram.read(self.index_register + yline as u16);
       }
       let collision = display.draw(x, y, &sprite, canvas);
       self.registers[0xF] = collision as u8;
       display.set_draw_flag(true);
       self.program_counter += 2;
   }


    fn skip_if_key_pressed(&mut self, opcode: u16, input: &mut Input){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        if input.get_keys()[self.registers[register] as usize] != 0 {
            self.program_counter += 4;
        }else{
            self.program_counter += 2;
        }
    }

    fn skip_if_key_not_pressed(&mut self, opcode: u16, input: &mut Input){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        if input.get_keys()[self.registers[register] as usize] == 0 {
            self.program_counter += 4;
        }else{
            self.program_counter += 2;
        }
    }

    fn set_register_delay_timer(&mut self, opcode: u16, timer: &mut Timer){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        self.registers[register] = timer.read_delay_timer();
        self.program_counter += 2;
    }

    fn wait_for_key_press(&mut self, opcode: u16, input: &mut Input) {
        let register = ((opcode & 0x0F00) >> 8) as usize;
        let mut key_pressed = false;
        let mut key_value = 0;

        while !key_pressed {
            let keys = input.get_keys();
            for i in 0..keys.len() {
                if keys[i] == 1 {
                    key_pressed = true;
                    key_value = i as u8;
                    break;
                }
            }
        }

        self.registers[register] = key_value;
        self.program_counter += 2;
    }


   fn set_delay_timer_register(&mut self, opcode: u16, timer: &mut Timer){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        timer.set_delay_timer(self.registers[register]);
        self.program_counter += 2;
    }

    fn set_sound_timer_register(&mut self,opcode: u16,timer: &mut Timer, sound: &mut Sound){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        timer.set_sound_timer(self.registers[register]);
        self.program_counter += 2;
    }

    fn add_index_register_register(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        self.index_register += self.registers[register] as u16;
        self.program_counter += 2;
    }

    fn set_index_register_sprite(&mut self, opcode: u16){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        self.index_register = self.registers[register] as u16 * 5;
        self.program_counter += 2;
    }

    fn store_bcd(&mut self,opcode: u16, ram: &mut RAM){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        let value = self.registers[register];
        ram.write(self.index_register, value / 100);
        ram.write(self.index_register + 1, (value / 10) % 10);
        ram.write(self.index_register + 2, (value % 100) % 10);
        self.program_counter += 2;
    }

    fn store_registers(&mut self, opcode: u16, ram: &mut RAM){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        for i in 0..=register{
            ram.write(self.index_register + i as u16, self.registers[i]);
        }
        self.program_counter += 2;
    }

    fn load_registers(&mut self, opcode: u16, ram: &mut RAM){
        let register = ((opcode & 0x0F00) >> 8) as usize;
        for i in 0..=register{
            self.registers[i] = ram.read(self.index_register + i as u16);
        }
        self.program_counter += 2;
    }
}
