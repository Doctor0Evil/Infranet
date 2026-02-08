use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// High-level route classification for Infranet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfranetRouteKind {
    BciControl,
    BioTelemetry,
    NeuroStreamIndex,
    OTAProposal,
    OTAArtifactProof,
    GovernanceChat,
    ModelUpdate,
    CivicXRGrid,
    NanoswarmControl,
    NanoswarmTelemetry,
}

/// Bostrom / OrganicCPU addressing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignAddress {
    pub subject_id: String,      // Bostrom address or zeta / 0x...
    pub ocpu_id: Option<String>, // Optional OrganicCPU DID
}

/// RoH slice for a packet or flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoHSlice {
    pub roh_before: f32,
    pub roh_after: f32,
    pub roh_ceiling: f32, // typically 0.3
}

/// Neurorights posture for this packet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsEnvelope {
    pub mental_privacy: bool,
    pub mental_integrity: bool,
    pub cognitive_liberty: bool,
    pub noncommercial_neural_data: bool,
    pub dreamstate_sensitive: bool,
    pub forbid_decision_use: bool,
}

/// Token class associated to the originating action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenClass {
    None,
    Smart,
    Evolve,
    Chat,
}

/// Capability descriptor for how this packet may be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityScope {
    pub biophysical_scope: String,   // e.g. "ReadOnly", "EnvelopeOnly"
    pub actuation_rights: String,    // e.g. "SuggestOnly", "ConfigOnly"
    pub safety_profile: String,      // e.g. "MonotoneSafetyUpdate"
    pub rights_profile: String,      // e.g. "NeurorightsBound"
}

/// Sovereign, governed packet model for Infranet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignPacket {
    pub src: SovereignAddress,
    pub dst: SovereignAddress,
    pub route: InfranetRouteKind,
    pub timestamp: SystemTime,
    pub roh: Option<RoHSlice>,
    pub neurorights: NeurorightsEnvelope,
    pub token_class: TokenClass,
    pub capability: CapabilityScope,
    /// Logical type of payload.
    pub payload_type: String,     // e.g. "ProposalRef", "DerivedMetric", "ChatFragment"
    /// Reference into local shards / objects (never raw .neuroaln).
    pub payload_ref: String,
    /// Optional Googolswarm / Organicchain hexstamp.
    pub hexstamp: Option<String>,
}
