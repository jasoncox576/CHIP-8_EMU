use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use ggez::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::audio::SoundSource;
use ggez::nalgebra as na;
use std::thread;


mod emu;



fn play_sound(timer : &mut f32, ctx : &mut Context) {
    let sound_path = Path::new("/beep.wav");
    let mut beep = audio::Source::new(ctx, sound_path).unwrap();
    beep.play();

    //let seconds: u64 = (*timer / 60).round() as u64;

    let millis : u64 = (*timer / 0.06).round() as u64;
    let duration  = Duration::from_millis(millis);
    thread::sleep(duration);
    *timer = 0.0;
}


fn get_keypress(ctx: &mut Context) -> u8 {

    let mut keycode : u8 = 255;

    if keyboard::is_key_pressed(ctx, KeyCode::Key1) {
        keycode = 0x1;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::Key2) {
        keycode = 0x2;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::Key3) {
        keycode = 0x3;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::Key4) {
        keycode = 0xC;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::Q) {
        keycode = 0x4;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::W) {
        keycode = 0x5;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::E) {
        keycode = 0x6;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::R) {
        keycode = 0xD;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::A) {
        keycode = 0x7;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::S) {
        keycode = 0x8;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::D) {
        keycode = 0x9;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::F) {
        keycode = 0xE;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::Z) {
        keycode = 0xA;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::X) {
        keycode = 0x0;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::C) {
        keycode = 0xB;
    }
    else if keyboard::is_key_pressed(ctx, KeyCode::V) {
        keycode = 0xF;
    }
    keycode
}




struct State {
    dt: std::time::Duration,
    pc : usize,
    rf : emu::Chip,
    current_instruction: u16,
    delay : u64,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        // necessary to control extremely fast fps
        timer::sleep(Duration::from_millis(self.delay));

    
        self.rf.delay_timer -= 0.06 * (self.dt.as_millis() as f32);
        if self.rf.delay_timer <= 0.0 { 
            self.rf.delay_timer = 0.0;
        }


        
        if !self.rf.keyboard_flag {
            let b1 : u16 = (self.rf.memory[self.pc] as u16 ) << 8;
            let b2 : u16 = self.rf.memory[self.pc + 1] as u16;

            self.current_instruction = b1 | b2;
            let next_instruction = b1 | b2;
            emu::decode(&mut self.rf, &mut self.pc, next_instruction, ctx);
        }



        let keycode : u8 = get_keypress(ctx);

        if keycode != 100 {
            let vx : usize = ((self.current_instruction & 0x0F00) >> 8) as usize;
            
            let full_opcode : u16 = (self.current_instruction & 0xF0FF);

            match full_opcode {

                0xE09E => {
                    if keycode == self.rf.data_regs[vx] {
                        self.pc += 2;
                    }
                }

                0xE0A1 => {
                    if keycode != self.rf.data_regs[vx] {
                        self.pc += 2;
                    }
                }

                0xF00A => {
                    self.rf.data_regs[vx] = keycode;
                }
                _ => (),
            }
            
            if self.rf.keyboard_flag {
                self.rf.keyboard_flag = false;
                self.pc += 2;
            }
        }
        if self.rf.sound_timer > 0.0 {
            play_sound(&mut self.rf.sound_timer, ctx);
        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if _keycode == KeyCode::Escape {
            event::quit(ctx);
        }
    
        if (_keycode == KeyCode::Tab) && (self.delay < 251) {
            self.delay += 5;            
        }

        if (_keycode == KeyCode::LShift) && (self.delay > 4) {
            self.delay -= 5;            
        }

    }





    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clears the screen to black
        if(self.rf.cls_flag) {
            graphics::clear(ctx, graphics::BLACK);
        }
        if(self.rf.draw_flag) {
            let mut screen_bitmap_rgba : [u8 ; 8192] = [0 ; 8192];


            for i in 0..2048 {
                let j = (4*i) as usize;

                let mut set_val = 0;

                if(self.rf.screen_bitmap[i] != 0) {
                    set_val = 255;
                }
                //screen_bitmap_rgba[j] = set_val;
                screen_bitmap_rgba[j+1] = set_val;
                //screen_bitmap_rgba[j+2] = set_val;
                // alpha channel
                screen_bitmap_rgba[j+3] = 255;
            }
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

    for i in (0..byte_buffer.len()-1).step_by(2) {
        memory[512+i] = byte_buffer[i];
        memory[512+i+1] = byte_buffer[i+1];
    }


    emu::memset_sprite_data(&mut memory);

    let mut rf = emu::Chip {
        data_regs: [0 ; 16],
        address_reg: 0,
        screen_bitmap: [0 ; 2048],
        cls_flag: false,
        draw_flag: false,
        keyboard_flag: false,
        delay_timer: 0.0,
        sound_timer: 0.0,
        memory: memory,
        stack: [0 ; 16],
        stack_pointer: 0,
    };



    let state = &mut State { dt: std::time::Duration::new(0, 0), pc : 512, rf : rf, current_instruction : 0, delay: 1};


    let window_cfg : conf::WindowMode = conf::WindowMode {
        width : 960.0,
        height : 480.0,
        maximized: false,
        fullscreen_type: conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("CHIP8-EMU", "jasoncox")
        .conf(c)
        .build()
        .unwrap();

    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);


    let rect : graphics::Rect = graphics::Rect::new(0.0, 0.0, 64.0, 32.0);

    graphics::set_mode(ctx, window_cfg);
    graphics::set_screen_coordinates(ctx, rect);
    graphics::set_window_title(ctx, "CHIP-8 EMU");





    // run main loop
    event::run(ctx, event_loop, state).unwrap();
}
