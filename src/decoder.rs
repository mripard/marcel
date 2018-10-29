use serde_yaml as yaml;
use std::collections::HashMap;
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

    #[serde(default)]
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
    state:	HashMap<u64, u64>,
}

impl Device {
    fn decode_register(&mut self, _reg: &RegisterWrite) {
        let mut iter = self.regs.iter();
        let reg_desc = match iter.find(|x| x.offset == (_reg.register - self.offset)) {
            None => {
                println!("Unknown Register: {:08x?}", _reg.register - self.offset);
                return;
            }
            Some(v) => v,
        };

        let reg_state = match self.state.get(&reg_desc.offset) {
            None => 0,
            Some(v) => *v,
        };

        println!("-- Register: {} (0x{:04x})", reg_desc.name, reg_desc.offset);

        let mut cache = _reg.value;
        for bit in &reg_desc.bits {
            let mask = 1 << bit.index;

            if (mask & (reg_state ^ cache)) == 0 {
                continue;
            }

            if (reg_state & mask) == 0 {
                println!("   + Bit: {} (0x{:08x})", bit.name, mask);
                cache = cache - mask;
            } else {
                println!("   - Bit: {} (0x{:08x})", bit.name, mask);
            }
        }

        if cache != 0 {
            println!("Undecoded values: {:08x?}", cache);
        }

        self.state.insert(reg_desc.offset, _reg.value);
    }

    pub fn new(filename: &str, offset: u64) -> Result<Device> {
        let file = File::open(&filename)?;
        let regs = yaml::from_reader(&file)?;

        Ok(Device {
            offset: offset,
            regs: regs,
            state: HashMap::new(),
        })
    }

    pub fn decode(&mut self, _trace: &Trace) {
        for write in &_trace.writes {
            self.decode_register(&write);
        }
    }
}
