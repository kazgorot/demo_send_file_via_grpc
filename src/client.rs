use mainflow::file_service_client::FileServiceClient;
use mainflow::HelloRequest;
use std::time::{Instant};
use std::path::{Path};
use std::fs::File;
use std::io::Read;
use std::env;
use log::{error, info};


pub mod mainflow {
    tonic::include_proto!("mainflow");
}

fn file_to_bytes(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    File::open(path).and_then(|mut file| {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let client_name = &args[1];
    let file_path = &args[2];
    let mut client = FileServiceClient::connect("http://[::1]:50051").await?;
    let file = Path::new(file_path);
    let file_bs = file_to_bytes(file)?;
    info!("send file: {file:?}, size: {}", file_bs.len());
    // println!("{:?}", file_bs);


    let request = tonic::Request::new(HelloRequest {
        client_name: client_name.into(),
        file_name: file.file_name().unwrap().to_str().unwrap().to_string(),
        data: file_bs,
    });

    let start = Instant::now();
    let response = client.say_hello(request).await?.into_inner();
    log::info!("response time: {:?}", start.elapsed());

    if response.status == 0 {
        info!("{}", response.message);
    } else {
        error!("RESPONSE={response:?}");
    }

    Ok(())
}