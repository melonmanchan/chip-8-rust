use std::io::Read;
use std::fs::File;

const FONT_SET: [u8; 80] = [
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

struct Chip8 {
    current_opcode: u16,
    // Memory of the emulator
    memory: [u8; 4096],

    // CPU registers (From 0 to E)
    v: [u8; 16],

    index_register: u16,
    program_counter: u16,

    // 64 x 32 screen of black-and-white pixels
    // Either on or off.
    graphics: [[u8; 64]; 32],

    // Timers
    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stack_pointer: u16,

    // Whichever keys are pressed
    keys: [u8; 16],
}

impl Chip8 {
    fn load_font_set(&mut self) {
        for i in 0..80 {
            self.memory[i] = FONT_SET[i];
        }
    }

    fn setup_graphics(&self) {

    }

    fn setup_input(&self) {

    }

    fn load_game(&mut self, game_path: String) {
        let mut file = File::open(&game_path).unwrap();
        let mut rom : Vec<u8> = Vec::new();
        file.read_to_end(&mut rom).unwrap();

        for i in 0..rom.len() {
            self.memory[0x200 + i] = rom[i];
        }
    }

    fn emulate_cycle(&mut self) {
        let pc = self.program_counter as usize;

        self.current_opcode = (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16);

        // TODO
        match self.current_opcode & 0xf000 {
            0x0000 => self.op_0xxx(),
            0x1000 => self.op_1xxx(),
            0x2000 => self.op_2xxx(),
            0x3000 => self.op_3xxx(),
            0x4000 => self.op_4xxx(),
            0x5000 => self.op_5xxx(),
            0x6000 => self.op_6xxx(),
            0x7000 => self.op_7xxx(),
            0x8000 => self.op_8xxx(),
            0x9000 => self.op_9xxx(),
            0xA000 => self.op_Axxx(),
            0xB000 => self.op_Bxxx(),
            0xC000 => self.op_Cxxx(),
            0xD000 => self.op_Dxxx(),
            0xE000 => self.op_Exxx(),
            0xF000 => self.op_Fxxx(),
            _ => println!("Opcode not implemented: {:X}", self.current_opcode)
        }

        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEPS")
            }

            self.sound_timer = self.sound_timer - 1;
        }
    }

    fn draw_graphics(&mut self) {

    }

    fn set_key_state(&mut self) {

    }

    fn op_0xxx(&mut self) {

    }

    fn op_1xxx(&mut self) {

    }

    fn op_2xxx(&mut self) {

    }

    fn op_3xxx(&mut self) {

    }

    fn op_4xxx(&mut self) {

    }

    fn op_5xxx(&mut self) {

    }

    fn op_6xxx(&mut self) {

    }

    fn op_7xxx(&mut self) {

    }

    fn op_8xxx(&mut self) {

    }

    fn op_9xxx(&mut self) {

    }

    fn op_Axxx(&mut self) {

    }

    fn op_Bxxx(&mut self) {

    }

    fn op_Cxxx(&mut self) {

    }

    fn op_Dxxx(&mut self) {

    }

    fn op_Exxx(&mut self) {

    }

    fn op_Fxxx(&mut self) {

    }
}


fn main() {
    let mut chip8: Chip8 = Chip8 {
        current_opcode:  0,

        memory: [0; 4096],

        v: [0; 16],

        index_register:  0x200,
        program_counter: 0x200,

        graphics: [[0; 64]; 32],

        delay_timer: 0,
        sound_timer: 0,

        stack: [0; 16],
        stack_pointer: 0,

        keys: [0; 16]
    };

    chip8.load_font_set();

    chip8.setup_graphics();

    chip8.setup_input();

    chip8.load_game(String::from("../../games/PONG"));

    loop {
        chip8.emulate_cycle();

        chip8.draw_graphics();

        chip8.set_key_state();
    }
}
