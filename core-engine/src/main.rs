use tonic::{transport::Server, Request, Response, Status};
use inference::inference_provider_server::{InferenceProvider, InferenceProviderServer};
use inference::{WeightPackage, TransferStatus};

// This tells Rust to include the code it auto-generated from your .proto file!
pub mod inference {
    tonic::include_proto!("inference"); 
}

// 1. Define our Master Node service
#[derive(Debug, Default)]
pub struct MyInferenceProvider {}

// 2. Implement the gRPC logic defined in your proto file
#[tonic::async_trait]
impl InferenceProvider for MyInferenceProvider {
    async fn distribute_weights(
        &self,
        request: Request<WeightPackage>,
    ) -> Result<Response<TransferStatus>, Status> {
        println!("📥 Received weight package from a worker!");
        
        // Send a success message back to the worker
        let reply = TransferStatus {
            received: true,
            node_id: "master-node-01".into(),
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

    // 3. Start the server and ATTACH the service!
    Server::builder()
        .add_service(InferenceProviderServer::new(provider)) // <-- This is the magic line that fixes the error!
        .serve(addr)
        .await?;

    Ok(())
}
