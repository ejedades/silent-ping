use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct SilentAudioPlayer {
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
    is_playing: bool,
}

impl SilentAudioPlayer {
    pub fn new() -> Self {
        Self {
            sink: None,
            _stream: None,
            is_playing: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.is_playing {
            return Ok(());
        }

        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio stream: {e}"))?;

        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Failed to create sink: {e}"))?;

        sink.append(SilentSource::new());
        sink.set_volume(0.0);

        self._stream = Some(stream);
        self.sink = Some(sink);
        self.is_playing = true;

        Ok(())
    }

    pub fn stop(&mut self) {
        if !self.is_playing {
            return;
        }

        if let Some(sink) = self.sink.take() {
            sink.stop();
        }

        self._stream = None;
        self.is_playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

/// Silent audio source — infinite silence
struct SilentSource {
    sample_rate: u32,
    channels: u16,
}

impl SilentSource {
    fn new() -> Self {
        Self {
            sample_rate: 48_000,
            channels: 2,
        }
    }
}

impl Iterator for SilentSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0.0)
    }
}

impl Source for SilentSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
