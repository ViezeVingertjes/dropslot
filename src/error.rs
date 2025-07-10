/// Error type for subscriber operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusError {
    /// Try receive-related errors.
    TryRecv {
        /// No new message is available.
        empty: bool,
        /// The topic has been dropped and no more messages will be sent.
        disconnected: bool,
    },
}

impl BusError {
    /// Creates an error for when the message queue is empty.
    pub fn message_queue_empty() -> Self {
        BusError::TryRecv {
            empty: true,
            disconnected: false,
        }
    }

    /// Creates an error for when the topic has been disconnected.
    pub fn topic_disconnected() -> Self {
        BusError::TryRecv {
            empty: false,
            disconnected: true,
        }
    }

    /// Returns true if the error is due to disconnection.
    pub fn is_disconnected(&self) -> bool {
        matches!(
            self,
            BusError::TryRecv {
                disconnected: true,
                ..
            }
        )
    }

    /// Returns true if the error is due to empty state.
    pub fn is_empty(&self) -> bool {
        matches!(self, BusError::TryRecv { empty: true, .. })
    }
}

impl std::fmt::Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusError::TryRecv { empty: true, .. } => {
                write!(f, "TryRecv error: No message available")
            }
            BusError::TryRecv {
                disconnected: true, ..
            } => write!(f, "TryRecv error: Topic disconnected"),
            _ => write!(f, "Unknown bus error"),
        }
    }
}

impl std::error::Error for BusError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_unreachable_case() {
        // This should never happen in practice, but we need to test the _ arm
        // We'll use unsafe to create an invalid variant for testing
        let error = BusError::TryRecv {
            empty: false,
            disconnected: false,
        };

        let display_str = format!("{error}");
        assert_eq!(display_str, "Unknown bus error");
    }
}
