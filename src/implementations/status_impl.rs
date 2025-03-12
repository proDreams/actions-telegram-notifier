use crate::enums::workflow_enums::PushStatus;

impl PushStatus {
    pub fn text(&self) -> &str {
        match self {
            PushStatus::Success => "success",
            PushStatus::Failure => "failure",
            PushStatus::Cancelled => "cancelled",
            PushStatus::Info => "info",
            PushStatus::Pending => "pending",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            PushStatus::Success => "✅",
            PushStatus::Failure => "🔴",
            PushStatus::Cancelled => "❌",
            PushStatus::Info => "🔔",
            PushStatus::Pending => "⌛",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "success" => PushStatus::Success,
            "failure" => PushStatus::Failure,
            "cancelled" => PushStatus::Cancelled,
            "pending" => PushStatus::Pending,
            _ => {
                eprintln!("Unknown status {}, set default status 'info'", s);
                PushStatus::Info
            }
        }
    }
}
