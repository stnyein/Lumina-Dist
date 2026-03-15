use tonic::{transport::Server, Request, Response, Status};
use inference::inference_provider_server::{InferenceProvider, InferenceProviderServer};
use inference::{WeightPackage, TransferStatus};

// Define the gRPC module from our proto
pub mod inference {
    tonic::include_proto!("inference");
}

#[derive(Default)]
pub struct MyInferenceEngine {}

#[tonic::async_trait]
impl InferenceProvider for MyInferenceEngine {
    async fn distribute_weights(
        &self,
        request: Request<WeightPackage>,
    ) -> Result<Response<TransferStatus>, Status> {
        let pkt = request.into_inner();
        
        println!("📡 Node Received: {} weights (Scale: {})", pkt.data.len(), pkt.scale);

        let reply = TransferStatus {
            received: true,
            node_id: "Lumina-Node-01".into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let engine = MyInferenceEngine::default();

    println!("🚀 Lumina-Dist Master Node listening on {}", addr);
    println!("💡 Status: Waiting for Distributed Workers...");

    Server::builder()
        .add_service(InferenceProviderServer::new(engine))
        .serve(addr)
        .await?;

    Ok(())
}