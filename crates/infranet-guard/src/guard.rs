use crate::policy::{PolicyEngine, PolicyDecision};
use infranet_core::packet::{SovereignPacket, InfranetRouteKind, TokenClass};

pub struct InfranetGuard {
    policy: PolicyEngine,
}

impl InfranetGuard {
    pub fn load_from_policies(dir: &std::path::Path) -> anyhow::Result<Self> {
        let policy = PolicyEngine::load_from_dir(dir)?;
        Ok(Self { policy })
    }

    pub fn evaluate_packet(&self, pkt: &SovereignPacket) -> PolicyDecision {
        // Example: mental privacy + dreamstate guards
        if pkt.neurorights.mental_privacy &&
           matches!(pkt.route, InfranetRouteKind::NeuroStreamIndex | InfranetRouteKind::BciControl)
        {
            return PolicyDecision::Deny {
                reason: "Neural stream metadata may not be routed for this subject".into(),
            };
        }

        if pkt.neurorights.dreamstate_sensitive &&
           pkt.token_class != TokenClass::None &&
           matches!(pkt.route, InfranetRouteKind::GovernanceChat | InfranetRouteKind::ModelUpdate)
        {
            return PolicyDecision::Deny {
                reason: "Dream-state-derived data cannot be used in decision-making routes".into(),
            };
        }

        // RoH ceiling and monotonicity checks if present
        if let Some(roh) = &pkt.roh {
            if roh.roh_after > roh.roh_ceiling {
                return PolicyDecision::Deny {
                    reason: format!("RoH {} exceeds ceiling {}", roh.roh_after, roh.roh_ceiling),
                };
            }
            if roh.roh_after > roh.roh_before {
                return PolicyDecision::Deny {
                    reason: "RoH monotone safety violated for this packet".into(),
                };
            }
        }

        // Token and capability gating per route
        self.policy.evaluate_packet(pkt)
    }
}
