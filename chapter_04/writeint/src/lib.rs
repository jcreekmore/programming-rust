use std::io;
pub trait WriteInt {
    fn write_i32(&mut self, i: i32) -> io::Result<()>;
}

impl<W: ?Sized + io::Write> WriteInt for W {
    fn write_i32(&mut self, i: i32) -> io::Result<()> {
        let mut buf: [u8; 4] = [0; 4];
        buf[0] = (i & 0xFF) as u8;
        buf[1] = ((i >> 8) & 0xFF) as u8;
        buf[2] = ((i >> 16) & 0xFF) as u8;
        buf[3] = ((i >> 24) & 0xFF) as u8;

        try!(self.write(&buf));
        Ok(())
    }
}

pub fn test() -> io::Result<()> {
    let mut csr = io::Cursor::new(vec![]);
    csr.write_i32(0x12345678)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test().unwrap();
    }
}
