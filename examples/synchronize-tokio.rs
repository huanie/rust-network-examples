use futures_util::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::{future, stream::TryStreamExt, StreamExt};
use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

const WEBSOCKET_ADDRESS: &str = "127.0.0.1:8888";
type WebsocketReceiver = SplitStream<WebSocketStream<TcpStream>>;
type WebsocketWriter = SplitSink<WebSocketStream<TcpStream>, Message>;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Payload {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let try_socket = TcpListener::bind(WEBSOCKET_ADDRESS).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", WEBSOCKET_ADDRESS);
    let (sender, _) = broadcast::channel(50);
    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            sender.clone(),
            sender.subscribe(),
            addr,
            stream,
        ));
    }

    Ok(())
}

async fn handle_connection(
    sender: Sender<Payload>,
    receiver: Receiver<Payload>,
    address: SocketAddr,
    stream: TcpStream,
) {
    println!("Incoming TCP connection from: {}", address);
    let websocket = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", address);

    let (write, read) = websocket.split();

    tokio::select! {
        _ = tokio::spawn(send_messages(write, receiver)) => {},
        _ = tokio::spawn(receive_messages(read, sender)) => {},
    }
}

// Got an update of one websocket, update all other websockets by broadcasting
async fn receive_messages(reader: WebsocketReceiver, send_more: Sender<Payload>) {
    reader
        .try_for_each(move |msg| {
            if let Ok(str) = msg.into_text() {
                match serde_json::from_str::<Payload>(&str) {
                    Ok(str) => {
                        println!("Message from websocket: {:?}", str);
                        send_more
                            .send(str)
                            .unwrap();
                    }
                    Err(_) => eprintln!("Could not parse message: {}", str),
                }
            }
            future::ok(())
        })
        .await
        .unwrap();
}

// Send data to websocket to update the UI
// When starting for the first time, send INIT request
async fn send_messages(mut writer: WebsocketWriter, mut receive_more: Receiver<Payload>) {
    while let Ok(msg) = receive_more.recv().await {
        println!(
            "Received message from broadcast, send it to websocket to synchronize in UI: {:?}",
            msg
        );
        if let Err(err) = writer
            .send(Message::Text(
                serde_json::to_string(&msg).unwrap(),
            ))
            .await
        {
            eprintln!("Couldn't send to websocket: {err}");
            writer.close().await.unwrap();
            break;
        }
    }
}
