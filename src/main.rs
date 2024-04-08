use rand::Rng;
// use std::sync::{Arc, Mutex};
// note: would love to have used this
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

    let mut list_of_users: Vec<u32> = Vec::new();

    // loop so that multiple clients are possible. without this, can only do one client
    loop {
        let (mut socket, addr) = listener.accept().await?;

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        // can't clone rx, have to subscribe? i think this is a quirk/feature in tokio

        // initialize the random user numbers
        let mut number = rand::thread_rng().gen_range(0..=1000);

        // check if the number is already used
        while list_of_users.contains(&number) {
            number = rand::thread_rng().gen_range(0..=1000);
        }
        list_of_users.push(number);

        // print to server which users connected
        let mut user_id = "user".to_owned();
        user_id.push_str(&number.to_string());
        println!("{} connected", &user_id);

        // spawn a thread everytime a different connection is open. without this, tasks are
        // blocked, single threaded, and queued. aka network is still blocked
        // at the task level
        tokio::spawn(async move {
            // read and write the stream concurrently
            let (reader, mut writer) = socket.split();

            // let the client know what their id is
            let mut user_knows_id: String = "You are ".to_owned();
            user_knows_id.push_str(&user_id);
            // \r\n\r\n pushes cursor two lines down and to the leftmost
            // side of terminal. \r means push to the left, \n is newline
            user_knows_id.push_str("\r\n\r\n");
            writer.write_all(user_knows_id.as_bytes()).await.unwrap();

            // read calls to the network socket
            let mut reader = BufReader::new(reader);
            // prepare to write msgs
            let mut line = String::new();

            // so that sending/receiving multiple msgs is possible for a client
            loop {
                // select which happens first: sending a msg or receiving one
                tokio::select! {
                    // i think this writes bc of the read_line
                //
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
                        } else {
                            let mut user_sees_their_msg: String = "(you) ".to_owned();
                            user_sees_their_msg.push_str(&msg);
                            writer.write_all(user_sees_their_msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
