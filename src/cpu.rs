use display::Display;
use keypad::Keypad;
use rand::ComplementaryMultiplyWithCarryGen;

pub struct Cpu {
    //index register
    pub i: u16,
    //program counter
    pub pc: u16,
    //memory
    pub memory: [u8; 4096],
    //register
    pub v: [u8; 16],
    //peripherals
    pub keypad: Keypad,
    pub display: Display,
    //stack
    pub stack: [u16; 16],
    //stack pointer
    pub sp: u8,
    //delay timer
    pub dt: u8,
    //RNG
    pub rand: ComplementaryMultiplyWithCarryGen,
}

fn read_word(mem: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8
        | (memory[(index + 1) as usize] as u16)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0, 
            memory: [0; 4096],
            v: [0; 16],
            display: Display::new(),
            keypad: Keypad::new(),
            stack: [0; 16],
            sp: 0,
            dt: 0
        }
    }

    pub fn reset(&mut self) {
        self = Cpu::new();
    }

    pub fn execute_cycle(&mut self) {
        let opcode: u16 = read_word(self.memory, self.pc);
        self.process_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    fn process_opcode(&mut self, opccode: u16) {
        //extract various opcode parameters
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0fff;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000f) as u8;

        //break up into nibbles
        let op_1 = (opcode & 0xf000) >> 12;
        let op_2 = (opcode & 0x0f00) >> 8;
        let op_3 = (opcode & 0x00f0) >> 4;
        let op_4 = opcode & 0x000f;

        //increment the program counter
        self.pc += 2;

        //Get operation
        match (op_1, op_2, op_3, op_4) {
            // CLS
            (0, 0, 0xE, 0) => self.display.cls(),
            // RET
            (0, 0, 0xE, 0xE) => {
                self.sp = self.sp - 1;
                self.pc = self.stack[self.sp as usize];
            },
            // JP
            (0x1, _, _, _) => self.pc = nnn,
            // CALL
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp + 1;
                self.pc = nnn;
            }
            // SE Vx KK
            (0x3, _, _, _) => self.pc += if vx == kk { 2 } else { 0 },
            // SNE Vx KK
            // SE Vx Vy
            // LD Vx
            //ADD Vx, byte
            //LD Vx, Vy
            //OR Vx, Vy
            //AND Vx, Vy
            //XOR Vx, Vy,
            //AND Vx Vy
            //SUB Vx, Vy
            //SHR Vx
            //SUBN Vx, Vy
            //SHL Vx
            //SNE Vx Vy
            //LD I
            // JP V0
            // RND
            // DRW
            // SKP Vx
            // SKNP Vx
            // LD Vx, DT
            // LD Vx, K
            // LD DT, Vx
            // ADD I, Vx
            // LD F, Vx
            //LD B, Vx
            //LD [I], Vx
            //LD Vx, [I]
            //Default
            (_, _, _, _) => ()
        }
    }
}

#[cgf(test)]
mod tests {
    use super::Cpu;


}

