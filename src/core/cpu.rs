
pub struct CPU {

    // 8 bit registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    // 16 bit registers
    pub sp: u16,    // Stack pointer
    pub pc: u16,    // Program counter

    // Flag registers
    pub zero: bool,    // Zero flag
    pub subtraction: bool,    // Subtraction flag
    pub half_carry: bool,    // Half carry flag
    pub carry: bool,    // Carry flag

    // Interrupt flags
    pub ime: bool,  // Interrupt master enable flag
    pub halt: bool  // Halt flag

}

impl CPU {

    pub fn new() -> Self {

        CPU {

            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            sp: 0x0,
            pc: 0x0,
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false,
            ime: false,
            halt: false

        }

    }

    pub fn reset(&mut self) {

        self.a= 0x0;
        self.b= 0x0;
        self.c= 0x0;
        self.d= 0x0;
        self.e= 0x0;
        self.h= 0x0;
        self.l= 0x0;
        self.sp= 0x0;
        self.pc= 0x0;
        self.zero= false;
        self.subtraction= false;
        self.half_carry= false;
        self.carry= false;
        self.ime= false;
        self.halt= false;

    }

}