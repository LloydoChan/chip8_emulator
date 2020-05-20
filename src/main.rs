extern crate sdl2;
use sdl2::Sdl;
use sdl2::video::{self, Window, WindowBuilder, WindowContext, WindowBuildError};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::pixels::Color;
use std::time::{Duration, Instant};


const WIDTH : u32 = 360;
const HEIGHT: u32 = 240;

fn init_window(context : &mut Sdl, width : u32, height : u32) -> Result<Window, WindowBuildError> {
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem.window("rust demo", width * 4, height * 4)
        .position_centered()
        .build();

    window
}

fn update_pix(numPix: usize, i : &mut u8, pixData: &mut Box<[u8]>){
    for pix in 0..numPix {
        pixData[pix * 3] = *i;
        pixData[pix * 3 + 1] = 68;
        pixData[pix * 3 + 2] = 132;
        *i = (*i + 1) % 255;
    }
}

fn main() {
    let mut sdl_context = sdl2::init().unwrap();
    let win = init_window(&mut sdl_context, WIDTH, HEIGHT);

    let mut canvas = win.unwrap().into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut frameBuffers : Vec<Texture> = vec![];

    let numPix = WIDTH * HEIGHT;

    for i in 0..2{
        frameBuffers.push(texture_creator
            .create_texture_target(texture_creator.default_pixel_format(), WIDTH, HEIGHT)
            .unwrap());
    }

    // create pixel data
    let mut pixData : Box<[u8]> = vec![0; WIDTH as usize * HEIGHT as usize * 3].into_boxed_slice();
    
    
    let mut i = 0;
    let mut frame_index = 0;
    loop {
        //beginning of loop
        let start = Instant::now();
        update_pix(numPix as usize, &mut i, &mut pixData);
        let texRef = &mut frameBuffers[frame_index];

        texRef.update(None, &pixData, 256);
        canvas.copy(&texRef, None, None);

        canvas.present();
        frame_index = ( frame_index + 1 ) % 2;

        i += 1;
        let millis = start.elapsed().as_millis() as u32;
        ::std::thread::sleep(Duration::new(0, 33_000 - millis));
    };
}