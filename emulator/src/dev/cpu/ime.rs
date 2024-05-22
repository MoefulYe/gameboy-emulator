pub struct InterruptMasterEnableRegsiter {
    enabled: bool,
    enabling_countdown: u8,
}

impl InterruptMasterEnableRegsiter {
    pub fn new() -> Self {
        Self {
            enabled: false,
            enabling_countdown: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabling_countdown = 2;
    }

    pub fn disable(&mut self) {
        self.enabling_countdown = 0;
        self.enabled = false;
    }

    pub fn countdown(&mut self) {
        if self.enabling_countdown > 0 {
            self.enabling_countdown -= 1;
            if self.enabling_countdown == 0 {
                self.enabled = true;
            }
        }
    }
}
