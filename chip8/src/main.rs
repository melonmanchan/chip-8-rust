fn setupGraphics(chip8: &mut Chip8) {

}

fn setupInput(chip8: &mut Chip8) {

}

fn initialize(chip8: &mut Chip8) {

}

fn loadGame(chip8: &mut Chip8) {

}

fn emulateCycle(chip8: &mut Chip8) {

}

fn drawGraphics(chip8: &mut Chip8) {

}

fn setKeyState(chip8: &mut Chip8) {

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
    graphics: [u8; 64 * 32],

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

        index_register:  0,
        program_counter: 0,

        graphics: [0; 64 * 32],

        delay_timer: 0,
        sound_timer: 0,

        stack: [0; 16],
        stack_pointer: 0,

        keys: [0; 16]
    };

    setupGraphics(&mut chip8);
    setupInput(&mut chip8);

    initialize(&mut chip8);
    loadGame(&mut chip8);

    println!("Hello, world!");
}
