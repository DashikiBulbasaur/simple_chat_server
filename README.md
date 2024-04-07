# Project description

This is a simple chat server written in Rust and Tokio where you can open several clients on your terminal, connect to the server, and talk to the other clients
you opened. 

There is a maximum capacity of 5 users/clients. This is an arbitrary number I decided on, as I'm pretty sure it can even accomodate up to 1000 clients and above.
But since you're the only one who can open the clients, and you're most likely manually opening them one by one, and furthermore you're just talking to yourself, 
there is no need to increase the capacity to even just 10 clients. 

# How to run it

First, please make sure that you have [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed. Then make sure you have telnet installed
and enabled on your system. 

Clone the project to your system. Go to the project directory using your terminal, type `cargo run`, and open at least two more terminals and type
`telnet localhost 8080` on them. You should connect to the localhost:8080 server that `cargo run` instantiated, and you can start typing and receiving
messages.

# Problems that I know of

1. user ids are posted on the server level, not user level.
2. when a user posts a message, it should show their user_id + their message.

For a list of possible fixes and other notes, please look at problems_changes_notes.txt

# Possible changes/updates

1. on the server level, it should say `user_id connected`.

# Notes on the project's conception

The inspiration from this project came from a desire to start understanding + working on asynchronous and network programming. A huge chunk of the code comes from
this [video](https://www.youtube.com/watch?v=T2mWg91sx-o), and I added the ability to have unique users that were immediately identifiable in the chat, which is closer to a 
real chat server/app.

# On the project as a precursor to a chat client

I looked at several Rust backend web frameworks such as Axum, Actix and Rocket, and I saw that they were pretty much built on Tokio, which is an asynchronous runtime
crate for Rust. Instead of jumping straight to working with the web frameworks, I wanted to gain a deeper understanding of asynchronous programming first so that I'd have
a good foundation when working with the backend frameworks.

All that to say that I learned a lot about asynchronous programming, and after this project I wanna move on to organically making a simple chat web client that I can host
on my vscode using port forwarding.
