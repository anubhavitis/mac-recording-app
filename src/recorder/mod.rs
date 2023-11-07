
use audio::AudioRecorder;

mod audio;
pub enum RecorderType {
    Audio
}

pub trait Recorder {
    fn start_recording(&mut self);
    fn stop_recording(&mut self);
}

impl Recorder for AudioRecorder {
    fn start_recording(&mut self) {
        self.start_recording();
    }

    fn stop_recording(&mut self) {
        self.stop_recording();
    }
}

pub fn get_recorder(param:RecorderType) -> Box<dyn Recorder>{
    match param {
        RecorderType::Audio => Box::new(AudioRecorder::new()),
    }
}