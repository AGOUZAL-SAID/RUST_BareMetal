use core::error;
use std::{io::{self, Write}, os::fd::IntoRawFd, u128};

pub const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

const IP: usize = 0;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Machine {
    memo : [u8;MEMORY_SIZE],
    registre : [u32; 16],
}

#[derive(Debug)]
pub enum Error {
    /// Attempt to create a machine with too large a memory
    MemoryOverflow,RegistreOverdepass
    // Add some more entries to represent different errors
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Errors
    /// This function returns an error when the memory exceeds `MEMORY_SIZE`.
    pub fn new(memory: &[u8]) -> Result<Self> {
        
        let size : usize = memory.len();
        
        if size > MEMORY_SIZE {return Err(Error::MemoryOverflow)}

        let mut mem :[u8; 4096] = [0;MEMORY_SIZE];

        mem.copy_from_slice(&memory[..]);
        let mut reg: [u32; 16] = [0;16];
        reg[1] = 10;
        reg[2] = 25;
        reg[3] = 0x1234ABCD;
        reg[5] = 65;
        
        let ma_machine : Machine = Machine{memo:mem ,registre : reg};

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
        let mem: &[u8]    = self.memory();
        let regs: &[u32]  = self.regs(); 
        let r0: usize     = regs[0] as usize ;
        let opcode : u8   = mem[r0];
        match opcode {
            1 => {
                let rd : u8  = mem[r0+1];
                let rs1: u8  = mem[r0+2];
                let rs2: u8  = mem[r0+3];
                self.set_reg(0, (r0+4) as u32)?;
                self.move_if(rd,rs1,rs2);
                Ok(false)
            }
            2 => {
                let rs1: u8    = mem[r0+2];
                let rs2: u8    = mem[r0+3];
                self.set_reg(0, (r0+3) as u32)?;
                self.store(rs1,rs2);
                Ok(false)
            }
            3 => {
                let rs1: u8    = mem[r0+2];
                let rs2: u8    = mem[r0+3];
                self.set_reg(0, (r0+3) as u32)?;
                self.load(rs1,rs2);
                Ok(false)
            }
            4 => {
                let rd : u8  = mem[r0+1];
                let rs1: u8  = mem[r0+2];
                let rs2: u8  = mem[r0+3];
                self.set_reg(0, (r0+4) as u32)?;
                self.loadimm(rd,rs1,rs2);
                Ok(false)
            }
            
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
        if reg > 15 {return Err(Error::RegistreOverdepass);}
        self.registre[reg] = value;
        Ok(())
    }

    /// Reference onto the machine current memory.
    #[must_use]
    pub fn memory(&self) -> &[u8] {
        &(self.memo)[..]
    }

    /// give a spicique registre 
    pub fn get_reg(&mut self, reg: usize)-> Result<u32>{
        if reg > 15 {
            return Err(Error::RegistreOverdepass);
        }
        Ok(self.regs()[reg])
    }

    /// store an item in the memory
    fn store_mem(&mut self,addres : usize, value :u32) ->Result<()>{
        if addres+3 > MEMORY_SIZE {return Err(Error::MemoryOverflow);}
        self.memo[addres]     = (value & 0xFF) as u8;
        self.memo[addres + 1] = ((value >> 8) & 0xFF) as u8;
        self.memo[addres + 2] = ((value >> 16) & 0xFF) as u8;
        self.memo[addres + 3] = ((value >> 24) & 0xFF) as u8;
        Ok(())
    }

    fn load_mem(&mut self,addres : usize)->Result<u32>{
        if addres+3 > MEMORY_SIZE {return Err(Error::MemoryOverflow);}
        let value : u32  =self.memo[addres] as u32 + self.memo[addres + 1] as u32 + self.memo[addres + 2] as u32 + self.memo[addres + 3] as u32 ;
        Ok(value)  
    }

    // instruction move_if
    fn move_if(&mut self,rd :u8 , rs1 : u8 , rs2 : u8)->Result<()>{
        let test: u32 =self.get_reg(rs2 as usize)?; 

        if test !=0 {
            let value: u32 = self.get_reg(rs1 as usize)?;
            self.set_reg(rd as usize,value);
            return Ok(());
        }
        Ok(())
    }

    fn store(&mut self,rs1 :u8 ,rs2 :u8)->Result<()>{
        let value: u32 = self.get_reg(rs2 as usize)?;
        let addres = self.get_reg(rs1 as usize)?;
        self.store_mem(addres as usize, value)?;
        Ok(())
    }
    fn load(&mut self,rs1 :u8 ,rs2 :u8)->Result<()>{
        let addres: u32 = self.get_reg(rs2 as usize)?;
        let value: u32 = self.load_mem(addres as usize)?;
        self.set_reg(rs1 as usize, value)?;
        Ok(())
    }
    fn loadimm(&mut self,rd :u8 , rs1 : u8 , rs2 : u8) ->Result<()>{
        let l  = rs1  as u16 ;
        let mut h  = rs2  as u16 ;
        h = h << 8 ;

        let somme  = l+h;
        let signed = somme as i16;
        let mut value  = somme as u32 ;
        if signed <0 {value = value + (0xFF as u32)<<24 + (0xff as u32) <<16;}

        self.set_reg(rd as usize, value)?;
        Ok(())



    }
}
