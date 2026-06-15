use anyhow::Result;
use cpvc::AudioDevice;

pub struct VolumeControl {
    pub device: AudioDevice,
}

pub trait AudioControl {
    fn new() -> Result<Self, Error>
    where
        Self: Sized;
    fn get_volume(&self) -> Result<u8, AudioError>;
    fn set_volume(&mut self, volume: u8) -> Result<(), AudioError>;
    fn get_mute(&self) -> Result<bool, AudioError>;
    fn set_mute(&mut self, mute: bool) -> Result<(), AudioError>;
}

impl AudioControl for VolumeControl {
    fn new() -> Result<Self, Error> {
        let device = AudioDevice::from_default()?;
        Ok(VolumeControl { device })
    }
    fn get_volume(&self) -> Result<u8, AudioError> {
        let volume = self.device.get_vol()?;
        Ok(volume)
    }
    fn set_volume(&mut self, volume: u8) -> Result<(), AudioError> {
        self.device.set_vol(volume)?;
        Ok(())
    }
    fn get_mute(&self) -> Result<bool, AudioError> {
        let mute = self.device.is_mute()?;
        Ok(mute)
    }
    fn set_mute(&mut self, mute: bool) -> Result<(), AudioError> {
        self.device.set_mute(mute)?;
        Ok(())
    }
}
