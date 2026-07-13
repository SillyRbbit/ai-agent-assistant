#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApprovalId(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApprovalState {
    Pending,
    Approved,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalDecisionInput {
    Approve,
    Reject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovalRequestInput {
    pub tool_name: String,
    pub action_hash: String,
    pub preview: String,
}

impl ApprovalRequestInput {
    #[must_use]
    pub fn new(
        tool_name: impl Into<String>,
        action_hash: impl Into<String>,
        preview: impl Into<String>,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            action_hash: action_hash.into(),
            preview: preview.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovalRequest {
    pub id: ApprovalId,
    pub tool_name: String,
    pub action_hash: String,
    pub preview: String,
    pub state: ApprovalState,
}

impl ApprovalRequest {
    #[must_use]
    pub fn from_input(id: ApprovalId, input: ApprovalRequestInput) -> Self {
        Self {
            id,
            tool_name: input.tool_name,
            action_hash: input.action_hash,
            preview: input.preview,
            state: ApprovalState::Pending,
        }
    }
}
