use hello_world::uppercase_client::UppercaseClient;
use hello_world::UpperRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UppercaseClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(UpperRequest {
        original: "hello world".into(),
    });

    let response = client.upper(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
