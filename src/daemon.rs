use std::time::Duration;
use tokio::time::{interval, Instant};
use crate::devices::speakers::Speaker;
use crate::devices::keyboards::apex_pro::ApexProGen3;
use crate::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Dynamic lighting canvas for synchronization
pub struct AnimationCanvas {
    pub arena_colors: [[u8; 3]; 4],
    pub apex_matrix: [u8; 650],
}

impl Default for AnimationCanvas {
    fn default() -> Self {
        Self {
            arena_colors: [[0; 3]; 4],
            apex_matrix: [0; 650],
        }
    }
}

/// The main daemon for interleaved multi-device streaming
pub struct Daemon {
    speaker: Box<dyn Speaker>,
    keyboard: ApexProGen3,
    running: Arc<AtomicBool>,
}

impl Daemon {
    pub fn new(speaker: Box<dyn Speaker>, keyboard: ApexProGen3) -> Self {
        Self {
            speaker,
            keyboard,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Stops the daemon loop
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Starts the interleaved synchronization heartbeat loop
    pub async fn start(&mut self, canvas_source: tokio::sync::watch::Receiver<AnimationCanvas>) -> Result<()> {
        self.running.store(true, Ordering::SeqCst);
        
        // 60ms to 65ms heartbeat window (we'll use 62ms)
        let mut ticker = interval(Duration::from_millis(62));
        
        // Prep the Apex Pro controller for real-time matrix animation updates
        let _ = self.keyboard.stream_initialization();

        let mut canvas_rx = canvas_source;

        while self.running.load(Ordering::SeqCst) {
            ticker.tick().await;

            // Get the latest frame from the canvas source
            let canvas = {
                let frame = canvas_rx.borrow();
                // Array slice memory updates logic to grab the data
                AnimationCanvas {
                    arena_colors: frame.arena_colors,
                    apex_matrix: frame.apex_matrix,
                }
            };

            // 1. Serialize and write the 72-byte SET_REPORT directly to Arena 7
            if let Err(e) = self.speaker.set_dynamic_colorshift(&canvas.arena_colors) {
                tracing::error!("Failed to update Arena 7: {:?}", e);
            }

            // 2. Wait 1ms to 5ms (let's sleep 2ms)
            tokio::time::sleep(Duration::from_millis(2)).await;

            // 3. Serialize and stream the 650-byte lighting matrix payload to Apex Pro
            if let Err(e) = self.keyboard.stream_matrix_buffer(&canvas.apex_matrix) {
                tracing::error!("Failed to update Apex Pro matrix: {:?}", e);
            }
        }

        // Clean break routine when animations cease
        self.clean_break()?;

        Ok(())
    }

    fn clean_break(&mut self) -> Result<()> {
        // Drop the active dynamic streaming loops and send uniform off/reset commands to zero out active device matrices
        let empty_arena = [[0u8; 3]; 4];
        let empty_apex = [0u8; 650];

        let _ = self.speaker.set_dynamic_colorshift(&empty_arena);
        tokio::time::sleep(std::time::Duration::from_millis(2));
        let _ = self.keyboard.stream_matrix_buffer(&empty_apex);

        Ok(())
    }
}
