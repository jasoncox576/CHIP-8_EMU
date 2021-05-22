use std::num::Wrapping;
use rand::Rng;
use ggez::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;






// struct containing all the registers,
// memory, stack, etc.
pub struct Chip {

    // contains the 16 8-bit registers, V0-VF
    pub data_regs: [u8 ; 16],

    // address register
    pub address_reg: u16,




    // easier to work with a flattened 32x64 map.
    pub screen_bitmap : [u8 ; 2048],



    // program memory
    pub memory : [u8 ; 4096],

    // call stack
    pub stack : [u16 ; 16],


    // set when screen should be cleared by ggez
    pub cls_flag : bool,

    // set when drawing
    pub draw_flag : bool,

    // set when waiting for keyboard input
    pub keyboard_flag : bool,
    

    pub delay_timer : f32,
    pub sound_timer : f32,

    pub stack_pointer : usize,

}






fn clear_screen(rf : &mut Chip) {
    rf.cls_flag = true;

    for i in 0..2048 {
        rf.screen_bitmap[i] = 0;
    }
}





fn jump(pc : &mut usize, ins : u16) {
    *pc = (ins & 0x0FFF) as usize;
}


/*
initializes the first 80 bytes of memory with the sprite data
for the characters 0 thru F
*/
pub fn memset_sprite_data(mem : &mut [u8 ; 4096]) {


    // "0"
    mem[0] = 0xF0;
    mem[1] = 0x90;
    mem[2] = 0x90;
    mem[3] = 0x90;
    mem[4] = 0xF0;
    
    // "1"
    mem[5] = 0x20;
    mem[6] = 0x60;
    mem[7] = 0x20;
    mem[8] = 0x20;
    mem[9] = 0x70;

    // "2"
    mem[10] = 0xF0;
    mem[11] = 0x10;
    mem[12] = 0xF0;
    mem[13] = 0x80;
    mem[14] = 0xF0;

    // "3"
    mem[15] = 0xF0;
    mem[16] = 0x10;
    mem[17] = 0xF0;
    mem[18] = 0x10;
    mem[19] = 0xF0;

    // "4"
    mem[20] = 0x90;
    mem[21] = 0x90;
    mem[22] = 0xF0;
    mem[23] = 0x10;
    mem[24] = 0x10;

    // "5"
    mem[25] = 0xF0;
    mem[26] = 0x80;
    mem[27] = 0xF0;
    mem[28] = 0x10;
    mem[29] = 0xF0;


    // "6"
    mem[30] = 0xF0;
    mem[31] = 0x80;
    mem[32] = 0xF0;
    mem[33] = 0x90;
    mem[34] = 0xF0;

    // "7"
    mem[35] = 0xF0;
    mem[36] = 0x10;
    mem[37] = 0x20;
    mem[38] = 0x40;
    mem[39] = 0x40;

    // "8"
    mem[40] = 0xF0;
    mem[41] = 0x90;
    mem[42] = 0xF0;
    mem[43] = 0x90;
    mem[44] = 0xF0;


    // "9"
    mem[45] = 0xF0;
    mem[46] = 0x90;
    mem[47] = 0xF0;
    mem[48] = 0x10;
    mem[49] = 0xF0;


    // "A"
    mem[50] = 0xF0;
    mem[51] = 0x90;
    mem[52] = 0xF0;
    mem[53] = 0x90;
    mem[54] = 0x90;


    // "B"
    mem[55] = 0xE0;
    mem[56] = 0x90;
    mem[57] = 0xE0;
    mem[58] = 0x90;
    mem[59] = 0xE0;


    // "C"
    mem[60] = 0xF0;
    mem[61] = 0x80;
    mem[62] = 0x80;
    mem[63] = 0x80;
    mem[64] = 0xF0;


    // "D"
    mem[65] = 0xE0;
    mem[66] = 0x90;
    mem[67] = 0x90;
    mem[68] = 0x90;
    mem[69] = 0xE0;


    // "E"
    mem[70] = 0xF0;
    mem[71] = 0x80;
    mem[72] = 0xF0;
    mem[73] = 0x80;
    mem[74] = 0xF0;


    // "F"
    mem[75] = 0xF0;
    mem[76] = 0x80;
    mem[77] = 0xF0;
    mem[78] = 0x80;
    mem[79] = 0x80;
}



pub fn decode(rf : &mut Chip, pc : &mut usize, ins: u16, ctx: &Context) {

    rf.cls_flag = false;
    rf.draw_flag = false;
    rf.keyboard_flag = false;

    let opcode : u8 = ((ins & 0xF000) >> 12) as u8;

    println!("pc: {} : {:#06x}", pc, ins);


    match opcode {


        0x0 => {

            if(ins ==  0x00E0) {
                clear_screen(rf);
            }

            if (ins == 0x00EE) && (rf.stack_pointer > 0)  {
                rf.stack_pointer -= 1;
                *pc = rf.stack[rf.stack_pointer] as usize;
                rf.stack[rf.stack_pointer] = 0;
            }
        },

        0x1 => jump(pc, ins),

        0x2 => {

            if rf.stack_pointer < 16 {
                let nnn : u16 = (ins &  0x0FFF);
                rf.stack[rf.stack_pointer] = *pc as u16;
                *pc = nnn.into();
                rf.stack_pointer += 1;
                return;
            }
        }

        0x3 => {
            // skip the next instruction if vx and nn are equal
            let vx : usize = ((ins & 0x0F00) >> 8) as usize;
            let nn : u8 = (ins & 0x00FF) as u8;

            if(rf.data_regs[vx] == nn) {
                *pc += 2;
            }
        }
        
        0x4 => {
            // skip the next instruction if vx and nn aren't equal
            let vx : usize = ((ins & 0x0F00)  >> 8) as usize;
            let nn : u8 = (ins & 0x00FF) as u8;

            if(rf.data_regs[vx] != nn) {
                *pc += 2;
            }
        }


        0x5 => {

            // skip the next instruction if vx and vy are equal
            let vx : usize = ((ins & 0x0F00) >> 8) as usize;
            let vy : usize = ((ins & 0x00F0) >> 4) as usize;

            let equal : bool = rf.data_regs[vx] == rf.data_regs[vy];
            if ((ins & 0xF) == 0) && equal {
               *pc += 2; 
            }
        }

        0x6 => {
            // constant-value update
            // set vx to nn
            let vx : usize = ((ins & 0x0F00) >> 8) as usize;
            let nn : u8 = (ins & 0x00FF) as u8;

            rf.data_regs[vx] = nn;
        },

        0x7 => {
            // constant-value update
            // add nn to vx
            let vx : usize = ((ins & 0x0F00) >> 8) as usize;
            let nn : u8 = (ins & 0x00FF) as u8;

            let data_wrapped  = Wrapping(rf.data_regs[vx]);
            let nn_wrapped = Wrapping(nn);

            rf.data_regs[vx] = (data_wrapped + nn_wrapped).0;


        },

        0x8 => {
            let xop : u8 = (ins & 0xF) as u8;
            
            let vx : usize = ((ins & 0x0F00) >> 8) as usize;
            let vy : usize = ((ins & 0x00F0) >> 4) as usize;

            match xop {

                // assignment
                0x0 => {
                    rf.data_regs[vx] = rf.data_regs[vy];
                }

                0x1 => {
                    rf.data_regs[vx]  = (rf.data_regs[vx] | rf.data_regs[vy]);
                }

                0x2 => {
                    rf.data_regs[vx]  = (rf.data_regs[vx] & rf.data_regs[vy]);
                }

                0x3 => {
                    rf.data_regs[vx]  = (rf.data_regs[vx] ^ rf.data_regs[vy]);
                }

                0x4 => {
                    if (rf.data_regs[vy] as u16) + (rf.data_regs[vx] as u16) > 255 {
                        rf.data_regs[15] = 1;
                    }
                    else {rf.data_regs[15] = 0;}

                    let x_wrapped = Wrapping(rf.data_regs[vx]);
                    let y_wrapped = Wrapping(rf.data_regs[vy]);

                    rf.data_regs[vx] = (x_wrapped + y_wrapped).0;
                }

                0x5 => {
                    if rf.data_regs[vy] > rf.data_regs[vx] {
                        rf.data_regs[15] = 0;
                    }
                    else {rf.data_regs[15] = 1;}

                    let x_wrapped = Wrapping(rf.data_regs[vx]);
                    let y_wrapped = Wrapping(rf.data_regs[vy]);

                    rf.data_regs[vx] = (x_wrapped - y_wrapped).0;
                    


                }

                // NOTE: ambiguous instruction depending on choice of implementation
                0x6 => {
                    // NOTE: Comment out this line depending on
                    // whether or not your program needs it.
                    //rf.data_regs[vx] = rf.data_regs[vy];

                    rf.data_regs[15] = 0b00000001 & rf.data_regs[vx];
                    rf.data_regs[vx] = rf.data_regs[vx] >> 1;
                }

                0x7 => {
                    // sets vx to vy - vx. vf set to 0 if borrow, 1 otherwise.
                    if rf.data_regs[vy] < rf.data_regs[vx] {
                        rf.data_regs[15] = 0;
                    }
                    else {rf.data_regs[15] = 1;}

                    let y_wrapped = Wrapping(rf.data_regs[vy]);
                    let x_wrapped = Wrapping(rf.data_regs[vx]);

                    rf.data_regs[vx] = (y_wrapped - x_wrapped).0;
                }

                // NOTE: ambiguous instruction depending on choice of implementation
                0xE => {
                    // NOTE: Comment out this line depending on
                    // whether or not your program needs it.
                    //rf.data_regs[vx] = rf.data_regs[vy];
                    rf.data_regs[15] = (0b10000000 & rf.data_regs[vx]) >> 7;
                    rf.data_regs[vx] = rf.data_regs[vx] << 1;
                }




                _ => {
                    //println!("Invalid instruction! (nested)");
                }

            }




        }

        0x9 => {
            if (ins & 0x000F) == 0 {
                let vx : usize = ((ins & 0x0F00) >> 8) as usize;
                let vy : usize = ((ins & 0x00F0) >> 4) as usize;

                // skips the next instruction if vx != vy
                if rf.data_regs[vx] != rf.data_regs[vy] {
                    *pc += 2;
                }
            }
            else {println!("Invalid instruction");}
        }


        0xA => {

            // sets the index address register to nnn
            let nnn : u16 = (ins & 0x0FFF) as u16;
            rf.address_reg = nnn;
        },

        0xB => {
            // jumps to address nnn + v0
            let nnn : u16 = (ins & 0x0FFF) as u16;
            *pc = (nnn + (rf.data_regs[0] as u16)) as usize;
        }

        0xC => {

            let vx : usize = ((ins & 0x0F00) >> 8) as usize;
            let nn : u8 = (ins & 0x00FF) as u8;

            // sets vx to the bitwise 'and' of nn with a random number 0-255


            let mut rng = rand::thread_rng();
            let random_val : u8 = rng.gen_range(0..=251);

            rf.data_regs[vx] = nn & random_val;
        }

        0xD => {

            rf.draw_flag = true;
        
            // draw stuff on the screen
            let vx : usize = ((ins & 0x0F00) >> 8) as usize;
            let vy : usize = ((ins & 0x00F0) >> 4) as usize;
            let n : u8 = (ins & 0x000F) as u8;


            let x = (rf.data_regs[vx] % 64);
            let y = (rf.data_regs[vy] % 32);
            
            rf.data_regs[15] = 0;

            //println!("x: {}, y: {}", x,y);
            
            let mut pixels_turned_off : bool = false;

            let start_address : usize = rf.address_reg as usize;

            for row in 0..n {
                for col in 0..8 {
                    if ((x+col) > 63) || ((y + row) > 31) {
                        break; 
                    }

                    // TODO fix this line- bad form.
                    let screen_idx : usize = (((y+row) as usize)*64 + (x+col) as usize) as usize;



                    let offset_address : usize = (start_address + (row as usize)) as usize;

                    let bit = 2u8.pow((7-col) as u32);

                    // TODO this is messy, fix.

                    let old_val = rf.screen_bitmap[screen_idx];

                    let mut sprite_val = 0;
                    if (bit & rf.memory[offset_address]) > 0 {
                        sprite_val = 255;
                    }

                    rf.screen_bitmap[screen_idx] = rf.screen_bitmap[screen_idx] ^ sprite_val;
                    let new_val = rf.screen_bitmap[screen_idx];

                    if new_val < old_val {
                        pixels_turned_off = true;
                    }
                }
            }
            
            if pixels_turned_off {
                // set vf register
                rf.data_regs[15] = 1;
            }

            //println!("Drawing on screen");
    
        },

        0xE => {
            
            let xop : u8 = (ins & 0x00FF) as u8;

            match xop {
            
                // skips next instruction if pressed keycode == contents of vx
                0x9E => {
                    // nothing here
                }
                // skips if opposite of above
                0xA1 => {
                    // nothing here
                }
                _ => println!("Invalid instruction!"),
            }
        }

        0xF => {

            let xop : u8 = (ins & 0x00FF) as u8;
            
            match xop {
                
                // sets vx to delay timer value
                0x07 => {
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;
                    rf.data_regs[vx] = rf.delay_timer.round() as u8;
                }

                // gets next keypress and stores in vx
                0x0A => {
                    /*
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;
                    let keycode = get_keypress(ctx, true);
                    rf.data_regs[vx] = keycode;
                    */
                    rf.keyboard_flag = true;
                    return;
                }


                // sets delay timer to val in vx
                0x15 => {
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;
                    rf.delay_timer = rf.data_regs[vx] as f32;
                }

                // sets sound timer to value in vx
                0x18 => {
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;
                    rf.sound_timer = rf.data_regs[vx] as f32;
                }

                // increments address register by value in vx
                0x1E => {
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;
                    rf.address_reg += rf.data_regs[vx] as u16;
                }

                // sets address register to address of sprite specified by val in vx
                0x29 => {
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;
                    rf.address_reg =  (5 * rf.data_regs[vx]).into();
                }
                


                0x33 => {
                    // stores value in vx as a binary-coded decimal
                    // in memory pointed to by address register
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;

                    let mut x : u8 = rf.data_regs[vx];
                    //TODO not sure if this range is correct.. check this
                    for i in (0..3).rev() {
                        
                        rf.memory[(rf.address_reg + i) as usize] = (x % 10);
                        x /= 10;
                    }
                }




                // store values of v0-vx into memory at address register
                0x55 => {
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;

                    for i in 0..(vx+1) {
                       let offset_idx = (rf.address_reg as usize) + i;
                       rf.memory[offset_idx] = rf.data_regs[i];
                    }
                }

                // load from memory into v0-vx at address register
                0x65 => {
                    let vx : usize = ((ins & 0x0F00) >> 8) as usize;

                    for i in 0..(vx+1) {
                       let offset_idx = (rf.address_reg as usize) + i;
                       rf.data_regs[i] = rf.memory[offset_idx];
                    }
                }


                _ => println!("Unknown instruction!"),


            }





        }

        _ => println!("ERR: unknown instruction! {:#06x} ", ins)
    }


    

    // TODO only increment pc if there wasn't a jump

    if !(opcode == 0x1) {*pc += 2};

}



