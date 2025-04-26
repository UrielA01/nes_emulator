use std::env;
use std::fs;

use bus::Bus;
use cpu::cpu::CPU;
use cpu::memory::Mem;
use rand::Rng;
use rom::Rom;
use sdl::sdl::{handle_user_input, read_screen_state};
use sdl2::pixels::PixelFormatEnum;

pub mod bus;
pub mod cpu;
pub mod rom;
pub mod sdl;

fn main() {
    const WINDOW_HEIGHT: u32 = (32.0 * 10.0) as u32;
    const WINDOW_WIDTH: u32 = (32.0 * 10.0) as u32;

    // Load the game
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let game_file: Vec<u8> = fs::read(file_path).unwrap();
    let rom = Rom::new(&game_file).unwrap();
    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();

    // Init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();

    let mut screen_state = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::rng();

    cpu.run_with_callback(move |cpu| {
        handle_user_input(cpu, &mut event_pump);
        cpu.mem_write(0xfe, rng.random_range(1..16));

        if read_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        ::std::thread::sleep(std::time::Duration::new(0, 70_00));
    });
}
