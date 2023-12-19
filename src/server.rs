use tonic::{transport::Server, Request, Response, Status};
use mainflow::file_service_server::{FileService, FileServiceServer};
use mainflow::{HelloReply, HelloRequest};
use std::path::{Path};
use std::fs::File;
use std::io::Write;
use log::{info, warn};

pub mod mainflow {
    tonic::include_proto!("mainflow");
}

#[derive(Default)]
pub struct MyFileService {}

fn process_request (req: &HelloRequest) -> std::io::Result<()> {
    info!("Got file: {} from {}", req.file_name, req.client_name);
    let s = format!("{}_{}", req.client_name, req.file_name);
    let dst_file_name = Path::new(&s);
    let mut f = File::create(&dst_file_name)?;
    match f.write_all(&req.data.to_vec()) {
        Ok(_)=> {
            info!("* saved as {s}");
            Ok(())
        },
        Err(e) => {warn!("{e:?}");
            Err(e)}
    }
}

#[tonic::async_trait]
impl FileService for MyFileService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {

        let req = request.into_inner();
        let (status, replay_data) = match process_request(&req) {
            Ok(_) => (0, "ok"),
            Err(_e) => (1, "error"),
        };

        let reply = HelloReply {
            message: format!("Hello {}! file `{}` has been processed!", req.client_name, req.file_name),
            status,
            replay_data: replay_data.into(),
        };
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr = "[::1]:50051".parse().unwrap();
    let fileservice = MyFileService::default();

    info!("File Server listening on {}", addr);

    Server::builder()
        .add_service(FileServiceServer::new(fileservice))
        .serve(addr)
        .await?;

    Ok(())
}
