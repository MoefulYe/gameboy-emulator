pub trait AudioOutput {
    fn set_samples(&mut self, left: f32, right: f32);
}

pub struct WebAudioOutput {
    left_buffer: Vec<f32>,
    right_buffer: Vec<f32>,
    last_left: f32,
    last_right: f32,
    volume: f32,
    freq_scale: f32,
}

impl WebAudioOutput {
    pub fn new(volume: f32, freq_scale: f32) -> Self {
        Self {
            left_buffer: Vec::with_capacity(1024),
            right_buffer: Vec::with_capacity(1024),
            volume,
            last_left: 0.0,
            last_right: 0.0,
            freq_scale: freq_scale.max(0.0),
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = match volume {
            n if n < 0.0 => 0.0,
            n if n > 1.0 => 1.0,
            _ => volume,
        }
    }

    pub fn set_freq_scale(&mut self, freq_scale: f32) {
        self.freq_scale = freq_scale.max(0.0)
    }

    pub fn clear_buffer(&mut self) {
        self.left_buffer.clear();
        self.right_buffer.clear();
    }

    pub fn reset(&mut self) {
        self.left_buffer.clear();
        self.right_buffer.clear();
        self.last_left = 0.0;
        self.last_right = 0.0;
    }

    pub unsafe fn buffer(&self) -> (js_sys::Float32Array, js_sys::Float32Array) {
        let left = js_sys::Float32Array::view(&self.left_buffer);
        let right = js_sys::Float32Array::view(&self.right_buffer);
        (left, right)
    }
}

impl AudioOutput for WebAudioOutput {
    fn set_samples(&mut self, left: f32, right: f32) {
        let right = right * self.volume;
        let left = left * self.volume;
        let step = 1.0 / self.freq_scale;
        let mut t = 0.0;
        while t < 1.0 {
            let alpha = t;
            let interpolated_left = self.last_left * (1.0 - alpha) + left * alpha;
            let interpolated_right = self.last_right * (1.0 - alpha) + right * alpha;
            self.left_buffer.push(interpolated_left);
            self.right_buffer.push(interpolated_right);
            t += step;
        }
        self.last_left = left;
        self.last_right = right;
    }
}
