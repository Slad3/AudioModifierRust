mod audio_modifiers;
mod models;
mod realtime_playback;

use crate::models::AudioDataThread;
use crate::realtime_playback::playback;
use axum::{routing::get, Router};
use rustfft::num_traits::real::Real;
use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::audio_modifiers::deepen_voice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, StreamConfig};
use std::sync::mpsc::Sender;

fn on_connect(
    socket: SocketRef,
    Data(data): Data<Value>,
    receiver: Arc<Mutex<Receiver<AudioDataThread>>>,
) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    let file = File::open("testData.json").expect("Could not open file");
    let reader = BufReader::new(file);

    let test_data: Vec<f64> = serde_json::from_reader(reader).expect("could not parse ");

    socket.on(
        "message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);
            socket.bin(bin).emit("message-back", data).ok();
        },
    );

    socket.on(
        "sendTestData",
        move |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);

            socket
                .bin(bin)
                .emit("testData", serde_json::json!({"data": test_data.clone()}))
                .ok();
            info!("Sent Data")
        },
    );

    socket.on("mic", move |socket: SocketRef, Data::<Value>(data)| {
        info!("Received event MIC: {:?}", data);
        for received in receiver.lock().unwrap().iter() {
            println!("Received");
            // socket
            //     .emit("testData", serde_json::json!({"data": vec![1, 3, 56,3, 9]}))
            //     .ok();
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (data_to_socket_sender, data_to_socket_receiver) = mpsc::channel::<AudioDataThread>();
    let data_to_socket_receiver = Arc::new(Mutex::new(data_to_socket_receiver));

    let audio_data = Arc::new(Mutex::new(Vec::new()));
    // let _ = playback(&audio_data);
    thread::spawn(move || {
        let _ = playback(&audio_data);
    }).join().expect("TODO: panic message");

    // tracing::subscriber::set_global_default(FmtSubscriber::default()).expect("asdfasdf");
    //
    // let (layer, io) = SocketIo::new_layer();
    //
    // io.ns("/", move |socket: SocketRef, Data(data): Data<Value>| {
    //     // on_connect(socket, data, Arc::clone(&data_to_socket_receiver))
    //
    //     info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    //     socket.emit("auth", data).ok();
    //
    //     let file = File::open("testData.json").expect("Could not open file");
    //     let reader = BufReader::new(file);
    //
    //     let test_data: Vec<f64> = serde_json::from_reader(reader).expect("could not parse ");
    //
    //     socket.on(
    //         "message",
    //         |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
    //             info!("Received event: {:?} {:?}", data, bin);
    //             socket.bin(bin).emit("message-back", data).ok();
    //         },
    //     );
    //
    //     socket.on(
    //         "sendTestData",
    //         move |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
    //             info!("Received event: {:?} {:?}", data, bin);
    //
    //             socket
    //                 .emit("testData", serde_json::json!({"data": test_data.clone()}))
    //                 .ok();
    //             info!("Sent Data")
    //         },
    //     );
    //
    //     socket.on(
    //         "mic",
    //         move |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
    //             info!("Received event MIC: {:?}", data);
    //
    //             // let received_data = data_to_socket_receiver.lock().unwrap().recv().expect("something").data;
    //             let received_data = audio_data.lock().unwrap().clone();
    //
    //             socket
    //                 .emit("testData", serde_json::json!({"data": received_data}))
    //                 .expect("TODO: panic message");
    //             for i in (0..5) {
    //                 let asdf = socket.emit(
    //                     "testData",
    //                     serde_json::json!({"data": vec![i, i/2, i*2, i, i+10, i]}),
    //                 ).ok();
    //                 println!("{:?}", &asdf);
    //                 thread::sleep(std::time::Duration::from_millis(2000));
    //             }
    //
    //
    //             // for received in data_to_socket_receiver.lock().unwrap().iter() {
    //             //     socket
    //             //         .bin(vec![vec![]])
    //             //         .emit("testData", serde_json::json!({"data": received.data.clone()}))
    //             //         .ok();
    //             // }
    //         },
    //     );
    // });
    //
    // let cors = CorsLayer::new().allow_origin(Any);
    //
    // let app = axum::Router::new().layer(layer).layer(cors);
    //
    // info!("Starting server");
    //
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:8008").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    Ok(())
}
