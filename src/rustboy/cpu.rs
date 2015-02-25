/*
 * rustboy/cpu.rs
 *
 * This is the CPU of the GameBoy emulator. A technical specification
 * of the GameBoy can be found here: http://problemkaputt.de/pandocs.htm
 *
 */

// Declarations
// These have Z80 added to the names because eventually ARM7TDMI architecture
// will also be defined here.
struct Z80_Clock;
struct Z80_CPU;
struct Z80_Registers;

// The Z80 keeps two internal clocks, m and t. t is a measure of CPU cycles
// and m is 1/4 of t.
struct Z80_Clock {
    m: u8,
    t: u8
}

/*
  16bit Hi   Lo   Name/Function
  AF    A    -    Accumulator & Flags
  BC    B    C    BC
  DE    D    E    DE
  HL    H    L    HL
  SP    -    -    Stack Pointer
  PC    -    -    Program Counter/Pointer
*/
struct Z80_Registers {
    // 8-bit registers:
    a: u8,      // acc and flags
    b: u8,      // BC
    c: u8,
    d: u8,      // DE
    e: u8,
    h: u8,      // HL
    l: u8,
    // 16-bit registers:
    af: u16,    // acc and flags
    bc: u16,    // BC
    de: u16,    // DE
    hl: u16,    // HL
    // pointers:
    sp: u16,    // stack pointer
    pc: u16     // program counter / instruction pointer
}

struct Z80_CPU {
    clock: Z80_Clock,
    //mem: memory,
    regs: Z80_Registers,
}

impl Z80_CPU {
    pub fn reset(&mut self) {
        // clear 8-bit registers
        self.regs.a = 0;
        self.regs.b = 0;
        self.regs.c = 0;
        self.regs.d = 0;
        self.regs.e = 0;
        self.regs.h = 0;
        self.regs.l = 0;
        // clear 16-bit registers
        self.regs.af = 0;
        self.regs.bc = 0;
        self.regs.de = 0;
        self.regs.hl = 0;
        // clear pointers
        self.regs.sp = 0; // beginning of stack
        self.regs.pc = 0; // back to first instruction

        // reset the clock
        self.clock.m = 0;
        self.clock.t = 0;
    }
}
