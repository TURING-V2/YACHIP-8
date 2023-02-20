use std::fs::read_dir;
use std::thread;
use std::time::Duration;
use std::io::stdin;

mod ram;
mod cpu;
mod input;
mod display;
mod timer;
mod sound;

use ram::RAM;
use cpu::CPU;
use input::Input;
use display::Display;
use timer::Timer;
use sound::Sound;


pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const PIXEL_SIZE: u32 = 10;

const RAM_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

fn main(){
    println!("Intializing...");

    let mut cpu = CPU::new();
    let mut ram = RAM::new();

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new();

    let mut input = Input::new(&sdl_context);

    let mut timer = Timer::new();
    let mut sound = Sound::new(&sdl_context);

    ram.load_fontset();

    let mut roms = Vec::new();
    for entry in read_dir("roms/").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            roms.push(path);
        }
    }

    println!("Available roms:");
    for (i, rom) in roms.iter().enumerate() {
        println!("{}: {}", i, rom.file_name().unwrap().to_str().unwrap());
    }

    let mut rom_index = String::new();
    println!("Select rom:");
    stdin().read_line(&mut rom_index).unwrap();
    let rom_index: usize = rom_index.trim().parse().unwrap();

    ram.load_rom(&roms[rom_index]);

    println!("Rom loaded..");

    let mut canvas = display.create_window_and_draw_to_screen(&sdl_context);

    loop{
        canvas.present();
        cpu.cycle(&mut ram, &mut display, &mut input, &mut timer, &mut sound, &mut canvas);
        sound.play_sound(&mut timer);
        thread::sleep(Duration::from_millis(1));
    }
}
