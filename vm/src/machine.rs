use std::io::{self, Write};

pub const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

const IP: usize = 0;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Machine {
    memo: [u8; MEMORY_SIZE],
    registre: [u32; NREGS],
}

#[derive(Debug)]
pub enum Error {
    /// Attempt to create a machine with too large a memory
    MemoryOverflow,
    RegistreOverdepass,
    OutputError,
    InstructionError, // Add some more entries to represent different errors
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Errors
    /// This function returns an error when the memory exceeds `MEMORY_SIZE`.
    pub fn new(memory: &[u8]) -> Result<Self> {
        let size: usize = memory.len();

        if size > MEMORY_SIZE {
            return Err(Error::MemoryOverflow);
        }

        let mut mem: [u8; 4096] = [0; MEMORY_SIZE];

        mem[..size].copy_from_slice(memory);
        let mut reg = [0; NREGS];
        reg[0] = IP as u32;
        let ma_machine: Machine = Machine {
            memo: mem,
            registre: reg,
        };

        Ok(ma_machine)
        // Implemention done
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on `fd`.
    pub fn run_on<T: Write>(&mut self, fd: &mut T) -> Result<()> {
        while !self.step_on(fd)? {}
        Ok(())
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on standard output.
    pub fn run(&mut self) -> Result<()> {
        self.run_on(&mut io::stdout().lock())
    }

    /// Execute the next instruction by doing the following steps:
    ///   - decode the instruction located at IP (register 0)
    ///   - increment the IP by the size of the instruction
    ///   - execute the decoded instruction
    ///
    /// If output instructions are run, they print on `fd`.
    /// If an error happens at either of those steps, an error is
    /// returned.
    ///
    /// In case of success, `true` is returned if the program is
    /// terminated (upon encountering an exit instruction), or
    /// `false` if the execution must continue.
    pub fn step_on<T: Write>(&mut self, fd: &mut T) -> Result<bool> {
        // fetch instruction
        let memory = self.memory();
        let mut mem = [0;MEMORY_SIZE];
        mem.copy_from_slice(memory);
        let r0: usize = self.regs()[0] as usize;
        if r0 == MEMORY_SIZE {return  Err(Error::InstructionError);}
        let opcode: u8 = mem[r0];
        match opcode {
            1 => {
                self.set_reg(0, (r0 + 4) as u32)?;
                self.is_last(r0+3)?;
                let rd: u8 = mem[r0 + 1];
                let rs1: u8 = mem[r0 + 2];
                let rs2: u8 = mem[r0 + 3];
                self.move_if(rd, rs1, rs2)?;
                Ok(false)
            }
            2 => {
                self.set_reg(0, (r0 + 3) as u32)?;
                self.is_last(r0+2)?;
                let rs1: u8 = mem[r0 + 1];
                let rs2: u8 = mem[r0 + 2];
                self.store(rs1, rs2)?;
                Ok(false)
            }
            3 => {
                self.set_reg(0, (r0 + 3) as u32)?;
                self.is_last(r0+2)?;
                let rs1: u8 = mem[r0 + 1];
                let rs2: u8 = mem[r0 + 2];
                self.load(rs1, rs2)?;
                Ok(false)
            }
            4 => {
                self.set_reg(0, (r0 + 4) as u32)?;
                self.is_last(r0+3)?;
                let rd: u8 = mem[r0 + 1];
                let rs1: u8 = mem[r0 + 2];
                let rs2: u8 = mem[r0 + 3];
                self.loadimm(rd, rs1, rs2)?;
                Ok(false)
            }

            5 => {
                self.set_reg(0, (r0 + 4) as u32)?;
                self.is_last(r0+3)?;
                let rd: u8 = mem[r0 + 1];
                let rs1: u8 = mem[r0 + 2];
                let rs2: u8 = mem[r0 + 3];
                self.sub(rd, rs1, rs2)?;
                println!("here");
                Ok(false)
            }

            6 => {
                self.set_reg(0, (r0 + 2) as u32)?;
                self.is_last(r0+1)?;
                let rs1: u8 = mem[r0 + 1];
                self.out(rs1, fd)?;
                Ok(false)
            }
            7 => {
                self.set_reg(0, (r0 + 1) as u32)?;
                Ok(true)
            }

            8 => {
                self.set_reg(0, (r0 + 2) as u32)?;
                self.is_last(r0+1)?;
                let rs1: u8 = mem[r0 + 1];
                self.out_number(rs1, fd)?;
                Ok(false)
            }
            _ => Err(Error::InstructionError),
        }

        // decode instruction
    }

    /// Similar to [`step_on`](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    #[must_use]
    pub fn regs(&self) -> &[u32] {
        &(self.registre)[..]
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<()> {
        if reg > 15 {
            return Err(Error::RegistreOverdepass);
        }
        self.registre[reg] = value;
        Ok(())
    }

    /// Reference onto the machine current memory.
    #[must_use]
    pub fn memory(&self) -> &[u8] {
        &(self.memo)[..]
    }

    /// give a spicique registre
    pub fn get_reg(& self, reg: usize) -> Result<u32> {
        if reg > 15 {
            return Err(Error::RegistreOverdepass);
        }
        let registre = self.regs()[reg];
        Ok(registre)
    }

    /// store an u32 in the memory
    fn store_mem(&mut self, addres: usize, value: u32) -> Result<()> {
        if addres + 3 > MEMORY_SIZE {
            return Err(Error::MemoryOverflow);
        }
        self.memo[addres] = (value & 0xFF) as u8;
        self.memo[addres + 1] = ((value >> 8) & 0xFF) as u8;
        self.memo[addres + 2] = ((value >> 16) & 0xFF) as u8;
        self.memo[addres + 3] = ((value >> 24) & 0xFF) as u8;
        Ok(())
    }

    /// load  an u32 in the memory
    fn load_mem(& self, addres: usize) -> Result<u32> {
        if addres + 3 > MEMORY_SIZE {
            return Err(Error::MemoryOverflow);
        }
        let value: u32 = self.memo[addres] as u32
            + ((self.memo[addres + 1] as u32) << 8)
            + ((self.memo[addres + 2] as u32) << 16)
            + ((self.memo[addres + 3] as u32) << 24);
        Ok(value)
    }

    // instruction move_if
    fn move_if(&mut self, rd: u8, rs1: u8, rs2: u8) -> Result<()> {
        let test: u32 = self.get_reg(rs2 as usize)?;

        if test != 0 {
            let value: u32 = self.get_reg(rs1 as usize)?;
            self.set_reg(rd as usize, value)?;
            return Ok(());
        }
        Ok(())
    }
    /// instruction store
    fn store(&mut self, rs1: u8, rs2: u8) -> Result<()> {
        let value: u32 = self.get_reg(rs2 as usize)?;
        let addres = self.get_reg(rs1 as usize)?;
        self.store_mem(addres as usize, value)?;
        Ok(())
    }
    /// instruction load
    fn load(&mut self, rs1: u8, rs2: u8) -> Result<()> {
        let addres: u32 = self.get_reg(rs2 as usize)?;
        let value: u32 = self.load_mem(addres as usize)?;
        self.set_reg(rs1 as usize, value)?;
        Ok(())
    }
    /// instruction loadimm
    fn loadimm(&mut self, rd: u8, rs1: u8, rs2: u8) -> Result<()> {
        let l = rs1 as u16;
        let mut h = rs2 as u16;
        h <<= 8;

        let somme = l + h;
        let signed = somme as i16;
        let mut value = somme as u32;
        if signed < 0 {
            value = value + ((0xFF_u32) << 24) + ((0xff_u32) << 16);
        }
        self.set_reg(rd as usize, value)?;
        Ok(())
    }
    /// instruction sub
    fn sub(& mut self, rd: u8, rs1: u8, rs2: u8) -> Result<()> {
        let a = self.get_reg(rs1 as usize)? as i128;
        let b = self.get_reg(rs2 as usize)? as i128;
        let value = (a - b) as u32;
        self.set_reg(rd as usize, value)?;
        Ok(())
    }
    /// instruction out
    fn out<T: Write>(& self, rs1: u8, fd: &mut T) -> Result<()> {
        let mut data = self.get_reg(rs1 as usize)?;
        data &= 0xff;
        let value = data as u8 as char;
        let miss = write!(fd, "{value}");
        match miss {
            Err(_) => Err(Error::OutputError),
            Ok(_) => Ok(()),
        }
    }
    /// instruction out_number
    fn out_number<T: Write>(& self, rs1: u8, fd: &mut T) -> Result<()> {
        let data = self.get_reg(rs1 as usize)?;
        let value = data as i32;
        let miss = write!(fd, "{value}");
        match miss {
            Err(_) => Err(Error::OutputError),
            Ok(_) => Ok(()),
        }
    }
    // verify that the instruction is not well placed in memory
    fn is_last(&mut self, r0 :usize)-> Result<()>{
        if r0  >= MEMORY_SIZE {
            self.set_reg(0, MEMORY_SIZE as u32)?;
            return Err(Error::MemoryOverflow)}
        Ok(())
    }
}
