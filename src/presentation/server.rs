use tonic::transport::Server;
use crate::infrastructure::greeter_service::GreeterService;
use crate::infrastructure::greeter_service::greeter::greeter_server::GreeterServer;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter_service = GreeterService::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter_service))
        .serve(addr)
        .await?;

    Ok(())
}
