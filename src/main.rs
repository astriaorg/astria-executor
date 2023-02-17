use execution::execution_service_server::{ExecutionService, ExecutionServiceServer};
use execution::{DoBlockRequest, DoBlockResponse};
use tonic::{Request, Response, Status, transport::Server};

pub mod execution {
    tonic::include_proto!("execution");
}

#[derive(Default)]
struct ExecutionServer {}

#[tonic::async_trait]
impl ExecutionService for ExecutionServer {
    async fn do_block(&self, _request: Request<DoBlockRequest>) -> Result<Response<DoBlockResponse>, Status> {
        // TODO - gen_execution_payload
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse()?;
    let server = ExecutionServer::default();
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
