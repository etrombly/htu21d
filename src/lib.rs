#![no_std]

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

/** Default I2C address for the HTU21D. */
pub const ADDRESS: u8 = 0x40;

#[allow(dead_code)]
#[derive(Copy, Clone)]
/// Register addresses
pub enum Register {
    READTEMP = 0xE3,
    READHUM = 0xE5,
    WRITEREG = 0xE6,
    READREG = 0xE7,
    RESET = 0xFE,
}

impl Register {
    /// Get register address.
    fn addr(self) -> u8 {
        self as u8
    }
}

pub struct Htu21df<I2C> {
    pub i2c: I2C,
}

impl<I2C, E> Htu21df<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
{
    /// Creates a new driver from a I2C peripheral
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let mut htu21df = Htu21df { i2c };

        htu21df.reset()?;

        Ok(htu21df)
    }

    /// write to register
    pub fn write_register(&mut self, reg: Register, data: u8) -> Result<(), E> {
        self.i2c.write(ADDRESS, &[reg.addr(), data])
    }

    pub fn reset(&mut self) -> Result<(), E> {
        self.i2c.write(ADDRESS, &[Register::RESET.addr()])?;
        Ok(())
    }

    pub fn get_user_reg(&mut self) -> Result<u8, E> {
        let mut buffer: [u8; 1] = [0];
        self.i2c
            .write_read(ADDRESS, &[Register::READTEMP.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    pub fn get_humidity(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 3] = [0; 3];
        self.i2c
            .write_read(ADDRESS, &[Register::READHUM.addr()], &mut buffer)?;
        // discard the status bits
        buffer[1] &= 0b1111_1100;
        let mut temp = f32::from(u16::from(buffer[0]) + (u16::from(buffer[1]) << 8));
        temp *= 125.0;
        temp /= 65536.0;
        temp -= 6.0;
        Ok(temp)
    }
}
