# Lumina-Dist: Decentralized AI Inference Engine

[![Built for TokenTon26](https://img.shields.io/badge/Hackathon-TokenTon26-blue)](https://superteam.fun/)
[![Track](https://img.shields.io/badge/Track-AI-orange)]()
[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-black?logo=rust)]()
[![Powered by Solana](https://img.shields.io/badge/Network-Solana-green?logo=solana)]()

**Lumina-Dist** is a bare-metal, distributed compute infrastructure designed to solve the **AI Memory Wall** for large-scale Diffusion Transformer (DiT) models. By combining Rust's zero-cost abstractions with Solana's high-throughput settlement, Lumina-Dist enables trustless, decentralized AI inference at scale.

---

## 🎥 Live Demonstration
**Watch the Terminal & Architecture Demo:** [[https://youtu.be/LX4d8_sCntY]]

---

## ⚠️ TokenTon26 Judges: Launch Status & Proof of Funds
**The AI agent and distributed compute nodes are fully functional and demonstrated in the video above.** 

However, during the final submission window, the sponsor's DeAura Limitless Launch platform experienced continuous network simulation errors ("Token launch failed"). 
- **Wallet / Liquidity:** My wallet holds sufficient funds (**0.72 SOL** and **0.06 VNXAU**) to cover the market ID and liquidity pool creation. 
- **Error Logs & Proof:** Please view the timestamped screenshots of the network errors, wallet balances, and payload sizes here: [[Insert Your Google Drive Folder Link Here]]

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

## 🛠 Technical Architecture
- **Core Engine:** Bare-metal `Rust` for memory safety and maximum hardware utilization.
- **Parallel Compute:** `Rayon` framework for multi-core matrix math distribution across worker nodes.
- **Networking:** High-performance `gRPC` over HTTP/2 for latency-free, node-to-node tensor transmission.
- **Consensus & Settlement:** `Solana` blockchain integration for permissionless compute allocation and micro-transactions.

## ⚙️ Quick Start

**1. Boot the Master Node (The Brain)**
```bash
cargo run --bin core-engine
