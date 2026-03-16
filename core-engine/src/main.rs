use tonic::{transport::Server, Request, Response, Status};
use inference::inference_provider_server::{InferenceProvider, InferenceProviderServer};
use inference::{WeightPackage, TransferStatus};
use ndarray::Array2;

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
        println!("📊 Declared Shape: {}x{}", payload.rows, payload.cols);

        let matrix: Array2<f32> = bincode::deserialize(&payload.data)
            .map_err(|e| Status::internal(format!("Failed to deserialize: {}", e)))?;

        println!("✅ Successfully reconstructed the AI matrix!");
        println!("🧮 First element in matrix is: {}", matrix[[0, 0]]);

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
    let addr = "[::1]:50051".parse()?;
    let provider = MyInferenceProvider::default();
    
    println!("🚀 Master Node booting up...");
    println!("📡 Listening for Worker connections on {}", addr);

    Server::builder()
        .add_service(InferenceProviderServer::new(provider))
        .serve(addr)
        .await?;

    Ok(())
}
