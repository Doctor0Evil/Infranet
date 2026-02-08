
use async_trait::async_trait;
use infranet_core::packet::{RoHSlice, SovereignPacket};
use std::fmt::Debug;

/// A concrete link driver (e.g., QUIC/TCP/serial) implements this.
#[async_trait]
pub trait LinkDriver: Send + Sync + Debug {
    async fn send(&self, pkt: SovereignPacket) -> anyhow::Result<()>;
    async fn recv(&self) -> anyhow::Result<SovereignPacket>;
}

/// Abstract view of RoH-aware routes in the mesh.
#[derive(Debug, Clone)]
pub struct MeshRoute {
    pub path_id: String,
    pub hops: Vec<String>, // logical node IDs
    pub roh_path_slice: Option<RoHSlice>,
}

/// Route selection based on RoHMeshCeiling and other metrics.
#[async_trait]
pub trait RouteSelector: Send + Sync + Debug {
    async fn select_route(
        &self,
        candidates: Vec<MeshRoute>,
        pkt: &SovereignPacket,
    ) -> Option<MeshRoute>;
}

/// A minimal mesh node which ties a LinkDriver and RouteSelector together.
pub struct MeshNode<D: LinkDriver, R: RouteSelector> {
    pub id: String,
    pub driver: D,
    pub selector: R,
}

impl<D: LinkDriver, R: RouteSelector> MeshNode<D, R> {
    pub fn new(id: String, driver: D, selector: R) -> Self {
        Self { id, driver, selector }
    }

    /// Send a packet using RoHMeshCeiling-aware routing.
    pub async fn send_with_routes(
        &self,
        pkt: SovereignPacket,
        routes: Vec<MeshRoute>,
    ) -> anyhow::Result<()> {
        if let Some(route) = self.selector.select_route(routes, &pkt).await {
            if let Some(roh) = &route.roh_path_slice {
                if let Some(pkt_roh) = &pkt.roh {
                    if roh.roh_after > roh.roh_ceiling
                        || roh.roh_after > pkt_roh.roh_ceiling
                    {
                        anyhow::bail!("RoHMeshCeiling violated on selected route");
                    }
                }
            }
            self.driver.send(pkt).await
        } else {
            anyhow::bail!("No admissible route for packet");
        }
    }
}
