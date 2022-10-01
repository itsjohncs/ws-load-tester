use std::{env, rc::Rc, sync::Arc, time::Duration};

use futures_util::{future, pin_mut, StreamExt, SinkExt};
use rand::Rng;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let connect_addr = env::args().nth(1).unwrap();
    let max_connections: i32 = env::args().nth(2).unwrap().parse().unwrap();

    let mut handles = vec![];
    for _ in 0..max_connections {
        let url = url::Url::parse(&connect_addr).unwrap();
        handles.push(tokio::spawn(async move {
            let mut connection = connect_async(url.as_ref()).await.unwrap().0;
            let (mut sender, mut receiver) = connection.split();
            loop {
                let expected = rand::thread_rng().gen::<i32>().to_string();
                sender.send(Message::Text(expected.clone())).await.unwrap();
                tokio::time::sleep(Duration::from_secs(4)).await;
                let foo = tokio::time::timeout(Duration::from_secs(10), receiver.next()).await.unwrap().unwrap().unwrap();
                match foo {
                    Message::Text(x) if x == expected => continue,
                    Message::Text(x) => panic!("Got {x}, expected {expected}."),
                    _ => panic!("Some other error"),
                }
            }
        }));
    }

    futures::future::join_all(handles).await;
}
