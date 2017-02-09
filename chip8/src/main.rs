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
    let mut chip8: Chip8;
    println!("Hello, world!");
}
