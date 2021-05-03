use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use ggez::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::nalgebra as na;


mod emu;


/*

fn wait_for_keypress(rf : &mut emu::RegisterFile) {

    loop {

        if keyboard::is_key_pressed(ctx, Key




    }
}
*/


/*
fn play_sound(timer : u8, ctx : &mut Context) {

    // todo random data, probably change later
    static sound_bytes : [u8 ; 9] = [255, 255, 255, 255, 255, 255, 255, 255, 255]; 
    static beep : audio::SoundData = audio::SoundData::from_bytes(&sound_bytes[..]);
    static source : audio::Source = audio::Source::from_data(ctx, beep);

    

    
    // sound timer should decrement at a rate of 60hz
    while timer > 0 {
        source.repeat();
        //timer -= 60
    }

}
*/


struct State {
    dt: std::time::Duration,
    pc : usize,
    rf : emu::RegisterFile,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);

        let b1 : u16 = (self.rf.memory[self.pc] as u16 ) << 8;
        let b2 : u16 = self.rf.memory[self.pc + 1] as u16;

        let next_ins : u16 = b1 | b2;

        emu::decode(&mut self.rf, &mut self.pc, next_ins);

        // TODO probably need a separate thread here for sound
        /*
        if rf.sound_timer > 0 {
            play_sound(rf.sound_timer, ctx);
        }
        */




        /*
        if rf.keyboard_flag {
            wait_for_keypress(&mut self.rf);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Key1)
        */

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clears the screen to black
        if(self.rf.cls_flag) {
            graphics::clear(ctx, graphics::BLACK);
        }
        if(self.rf.draw_flag) {
            let mut screen_bitmap_rgba : [u8 ; 8192] = [0 ; 8192];


            for i in 0..2047 {
                let j = (4*i) as usize;

                let mut set_val = 0;

                if(self.rf.screen_bitmap[i] != 0) {
                    set_val = 255;
                }
                screen_bitmap_rgba[j] = set_val;
                screen_bitmap_rgba[j+1] = set_val;
                screen_bitmap_rgba[j+2] = set_val;
                // alpha channel
                screen_bitmap_rgba[j+3] = 255;
            }


            //let screen_slice = &self.rf.screen_bitmap[..];
            let screen_slice = &screen_bitmap_rgba[..];



            //TODO bottom right corner isn't set for some reason


            let screen : graphics::Image = ggez::graphics::Image::from_rgba8(ctx, 64, 32, screen_slice).unwrap(); 
            graphics::draw(ctx, &screen, (na::Point2::new(0.0,0.0),))?;

            // actually puts stuff on the screen
            graphics::present(ctx)?;
        }


        Ok(())
    }
}







fn main() {

    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);

    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {} : {}", display, why),
        Ok(file) => file,
    };

	let mut byte_buffer = Vec::new();
	file.read_to_end(&mut byte_buffer);


    let mut memory : [u8 ; 4096] = [0 ; 4096];

    for i in (0..byte_buffer.len()+1).step_by(2) {
        memory[512+i] = byte_buffer[i];
        memory[512+i+1] = byte_buffer[i+1];
    }


    emu::memset_sprite_data(&mut memory);

    let mut rf = emu::RegisterFile {
        data_regs: [0 ; 16],
        address_reg: 0,
        screen_bitmap: [0 ; 2048],
        cls_flag: false,
        draw_flag: false,
        keyboard_flag: false,
        delay_timer: 0,
        sound_timer: 0,
        memory: memory,
        stack: [0 ; 16],
        stack_pointer: 0,
    };


    let state = &mut State { dt: std::time::Duration::new(0, 0), pc : 512, rf : rf};

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("CHIP8-EMU", "jasoncox")
        .conf(c)
        .build()
        .unwrap();



    let rect : graphics::Rect = graphics::Rect::new(0.0, 0.0, 64.0, 32.0);

    graphics::set_screen_coordinates(ctx, rect);

    graphics::set_window_title(ctx, "CHIP-8 EMU");





    // run main loop
    event::run(ctx, event_loop, state).unwrap();
}
