//! Vim-style key sequence handling.

use egui::{Context, Event, KeyboardShortcut};
use std::collections::VecDeque;

/// Handles Vim-style key sequences with timeout
pub struct KeySequenceHandler {
    /// Buffer of recent key presses
    buffer: VecDeque<KeyboardShortcut>,
    /// Time when last key was pressed (seconds since start, from egui context)
    last_key_time: Option<f64>,
    /// Timeout duration in seconds
    timeout_seconds: f64,
    /// Maximum sequence length to keep
    max_buffer_size: usize,
}

impl KeySequenceHandler {
    /// Create a new sequence handler with given timeout
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            buffer: VecDeque::new(),
            last_key_time: None,
            timeout_seconds: timeout_ms as f64 / 1000.0,
            max_buffer_size: 10, // More than enough for Vim sequences
        }
    }

    /// Update the handler with new input events
    pub fn update(&mut self, ctx: &Context) {
        let current_time = ctx.input(|i| i.time);

        // Clear buffer if timeout expired
        if let Some(last_time) = self.last_key_time
            && current_time - last_time > self.timeout_seconds {
                self.buffer.clear();
                self.last_key_time = None;
            }

        // Process key press events
        for event in ctx.input(|i| i.raw.events.clone()) {
            if let Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } = event
            {
                let shortcut = KeyboardShortcut::new(modifiers, key);
                self.buffer.push_back(shortcut);
                self.last_key_time = Some(current_time);

                // Keep buffer size limited
                if self.buffer.len() > self.max_buffer_size {
                    self.buffer.pop_front();
                }
            }
        }
    }

    /// Check if a sequence matches the recent key presses
    pub fn check_sequence(&mut self, ctx: &Context, sequence: &[KeyboardShortcut]) -> bool {
        // Update first to process any new events
        self.update(ctx);

        if sequence.is_empty() {
            return false;
        }

        // Check if buffer ends with the sequence
        if self.buffer.len() < sequence.len() {
            return false;
        }

        let recent: Vec<_> = self
            .buffer
            .iter()
            .rev()
            .take(sequence.len())
            .rev()
            .copied()
            .collect();

        if recent == sequence {
            // Clear buffer on successful match
            self.buffer.clear();
            self.last_key_time = None;
            return true;
        }

        false
    }

    /// Clear the sequence buffer
    #[expect(dead_code)]
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.last_key_time = None;
    }

    /// Get the current sequence buffer (for debugging)
    #[expect(dead_code)]
    pub fn buffer(&self) -> &VecDeque<KeyboardShortcut> {
        &self.buffer
    }

    /// Set the timeout duration
    #[expect(dead_code)]
    pub fn set_timeout_ms(&mut self, timeout_ms: u64) {
        self.timeout_seconds = timeout_ms as f64 / 1000.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Key, Modifiers};

    fn create_test_context() -> Context {
        Context::default()
    }

    #[test]
    fn test_sequence_matching() {
        let mut handler = KeySequenceHandler::new(1000);
        let ctx = create_test_context();

        // Simulate "g" then "g" presses
        let seq = vec![
            KeyboardShortcut::new(Modifiers::NONE, Key::G),
            KeyboardShortcut::new(Modifiers::NONE, Key::G),
        ];

        // Should not match empty buffer
        assert!(!handler.check_sequence(&ctx, &seq));

        // TODO: Need to simulate key events for proper testing
        // This would require mocking egui's input system
    }

    #[test]
    fn test_timeout_clears_buffer() {
        let mut handler = KeySequenceHandler::new(10); // Very short timeout (10ms)
        let ctx = create_test_context();

        // Simulate a key press
        // Note: We can't easily simulate time passing in tests without mocking
        // This test is limited but at least verifies the structure works
        handler.update(&ctx);

        // The buffer should be empty since no keys were actually pressed
        assert!(handler.buffer.is_empty());
        assert!(handler.last_key_time.is_none());
    }
}
