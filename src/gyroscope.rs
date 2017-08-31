//! Interface to the gyroscope.

use errors::{ErrorKind, Result, ResultExt};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;


/// The I2C address of the gyroscope.
const I2C_ADDRESS: u16 = 0xD4 >> 1;


/// Interface to the L3GD20H digital gyroscope.
pub struct Gyroscope<Dev>
    where Dev: I2CDevice
{
    device: Dev,
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
    where Dev: I2CDevice
{
    /// Initialize the gyroscope from an I2C device.
    pub fn from_i2c_device(device: Dev) -> Result<Gyroscope<Dev>> {
        let gyroscope = Gyroscope { device };
        Ok(gyroscope)
    }

    /// Read the gyroscope.
    ///
    /// Returns a tuple of (x, y, z) rotational velocities.
    pub fn read_rotation(&mut self) -> Result<(i16, i16, i16)> {
        bail!("Not implemented")
    }
}
