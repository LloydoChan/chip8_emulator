extern crate sdl2;
use sdl2::Sdl;
use sdl2::video::{self, Window, WindowBuilder, WindowContext, WindowBuildError};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::pixels::{Color, PixelFormatEnum};
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

use Chip8::chip8;


const WIDTH : u32 = 64;
const HEIGHT: u32 = 32;

fn init_window(context : &mut Sdl, width : u32, height : u32) -> Result<Window, WindowBuildError> {
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem.window("chip-8 demo", width * 8, height * 8)
        .position_centered()
        .build();

    window
}

fn expand_vram(cpu : &chip8::Chip_HW, pixData : &mut [u8]){
    let numPix = HEIGHT * WIDTH;
    let vram = cpu.get_vram();

    for i in 0..HEIGHT {
        for j in 0..WIDTH{
            // let read_offset = i * WIDTH + j;
             let write_offset = 0;//read_offset * 4;
            // let value = vram[read_offset as usize];

            pixData[write_offset as usize] = 0;
            pixData[(write_offset + 1) as usize] = 0;
            pixData[(write_offset + 2) as usize] = 0;
            pixData[(write_offset + 3) as usize] = 0;
        }
    }
}

fn main() {

    // load ROM
    let mut args = env::args();
    //let rom_file_name = args.nth(1).unwrap();
    let rom_file_name = args.nth(1).unwrap();    

    let rom = load_binary(rom_file_name);
    //let bin_rom = load_binary(rom_file_name);
    // create chip9
    let mut myChip8 : chip8::Chip_HW = chip8::Chip_HW::new();

    // do SDL init stuff
    let mut sdl_context = sdl2::init().unwrap();
    let win = init_window(&mut sdl_context, WIDTH, HEIGHT);
    let unrapped = win.unwrap();

    let mut canvas = unrapped.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut frameBuffers : Vec<Texture> = vec![];


    for i in 0..2{
        frameBuffers.push(texture_creator
            .create_texture_target(PixelFormatEnum::RGB888, WIDTH, HEIGHT)
            .unwrap());
    }

    myChip8.load_rom(&rom);

    
    // create pixel data
    let mut pixData : Box<[u8]> = vec![0; WIDTH as usize * HEIGHT as usize * 4 ].into_boxed_slice();
    let mut frame_index = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        //beginning of loop
        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode), .. } => {
                   key_response(&mut myChip8, Keycode, 1)
                },
                Event::KeyUp { keycode: Some(Keycode), ..} => {
                    key_response(&mut myChip8, Keycode, 0)
                }
                _ => {}
            }
        }

        myChip8.run();
        let vram = &myChip8;
        expand_vram(&vram, &mut pixData);
        let texRef = &mut frameBuffers[frame_index];

        texRef.update(None, &pixData, (WIDTH * 4) as usize);
        canvas.copy(&texRef, None, None);

        canvas.present();
        frame_index = ( frame_index + 1 ) % 2;

        ::std::thread::sleep(Duration::new(0, 16_000_000));
    };
}

fn load_binary<P: AsRef<Path>>(path : P) -> Box<[u8]> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf);
    file_buf.into_boxed_slice()
}

fn key_response(myChip8 : &mut chip8::Chip_HW, keycode : Keycode, up : u8){
    match keycode {
        Keycode::Num0 => myChip8.set_key(0, up),
        Keycode::Num1 => myChip8.set_key(1, up),
        Keycode::Num2 => myChip8.set_key(2, up),
        Keycode::Num3 => myChip8.set_key(3, up),
        Keycode::Num4 => myChip8.set_key(4, up),
        Keycode::Num5 => myChip8.set_key(5, up),
        Keycode::Num6 => myChip8.set_key(6, up),
        Keycode::Num7 => myChip8.set_key(7, up),
        Keycode::Num8 => myChip8.set_key(8, up),
        Keycode::Num9 => myChip8.set_key(9, up),
        Keycode::A => myChip8.set_key(10, up),
        Keycode::B => myChip8.set_key(11, up),
        Keycode::C => myChip8.set_key(12, up),
        Keycode::D => myChip8.set_key(13, up),
        Keycode::E => myChip8.set_key(14, up),
        Keycode::F => myChip8.set_key(15, up),
        _ => println!("nowt")
   }
}