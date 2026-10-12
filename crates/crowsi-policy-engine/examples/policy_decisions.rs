//! Pure policy demonstration with synthetic contract inputs; not a real authority.
#[path = "../tests/support/mod.rs"]
mod synthetic;
use crowsi_control_contracts::{ControlAction, DecisionEffect};
use crowsi_policy_engine::{PostureState, ReasonCode};

fn main() {
    let mut fixture = synthetic::Fixture::new(ControlAction::Quarantine);
    let permitted = fixture.evaluate();
    assert_eq!(permitted.effect, DecisionEffect::Permit);
    fixture.current_epoch += 1;
    let revoked = fixture.evaluate();
    assert_eq!(revoked.effect, DecisionEffect::Deny);
    assert_eq!(revoked.reason, ReasonCode::Revoked);
    fixture.current_epoch -= 1;
    fixture.workload_state = PostureState::Unknown;
    let unknown = fixture.evaluate();
    assert_eq!(unknown.effect, DecisionEffect::Deny);
    assert_eq!(unknown.reason, ReasonCode::UnknownWorkloadPosture);
    println!(
        "Synthetic policy: {:?}; revocation: {:?}; missing evidence: {:?}",
        permitted.effect, revoked.reason, unknown.reason
    );
    println!(
        "No signature authentication, effect dispatch, native custody or account operation performed"
    );
}
