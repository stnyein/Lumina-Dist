# 🧠 Lumina-Dist: Distributed AI Inference Engine
A bare-metal distributed AI inference engine built in Rust, designed to conquer the "Memory Wall" by distributing massive Tensor computations across a local network of machines.

## ⚠️ The Problem: The AI Memory Wall
Modern Large Language Models (LLMs) are massive. A 70-billion parameter model requires over 140GB of memory just to load. 
1. **The Memory Crash:** Trying to load a massive AI brain into a single standard computer causes an immediate out-of-memory crash. 
2. **The Compute Bottleneck:** Single-threaded processors take agonizingly long to calculate billions of matrix operations.

## 🚀 The Solution: Distributed Tensor Parallelism
**Lumina-Dist** acts as a cluster manager to solve this problem from scratch. 
Instead of forcing one machine to hold an impossibly heavy AI model, Lumina-Dist slices the neural network weights into smaller shards, distributes them across multiple worker nodes over a custom gRPC network, forces all machines to compute the math simultaneously across all CPU cores, and stitches the answers back together.

It is the difference between asking one person to translate a 1,000-page dictionary, and ripping the dictionary into chapters to give to a team of 10 people to translate simultaneously.

## ⚙️ Architecture & Tech Stack
Lumina-Dist is built using a custom microservices architecture:
* **Network Layer (`tonic` / gRPC & Protocol Buffers):** Lightning-fast, typed communication between the Master Node and Worker Nodes.
* **Serialization (`bincode`):** Highly optimized, memory-safe compression of complex multi-dimensional mathematical objects (`ndarray`) into raw byte streams for network transit.
* **Tensor Sharding (`ndarray`):** The Master Node surgically generates and slices massive mathematical matrices to prevent worker memory overload.
* **Parallel Compute (`rayon`):** Worker nodes dynamically ignite every available local CPU core to execute simultaneous multi-threaded matrix multiplication.

## 🛠️ Quickstart

**1. Boot the Master Node (The Manager)**
The Master will generate a massive matrix, shard it perfectly in half, and wait for workers to connect.
```bash
cd core-engine
cargo run --bin core-engine
