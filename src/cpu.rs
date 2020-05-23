use crate::chip8::hw_bundle;

const VF : usize = 15;

const debugOn : bool = false;

#[derive(Debug)]
pub struct CPU{
    registers : [u8; 16],
    address   : usize,
    pc_reg    : usize,
    stack     : Stack,
    halt      : bool
}

#[derive(Debug, Default)]
struct Stack{
    addresses : [u32; 16],
    stackpointer : u8
}

impl CPU{

    pub fn new() -> Self{
        CPU {
            registers : [0; 16],
            address   : 0,
            pc_reg    : 0x200,
            stack     : Stack::default(),
            halt      : false
        }
    }

    pub fn next_instruction(&mut self, chip : &mut hw_bundle){
        let bytecode1 : u8 = chip.read_ram_value(self.pc_reg); 
        let bytecode2 : u8 = chip.read_ram_value(self.pc_reg + 1 as usize);
        let bytecode = ((bytecode1 as u16) << 8) | bytecode2 as u16;

        self.decode_instruction(bytecode, chip);

        if !self.halt{
            self.pc_reg += 2;
        } 
    }

    fn decode_instruction(&mut self, bytecode : u16, chip : &mut hw_bundle){
        let first_nibble = (bytecode >> 12) as u8;

        match first_nibble {
            0 => self.deal_with_zero_nibble_codes(bytecode, chip),
            1 => self.deal_with_one_nibble_codes(bytecode, chip),
            2 => self.deal_with_two_nibble_codes(bytecode, chip),
            3 => self.deal_with_three_nibble_codes(bytecode, chip),
            4 => self.deal_with_four_nibble_codes(bytecode, chip),
            5 => self.deal_with_five_nibble_codes(bytecode, chip),
            6 => self.deal_with_six_nibble_codes(bytecode, chip),
            7 => self.deal_with_seven_nibble_codes(bytecode, chip),
            8 => self.deal_with_eight_nibble_codes(bytecode),
            9 => self.deal_with_nine_nibble_codes(bytecode, chip),
            0xA => self.deal_with_A_nibble_codes(bytecode),
            0xB => self.deal_with_B_nibble_codes(bytecode),
            0xC => self.deal_with_C_nibble_codes(bytecode),
            0xD => self.deal_with_D_nibble_codes(bytecode, chip),
            0xE => self.deal_with_E_nibble_codes(bytecode, chip),
            0xF => self.deal_with_F_nibble_codes(bytecode, chip),
            _ => panic!()
        }
    }

    fn deal_with_zero_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
            let byte = bytecode & 0xFF;
            match byte {
                0xE0 => self.clear_screen(chip),
                0xEE => self.return_from_subroutine(),
                _ => self.call_program(bytecode)
            }
    }

    fn clear_screen(&mut self, chip : &mut hw_bundle) {
        if debugOn{
            println!("clear screen!");
        }
        let num_bytes = 8 * 32; //8 bytes - 64 pixels across, 32 of these down
        for i in 0..num_bytes {
            chip.write_vram_value(i, 0);
        }
    }

    fn return_from_subroutine(&mut self) {
        // get address from stack pointer
        if debugOn{
            println!("return from subroutine");
        }
        self.stack.stackpointer -= 1;
        let addr = self.stack.addresses[self.stack.stackpointer as usize];
        self.pc_reg = addr as usize;
    }

    fn deal_with_one_nibble_codes(&mut self, bytecode : u16,chip : &mut hw_bundle){
        if debugOn{
            println!("jump");
        }
        let addr = bytecode & 0xFFF;

        if debugOn{
            println!("addr {:#x}", addr);
        }

        self.pc_reg = addr as usize;
    }

    fn deal_with_two_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        //calls a subroutine!
        if debugOn{
            println!("call sub routine");
        }
        let addr = bytecode & 0xFFF;
        // store current addr
        self.stack.addresses[self.stack.stackpointer as usize] = self.pc_reg as u32;
        self.stack.stackpointer += 1;

        self.pc_reg = addr as usize;
    }

    fn deal_with_three_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("skip next if equal");
        }
        let reg   = (bytecode >> 8) & 0xF;
        let value = bytecode  & 0xFF;
        if debugOn{
            println!("reg {} has {:#x} and test against val {:#x}", reg, self.registers[reg as usize], value);
        }
        if self.registers[reg as usize] == value as u8 {
            // skip!
            self.pc_reg += 2;
        }
    }

    fn deal_with_four_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("skip next if not equal");
        }
        let reg   = (bytecode >> 8) & 0xF;
        let value = bytecode  & 0xFF;
        if self.registers[reg as usize] != value as u8 {
            // skip!
            self.pc_reg += 2;
        }
    }

    fn deal_with_five_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("skip if vx == vy");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        if self.registers[x as usize] == self.registers[y as usize] {
            // skip!
            self.pc_reg += 2;
        }
    }

    fn deal_with_six_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("set reg to value");
        }
        let reg   = (bytecode >> 8) & 0xF;
        let value = bytecode  & 0xFF;
        self.registers[reg as usize] = value as u8;
        if debugOn{
            println!("register {} now holds {:#x}", reg, value);
        }
    }

    fn deal_with_seven_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("add val to register");
        }
        let reg = (bytecode >> 8) & 0xF;
        let num = bytecode & 0xFF;
        let value = (self.registers[reg as usize] as u16 + num) & 0xFF;

        self.registers[reg as usize] = value as u8;
    }

    fn deal_with_eight_nibble_codes(&mut self, bytecode : u16){
        let last_nibble = bytecode & 0xF;

        match last_nibble {
              0 => self.assign_value(bytecode),
              1 => self.assign_or(bytecode),
              2 => self.assign_and(bytecode),
              3 => self.assign_xor(bytecode),
              4 => self.add_regs(bytecode),
              5 => self.sub_regs(bytecode),
              6 => self.store_and_shift(bytecode), 
              7 => self.sub_and_store(bytecode),
            0xE => self.store_most_and_shift(bytecode),
            _=> panic!()
        }
    }

    fn assign_value(&mut self, bytecode : u16){
        if debugOn{
            println!("assign value");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valY;
    }

    fn assign_or(&mut self, bytecode : u16){
        if debugOn{
            println!("assign or");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valX | valY;
    }

    fn assign_and(&mut self, bytecode : u16){
        if debugOn{
            println!("assign and");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valX & valY;
    }

    fn assign_xor(&mut self, bytecode : u16){
        if debugOn{
            println!("assign xor");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valX ^ valY;
    }

    fn add_regs(&mut self, bytecode : u16){
        if debugOn{
            println!("add regs");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];
        let result = valX as u16 + valY as u16;
        self.registers[x as usize] = result as u8;
        if result > 0xFF {
            //panic!("carry!");
            self.registers[VF as usize] = 1;
        }else{
            self.registers[VF as usize] = 0;
        }
    }

    fn sub_regs(&mut self, bytecode : u16){
        if debugOn{
            println!("sub regs");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];

        if valX < valY {
            self.registers[x as usize] = valY - valX;
            self.registers[VF as usize] = 1;
        }else{
            self.registers[x as usize] -= valY;
            self.registers[VF as usize] = 0;
        }
    }

    fn store_and_shift(&mut self, bytecode : u16){
        if debugOn{
            println!("store and shift");
        }
        let reg = (bytecode >> 8) & 0xF;
        let val = self.registers[reg as usize];
        self.registers[15] = (val & 0x1);
        self.registers[reg as usize] = val >> 1;
        //panic!();
    }

    fn sub_and_store(&mut self, bytecode : u16){
        println!("sub_and_store!");
        panic!();
    }

    fn store_most_and_shift(&mut self, bytecode : u16){
        println!("store_most_and_shift!");
        let reg = (bytecode >> 8) & 0xF;
        let val = self.registers[reg as usize];
        self.registers[15] = (val & 0x80);
        self.registers[reg as usize] = val << 1;
        panic!();
    }
            
    fn deal_with_nine_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("deal with 9 nibble");
        }
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;

        let xVal = self.registers[x as usize];
        let yVal = self.registers[y as usize];

        if xVal != yVal {
            self.pc_reg += 2;
        }
    }

    fn deal_with_A_nibble_codes(&mut self, bytecode : u16){
        if debugOn{
            println!("I to nnn");
        }
        let addr   = bytecode & 0xFFF;
        self.address = addr as usize;
    }

    fn deal_with_B_nibble_codes(&mut self, bytecode : u16){
        if debugOn{
            println!("jump to NNN");
        }
        let nnn = bytecode & 0xFFF;
        let addr = nnn + self.registers[0 as usize] as u16;
        self.pc_reg = addr as usize;
    }

    fn deal_with_C_nibble_codes(&mut self, bytecode : u16){
        let kk = bytecode & 0xFF;
        let reg = (bytecode >> 8) & 0xF;
        let rand = 128;
        self.registers[reg as usize] = (kk & rand) as u8;
    }

    fn deal_with_D_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("draw sprite");
        }
        let vx = (bytecode >> 8) & 0xF;
        let vy = (bytecode >> 4) & 0xF;

        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        let height = bytecode & 0xF;
        let mut start_address = self.address;

        // get x 0ffset
        let x_offset = x % 8;
        let x_start = x / 8;

        let mut start_offset = (y * 8 + x_start) as usize;
        let mut flipped = false;

        for i in 0..height {

            // xor the address val with the video ram
            // get address value that we want to copy from RAM
            let orig_byte = chip.read_ram_value(start_address as usize);

            // create mask
            let first_byte_source = orig_byte >> x_offset;
            let mut second_byte_source = 0x00; 
            
            if x_offset != 0{
                second_byte_source = orig_byte << (8 - x_offset);
            }

            let first_byte_video = chip.read_ram_value(start_offset as usize);
            let second_byte_video = chip.read_ram_value((start_offset + 1) as usize);

            chip.xor_vram_value(start_offset as usize, first_byte_source);
            chip.xor_vram_value((start_offset + 1) as usize, second_byte_source);

            let new_vram_value_one = chip.read_vram_value(start_offset as usize);
            let new_vram_value_two = chip.read_vram_value((start_offset + 1) as usize);

            if new_vram_value_one != first_byte_video ||
               new_vram_value_two != second_byte_video {
                   flipped = true;
            }

            start_offset += 8;
            start_address += 1;
        }

        if flipped {
            self.registers[VF as usize] = 1;
        }else{
            self.registers[VF as usize] = 0;
        }

    }

    fn deal_with_E_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        let byte = bytecode & 0xFF;
        match byte{
            0x9E => self.skip_instruction_if_key_pressed(bytecode, chip),
            0xA1 => self.skip_instruction_if_key_not_pressed(bytecode, chip),
            _=> panic!()
        }
    }

    fn deal_with_F_nibble_codes(&mut self, bytecode : u16, chip : &mut hw_bundle){
        // depends on last byte
        let byte = bytecode & 0xFF;
        match byte{
               0x07 => self.set_reg_to_delay(bytecode, chip),
               0x0A => self.await_key_press(bytecode, chip),
               0x15 => self.set_delay_timer(bytecode, chip),
               0x18 => self.set_sound_timer(bytecode, chip),
               0x1E => self.add_vx_to_i(bytecode),
               0x29 => self.set_sprite_loc(bytecode),
               0x33 => self.set_BCD(bytecode, chip),
               0x55 => self.store_regs_in_memory(bytecode, chip),
               0x65 => self.fill_regs(bytecode, chip),
            _=> panic!()
        }
    }

    fn set_reg_to_delay(&mut self, bytecode : u16, chip : &mut hw_bundle){
        let reg = (bytecode >> 8) & 0xF;
        self.registers[reg as usize] = chip.get_delay_timer_count();
    }

    fn await_key_press(&mut self, bytecode : u16,  chip : &mut hw_bundle){
        //println!("await keypress");

        if !self.halt {
            self.halt = true;
        } else {
            // is any key pressed yet?
            //println!("check keys");
            for i in 0..16 {
                if chip.read_key(i as usize) != 0 {
                    self.halt = false;
                    let reg = bytecode >> 8 & 0xF;
                    self.registers[reg as usize] = i;
                    self.pc_reg += 2;
                }
            }
        }
        
    }

    fn set_delay_timer(&mut self, bytecode: u16, chip : &mut hw_bundle){
        println!("set sound timer");
        let reg = (bytecode >> 8) & 0xF;
        chip.set_delay_timer_count( self.registers[reg as usize]);
    }

    fn set_sound_timer(&mut self, bytecode: u16, chip : &mut hw_bundle){
        println!("set sound timer");
        let reg = (bytecode >> 8) & 0xF;
        chip.set_sound_timer_count( self.registers[reg as usize]);
    }

    fn set_BCD(&mut self, bytecode : u16, chip : &mut hw_bundle){
        println!("set BCD!");
        let reg = (bytecode >> 8) & 0xF;
        let mut value = self.registers[reg as usize];
        let hundreds = value / 100;
        value = value - hundreds * 100;
        let mut address = self.address;
        chip.write_ram_value(address as usize, value);
        address += 1;
        let tens = value / 10;
        chip.write_ram_value(address as usize, tens);
        address += 1;
        value = value - tens * 10;
        let digit = value;
        chip.write_ram_value(address as usize, digit);
        println!("{} {} {} {}", hundreds, tens, digit, value);
    }


    fn set_sprite_loc(&mut self, bytecode : u16){
        if debugOn{
            println!("set sprite location");
        }
        let reg = (bytecode >> 8) & 0xF;
        let character = self.registers[reg as usize];

        self.address = (character * 0x5) as usize;
    }

    fn fill_regs(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("fill regs!");
        }
        let end = (bytecode >> 8) & 0xF;
        let mut addr = self.address;
        for i in 0..=end {
            self.registers[i as usize] = chip.read_ram_value(addr as usize);
            addr += 1;
            if debugOn{
                println!("reg {} out of {}", i, end);
            }
        }
    }

    fn store_regs_in_memory(&mut self, bytecode : u16, chip : &mut hw_bundle){
        if debugOn{
            println!("store regs in mem");
        }
        let reg = (bytecode >> 8) & 0xF;
        let mut address = self.address;
        for i in 0..=reg {
            chip.write_ram_value(address as usize, self.registers[i as usize]);
            address += 1;
            if debugOn{
                println!("reg {} out of {}", i, reg);
            }
        }
    }

    fn add_vx_to_i(&mut self, bytecode : u16){
        if debugOn{
            println!("add vx to i");
        }
        let reg = (bytecode >> 8) & 0xF;
        let val = self.registers[reg as usize];
        self.address += val as usize;
    }

    fn call_program(&mut self, bytecode : u16){
        println!("call program at addr {:#x}", bytecode & 0xFFF);
        panic!();
    }

    fn skip_instruction_if_key_pressed(&mut self, bytecode: u16, chip : &mut hw_bundle){
        if debugOn{
            println!("skip instruction if key pressed");
        }
        let reg = (bytecode >> 8) & 0xF;
        let keyStored = self.registers[reg as usize];
       
        if chip.read_key(keyStored as usize) == 1 {
            self.pc_reg += 2;
        }

        if debugOn{
            println!("key {} pressed!", reg);
        }
    }

    fn skip_instruction_if_key_not_pressed(&mut self, bytecode: u16, chip : &mut hw_bundle){
        if debugOn{
            println!("skip instruction if key not pressed");
        }

        let reg = (bytecode >> 8) & 0xF;
        let keyStored = self.registers[reg as usize];
        if chip.read_key(keyStored as usize) == 0 {
            self.pc_reg += 2;
        }

        if debugOn{
            println!("key {} not pressed!", reg);
        }
    }
}