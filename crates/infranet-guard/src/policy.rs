use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use infranet_core::packet::{InfranetRouteKind, SovereignPacket, TokenClass};

/// Minimal neurorights policy as loaded from `.neurorights.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsPolicy {
    pub mental_privacy: bool,
    pub cognitive_liberty: bool,
    pub forbid_decision_use: bool,
    pub dreamstate_sensitive: bool,
    pub soulnontradeable: bool,
    pub storagescope: String,
}

/// Minimal Tsafe kernel (RoH envelope) as loaded from `.tsafe.aln` / `.rohmodel.aln` after parsing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsafeKernel {
    pub roh_ceiling: f32,
}

/// High-level decision result.
#[derive(Debug, Clone)]
pub enum PolicyDecision {
    Allow,
    Deny { reason: String },
    AllowWithConstraints { reason: String, redactions: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct PolicyEngine {
    neurorights: NeurorightsPolicy,
    tsafe: TsafeKernel,
}

impl PolicyEngine {
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> anyhow::Result<Self> {
        let dir = dir.as_ref();

        // `.neurorights.json`
        let nr_raw = fs::read_to_string(dir.join("neurorights.json"))?;
        let neurorights: NeurorightsPolicy = serde_json::from_str(&nr_raw)?;

        // For now, assume `.tsafe.aln` is JSON-compatible; a real impl would parse ALN.
        let tsafe_raw = fs::read_to_string(dir.join("tsafe.aln"))?;
        let tsafe: TsafeKernel = serde_json::from_str(&tsafe_raw)?;

        Ok(Self { neurorights, tsafe })
    }

    /// Route/role/token-aware evaluation hook.
    pub fn evaluate_packet(&self, pkt: &SovereignPacket) -> PolicyDecision {
        // 1. Mental privacy & neurostream exports.
        if self.neurorights.mental_privacy {
            if matches!(
                pkt.route,
                InfranetRouteKind::NeuroStreamIndex | InfranetRouteKind::BciControl
            ) && pkt.capability.biophysical_scope != "DerivedOnly"
            {
                return PolicyDecision::Deny {
                    reason: "Mental privacy: raw or index-level neurostream routing is forbidden"
                        .into(),
                };
            }
        }

        // 2. Dream state + decision use.
        if self.neurorights.dreamstate_sensitive && self.neurorights.forbid_decision_use {
            if matches!(
                pkt.route,
                InfranetRouteKind::GovernanceChat
                    | InfranetRouteKind::ModelUpdate
                    | InfranetRouteKind::OTAProposal
            ) {
                return PolicyDecision::Deny {
                    reason:
                        "Dream-state-sensitive data cannot be used in governance/model/OTA routes"
                            .into(),
                };
            }
        }

        // 3. Token class constraints: CHAT is non-actuating.
        if pkt.token_class == TokenClass::Chat {
            if pkt.capability.actuation_rights != "SuggestOnly" {
                return PolicyDecision::Deny {
                    reason: "CHAT tokened routes must be SuggestOnly / non-actuating".into(),
                };
            }
        }

        // 4. SMART cannot be used for deep evolution routes.
        if pkt.token_class == TokenClass::Smart {
            if matches!(
                pkt.route,
                InfranetRouteKind::OTAProposal | InfranetRouteKind::NanoswarmControl
            ) && pkt.capability.safety_profile == "DeepEvolution"
            {
                return PolicyDecision::Deny {
                    reason: "SMART cannot authorize deep evolution or nanoswarm structural changes"
                        .into(),
                };
            }
        }

        // 5. EVOLVE-only scopes for OTA / structural changes.
        if matches!(
            pkt.route,
            InfranetRouteKind::OTAProposal | InfranetRouteKind::NanoswarmControl
        ) && pkt.capability.safety_profile == "DeepEvolution"
            && pkt.token_class != TokenClass::Evolve
        {
            return PolicyDecision::Deny {
                reason: "DeepEvolution routes require EVOLVE token class".into(),
            };
        }

        // 6. RoH ceiling + monotonicity (if a slice is present).
        if let Some(roh) = &pkt.roh {
            if roh.roh_after > self.tsafe.roh_ceiling {
                return PolicyDecision::Deny {
                    reason: format!(
                        "RoH {} exceeds Tsafe ceiling {}",
                        roh.roh_after, self.tsafe.roh_ceiling
                    ),
                };
            }
            if roh.roh_after > roh.roh_before {
                return PolicyDecision::Deny {
                    reason: "RoH monotone safety violated for this packet".into(),
                };
            }
        }

        PolicyDecision::Allow
    }
}
