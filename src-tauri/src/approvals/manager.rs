use std::collections::BTreeMap;

use thiserror::Error;

use super::types::{
    ApprovalDecisionInput, ApprovalId, ApprovalRequest, ApprovalRequestInput, ApprovalState,
};

pub type ApprovalResult<T> = Result<T, ApprovalError>;

pub trait ApprovalManager {
    fn create_request(&mut self, input: ApprovalRequestInput) -> ApprovalResult<ApprovalRequest>;
    fn decide(
        &mut self,
        id: ApprovalId,
        decision: ApprovalDecisionInput,
    ) -> ApprovalResult<ApprovalRequest>;
    fn get(&self, id: ApprovalId) -> ApprovalResult<ApprovalRequest>;
    fn list_pending(&self) -> Vec<ApprovalRequest>;
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ApprovalError {
    #[error("approval tool name must not be empty")]
    EmptyToolName,
    #[error("approval action hash must not be empty")]
    EmptyActionHash,
    #[error("approval preview must not be empty")]
    EmptyPreview,
    #[error("approval not found: {0}")]
    NotFound(u64),
    #[error("approval has already been decided: {0}")]
    AlreadyDecided(u64),
    #[error("approval id space is exhausted")]
    IdSpaceExhausted,
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryApprovalManager {
    next_id: u64,
    requests: BTreeMap<ApprovalId, ApprovalRequest>,
}

impl InMemoryApprovalManager {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApprovalManager for InMemoryApprovalManager {
    fn create_request(&mut self, input: ApprovalRequestInput) -> ApprovalResult<ApprovalRequest> {
        if input.tool_name.trim().is_empty() {
            return Err(ApprovalError::EmptyToolName);
        }

        if input.action_hash.trim().is_empty() {
            return Err(ApprovalError::EmptyActionHash);
        }

        if input.preview.trim().is_empty() {
            return Err(ApprovalError::EmptyPreview);
        }

        let Some(next_id) = self.next_id.checked_add(1) else {
            return Err(ApprovalError::IdSpaceExhausted);
        };

        self.next_id = next_id;
        let request = ApprovalRequest::from_input(ApprovalId(self.next_id), input);
        self.requests.insert(request.id, request.clone());
        Ok(request)
    }

    fn decide(
        &mut self,
        id: ApprovalId,
        decision: ApprovalDecisionInput,
    ) -> ApprovalResult<ApprovalRequest> {
        let Some(request) = self.requests.get_mut(&id) else {
            return Err(ApprovalError::NotFound(id.0));
        };

        if request.state != ApprovalState::Pending {
            return Err(ApprovalError::AlreadyDecided(id.0));
        }

        request.state = match decision {
            ApprovalDecisionInput::Approve => ApprovalState::Approved,
            ApprovalDecisionInput::Reject => ApprovalState::Rejected,
        };

        Ok(request.clone())
    }

    fn get(&self, id: ApprovalId) -> ApprovalResult<ApprovalRequest> {
        self.requests
            .get(&id)
            .cloned()
            .ok_or(ApprovalError::NotFound(id.0))
    }

    fn list_pending(&self) -> Vec<ApprovalRequest> {
        self.requests
            .values()
            .filter(|request| request.state == ApprovalState::Pending)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{ApprovalError, ApprovalManager, InMemoryApprovalManager};
    use crate::approvals::types::{
        ApprovalDecisionInput, ApprovalId, ApprovalRequestInput, ApprovalState,
    };

    #[test]
    fn creates_and_lists_pending_approvals_in_order() {
        let mut manager = InMemoryApprovalManager::new();

        let first = manager.create_request(ApprovalRequestInput::new(
            "create_calendar_event",
            "hash-1",
            "Create calendar event",
        ));
        let second = manager.create_request(ApprovalRequestInput::new(
            "create_reminder",
            "hash-2",
            "Create reminder",
        ));

        assert!(matches!(first, Ok(request) if request.id == ApprovalId(1)));
        assert!(matches!(second, Ok(request) if request.id == ApprovalId(2)));

        let ids: Vec<ApprovalId> = manager
            .list_pending()
            .into_iter()
            .map(|request| request.id)
            .collect();

        assert_eq!(ids, vec![ApprovalId(1), ApprovalId(2)]);
    }

    #[test]
    fn records_approval_decisions() {
        let mut manager = InMemoryApprovalManager::new();
        let request = manager.create_request(ApprovalRequestInput::new(
            "create_calendar_event",
            "hash-1",
            "Create calendar event",
        ));

        assert!(matches!(request, Ok(request) if request.id == ApprovalId(1)));
        assert!(matches!(
            manager.decide(ApprovalId(1), ApprovalDecisionInput::Approve),
            Ok(request) if request.state == ApprovalState::Approved
        ));
        assert_eq!(manager.list_pending(), Vec::new());
    }

    #[test]
    fn rejects_unknown_or_already_decided_approvals() {
        let mut manager = InMemoryApprovalManager::new();
        assert_eq!(
            manager.decide(ApprovalId(404), ApprovalDecisionInput::Reject),
            Err(ApprovalError::NotFound(404))
        );

        let created = manager.create_request(ApprovalRequestInput::new(
            "create_reminder",
            "hash-2",
            "Create reminder",
        ));
        assert!(created.is_ok());
        assert!(manager
            .decide(ApprovalId(1), ApprovalDecisionInput::Reject)
            .is_ok());
        assert_eq!(
            manager.decide(ApprovalId(1), ApprovalDecisionInput::Approve),
            Err(ApprovalError::AlreadyDecided(1))
        );
    }
}
