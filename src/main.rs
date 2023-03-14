use color_eyre::eyre::Result;

use astria_executor::execution_server::execution::execution_service_server::ExecutionServiceServer;
use astria_executor::execution_server::ExecutionRpcServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let address = "[::1]:50051".parse()?;
    let server = ExecutionRpcServer::default();
    Server::builder()
        .add_service(ExecutionServiceServer::new(server))
        .serve(address)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO
        assert_eq!(true, true);
    }
}
