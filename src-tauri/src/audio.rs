use std::thread;
use std::time::Duration;
use rodio::{OutputStream, Sink, source::SineWave, Source};

pub struct SilentAudioPlayer {
    _stream: Option<OutputStream>,
    is_running: bool,
}

impl SilentAudioPlayer {
    pub fn new() -> Self {
        Self {
            _stream: None,
            is_running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            return Ok(()); // already running
        }

        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio stream: {e}"))?;
        self._stream = Some(stream);

        self.is_running = true;

        // Spawn a thread to periodically play short bursts
        let handle = stream_handle.clone();
        thread::spawn(move || {
            loop {
                // Each burst: 2 seconds of 15Hz sine wave, very low amplitude
                let sink = Sink::try_new(&handle).unwrap();
                let source = SineWave::new(15.0)
                    .amplify(0.001) // extremely low volume
                    .take_duration(Duration::from_secs(2));
                sink.append(source);
                sink.sleep_until_end();

                // Wait 5 minutes before next burst
                thread::sleep(Duration::from_secs(5 * 60));
            }
        });

        Ok(())
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        self._stream = None; // drops the stream, stops all audio
    }

    pub fn is_playing(&self) -> bool {
        self.is_running
    }
}
