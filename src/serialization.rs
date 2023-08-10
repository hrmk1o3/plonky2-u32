use alloc::vec::Vec;
use plonky2::{util::serialization::{Buffer, IoResult, Read, Write, IoError}, iop::{target::Target, wire::Wire}};

use crate::gadgets::arithmetic_u32::U32Target;

pub trait WriteU32 {
    fn write_target_u32(&mut self, x: U32Target) -> IoResult<()>;
}

// fn write_u8(input: &mut Vec<u8>, x: u8) -> IoResult<()> {
//     input.write_all(&[x])
// }

fn write_bool(input: &mut Vec<u8>, x: bool) -> IoResult<()> {
    input.write_u8(u8::from(x))
}

fn write_usize(input: &mut Vec<u8>, x: usize) -> IoResult<()> {
    input.write_all(&(x as u64).to_le_bytes())
}

impl WriteU32 for Vec<u8> {
    #[inline]
    fn write_target_u32(&mut self, x: U32Target) -> IoResult<()> {
        // self.write_target(x.0)
        match x.0 {
            Target::Wire(Wire { row, column }) => {
                write_bool(self, true)?;
                write_usize(self,row)?;
                write_usize(self,column)?;
            }
            Target::VirtualTarget { index } => {
                write_bool(self,false)?;
                write_usize(self,index)?;
            }
        };

        Ok(())
    }
}

pub trait ReadU32 {
    fn read_target_u32(&mut self) -> IoResult<U32Target>;
}

fn read_bool(buffer: &mut Buffer) -> IoResult<bool> {
    let i = buffer.read_u8()?;
    match i {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(IoError),
    }
}

fn read_usize(buffer: &mut Buffer) -> IoResult<usize> {
    let mut buf = [0; 8];
    buffer.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf) as usize)
}

impl ReadU32 for Buffer {
    #[inline]
    fn read_target_u32(&mut self) -> IoResult<U32Target> {
        let is_wire = read_bool(self)?;
        let result = if is_wire {
            let row = read_usize(self)?;
            let column = read_usize(self)?;
            
            Target::wire(row, column)
        } else {
            let index = read_usize(self)?;
            
            Target::VirtualTarget { index }
        };

        Ok(U32Target(result))
    }
}
