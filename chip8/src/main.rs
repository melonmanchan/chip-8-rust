
fn main() {
    let mut current_opcode: u16;

    // Memory of the emulator
    let memory: [u8; 4096];

    // CPU registers (From 0 to E)
    let V: [u8; 16];

    let mut index_register: u16;
    let mut program_counter: u16;

    // 64 x 32 screen of black-and-white pixels
    // Either on or off.
    let graphics: [u8; 64 * 32];

    // Timers
    let mut delay_timer: u8;
    let mut sound_timer: u8;

    let stack: [u16; 16];
    let mut stack_pointer: u16;

    // Whichever keys are pressed
    let keys: [u8; 16];

    println!("Hello, world!");
}
