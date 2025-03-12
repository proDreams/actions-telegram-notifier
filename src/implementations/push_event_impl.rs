use crate::structures::event_structures::PushEvent;

impl PushEvent {
    pub fn get_compare(&self) -> Option<String> {
        self.compare.clone().or_else(|| self.compare_url.clone())
    }
}