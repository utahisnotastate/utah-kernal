//! Chrono-Scheduler — lightweight predictive intent engine (Markov-style task forecasting).

extern crate alloc;

use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

/// Maximum recent actions kept for prediction.
const HISTORY_CAPACITY: usize = 32;

/// Tracks user history to predict the next intent.
pub struct ChronoScheduler {
    history: Vec<u32>,
    /// Next action the kernel has pre-staged in the background.
    staged_action: Option<u32>,
}

impl ChronoScheduler {
    pub fn new() -> Self {
        ChronoScheduler {
            history: Vec::new(),
            staged_action: None,
        }
    }

    /// Records an action and predicts what the user will do next.
    pub fn record_and_predict(&mut self, action_id: u32) -> Option<u32> {
        self.history.push(action_id);
        if self.history.len() > HISTORY_CAPACITY {
            let overflow = self.history.len() - HISTORY_CAPACITY;
            self.history.drain(0..overflow);
        }

        let predicted = predict_next_action(&self.history, action_id);
        if let Some(next_action) = predicted {
            self.pre_stage_intent(next_action);
        }
        predicted
    }

    /// Pre-allocates a small memory footprint for the predicted next task.
    fn pre_stage_intent(&mut self, action_id: u32) {
        self.staged_action = Some(action_id);
        // Warm the allocator with a tiny buffer representing future task state.
        let _preallocation_stub = alloc::vec![0u8; 64];
        let _ = _preallocation_stub;
        crate::display_text_on_screen(b"[CHRONO] Predictive intent pre-staged.");
    }

    /// Returns and clears a previously staged predictive intent, if any.
    pub fn take_staged_intent(&mut self) -> Option<u32> {
        self.staged_action.take()
    }
}

/// Simple first-order Markov-style transitions (expandable in production).
fn predict_next_action(history: &[u32], latest: u32) -> Option<u32> {
    // Master-copy rule: action 1 is usually followed by action 2.
    if latest == 1 {
        return Some(2);
    }
    if latest == 2 {
        return Some(3);
    }
    if latest == 3 {
        return Some(4);
    }

    // Second-order pattern: 10 -> 11 -> 12 workflow chain.
    if history.len() >= 2 {
        let previous = history[history.len() - 2];
        if previous == 10 && latest == 11 {
            return Some(12);
        }
    }

    None
}

lazy_static! {
    static ref GLOBAL_CHRONO_SCHEDULER: Mutex<ChronoScheduler> =
        Mutex::new(ChronoScheduler::new());
}

/// Records an action globally and returns the predicted next action ID, if any.
pub fn record_and_predict_global(action_id: u32) -> Option<u32> {
    if !crate::kernel_config::temporal_sequencing_enabled() {
        return None;
    }
    GLOBAL_CHRONO_SCHEDULER
        .lock()
        .record_and_predict(action_id)
}

/// Takes a pre-staged predictive intent from the global scheduler.
pub fn take_staged_intent_global() -> Option<u32> {
    GLOBAL_CHRONO_SCHEDULER.lock().take_staged_intent()
}
