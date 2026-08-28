use std::error::Error;

use ai_agent_assistant_lib::{
    ResearchKnowledgeDemoHost, ResearchKnowledgeDemoLifecycleError,
    ResearchKnowledgeDemoLifecycleEventKind, ResearchKnowledgeDemoLifecycleState,
};

#[test]
fn public_rust_contract_is_no_input_bounded_and_manually_stepped() -> Result<(), Box<dyn Error>> {
    let mut host = ResearchKnowledgeDemoHost::new();
    let idle = host.snapshot();
    assert_eq!(idle.state(), ResearchKnowledgeDemoLifecycleState::Idle);
    assert_eq!(idle.presentation_epoch(), 0);
    assert_eq!(idle.revision(), 0);
    assert!(idle.journal().is_empty());

    let started = host.start()?;
    assert_eq!(
        started.state(),
        ResearchKnowledgeDemoLifecycleState::Research
    );
    assert_eq!(started.presentation_epoch(), 1);
    assert_eq!(started.revision(), 1);

    let knowledge = host.advance()?;
    assert_eq!(
        knowledge.snapshot().state(),
        ResearchKnowledgeDemoLifecycleState::Knowledge
    );
    assert_eq!(
        knowledge
            .entries()
            .iter()
            .map(|entry| entry.kind())
            .collect::<Vec<_>>(),
        [
            ResearchKnowledgeDemoLifecycleEventKind::ResearchCompleted,
            ResearchKnowledgeDemoLifecycleEventKind::KnowledgeStarted,
        ]
    );

    let synthesis = host.advance()?;
    assert_eq!(
        synthesis.snapshot().state(),
        ResearchKnowledgeDemoLifecycleState::Synthesis
    );
    let succeeded = host.advance()?;
    assert_eq!(
        succeeded.snapshot().state(),
        ResearchKnowledgeDemoLifecycleState::Succeeded
    );
    assert_eq!(succeeded.snapshot().journal().len(), 6);

    let terminal = host.snapshot();
    assert_eq!(
        host.advance(),
        Err(ResearchKnowledgeDemoLifecycleError::InvalidTransition)
    );
    assert_eq!(host.snapshot(), terminal);
    Ok(())
}
