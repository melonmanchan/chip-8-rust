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
    fn setup_graphics(&self) {

    }

    fn setup_input(&self) {

    }

    fn load_game(&self) {

    }

    fn emulate_cycle(&mut self) {
        let pc = self.program_counter as usize;

        let current_opcode: u16 = (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16);

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

    fn draw_graphics(&self) {

    }

    fn set_key_state(&self) {

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

    chip8.setup_graphics();

    chip8.setup_input();


    chip8.load_game();

    loop {
        chip8.emulate_cycle();

        chip8.draw_graphics();

        chip8.set_key_state();
    }
}
