use anyhow::Result;
use volumecontrol::AudioDevice;

#[derive(Debug)]
pub struct VolumeControl {
    pub device: AudioDevice,
}

pub trait AudioControl {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn get_volume(&self) -> Result<u8>;
    fn set_volume(&mut self, volume: u8) -> Result<()>;
    fn get_mute(&self) -> Result<bool>;
    fn set_mute(&mut self, mute: bool) -> Result<()>;
}

impl AudioControl for VolumeControl {
    fn new() -> Result<Self> {
        let device = AudioDevice::from_default()?;
        Ok(VolumeControl { device })
    }
    fn get_volume(&self) -> Result<u8> {
        let volume = self.device.get_vol()?;
        Ok(volume)
    }
    fn set_volume(&mut self, volume: u8) -> Result<()> {
        self.device.set_vol(volume)?;
        Ok(())
    }
    fn get_mute(&self) -> Result<bool> {
        let mute = self.device.is_mute()?;
        Ok(mute)
    }
    fn set_mute(&mut self, mute: bool) -> Result<()> {
        self.device.set_mute(mute)?;
        Ok(())
    }
}
