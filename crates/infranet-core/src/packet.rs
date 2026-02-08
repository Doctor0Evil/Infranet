use serde::{Serialize, Deserialize};
use std::time::SystemTime;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignAddress {
    pub subject_id: String,      // Bostrom address or zeta / 0x...
    pub ocpu_id: Option<String>, // Optional OrganicCPU DID
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoHSlice {
    pub roh_before: f32,
    pub roh_after: f32,
    pub roh_ceiling: f32,        // Typically 0.3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsEnvelope {
    pub mental_privacy: bool,
    pub mental_integrity: bool,
    pub cognitive_liberty: bool,
    pub noncommercial_neural_data: bool,
    pub dreamstate_sensitive: bool,
    pub forbid_decision_use: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenClass {
    None,
    Smart,
    Evolve,
    Chat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityScope {
    pub biophysical_scope: String,   // e.g. "ReadOnly", "EnvelopeOnly"
    pub actuation_rights: String,    // e.g. "SuggestOnly", "ConfigOnly"
    pub safety_profile: String,      // e.g. "MonotoneSafetyUpdate"
    pub rights_profile: String,      // e.g. "NeurorightsBound"
}

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
    pub payload_type: String,     // e.g. "ProposalRef", "DerivedMetric", "ChatFragment"
    pub payload_ref: String,      // Path or ID to local shard (never raw .neuroaln on wire)
    pub hexstamp: Option<String>, // Hash / Googolswarm anchor
}
