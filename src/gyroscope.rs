//! Interface to the gyroscope.

use errors::{Error, ErrorKind, Result, ResultExt};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use registers;


/// The I2C address of the gyroscope.
const I2C_ADDRESS: u16 = 0xD4 >> 1;


/// Interface to the L3GD20H digital gyroscope.
pub struct Gyroscope<Dev>
    where Dev: I2CDevice
{
    device: Dev,
    range: MeasurementRange,
}


/// Settings for the measurement range of the gyroscope.
pub enum MeasurementRange {
    /// +/- 245 degrees per second
    Dps245,
    /// +/- 500 degrees per second
    Dps500,
    /// +/- 2000 degrees per second
    Dps2000,
}


impl Gyroscope<LinuxI2CDevice> {
    /// Initialize the gyroscope for a Linux I2C device.
    pub fn new<Path>(path: Path) -> Result<Gyroscope<LinuxI2CDevice>>
        where Path: AsRef<::std::path::Path>
    {
        let device =
            LinuxI2CDevice::new(&path, I2C_ADDRESS).chain_err(|| ErrorKind::FailedToOpenDevice)?;

        Gyroscope::from_i2c_device(device)
    }
}


impl<Dev> Gyroscope<Dev>
    where Dev: I2CDevice,
          Error: From<Dev::Error>
{
    /// Initialize the gyroscope from an I2C device.
    pub fn from_i2c_device(mut device: Dev) -> Result<Gyroscope<Dev>> {
        use registers::{CTRL_1, PD};

        // Set power mode to on
        let bits = device.smbus_read_byte_data(CTRL_1)?;
        let mut ctrl1 = registers::Ctrl1::from_bits_truncate(bits);
        ctrl1.insert(PD);
        device.smbus_write_byte_data(CTRL_1, ctrl1.bits())?;

        let range = MeasurementRange::Dps245;
        let gyroscope = Gyroscope { device, range };
        Ok(gyroscope)
    }

    /// Read the gyroscope.
    ///
    /// Returns a tuple of (x, y, z) rotational velocities in degrees per second.
    pub fn read_rotation(&mut self) -> Result<(f32, f32, f32)> {
        use byteorder::{LittleEndian, ReadBytesExt};
        use std::io::Cursor;

        let data = self.device
            .smbus_read_i2c_block_data(registers::OUT_X_L, 6)?;

        let mut cursor = Cursor::new(&data);

        // Scale of a unit, in degrees per second.
        // Refer to Table 3 of the datasheet.
        let scale = 0.001 *
                    match self.range {
                        MeasurementRange::Dps245 => 8.75,
                        MeasurementRange::Dps500 => 17.50,
                        MeasurementRange::Dps2000 => 70.00,
                    };

        let x = cursor.read_i16::<LittleEndian>()? as f32 * scale;
        let y = cursor.read_i16::<LittleEndian>()? as f32 * scale;
        let z = cursor.read_i16::<LittleEndian>()? as f32 * scale;

        let out = (x, y, z);
        Ok(out)
    }

    /// Set the measurement range.
    pub fn set_range(&mut self, range: MeasurementRange) -> Result<()> {
        use registers::{Ctrl4, CTRL_4, FS1, FS0};

        let bits = self.device.smbus_read_byte_data(CTRL_4)?;
        let mut flags = Ctrl4::from_bits_truncate(bits);

        flags.remove(FS1 | FS0);
        let setting = match range {
            MeasurementRange::Dps245 => Ctrl4::empty(),
            MeasurementRange::Dps500 => FS0,
            MeasurementRange::Dps2000 => FS1,
        };
        flags.insert(setting);

        self.device.smbus_write_byte_data(CTRL_4, flags.bits())?;
        self.range = range;

        Ok(())
    }
}
