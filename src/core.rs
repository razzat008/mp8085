use crate::{CPU, CPUFlags, Mem};

// Register order used throughout the 8085 instruction encoding.
// 0:B 1:C 2:D 3:E 4:H 5:L 6:M 7:A
impl CPU {
    pub fn fetch(&mut self) -> u8 {
        let v = self.mem_read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        v
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        (hi << 8) | lo
    }

    // ---- register pairs -------------------------------------------------
    // 8085 follows little-endian convention
    // get content of hl register pair
    pub fn hl(&self) -> u16 {
        ((self.reg_h as u16) << 8) | self.reg_l as u16
    }
    // set "v" as content for the register pair hl
    pub fn set_hl(&mut self, v: u16) {
        self.reg_h = (v >> 8) as u8; // extract upper 8 bits
        self.reg_l = v as u8; // truncate lower 8 bits; since v is u16
    }
    fn bc(&self) -> u16 {
        ((self.reg_b as u16) << 8) | self.reg_c as u16
    }
    fn set_bc(&mut self, v: u16) {
        self.reg_b = (v >> 8) as u8;
        self.reg_c = v as u8;
    }
    fn de(&self) -> u16 {
        ((self.reg_d as u16) << 8) | self.reg_e as u16
    }
    fn set_de(&mut self, v: u16) {
        self.reg_d = (v >> 8) as u8;
        self.reg_e = v as u8;
    }

    fn reg(&self, r: u8) -> u8 {
        match r {
            0 => self.reg_b,
            1 => self.reg_c,
            2 => self.reg_d,
            3 => self.reg_e,
            4 => self.reg_h,
            5 => self.reg_l,
            6 => self.mem_read(self.hl()),
            7 => self.reg_a,
            _ => unreachable!(),
        }
    }

    fn set_reg(&mut self, r: u8, v: u8) {
        match r {
            0 => self.reg_b = v,
            1 => self.reg_c = v,
            2 => self.reg_d = v,
            3 => self.reg_e = v,
            4 => self.reg_h = v,
            5 => self.reg_l = v,
            6 => {
                let a = self.hl();
                self.mem_write(a, v);
            }
            7 => self.reg_a = v,
            _ => unreachable!(),
        }
    }

    // ---- stack ----------------------------------------------------------
    pub fn push(&mut self, v: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(2);
        self.mem_write(self.stack_pointer, v as u8);
        self.mem_write(self.stack_pointer.wrapping_add(1), (v >> 8) as u8);
    }

    pub fn pop(&mut self) -> u16 {
        let lo = self.mem_read(self.stack_pointer) as u16;
        let hi = self.mem_read(self.stack_pointer.wrapping_add(1)) as u16;
        self.stack_pointer = self.stack_pointer.wrapping_add(2);
        (hi << 8) | lo
    }

    // ---- flags ----------------------------------------------------------
    // sets "c" as the state of carry flag where c is a bool
    fn set_carry_flag(&mut self, c: bool) {
        self.status.set(CPUFlags::CARRY_F, c);
    }

    // setting accumulator
    fn set_accumulator_to(&mut self, c: bool) {
        self.status.set(CPUFlags::AUXC_F, c);
    }

    // getter methods to check if flag_x are active
    fn zero(&self) -> bool {
        self.status.contains(CPUFlags::ZERO_F)
    }
    fn carry(&self) -> bool {
        self.status.contains(CPUFlags::CARRY_F)
    }
    fn parity(&self) -> bool {
        self.status.contains(CPUFlags::PARITY_F)
    }
    fn sign(&self) -> bool {
        self.status.contains(CPUFlags::SIGN_F)
    }

    // S, Z, P from an 8-bit result
    fn set_szp(&mut self, v: u8) {
        self.status.set(CPUFlags::ZERO_F, v == 0);
        self.status.set(CPUFlags::SIGN_F, v & 0x80 != 0);
        self.status.set(CPUFlags::PARITY_F, v.count_ones() % 2 == 0);
    }

    fn set_logic_flags(&mut self, v: u8) {
        self.set_szp(v);
        self.set_carry_flag(false);
        self.set_accumulator_to(false);
    }

    // condition code index: 0:NZ 1:Z 2:NC 3:C 4:PO 5:PE 6:P 7:M
    fn cc(&self, c: u8) -> bool {
        match c {
            0 => !self.zero(),
            1 => self.zero(),
            2 => !self.carry(),
            3 => self.carry(),
            4 => !self.parity(),
            5 => self.parity(),
            6 => !self.sign(),
            _ => self.sign(),
        }
    }

    // ---- arithmetic -----------------------------------------------------
    fn add_a(&mut self, b: u8, carry: bool) {
        let a = self.reg_a;
        let c = carry as u8;
        let r = a as u16 + b as u16 + c as u16;
        let ac = (a & 0xF) + (b & 0xF) + c > 0xF;
        let r8 = r as u8;
        self.set_szp(r8);
        self.set_carry_flag(r > 0xFF);
        self.set_accumulator_to(ac);
        self.reg_a = r8;
    }

    fn sub_a(&mut self, b: u8, carry: bool, store: bool) {
        let a = self.reg_a;
        let c = carry as u8;
        let r = a as i16 - b as i16 - c as i16;
        let ac = (a & 0xF) < (b & 0xF) + c;
        let r8 = r as u8;
        self.set_szp(r8);
        self.set_carry_flag(r < 0);
        self.set_accumulator_to(ac);
        if store {
            self.reg_a = r8;
        }
    }

    fn inr(&mut self, r: u8) {
        let v = self.reg(r).wrapping_add(1);
        self.set_szp(v);
        self.set_accumulator_to(v & 0xF == 0);
        self.set_reg(r, v);
    }

    fn dcr(&mut self, r: u8) {
        let old = self.reg(r);
        let v = old.wrapping_sub(1);
        self.set_szp(v);
        self.set_accumulator_to(old & 0xF == 0);
        self.set_reg(r, v);
    }

    fn dad(&mut self, v: u16) {
        let r = self.hl() as u32 + v as u32;
        self.set_hl(r as u16);
        self.set_carry_flag(r > 0xFFFF);
    }

    fn daa(&mut self) {
        let a = self.reg_a as i16;
        let ac_old = self.status.contains(CPUFlags::AUXC_F);
        let cy_old = self.status.contains(CPUFlags::CARRY_F);

        let mut add: i16 = 0;
        if (a & 0xF) > 9 || ac_old {
            add |= 0x06;
        }
        if a > 0x99 || cy_old {
            add |= 0x60;
        }

        let r = a + add;
        let r8 = r as u8;
        self.set_szp(r8);
        self.set_accumulator_to((a & 0xF) + (add & 0xF) > 0xF);
        self.set_carry_flag(add & 0x60 != 0 || r > 0xFF);
        self.reg_a = r8;
    }

    // ---- execution ------------------------------------------------------
    pub fn execute(&mut self, op: u8) {
        match op {
            // NOP / HLT / interrupts
            0x00 => {}
            0x76 => self.halted = true,
            0xFB => self.int_enable = true,  // EI
            0xF3 => self.int_enable = false, // DI
            0x20 => {}                       // RIM (stub)
            0x30 => {}                       // SIM (stub)

            // memory / direct addressing
            0x02 => {
                let a = self.bc();
                let v = self.reg_a;
                self.mem_write(a, v);
            } // STAX B
            0x12 => {
                let a = self.de();
                let v = self.reg_a;
                self.mem_write(a, v);
            } // STAX D
            0x0A => {
                let a = self.bc();
                self.reg_a = self.mem_read(a);
            } // LDAX B
            0x1A => {
                let a = self.de();
                self.reg_a = self.mem_read(a);
            } // LDAX D
            0x22 => {
                let a = self.fetch_u16();
                let v = self.hl();
                self.mem_write(a, v as u8);
                self.mem_write(a + 1, (v >> 8) as u8);
            } // SHLD
            0x2A => {
                let a = self.fetch_u16();
                let lo = self.mem_read(a) as u16;
                let hi = self.mem_read(a + 1) as u16;
                self.set_hl((hi << 8) | lo);
            } // LHLD
            0x32 => {
                let a = self.fetch_u16();
                self.mem_write(a, self.reg_a);
            } // STA
            0x3A => {
                let a = self.fetch_u16();
                self.reg_a = self.mem_read(a);
            } // LDA
            // register pair immediate / inc / dec / add / swap
            0x01 => {
                let v = self.fetch_u16();
                self.set_bc(v);
            } // LXI B
            0x11 => {
                let v = self.fetch_u16();
                self.set_de(v);
            } // LXI D
            0x21 => {
                let v = self.fetch_u16();
                self.set_hl(v);
            } // LXI H
            0x31 => {
                let v = self.fetch_u16();
                self.stack_pointer = v;
            } // LXI SP
            0x03 => self.set_bc(self.bc().wrapping_add(1)),
            0x13 => self.set_de(self.de().wrapping_add(1)),
            0x23 => self.set_hl(self.hl().wrapping_add(1)),
            0x33 => self.stack_pointer = self.stack_pointer.wrapping_add(1),
            0x0B => self.set_bc(self.bc().wrapping_sub(1)),
            0x1B => self.set_de(self.de().wrapping_sub(1)),
            0x2B => self.set_hl(self.hl().wrapping_sub(1)),
            0x3B => self.stack_pointer = self.stack_pointer.wrapping_sub(1),
            0x09 => {
                let v = self.bc();
                self.dad(v);
            }
            0x19 => {
                let v = self.de();
                self.dad(v);
            }
            0x29 => {
                let v = self.hl();
                self.dad(v);
            }
            0x39 => {
                let v = self.stack_pointer;
                self.dad(v);
            }

            // accumulator logic / rotates / flags
            0x2F => self.reg_a = !self.reg_a, // CMA
            0x3F => {
                let c = !self.carry();
                self.set_carry_flag(c);
            } // CMC
            0x37 => self.set_carry_flag(true), // STC
            0x27 => self.daa(),               // DAA
            0x07 => {
                // RLC
                let a = self.reg_a;
                let c = a & 0x80 != 0;
                self.reg_a = (a << 1) | c as u8;
                self.set_carry_flag(c);
            }
            0x0F => {
                // RRC
                let a = self.reg_a;
                let c = a & 0x01 != 0;
                self.reg_a = (a >> 1) | ((c as u8) << 7);
                self.set_carry_flag(c);
            }
            0x17 => {
                // RAL
                let a = self.reg_a;
                let old_cy = (self.status.bits() & 1) as u8;
                let c = a & 0x80 != 0;
                self.reg_a = (a << 1) | old_cy;
                self.set_carry_flag(c);
            }
            0x1F => {
                // RAR
                let a = self.reg_a;
                let old_cy = (self.status.bits() & 1) as u8;
                let c = a & 0x01 != 0;
                self.reg_a = (a >> 1) | (old_cy << 7);
                self.set_carry_flag(c);
            }

            // immediate ALU with accumulator
            0xC6 => {
                let v = self.fetch();
                self.add_a(v, false);
            } // ADI
            0xCE => {
                let v = self.fetch();
                self.add_a(v, true);
            } // ACI
            0xD6 => {
                let v = self.fetch();
                self.sub_a(v, false, true);
            } // SUI
            0xDE => {
                let v = self.fetch();
                self.sub_a(v, true, true);
            } // SBI
            0xE6 => {
                let v = self.fetch();
                self.reg_a &= v;
                self.set_logic_flags(self.reg_a);
            } // ANI
            0xEE => {
                let v = self.fetch();
                self.reg_a ^= v;
                self.set_logic_flags(self.reg_a);
            } // XRI
            0xF6 => {
                let v = self.fetch();
                self.reg_a |= v;
                self.set_logic_flags(self.reg_a);
            } // ORI
            0xFE => {
                let v = self.fetch();
                self.sub_a(v, false, false);
            } // CPI

            // control flow
            0xC3 => {
                let a = self.fetch_u16();
                self.program_counter = a;
            } // JMP
            0xC2 => {
                let a = self.fetch_u16();
                if self.cc(0) {
                    self.program_counter = a;
                }
            } // JNZ
            0xCA => {
                let a = self.fetch_u16();
                if self.cc(1) {
                    self.program_counter = a;
                }
            } // JZ
            0xD2 => {
                let a = self.fetch_u16();
                if self.cc(2) {
                    self.program_counter = a;
                }
            } // JNC
            0xDA => {
                let a = self.fetch_u16();
                if self.cc(3) {
                    self.program_counter = a;
                }
            } // JC
            0xE2 => {
                let a = self.fetch_u16();
                if self.cc(4) {
                    self.program_counter = a;
                }
            } // JPO
            0xEA => {
                let a = self.fetch_u16();
                if self.cc(5) {
                    self.program_counter = a;
                }
            } // JPE
            0xF2 => {
                let a = self.fetch_u16();
                if self.cc(6) {
                    self.program_counter = a;
                }
            } // JP
            0xFA => {
                let a = self.fetch_u16();
                if self.cc(7) {
                    self.program_counter = a;
                }
            } // JM

            0xCD => {
                let a = self.fetch_u16();
                let pc = self.program_counter;
                self.push(pc);
                self.program_counter = a;
            } // CALL
            0xC4 => {
                let a = self.fetch_u16();
                if self.cc(0) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }
            0xCC => {
                let a = self.fetch_u16();
                if self.cc(1) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }
            0xD4 => {
                let a = self.fetch_u16();
                if self.cc(2) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }
            0xDC => {
                let a = self.fetch_u16();
                if self.cc(3) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }
            0xE4 => {
                let a = self.fetch_u16();
                if self.cc(4) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }
            0xEC => {
                let a = self.fetch_u16();
                if self.cc(5) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }
            0xF4 => {
                let a = self.fetch_u16();
                if self.cc(6) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }
            0xFC => {
                let a = self.fetch_u16();
                if self.cc(7) {
                    let pc = self.program_counter;
                    self.push(pc);
                    self.program_counter = a;
                }
            }

            0xC9 => {
                let pc = self.pop();
                self.program_counter = pc;
            } // RET
            0xC0 => {
                if self.cc(0) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }
            0xC8 => {
                if self.cc(1) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }
            0xD0 => {
                if self.cc(2) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }
            0xD8 => {
                if self.cc(3) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }
            0xE0 => {
                if self.cc(4) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }
            0xE8 => {
                if self.cc(5) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }
            0xF0 => {
                if self.cc(6) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }
            0xF8 => {
                if self.cc(7) {
                    let pc = self.pop();
                    self.program_counter = pc;
                }
            }

            0xE9 => self.program_counter = self.hl(), // PCHL

            // stack ops
            0xC5 => {
                let v = self.bc();
                self.push(v);
            } // PUSH B
            0xD5 => {
                let v = self.de();
                self.push(v);
            } // PUSH D
            0xE5 => {
                let v = self.hl();
                self.push(v);
            } // PUSH H
            0xF5 => {
                let psw = ((self.reg_a as u16) << 8) | (self.status.bits() as u16 | 0x22);
                self.push(psw);
            }
            0xC1 => {
                let v = self.pop();
                self.set_bc(v);
            } // POP B
            0xD1 => {
                let v = self.pop();
                self.set_de(v);
            } // POP D
            0xE1 => {
                let v = self.pop();
                self.set_hl(v);
            } // POP H
            0xF1 => {
                let v = self.pop();
                self.reg_a = (v >> 8) as u8;
                self.status = CPUFlags::from_bits_truncate(v as u8);
            }
            0xE3 => {
                // XTHL
                let lo = self.mem_read(self.stack_pointer);
                let hi = self.mem_read(self.stack_pointer.wrapping_add(1));
                self.mem_write(self.stack_pointer, self.reg_l);
                self.mem_write(self.stack_pointer.wrapping_add(1), self.reg_h);
                self.reg_l = lo;
                self.reg_h = hi;
            }

            0xF9 => self.stack_pointer = self.hl(), // SPHL

            // I/O
            0xDB => {
                let port = self.fetch();
                self.reg_a = self.io[port as usize];
            } // IN
            0xD3 => {
                let port = self.fetch();
                self.io[port as usize] = self.reg_a;
            } // OUT

            // swap register pairs (DE <-> HL)
            0xEB => {
                let h = self.reg_h;
                let l = self.reg_l;
                self.reg_h = self.reg_d;
                self.reg_l = self.reg_e;
                self.reg_d = h;
                self.reg_e = l;
            }

            // MOV r, r (0x40..=0x7F, excluding 0x76 which is HLT)
            o @ 0x40..=0x7F => {
                let dst = (o >> 3) & 7;
                let src = o & 7;
                let v = self.reg(src);
                self.set_reg(dst, v);
            }

            // INR r / DCR r / MVI r
            o @ 0x04..=0x3C if (o & 0x07) == 0x04 => self.inr((o >> 3) & 7),
            o @ 0x05..=0x3D if (o & 0x07) == 0x05 => self.dcr((o >> 3) & 7),
            o @ 0x06..=0x3E if (o & 0x07) == 0x06 => {
                let v = self.fetch();
                self.set_reg((o >> 3) & 7, v);
            }

            // register ALU ops
            0x80..=0x87 => {
                let v = self.reg(op & 7);
                self.add_a(v, false);
            } // ADD
            0x88..=0x8F => {
                let v = self.reg(op & 7);
                self.add_a(v, true);
            } // ADC
            0x90..=0x97 => {
                let v = self.reg(op & 7);
                self.sub_a(v, false, true);
            } // SUB
            0x98..=0x9F => {
                let v = self.reg(op & 7);
                self.sub_a(v, true, true);
            } // SBB
            0xA0..=0xA7 => {
                let v = self.reg(op & 7);
                self.reg_a &= v;
                let a = self.reg_a;
                self.set_logic_flags(a);
            } // ANA
            0xA8..=0xAF => {
                let v = self.reg(op & 7);
                self.reg_a ^= v;
                let a = self.reg_a;
                self.set_logic_flags(a);
            } // XRA
            0xB0..=0xB7 => {
                let v = self.reg(op & 7);
                self.reg_a |= v;
                let a = self.reg_a;
                self.set_logic_flags(a);
            } // ORA
            0xB8..=0xBF => {
                let v = self.reg(op & 7);
                self.sub_a(v, false, false);
            } // CMP

            // RST n
            o if (o & 0xC7) == 0xC7 => {
                let n = ((o >> 3) & 7) as u16;
                let pc = self.program_counter;
                self.push(pc);
                self.program_counter = n * 8;
            }

            other => panic!(
                "unimplemented/undefined opcode {other:#04x} at pc {:#06x}",
                self.program_counter.wrapping_sub(1)
            ),
        }
    }
}
