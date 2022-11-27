use hello::say_client::SayClient;
use hello::SayRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to the server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    // creating gRPC client from channel
    let mut client = SayClient::new(channel);
    
    // creating a new request
    let request = tonic::Request::new(
            SayRequest {
                name:String::from("someone")   
            }
        );
    
    // sending a request and waiting for a response
    
    let response = client.send(request).await?.into_inner();
    
    println!("RESPONSE={:?}", response);
    
    Ok(())
    
}
