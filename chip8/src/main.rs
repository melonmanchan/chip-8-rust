fn setup_graphics(chip8: &mut Chip8) {

}

fn setup_input(chip8: &mut Chip8) {

}

fn load_game(chip8: &mut Chip8) {

}

fn emulate_cycle(chip8: &mut Chip8) {
    let pc = chip8.program_counter as usize;

    let current_opcode: u16 = (chip8.memory[pc] as u16) << 8 | (chip8.memory[pc + 1] as u16);


    if chip8.delay_timer > 0 {
        chip8.delay_timer = chip8.delay_timer - 1;
    }

    if chip8.sound_timer > 0 {
        if chip8.sound_timer == 1 {
            println!("BEEPS")
        }

        chip8.sound_timer = chip8.sound_timer - 1;
    }
}

fn draw_graphics(chip8: &mut Chip8) {

}

fn set_key_state(chip8: &mut Chip8) {

}

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

    setup_graphics(&mut chip8);
    setup_input(&mut chip8);

    load_game(&mut chip8);

    loop {
        emulate_cycle(&mut chip8);

        draw_graphics(&mut chip8);

        set_key_state(&mut chip8);
    }
}
