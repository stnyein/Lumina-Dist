use tonic::{transport::Server, Request, Response, Status};
use inference::inference_provider_server::{InferenceProvider, InferenceProviderServer};
use inference::{WeightPackage, TransferStatus};
use ndarray::{Array2, Axis};

pub mod inference {
    tonic::include_proto!("inference"); 
}

#[derive(Debug, Default)]
pub struct MyInferenceProvider {}

#[tonic::async_trait]
impl InferenceProvider for MyInferenceProvider {
    async fn distribute_weights(
        &self,
        request: Request<WeightPackage>,
    ) -> Result<Response<TransferStatus>, Status> {
        let payload = request.into_inner();
        
        println!("📥 Received network payload from worker!");
        let matrix: Array2<f32> = bincode::deserialize(&payload.data)
            .map_err(|e| Status::internal(format!("Failed to deserialize: {}", e)))?;

        println!("✅ Reconstructed Matrix: {}x{}", payload.rows, payload.cols);

        let reply = TransferStatus {
            received: true,
            node_id: "master-node-01".into(),
            result_data: vec![],
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Master Node booting up...");

    // ==========================================
    // PHASE 4: TENSOR SHARDING DEMO
    // ==========================================
    println!("🧠 Generating a massive 8x4 AI weight matrix...");
    let master_matrix = Array2::<f32>::from_elem((8, 4), 7.77);
    println!("📊 Master Matrix Shape: {} rows, {} cols", master_matrix.nrows(), master_matrix.ncols());

    println!("⚔️ Sharding matrix perfectly in half for distributed workers...");
    
    // THE SPLIT: Cut the matrix along the rows (Axis 0) at index 4
    let (shard_1, shard_2) = master_matrix.view().split_at(Axis(0), 4);
    
    println!("📦 Shard 1 Shape (Ready for Worker 1): {} rows, {} cols", shard_1.nrows(), shard_1.ncols());
    println!("📦 Shard 2 Shape (Ready for Worker 2): {} rows, {} cols", shard_2.nrows(), shard_2.ncols());
    // ==========================================

    let addr = "[::1]:50051".parse()?;
    let provider = MyInferenceProvider::default();
    
    println!("📡 Listening for Worker connections on {}", addr);

    Server::builder()
        .add_service(InferenceProviderServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}
