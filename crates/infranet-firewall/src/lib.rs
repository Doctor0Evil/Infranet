
use infranet_core::packet::{InfranetRouteKind, SovereignPacket, TokenClass};

/// A simple enum summarizing firewall verdicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallVerdict {
    Allow,
    Block,
    Quarantine,
}

/// Trait to abstract an LLM-firewall engine (NeuralTrust-like).
pub trait FirewallEngine: Send + Sync {
    fn classify(&self, text: &str) -> FirewallVerdict;
}

/// A basic, pluggable firewall wrapper for Infranet chat/OTA routes.
pub struct InfranetFirewall<E: FirewallEngine> {
    engine: E,
}

impl<E: FirewallEngine> InfranetFirewall<E> {
    pub fn new(engine: E) -> Self {
        Self { engine }
    }

    /// Evaluate a packet prior to passing into LLM / OTA pipelines.
    pub fn evaluate_packet(&self, pkt: &SovereignPacket) -> FirewallVerdict {
        // Only apply firewall to chat + OTA + model-related routes.
        match pkt.route {
            InfranetRouteKind::GovernanceChat
            | InfranetRouteKind::OTAProposal
            | InfranetRouteKind::ModelUpdate => {
                // For CHAT routes, treat payload_ref as prompt key / content label.
                let key = format!(
                    "{}:{}:{}",
                    pkt.src.subject_id, pkt.payload_type, pkt.payload_ref
                );
                let verdict = self.engine.classify(&key);

                // Optionally tighten policy for EVOLVE-related OTA proposals.
                if pkt.route == InfranetRouteKind::OTAProposal
                    && pkt.token_class == TokenClass::Evolve
                    && matches!(verdict, FirewallVerdict::Allow)
                {
                    // For high-risk EVOLVE routes, upgrade borderline cases to Quarantine
                    // in a real impl using scores; here we pass through.
                    FirewallVerdict::Allow
                } else {
                    verdict
                }
            }
            _ => FirewallVerdict::Allow,
        }
    }
}

/// Example dummy engine – replace with real NeuralTrust integration.
pub struct DummyEngine;

impl FirewallEngine for DummyEngine {
    fn classify(&self, _text: &str) -> FirewallVerdict {
        FirewallVerdict::Allow
    }
}
