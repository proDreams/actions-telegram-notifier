use crate::enums::workflow_enums::Status;

impl Status {
    pub fn text(&self) -> &str {
        match self {
            Status::Success => "success",
            Status::Failure => "failure",
            Status::Cancelled => "cancelled",
            Status::Info => "info",
            Status::Pending => "pending",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Status::Success => "✅",
            Status::Failure => "🔴",
            Status::Cancelled => "❌",
            Status::Info => "🔔",
            Status::Pending => "⌛",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "success" => Status::Success,
            "failure" => Status::Failure,
            "cancelled" => Status::Cancelled,
            "pending" => Status::Pending,
            _ => {
                eprintln!("Unknown status {}, set default status 'info'", s);
                Status::Info
            }
        }
    }
}
