use inference::inference_provider_client::InferenceProviderClient;
use inference::WeightPackage;

pub mod inference {
    tonic::include_proto!("inference");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️ Worker Node starting up...");
    println!("🔗 Connecting to Master Node at http://[::1]:50051...");

    // Connect to the Master Node
    let mut client = InferenceProviderClient::connect("http://[::1]:50051").await?;
    println!("✅ Connected successfully!");

    // Create a fake tensor package to send
    let request = tonic::Request::new(WeightPackage {
        data: vec![255, 128, 64, 32], // Fake tensor bytes
        scale: 0.5,
    });

    println!("📦 Sending tensor weights to Master...");
    let response = client.distribute_weights(request).await?;

    println!("🎉 Master replied: {:?}", response.into_inner());

    Ok(())
}
