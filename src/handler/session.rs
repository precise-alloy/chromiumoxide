use chromiumoxide_cdp::cdp::browser_protocol::target::{SessionId, TargetId};

/// Represents a Session within the cpd.
#[derive(Debug, Clone)]
pub struct Session {
    /// Identifier for this session.
    id: SessionId,
    /// The identifier of the target this session is attached to.
    target_id: TargetId,
    /// The parent session if this session was auto-attached under another target's session.
    parent_session: Option<SessionId>,
}
impl Session {
    pub fn new(id: SessionId, target_id: TargetId) -> Self {
        Self {
            id,
            target_id,
            parent_session: None,
        }
    }

    pub fn new_child(id: SessionId, target_id: TargetId, parent_session: SessionId) -> Self {
        Self {
            id,
            target_id,
            parent_session: Some(parent_session),
        }
    }

    pub fn session_id(&self) -> &SessionId {
        &self.id
    }

    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    pub fn parent_session(&self) -> Option<&SessionId> {
        self.parent_session.as_ref()
    }
}
