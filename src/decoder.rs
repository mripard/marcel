use serde_yaml as yaml;
use std::fmt;
use std::fs::File;

use error::Result;
use trace::RegisterWrite;
use trace::Trace;

#[derive(Deserialize)]
struct Bit {
    name:	String,
    index:	u8,
}

impl fmt::Debug for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {:?} @ {:?}", self.name, self.index)
    }
}

#[derive(Deserialize)]
struct Register {
    name:	String,
    offset:	u64,
    bits:	Vec<Bit>,
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {:?} offset: {:?}\n", self.name, self.offset);

        for field in &self.bits {
            write!(f, "\t- {:?}\n", field);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Device {
    offset:	u64,
    regs:	Vec<Register>,
}

impl Device {
    fn decode_register(&self, _reg: &RegisterWrite) {
        let mut iter = self.regs.iter();
        let reg_desc = match iter.find(|x| x.offset == (_reg.register - self.offset)) {
            None => {
                println!("Unknown Register: {:08x?}", _reg.register - self.offset);
                return;
            }
            Some(v) => v,
        };

        println!("-- Register: {} (0x{:04x})", reg_desc.name, reg_desc.offset);

        for bit in &reg_desc.bits {
            if ((1 << bit.index) & _reg.value) != 0 {
                println!("   + Bit: {} (0x{:08x})", bit.name, 1 << bit.index);
            }
        }
    }

    pub fn new(filename: &str, offset: u64) -> Result<Device> {
        let file = File::open(&filename)?;
        let regs = yaml::from_reader(&file)?;

        Ok(Device {
            offset: offset,
            regs: regs,
        })
    }

    pub fn decode(&self, _trace: &Trace) {
        for write in &_trace.writes {
            self.decode_register(&write);
        }
    }
}
