use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Decoder};

use bytes::Bytes;
use futures::sink::SinkExt;
use futures::StreamExt;

use bincode;
use serde::{Deserialize, Serialize};

mod http_frame;
use http_frame::{Body, Header, HttpFrame};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub ticker: String,
    pub amount: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut framed = BytesCodec::new().framed(stream);

    let message = HttpFrame {
        header: Header {
            method: "POST".to_string(),
            uri: "www.freshcutswags.com/stock/purchase".to_string(),
        },
        body: Body {
            ticker: "BYND".to_string(),
            amount: 3.2,
        },
    };
    let message_bin = bincode::serialize(&message).unwrap();
    let sending_message = Bytes::from(message_bin);
    framed.send(sending_message).await.unwrap();

    let message = framed.next().await.unwrap().unwrap();
    let message = bincode::deserialize::<HttpFrame>(&message).unwrap();
    println!("{:?}", message);
    Ok(())
}
