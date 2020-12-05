use cpu::Cpu;
use display::Display;
use keypad::Keypad;

static mut CPU: Cpu = Cpu.new();

#[no_mangle]
pub fn reset() {
    CPU.reset();
}

#[no_mangle]
pub fn get_memory() -> &'static [u8; 4096] {
    &CPU.memory
}

#[no_mangle]
pub fn get_display() -> &'static [u8; 2048] {
    &CPU.display.memory
}

#[no_mangle]
pub fn key_down(i: u8) {
    CPU.keypad.key_down(i);
}

#[no_mangle]
pub fn key_up(i: u8) {
    CPU.keypad.key_up(i);
}

#[no_mangle]
pub fn get_register_v() -> &'static [u8; 16] {
    &CPU.v
}

#[no_mangle]
pub fn get_register_i() -> u16 {
    CPU.i
}

#[no_mangle]
pub fn get_register_pc() -> u16 {
    CPU.pc
}

#[no_mangle]
pub fn execute_cycle() {
    CPU.execute_cycle();
}

#[no_mangle]
pub fn decrement_timers() {
    CPU.decrement_timers();
}

