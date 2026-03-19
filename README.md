# Lumina-Dist: Decentralized AI Inference Engine

[![Built for TokenTon26](https://img.shields.io/badge/Hackathon-TokenTon26-blue)](https://superteam.fun/)
[![Track](https://img.shields.io/badge/Track-AI-orange)]()
[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-black?logo=rust)]()
[![Powered by Solana](https://img.shields.io/badge/Network-Solana-green?logo=solana)]()

**Lumina-Dist** is a bare-metal, distributed compute infrastructure designed to solve the **AI Memory Wall** for large-scale Diffusion Transformer (DiT) models. By combining Rust's zero-cost abstractions with Solana's high-throughput settlement, Lumina-Dist enables trustless, decentralized AI inference at scale.

---

## 🎥 Live Demonstration

**Watch the Terminal & Architecture Demo:**
https://youtu.be/LX4d8_sCntY

---

## ⚠️ TokenTon26 Judges: Launch Status & Proof of Funds

**The AI agent and distributed compute nodes are fully functional and demonstrated in the video above.**

However, during the final submission window, the sponsor's DeAura Limitless Launch platform experienced continuous network simulation errors ("Token launch failed").

* **Wallet / Liquidity:** My wallet holds sufficient funds (**0.72 SOL** and **0.06 VNXAU**) to cover the market ID and liquidity pool creation.
* **Error Logs & Proof:** Please view the timestamped screenshots of the network errors, wallet balances, and payload sizes here:
  [Insert Your Google Drive Folder Link Here]

*Note: I am prepared to deploy the `$LUMINA` liquidity pool the exact moment the DeAura smart contracts stabilize.*

---

## 🧠 The Problem vs. The Solution

Running massive AI models requires centralized clusters with astronomical VRAM limits. Lumina-Dist flips this model by breaking heavy inference tasks into lightweight shards.

### 🚀 The Agentic Moat

At the core of Lumina-Dist is an autonomous Rust-native Agent. It acts as an intelligent gatekeeper:

1. **Scouts** the decentralized network for available compute nodes.
2. **Verifies** on-chain compute authorization via `$LUMINA` token payments on the Solana mainnet.
3. **Shards** massive AI weight matrices dynamically based on cluster VRAM.
4. **Reassembles** the computed tensor payloads via high-speed gRPC.

---

## 🛠 Technical Architecture

* **Core Engine:** Bare-metal `Rust` for memory safety and maximum hardware utilization.
* **Parallel Compute:** `Rayon` framework for multi-core matrix math distribution across worker nodes.
* **Networking:** High-performance `gRPC` over HTTP/2 for latency-free, node-to-node tensor transmission.
* **Consensus & Settlement:** `Solana` blockchain integration for permissionless compute allocation and micro-transactions.

---

# 🛡️ Safety & Guardrails

Lumina-Dist implements strict on-chain and off-chain safeguards:
* **Permissioned Execution:** The Master Node will not allocate compute or distribute shards without prior cryptographic verification of a `$LUMINA` transaction on Solana.
* **Bare-Metal Isolation:** By utilizing Rust, worker nodes execute matrix math in memory-safe, isolated environments to prevent buffer overflows or malicious payload injections.
* **Network Integrity:** gRPC transmissions are strictly typed and compressed, preventing malformed data from crashing the Reassembly Brain.

---

## 🚀 Launch Plan & Token Utility

* **Phase 1 (Hackathon MVP):** Deploy the MVP inference engine and establish the `$LUMINA` liquidity pool via DeAura (pending sponsor platform stabilization).
* **Phase 2 (Testnet Scaling):** Onboard independent GPU/CPU operators to run `worker-nodes`, using `$LUMINA` emissions to bootstrap the network's decentralized compute capacity.
* **Phase 3 (Mainnet):** Open the Master Node API to consumer AI applications, requiring them to burn/spend `$LUMINA` to access the decentralized inference power.

---

## ⚙️ Quick Start

### 1. Boot the Master Node (The Brain)

```bash
cargo run --bin core-engine
```

Monitors Solana for `$LUMINA` authorization, boots up the weight matrix, and prepares the DiT sharding strategy.

---

### 2. Launch a Worker Node (The Muscle)

```bash
cargo run --bin worker-node -- --port 50051
```

Connects to the Master, ingests the matrix shard, processes it using parallel CPU cores, and returns the compressed payload via gRPC.

---

> **⚖️ Disclosures (per hackathon rules):** 
> The core infrastructure and agentic logic of Lumina-Dist are custom-written in bare-metal Rust. To achieve high-performance distributed computing, this project utilizes the open-source `Rayon` crate for parallelization and the open-source `gRPC` framework for node-to-node network transmission.
