use table::DECODE_TABLE;

use super::octets;

pub mod table;
struct Decoder {
    state: usize,
    maybe_eos: bool,
}


impl Decoder {
    fn new() -> Decoder {
        Decoder {
            state: 0,
            maybe_eos: false,
        }
    }

    // Decodes 4 bits
    fn decode4(&mut self, input: u8) -> Result<Option<u8>,()> {
        const MAYBE_EOS: u8 = 1;//可能结束
        const DECODED: u8 = 2;
        const ERROR: u8 = 4;

        // (next-state, byte, flags)
        let (next, byte, flags) = DECODE_TABLE[self.state][input as usize];

        if flags & ERROR == ERROR {
            // Data followed the EOS marker
  
            return Err(());
        }

        let ret = if flags & DECODED == DECODED {
            Some(byte)
        } else {
            None
        };

        self.state = next;
        self.maybe_eos = flags & MAYBE_EOS == MAYBE_EOS;

        Ok(ret)
    }

    fn is_final(&self) -> bool {
        self.state == 0 || self.maybe_eos
    }
}

pub fn decode(b: &mut octets::Octets) -> Result<Vec<u8>,()> {
    
    let mut out = Vec::with_capacity(b.len() << 1);

    let mut decoder = Decoder::new();

    while b.cap() > 0 {
        let byte = b.get_u8().unwrap();
        

        if let Some(b) = decoder.decode4(byte >> 4)? {
            out.push(b);
        }

        if let Some(b) = decoder.decode4(byte & 0xf)? {
            out.push(b);
        }
    }
   
    if !decoder.is_final() {
        return Err(());
    }

    Ok(out)
}
