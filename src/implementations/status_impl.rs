use crate::enums::workflow_enums::EventStatus;

impl EventStatus {
    pub fn text(&self) -> &str {
        match self {
            EventStatus::Success => "success",
            EventStatus::Failure => "failure",
            EventStatus::Cancelled => "cancelled",
            EventStatus::Info => "info",
            EventStatus::Pending => "pending",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            EventStatus::Success => "✅",
            EventStatus::Failure => "🔴",
            EventStatus::Cancelled => "❌",
            EventStatus::Info => "🔔",
            EventStatus::Pending => "⌛",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "success" => EventStatus::Success,
            "failure" => EventStatus::Failure,
            "cancelled" => EventStatus::Cancelled,
            "pending" => EventStatus::Pending,
            _ => {
                eprintln!("Unknown status {}, set default status 'info'", s);
                EventStatus::Info
            }
        }
    }
}
