use tonic::{Request, Response, Status};

use crate::usecase::say_hello::SayHelloUseCase;
use crate::domain::greeter::GreeterUseCase;

pub mod greeter {
    tonic::include_proto!("greeter");
}

use greeter::greeter_server::Greeter;
use greeter::{HelloRequest, HelloReply};

#[derive(Default)]
pub struct GreeterService {
    pub usecase: SayHelloUseCase,
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let reply = self.usecase.say_hello(name);
        Ok(Response::new(HelloReply { message: reply }))
    }
}
