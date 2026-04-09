use std::sync::mpsc::Receiver;
use sulfate_core::video_feed::RawVideoFrame;
use xcap::{Frame, Monitor, VideoRecorder, XCapResult as Result};

pub struct ScreenVideoFeed {
    record_handler: VideoRecorder,
    frame_receiver: Receiver<Frame>,
}

impl ScreenVideoFeed {
    pub fn fetch_available_monitors() -> Result<Vec<Monitor>> {
        xcap::Monitor::all()
    }

    pub fn record_monitor(monitor: Monitor) -> Result<Self> {
        let (record_handler, frame_receiver) = monitor.video_recorder()?;

        Ok(ScreenVideoFeed {
            record_handler,
            frame_receiver,
        })
    }

    pub fn start(&self) -> Result<()> {
        self.record_handler.start()
    }

    pub fn stop(&self) -> Result<()> {
        self.record_handler.stop()
    }

    pub fn latest_frame(&self) -> Option<RawVideoFrame> {
        let mut latest_frame = None;

        while let Ok(frame) = self.frame_receiver.try_recv() {
            latest_frame = Some(frame)
        }

        latest_frame.map(|frame| RawVideoFrame {
            width: frame.width,
            height: frame.height,
            bytes: frame.raw,
        })
    }
}
