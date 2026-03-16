use inference::inference_provider_client::InferenceProviderClient;
use inference::WeightPackage;
use ndarray::Array2;
use rayon::prelude::*;

pub mod inference {
    tonic::include_proto!("inference");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️ Worker Node starting up...");

    let rows = 4;
    let cols = 4;
    println!("🧠 Worker generating {}x{} matrix shard...", rows, cols);
    
    // We start with 3.14
    let mut matrix = Array2::<f32>::from_elem((rows, cols), 3.14);

    println!("⚡ IGNITING RAYON: Spreading matrix math across all CPU cores!");
    
    // THE MAGIC LINE: par_iter_mut() forces the computer into multi-core parallel processing.
    // We are simulating a heavy Neural Network layer activation by multiplying everything by 2.0!
    matrix.as_slice_mut().unwrap().par_iter_mut().for_each(|val| {
        *val = *val * 2.0; 
    });

    println!("✅ Parallel compute finished! First element is now: {}", matrix[[0,0]]);

    let serialized_matrix = bincode::serialize(&matrix).unwrap();
    println!("📦 Compressed computed shard into {} bytes", serialized_matrix.len());

    println!("🔗 Connecting to Master Node at http://[::1]:50051...");
    let mut client = InferenceProviderClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(WeightPackage {
        data: serialized_matrix,
        rows: rows as u32,
        cols: cols as u32,
    });

    println!("🚀 Transmitting computed tensor weights to Master...");
    let response = client.distribute_weights(request).await?;

    println!("🎉 Master successfully received payload: {}", response.into_inner().received);

    Ok(())
}
