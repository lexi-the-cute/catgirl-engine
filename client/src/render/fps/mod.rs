use std::collections::VecDeque;

/// FPS tracker
#[derive(Debug, Default)]
pub struct FPS {
    /// Number of frames counted per second
    pub counted_frames: u64,

    /// Last time a frame was rendered
    pub frame_time_measurement: u64,

    /// Last X frame counts
    pub frame_counts: VecDeque<u64>,

    /// Maximum number of elements in frame count vector
    pub frame_count_max: usize,
}

impl FPS {
    /// Create a new FPS tracker
    #[must_use]
    pub fn new() -> Self {
        Self {
            counted_frames: 0,
            frame_time_measurement: utils::get_current_time_seconds(),
            frame_counts: VecDeque::new(),
            frame_count_max: 10,
        }
    }

    /// Get the number of frames that have been counted
    #[must_use]
    pub fn get_counted_frames(&self) -> u64 {
        self.counted_frames
    }

    /// Reset the frame counter back to zero
    pub fn reset_frame_count(&mut self) {
        self.counted_frames = 0;
    }

    /// Increase the count of frames rendered
    pub fn count_frame(&mut self) {
        self.counted_frames += 1;
    }

    /// Check if the FPS has exceeded the cap
    #[must_use]
    pub fn is_less_than_cap(&self) -> bool {
        self.get_counted_frames() < utils::args::get_args().fps_cap
    }

    /// Get and update the last measured frame time
    pub fn get_frame_time_measurement(&mut self) -> u64 {
        self.frame_time_measurement = utils::get_current_time_seconds();

        self.frame_time_measurement
    }

    /// Get average FPS
    pub fn get_average_counted_frames(&self) -> u64 {
        let mut frame_counts: u64 = 0;
        let mut size: u64 = 0;
        for i in self.frame_counts.iter() {
            frame_counts += i;
            size += 1;
        }

        trace!(
            "Frame Counts: {frame_counts} - Size: {size} - Both: {}",
            frame_counts / size
        );
        frame_counts /= size;

        frame_counts
    }

    /// Records frame count history for calculating averages
    pub fn record_frame_count_history(&mut self) {
        if self.frame_counts.len() >= self.frame_count_max {
            self.frame_counts.pop_front();
        }

        self.frame_counts.push_back(self.get_counted_frames());
    }

    /// Check if at least one second has passed by since the last measurement
    pub fn one_second_passed(&mut self) -> bool {
        let passed: bool = self.frame_time_measurement < self.get_frame_time_measurement();

        if passed {
            self.record_frame_count_history();
        }

        passed
    }
}
