# Lumina-Dist 🌌

[![Rust-CI](https://github.com/stnyein/Lumina-Dist/actions/workflows/rust.yml/badge.svg)](https://github.com/stnyein/Lumina-Dist/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Lumina-Dist** is a high-performance, distributed inference engine for Large Language Models (LLMs) built entirely in Rust. Designed for academic research in distributed systems, it partitions model weights across multiple worker nodes to minimize latency and maximize throughput over gRPC.

## 📑 Abstract
As LLM parameters scale, single-node inference becomes a severe memory and compute bottleneck. Lumina-Dist implements a decentralized worker architecture to perform distributed matrix multiplications and attention mechanisms across a cluster of low-cost nodes, bypassing the need for massive, monolithic GPUs.

## 🏗️ Architecture Design

` ` `mermaid
graph TD;
    Client-->|gRPC Request| MasterNode[Core Engine / Master Node];
    MasterNode-->|Partition 1| Worker1[Worker Node 1];
    MasterNode-->|Partition 2| Worker2[Worker Node 2];
    Worker1-->|Aggregated Tensor| MasterNode;
    Worker2-->|Aggregated Tensor| MasterNode;
` ` `

## 🔬 Theoretical Foundation
Lumina-Dist shards the standard Scaled Dot-Product Attention across distributed nodes:

$$\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}}\right)V$$

By chunking the Key ($K$) and Value ($V$) matrices, compute overhead is distributed across the network, allowing for larger batch processing without hitting hardware limits on a single machine.

## 🛠️ Tech Stack
* **Core Language:** Rust (2021 Edition)
* **RPC Framework:** Tonic (gRPC)
* **Serialization:** Prost (Protocol Buffers)
* **Concurrency:** Tokio & Rayon
