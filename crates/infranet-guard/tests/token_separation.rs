use infranet_core::packet::{
    CapabilityScope, InfranetRouteKind, NeurorightsEnvelope, RoHSlice, SovereignAddress,
    SovereignPacket, TokenClass,
};
use infranet_guard::{InfranetGuard, PolicyDecision};
use std::{path::Path, time::SystemTime};

fn mk_base_packet() -> SovereignPacket {
    SovereignPacket {
        src: SovereignAddress {
            subject_id: "bostrom18sd2u...".into(),
            ocpu_id: None,
        },
        dst: SovereignAddress {
            subject_id: "bostrom18sd2u...".into(),
            ocpu_id: None,
        },
        route: InfranetRouteKind::GovernanceChat,
        timestamp: SystemTime::now(),
        roh: Some(RoHSlice {
            roh_before: 0.1,
            roh_after: 0.1,
            roh_ceiling: 0.3,
        }),
        neurorights: NeurorightsEnvelope {
            mental_privacy: false,
            mental_integrity: true,
            cognitive_liberty: true,
            noncommercial_neural_data: true,
            dreamstate_sensitive: false,
            forbid_decision_use: false,
        },
        token_class: TokenClass::Chat,
        capability: CapabilityScope {
            biophysical_scope: "ReadOnly".into(),
            actuation_rights: "SuggestOnly".into(),
            safety_profile: "MonotoneSafetyUpdate".into(),
            rights_profile: "NeurorightsBound".into(),
        },
        payload_type: "ChatFragment".into(),
        payload_ref: "answer:1234".into(),
        hexstamp: None,
    }
}

#[test]
fn chat_routes_are_non_actuating() {
    let guard = InfranetGuard::load_from_policies(Path::new("policies")).unwrap();
    let mut pkt = mk_base_packet();

    // CHAT + SuggestOnly should pass.
    let decision = guard.evaluate(&pkt);
    matches!(decision, PolicyDecision::Allow);

    // CHAT + ConfigOnly must be denied.
    pkt.capability.actuation_rights = "ConfigOnly".into();
    let decision = guard.evaluate(&pkt);
    match decision {
        PolicyDecision::Deny { .. } => {}
        _ => panic!("CHAT + ConfigOnly must be denied"),
    }
}

#[test]
fn smart_cannot_do_deep_evolution() {
    let guard = InfranetGuard::load_from_policies(Path::new("policies")).unwrap();
    let mut pkt = mk_base_packet();

    pkt.route = InfranetRouteKind::OTAProposal;
    pkt.token_class = TokenClass::Smart;
    pkt.capability.safety_profile = "DeepEvolution".into();

    let decision = guard.evaluate(&pkt);
    match decision {
        PolicyDecision::Deny { .. } => {}
        _ => panic!("SMART must not authorize DeepEvolution OTAProposal routes"),
    }
}

#[test]
fn evolve_required_for_deep_evolution_routes() {
    let guard = InfranetGuard::load_from_policies(Path::new("policies")).unwrap();
    let mut pkt = mk_base_packet();

    pkt.route = InfranetRouteKind::NanoswarmControl;
    pkt.token_class = TokenClass::Smart;
    pkt.capability.safety_profile = "DeepEvolution".into();

    let decision = guard.evaluate(&pkt);
    match decision {
        PolicyDecision::Deny { .. } => {}
        _ => panic!("DeepEvolution NanoswarmControl must require EVOLVE"),
    }

    pkt.token_class = TokenClass::Evolve;
    let decision = guard.evaluate(&pkt);
    match decision {
        PolicyDecision::Allow | PolicyDecision::AllowWithConstraints { .. } => {}
        _ => panic!("EVOLVE should be admissible for DeepEvolution, subject to other guards"),
    }
}
