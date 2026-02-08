 # Infranet: Neuro-Sovereign Mesh for Augmented Citizens

> Rust-only, neurorights-bound networking layer for NeuroPC, OrganicCPU, and sovereign kernels under the Bostrom stack.[file:2][file:1]

---

## 1. Vision and Scope

Infranet is the **sovereign** networking and coordination plane that links NeuroPC nodes, OrganicCPU hosts, nanoswarms, and civic XR grids without ever breaking neurorights, Risk-of-Harm (RoH) ceilings, or Bostrom identity boundaries.[file:1][file:2] It treats every packet, stream, and upgrade as a governed SovereignArtifact rather than a raw message, anchored to Bostrom addresses and biophysical envelopes.[file:1][file:2]

Core goals:

- Make all AI/BCI networking *text-and-policy first, actuation second*, enforced by Rust guard kernels.[file:3][file:1]
- Keep raw neural, lifeforce, and sovereign-kernel shards off the wire; only derived, typed statements cross Infranet.[file:2][file:3]
- Bind every route and channel to neurorights, RoH ≤ 0.3, and stake/evolution rules under SMART/EVOLVE/CHAT governance.[file:1][file:3]

Infranet lives as a Rust workspace wired into your sovereign filesystem (neuro-eXpFS / NeuroXFS) and sovereigntycore guard stack.[file:2][file:1]

---

## 2. Anchor Identities and Sovereign Roots

Infranet is keyed by Bostrom addresses and OrganicCPU subjects.[file:2][file:1]

Canonical subjects:

- Primary: `bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7` (host sovereign root).[file:2][file:1]
- Alternate secure: `bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc` (Google-linked, RT-monitored).[file:2]
- Safe alternates:
  - `zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8` (lab-safe).[file:2]
  - `0x519fC0eB4111323Cac44b70e1aE31c30e405802D` (ERC-20 compatible, on-chain anchors).[file:2][file:1]

Each subject gets a sovereign workspace layout via `neuro-workspace.manifest.aln`, pointing at:[file:2][file:1]

- `.rohmodel.aln` – RoH model, ceiling 0.3, monotone safety.[file:1]
- `.neurorights.json` – neurorights policy.[file:1]
- `.stake.aln` – Host / OrganicCPU / ResearchAgent roles and multisig scopes.[file:1]
- `.smart.json`, `.evolve-token.json` – SMART/EVOLVE token policies.[file:1]
- `.evolve.jsonl`, `.donutloop.aln` – evolution stream and hash-linked ledger.[file:1]
- `.ocpu`, `.ocpuenv`, `.lifeforce.aln`, `.biosession.aln`, `.vkernel.aln`, `.tsafe.aln` – bioscale envelopes and Tsafe kernels.[file:1][file:2]

Infranet never routes around these; it reads them as the *constitution* for every connection and route.[file:1][file:3]

---

## 3. High-Level Architecture

### 3.1 Rings and Guards

Infranet adopts a three-ring model aligned with Tsafe Cortex Gate and OWASP LLM01 style boundaries:[file:3]

- Outer ring – Untrusted AI / network zone: LLM APIs, RAG sources, OTA manifests, remote XR nodes, civic controllers.[file:3]
- Middle ring – Rust **Sovereign Boundary**:
  - Infranet daemons, guard crates, policy engines, firewalls (NeuralTrust-class), and filesystem drivers.[file:3][file:2]
- Inner ring – Sovereign kernel:
  - `.rohmodel.aln`, `.stake.aln`, `.neurorights.json`, `.evolve.jsonl`, `.donutloop.aln`, `.bchainproof.json`, organic logs.[file:1]

All Infranet flows terminate in the Rust boundary; no external node, LLM, or plugin can directly touch the inner ring.[file:3][file:1]

### 3.2 Core Components

Infranet is designed as a Rust workspace like:

```text
infranet/
  Cargo.toml

  crates/
    infranet-core/              # Core types, routes, capabilities
    infranet-guard/             # Neurorights + RoH + Tsafe evaluation
    infranet-mesh/              # Mesh / overlay routing
    infranet-firewall/          # NeuralTrust-style AI firewall bindings
    infranet-bchain/            # Googolswarm / Organicchain proofs
    infranet-neurofs-bridge/    # Bridge to neuro-eXpFS / NeuroXFS
    infranet-cli/               # Operator tools

  policies/
    neuro-workspace.manifest.aln
    rohmodel.aln
    neurorights.json
    stake.aln
    smart.json
    evolve-token.json
    tsafe.aln
    vkernel.aln

  ledger/
    evolve.jsonl
    donutloop.aln
    bchainproof.json

  docs/
    Infranet.md                 # This file
