//chip8.rs
use std::rc::Rc;

const VF : u8 = 15;

// memory map
const MEM_BEGIN : u16 = 0x200;
const DISPLAY_REFRESH : u16 = 0xF00;
const MISC : u16 = 0xEA0;

#[derive(Debug)]
struct CPU{
    registers : [u8; 16],
    address   : u16,
    pc_reg    : usize,
    delayTimer : Timer,
    soundTimer : Timer,
    stack      : Stack,
    memory     : Memory,
}

impl CPU{

    fn new() -> Self{
        CPU {
            registers : [0; 16],
            address   : 0,
            pc_reg    : 0x200,
            delayTimer : Timer::default(),
            soundTimer : Timer::default(),
            memory     : Memory::default(),
            stack      : Stack::default(),
        }
    }

    pub fn load_rom(&mut self, rom : &Box<[u8]>){
        let mut mem_start : usize = 0x200;
        for i in 0..rom.len() {
            self.memory.ram[mem_start] = rom[i as usize];
            mem_start += 1;
        }
    }

    fn next_instruction(&mut self){
        let bytecode1 : u8 = self.memory.ram[self.pc_reg]; 
        let bytecode2 : u8 = self.memory.ram[self.pc_reg + 1];
        self.pc_reg += 2;

        let bytecode = ((bytecode1 as u16) << 8) | bytecode2 as u16;

        self.decode_instruction(bytecode);

        println!("{:#x} ", bytecode);
    }

    fn decode_instruction(&mut self, bytecode : u16){
        let first_nibble = (bytecode >> 12) as u8;

        match first_nibble {
            0 => self.deal_with_zero_nibble_codes(bytecode),
            1 => self.deal_with_one_nibble_codes(bytecode),
            2 => self.deal_with_two_nibble_codes(bytecode),
            3 => self.deal_with_three_nibble_codes(bytecode),
            4 => self.deal_with_four_nibble_codes(bytecode),
            5 => self.deal_with_five_nibble_codes(bytecode),
            6 => self.deal_with_six_nibble_codes(bytecode),
            7 => self.deal_with_seven_nibble_codes(bytecode),
            8 => self.deal_with_eight_nibble_codes(bytecode),
            9 => self.deal_with_nine_nibble_codes(bytecode),
            0xA => self.deal_with_A_nibble_codes(bytecode),
            0xB => self.deal_with_B_nibble_codes(bytecode),
            0xC => self.deal_with_C_nibble_codes(bytecode),
            0xD => self.deal_with_D_nibble_codes(bytecode),
            0xE => self.deal_with_E_nibble_codes(bytecode),
            0xF => self.deal_with_F_nibble_codes(bytecode),
            _ => panic!()
        }
    }

    fn deal_with_zero_nibble_codes(&mut self, bytecode : u16){
            let byte = bytecode & 0xFF;
            match byte {
                0xE0 => self.clear_screen(),
                0xEE => self.return_from_subroutine(),
                _ => self.call_program(bytecode)
            }
    }

    fn deal_with_one_nibble_codes(&mut self, bytecode : u16){
        println!("jump!");
        let addr = bytecode & 0xFFF;
        println!("addr {:#x}", addr);
        self.pc_reg = addr as usize;
    }

    fn deal_with_two_nibble_codes(&mut self, bytecode : u16){
        //calls a subroutine!
        println!("call a subroutine");
        let addr = bytecode & 0xFFF;
        // store current addr
        self.stack.addresses[self.stack.stackpointer as usize] = self.pc_reg as u32;
        self.stack.stackpointer += 1;

        self.pc_reg = addr as usize;
    }

    fn deal_with_three_nibble_codes(&mut self, bytecode : u16){
        println!("skip next if vx == nn");
        let reg   = (bytecode >> 8) & 0xF;
        let value = bytecode  & 0xFF;
        if self.registers[reg as usize] == value as u8 {
            // skip!
            self.pc_reg += 2;
        }
    }

    fn deal_with_four_nibble_codes(&mut self, bytecode : u16){
        println!("skip next if vx != nn");
        let reg   = (bytecode >> 8) & 0xF;
        let value = bytecode  & 0xFF;
        if self.registers[reg as usize] != value as u8 {
            // skip!
            self.pc_reg += 2;
        }
    }

    fn deal_with_five_nibble_codes(&mut self, bytecode : u16){
        println!("skip if vx == vy");
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        if self.registers[x as usize] == self.registers[y as usize] {
            // skip!
            self.pc_reg += 2;
        }
    }

    fn deal_with_six_nibble_codes(&mut self, bytecode : u16){
        println!("setvx to nn {:#x}", bytecode);
        let reg   = (bytecode >> 8) & 0xF;
        let value = bytecode  & 0xFF;
        self.registers[reg as usize] = value as u8;
        //println!("{:#?}", self.registers);
    }

    fn deal_with_seven_nibble_codes(&mut self, bytecode : u16){
        println!("add val to register!");
        let reg = (bytecode >> 8) & 0xF;
        let num = bytecode & 0xFF;

        self.registers[reg as usize] += num as u8;
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
        println!("assign value!");
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valY;
    }

    fn assign_or(&mut self, bytecode : u16){
        println!("assign_or!");
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valX | valY;
    }

    fn assign_and(&mut self, bytecode : u16){
        println!("assign_and!");
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valX & valY;
    }

    fn assign_xor(&mut self, bytecode : u16){
        println!("assign_xor!");
        let x = (bytecode >> 8) & 0xF;
        let y = (bytecode >> 4) & 0xF;
        let valX = self.registers[x as usize];
        let valY = self.registers[y as usize];
        self.registers[x as usize] = valX ^ valY;
    }

    fn add_regs(&mut self, bytecode : u16){
        println!("add_regs!");
        panic!();
    }

    fn sub_regs(&mut self, bytecode : u16){
        println!("sub_regs!");
        panic!();
    }

    fn store_and_shift(&mut self, bytecode : u16){
        println!("store_and_shift!");
        let reg = (bytecode >> 8) & 0xF;
        let val = self.registers[reg as usize];
        self.registers[15] = (val & 0x1);
        self.registers[reg as usize] = val >> 1;
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
    }
            
    fn deal_with_nine_nibble_codes(&self, bytecode : u16){
        println!("deal_with_nine_nibble_codes!");
        panic!();
    }

    fn deal_with_A_nibble_codes(&mut self, bytecode : u16){
        println!("I to nnn");
        let addr   = bytecode & 0xFFF;
        self.address = addr;
    }

    fn deal_with_B_nibble_codes(&mut self, bytecode : u16){
        println!("jump to address NNN");
        let nnn = bytecode & 0xFFF;
        let addr = nnn + self.registers[0 as usize] as u16;
        self.pc_reg = addr as usize;
    }

    fn deal_with_C_nibble_codes(&mut self, bytecode : u16){
        // todo
    }

    fn deal_with_D_nibble_codes(&mut self, bytecode : u16){
        // todo
    }

    fn deal_with_E_nibble_codes(&mut self, bytecode : u16){
        let byte = bytecode & 0xFF;
        match byte{
            0x9E => self.skip_instruction_if_key_pressed(bytecode),
            0xA1 => self.skip_instruction_if_key_not_pressed(bytecode),
            _=> panic!()
        }
    }

    fn deal_with_F_nibble_codes(&mut self, bytecode : u16){
        // depends on last byte
        println!("f nibble {:#x}", bytecode);
        let byte = bytecode & 0xFF;
        match byte{
            0x07 => self.set_reg_to_delay(bytecode),
            // 0x0A => ,
            // 0x15 => ,
            // 0x18 => ,
               0x1E => self.add_vx_to_i(bytecode),
               0x29 => self.set_sprite_loc(bytecode),
            // 0x33 => ,
            0x55 => self.store_regs_in_memory(bytecode),
            0x65 => self.fill_regs(bytecode),
            _=> panic!()
        }
    }

    fn set_sprite_loc(&mut self, bytecode : u16){
        println!("set sprite location");
        let reg = (bytecode >> 8) & 0xF;
        let character = self.registers[reg as usize];
        // TODO
    }

    fn fill_regs(&mut self, bytecode : u16){
        println!("fill regs!");
        let end = (bytecode >> 8) & 0xF;
        let mut addr = self.address;
        for i in 0.. end {
            self.registers[i as usize] = self.memory.ram[addr as usize];
            addr += 1;
        }
    }

    fn add_vx_to_i(&mut self, bytecode : u16){
        println!("add vx to i");
        let reg = (bytecode >> 8) & 0xF;
        let val = self.registers[reg as usize];

        self.address += (val as u16);

        if self.address > 0xFFF {
            self.registers[15 as usize] = 1;
        }else{
            self.registers[15 as usize] = 0;
        }
    }

    fn call_program(&mut self, bytecode : u16){
        println!("call program at addr {:#x}", bytecode & 0xFFF);
    }

    fn clear_screen(&mut self) {
        println!("clear screen!");
    }

    fn skip_instruction_if_key_pressed(&mut self, bytecode: u16){
        println!("skip instruction if key pressed!");
        let reg = (bytecode >> 8) & 0xF;
        let keyStored = self.registers[reg as usize];

        //check key val
    }

    fn skip_instruction_if_key_not_pressed(&mut self, bytecode: u16){
        println!("skip instruction if key not pressed!");

        let reg = (bytecode >> 8) & 0xF;
        let keyStored = self.registers[reg as usize];
    }

    fn return_from_subroutine(&mut self) {
        // get address from stack pointer
        println!("return from subroutine");
        self.stack.stackpointer -= 1;
        let addr = self.stack.addresses[self.stack.stackpointer as usize];
        println!("{:#x}", addr);
        self.pc_reg = addr as usize;
    }

    fn set_reg_to_delay(&mut self, bytecode : u16){
        let reg = (bytecode >> 8) & 0xF;
        self.registers[reg as usize] = self.delayTimer.count;
    }

    fn store_regs_in_memory(&mut self, bytecode : u16){
        println!("store regs in mem");
        let reg = (bytecode >> 8) & 0xF;
        let mut address = self.address;
        for i in 0..reg {
            // store in memory somehow
            self.memory.ram[address as usize] = self.registers[i as usize];
            address += 1;
        }
    }

}

#[derive(Debug, Default)]
struct Stack{
    addresses : [u32; 16],
    stackpointer : u8
}

#[derive(Debug, Default)]
struct Timer{
    count : u8
}

#[derive(Debug, Default)]
struct Memory{
    ram  : Box<[u8]>,
    vram : Box<[u8]>
}

impl Memory{
    pub fn default() -> Self{
        Memory{
            ram : vec![0; 4096].into_boxed_slice(),
            vram : vec![0; 64 * 32].into_boxed_slice()
        }
    }
}

#[derive(Debug)]
pub struct Chip_HW{
    cpu : CPU,
}

impl Chip_HW{
    pub fn new( ) -> Self{
        Chip_HW{
            cpu : CPU::new(),
        }
    }

    pub fn load_rom(&mut self, rom : &Box<[u8]>){
        self.cpu.load_rom(rom);
    }

    pub fn run(&mut self){
        self.cpu.next_instruction();
    }

    
}