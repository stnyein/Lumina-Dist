use tonic::{transport::Server, Request, Response, Status};
use inference::inference_provider_server::{InferenceProvider, InferenceProviderServer};
use inference::{WeightPackage, TransferStatus};
use ndarray::{Array2, Axis, concatenate};

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
        
        println!("📥 Received COMPUTED payload from worker over the network!");
        
        let computed_shard: Array2<f32> = bincode::deserialize(&payload.data)
            .map_err(|e| Status::internal(format!("Failed to deserialize: {}", e)))?;

        println!("✅ Reconstructed Computed Shard: {}x{}", computed_shard.nrows(), computed_shard.ncols());
        println!("🔥 Worker Result (First Element): {}", computed_shard[[0, 0]]);
        println!("🧩 Ready to stitch this computed chunk back into the main brain!");

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

    println!("🧠 Generating a massive 8x4 AI weight matrix...");
    let master_matrix = Array2::<f32>::from_elem((8, 4), 3.14);

    println!("⚔️ Sharding matrix perfectly in half for distributed workers...");
    let (shard_1, shard_2) = master_matrix.view().split_at(Axis(0), 4);
    
    println!("📦 Shard 1 (4x4) isolated for Worker 1.");
    println!("📦 Shard 2 (4x4) isolated for Worker 2.");

    println!("🩹 STITCHING SIMULATION: Taping the pieces back together...");
    
    // THE MAGIC STITCH: Gluing the two matrices back together along Axis 0 (rows)
    let stitched_matrix = concatenate(Axis(0), &[shard_1, shard_2]).unwrap();
    
    println!("🏆 Final Stitched Matrix Shape: {} rows, {} cols", stitched_matrix.nrows(), stitched_matrix.ncols());
    println!("✅ The AI Brain is fully reassembled!");

    let addr = "[::1]:50051".parse()?;
    let provider = MyInferenceProvider::default();
    
    println!("📡 Listening for real Worker connections on {}", addr);

    Server::builder()
        .add_service(InferenceProviderServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}
