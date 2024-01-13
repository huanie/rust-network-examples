use tonic::{transport::Server, Request, Response, Status};

use hello_world::uppercase_server::{Uppercase, UppercaseServer};
use hello_world::{UpperReply, UpperRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyUppercase {}

#[tonic::async_trait]
impl Uppercase for MyUppercase {
    async fn upper(
        &self,
        request: Request<UpperRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UpperReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        
        let reply = UpperReply {
            uppercased: request.into_inner().original.to_uppercase(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let greeter = MyUppercase::default();

    Server::builder()
        .add_service(UppercaseServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
