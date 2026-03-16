use inference::inference_provider_client::InferenceProviderClient;
use inference::WeightPackage;
use ndarray::Array2;

pub mod inference {
    tonic::include_proto!("inference");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️ Worker Node starting up...");
    
    let rows = 4;
    let cols = 4;
    let matrix = Array2::<f32>::from_elem((rows as usize, cols as usize), 3.14);
    
    println!("🧠 Generated a {}x{} AI Matrix", rows, cols);

    let serialized_matrix = bincode::serialize(&matrix).unwrap();
    println!("📦 Compressed matrix into {} bytes", serialized_matrix.len());

    println!("🔗 Connecting to Master Node at http://[::1]:50051...");
    let mut client = InferenceProviderClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(WeightPackage {
        data: serialized_matrix,
        rows: rows as u32,
        cols: cols as u32,
    });

    println!("🚀 Transmitting tensor weights to Master...");
    let response = client.distribute_weights(request).await?;

    println!("🎉 Master successfully received payload: {}", response.into_inner().received);

    Ok(())
}
