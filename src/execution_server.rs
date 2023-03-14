use tonic::{Request, Response, Status};

use execution::execution_service_server::ExecutionService;
use execution::{DoBlockRequest, DoBlockResponse};

pub mod execution {
    include!("execution.v1.rs");
}

#[derive(Default)]
pub struct ExecutionRpcServer {}

#[tonic::async_trait]
impl ExecutionService for ExecutionRpcServer {
    async fn do_block(
        &self,
        _request: Request<DoBlockRequest>,
    ) -> Result<Response<DoBlockResponse>, Status> {
        // TODO - gen_execution_payload
        println!("ExecutionServer.do_block {:#?}", _request);
        Ok(Response::new(DoBlockResponse {
            state_root: vec![1, 2, 3],
        }))
    }
}
