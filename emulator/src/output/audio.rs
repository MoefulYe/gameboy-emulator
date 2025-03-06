use crate::external::emulator_audio_callback;
pub trait AudioOutput {
    fn set_samples(&mut self, left: f32, right: f32);
}

pub struct WebAudioOutput {
    left_buffer: Vec<f32>,
    right_buffer: Vec<f32>,
    volume: f32,
}

impl WebAudioOutput {
    pub fn new(volume: f32) -> Self {
        Self {
            left_buffer: Vec::with_capacity(1024),
            right_buffer: Vec::with_capacity(1024),
            volume,
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = match volume {
            n if n < 0.0 => 0.0,
            n if n > 1.0 => 1.0,
            _ => volume,
        }
    }

    pub fn clear_buffer(&mut self) {
        self.left_buffer.clear();
        self.right_buffer.clear();
    }

    pub fn reset(&mut self) {
        self.clear_buffer();
    }

    pub fn update(&mut self) {
        // let left_buffer = unsafe { js_sys::Float32Array::view(&self.left_buffer) };
        // let right_buffer = unsafe { js_sys::Float32Array::view(&self.right_buffer) };
        emulator_audio_callback(&self.left_buffer, &self.right_buffer);
    }
}

impl AudioOutput for WebAudioOutput {
    fn set_samples(&mut self, left: f32, right: f32) {
        // let right = right * self.volume;
        // let left = left * self.volume;
        // let step = 1.0 / self.freq_scale;
        // let mut t = 0.0;
        // while t < 1.0 {
        //     let alpha = t;
        //     let interpolated_left = self.last_left * (1.0 - alpha) + left * alpha;
        //     let interpolated_right = self.last_right * (1.0 - alpha) + right * alpha;
        //     self.left_buffer.push(interpolated_left);
        //     self.right_buffer.push(interpolated_right);
        //     t += step;
        // }
        // self.last_left = left;
        // self.last_right = right;
        let left_sample = left * self.volume;
        let right_sample = right * self.volume;
        self.left_buffer.push(left_sample);
        self.right_buffer.push(right_sample);
    }
}
