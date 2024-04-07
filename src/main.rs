use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // only 5 clients can tune in at the same time
    let (tx, _rx) = broadcast::channel(5);

    // note to self: do I even need Arc in this scenario? I feel like the vec
    // isn't really shared across threads, cuz each thread will only ever
    // access this once and that's at the beginning. I'm prob wrong.
    let mut list_of_users: Vec<u32> = Vec::new();

    // loop so that multiple clients are possible. without this, can only do one client
    loop {
        let mut number = rand::thread_rng().gen_range(0..=1000);

        while list_of_users.contains(&number) {
            number = rand::thread_rng().gen_range(0..=1000);
        }

        list_of_users.push(number);

        let mut user_id: String = "user".to_owned();
        // ideally i'd love to push ownership of number to user_id so
        // number is just gone, but this works at least
        user_id.push_str(&number.to_string());

        // this is here cuz i'm scared if it's on spawn,
        // it's just gonna keep repeating
        println!("Your user id is {}", &user_id);

        let (mut socket, addr) = listener.accept().await?;

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        // can't clone rx, have to subscribe? i think this is a quirk/feature in tokio

        // spawn a thread everytime a different connection is open. without this, tasks are
        // blocked, single threaded, and queued. aka network is still blocked
        // at the task level
        tokio::spawn(async move {
            // read and write the stream concurrently
            let (reader, mut writer) = socket.split();

            // read calls to the network socket
            let mut reader = BufReader::new(reader);
            // prepare to write msgs
            let mut line = String::new();

            // so that sending/receiving multiple msgs is possible for a client
            loop {
                // select which happens first: sending a msg or receiving one
                tokio::select! {
                    // i think this writes bc of the read_line
                    // thank u bufreader
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            // this doesn't even work. this is supposed
                            // to close the connection if no msg is sent
                            break;
                        }

                        line = user_id.clone() + ": " + &line;
                        tx.send((line.clone(), addr)).unwrap();
                    }
                    // the part that receives msgs from the network and prints them
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
