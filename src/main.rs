use astria_execution_apis_rpc::execution::execution_service_server::ExecutionServiceServer;
use color_eyre::eyre::Result;

use tonic::transport::Server;
use astria_executor::execution_server::ExecutionRpcServer;

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
