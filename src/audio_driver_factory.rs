use audio_driver::AudioDriver;

#[cfg(target_os = "macos")]
use core_audio_driver::CoreAudioDriver;

pub fn create_default() -> Box<AudioDriver> {
    Box::new(
        if cfg!(target_os = "macos") {
            CoreAudioDriver::new()
        } else {
            unreachable!()
        })
}
