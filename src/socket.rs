extern crate websocket;

use self::websocket::OwnedMessage;
use async_std::channel::{unbounded, Receiver, Sender};
use async_std::task;
use std::io::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::SystemTime;
use websocket::{ClientBuilder, Message, WebSocketError};

use self::websocket::client::sync::Client;
use crate::error;
use crate::event;
use std::net::TcpStream;

pub struct Channel {
    ws_address: String,
    from_main_sender: Sender<event::ZoneEvent>,
    from_main_receiver: Arc<Mutex<Receiver<event::ZoneEvent>>>,
    from_websocket_sender: Arc<Mutex<Sender<event::ZoneEvent>>>,
    pub from_websocket_receiver: Receiver<event::ZoneEvent>,
    ws_reader_handle: Option<JoinHandle<()>>,
    ws_reader_closed: Arc<Mutex<bool>>,
    ws_sender_handle: Option<JoinHandle<()>>,
    ws_sender_closed: Arc<Mutex<bool>>,
    closing: bool,
}

impl Channel {
    pub fn new(ws_address: String) -> Self {
        let (from_main_sender, from_main_receiver) = unbounded();
        let (from_websocket_sender, from_websocket_receiver) = unbounded();
        let from_main_receiver = Arc::new(Mutex::new(from_main_receiver));
        let from_websocket_sender = Arc::new(Mutex::new(from_websocket_sender));
        let ws_reader_closed = Arc::new(Mutex::new(false));
        let ws_sender_closed = Arc::new(Mutex::new(false));

        Self {
            ws_address,
            from_main_sender,
            from_main_receiver,
            ws_reader_handle: None,
            ws_reader_closed,
            ws_sender_handle: None,
            ws_sender_closed,
            from_websocket_sender,
            from_websocket_receiver,
            closing: false,
        }
    }

    fn create_ws_client(&mut self) -> Result<Client<TcpStream>, error::Error> {
        match ClientBuilder::new(self.ws_address.as_str()) {
            Ok(mut ws_client) => match ws_client.connect_insecure() {
                Ok(ws_client) => Ok(ws_client),
                Err(err) => {
                    return Err(error::Error::new(format!(
                        "Fail to connect websocket: {}",
                        err
                    )))
                }
            },
            Err(err) => return Err(error::Error::new(format!("Bad websocket url: {}", err))),
        }
    }

    pub fn connect(&mut self) -> Result<(), error::Error> {
        let from_main_receiver = Arc::clone(&self.from_main_receiver);
        let from_websocket_sender = Arc::clone(&self.from_websocket_sender);
        let ws_reader_closed = Arc::clone(&self.ws_reader_closed);
        let ws_sender_closed = Arc::clone(&self.ws_sender_closed);
        let ws_client = self.create_ws_client()?;
        let (mut ws_reader, mut ws_writer) = ws_client.split().unwrap();

        // ws reader
        let ws_reader_handle = thread::spawn(move || {
            task::block_on(async {
                let from_websocket_sender = from_websocket_sender
                    .lock()
                    .expect("Fail to acquire from_websocket_sender lock");
                for message in ws_reader.incoming_messages() {
                    match message {
                        Ok(OwnedMessage::Text(message)) => {
                            match serde_json::from_str(&message) {
                                Ok(value) => match event::ZoneEvent::from_value(value) {
                                    Ok(event) => {
                                        if let event::ZoneEventType::ServerPermitClose =
                                            event.event_type
                                        {
                                            log::info!("Receive close event from websocket");
                                            break;
                                        }

                                        if let Err(_) = from_websocket_sender.send(event).await {
                                            log::error!(
                                            "Something went wrong during process of received event"
                                            );
                                            break;
                                        }
                                    }
                                    Err(err) => log::error!("Error while decoding event: {}", err),
                                },
                                Err(err) => {
                                    log::error!("Error when interpreting event as str: {}", err)
                                }
                            };
                        }
                        Ok(OwnedMessage::Close(_)) => {
                            log::info!("Close web socket message received");
                            break;
                        }
                        Err(WebSocketError::NoDataAvailable) => {
                            log::error!("Web socket error: NoDataAvailable");
                            break;
                        }
                        _ => log::error!(
                            "WebSocket(receiver): Unknown websocket message received: {:?}",
                            message
                        ), // TODO add ping/pong (OwnedMessage::ping|pong)
                    }
                }
            });

            let mut closed = ws_reader_closed.lock().unwrap();
            *closed = true;
            log::info!("Web socket reader is closed");
        });

        // ws sender
        let ws_sender_handle = thread::spawn(move || {
            task::block_on(async {
                let from_main_receiver = from_main_receiver.lock().unwrap();

                while let Ok(received) = from_main_receiver.recv().await {
                    let message_json_str = match serde_json::to_string(&received) {
                        Ok(message_json_str) => message_json_str,
                        Err(err) => {
                            log::error!("Error during serialisation of event: {}", err);
                            continue;
                        }
                    };
                    let message = Message::text(message_json_str);
                    if let Err(err) = ws_writer.send_message(&message) {
                        log::error!("Error during send message: {}", err);
                    };

                    if let event::ZoneEventType::ClientWantClose = received.event_type {
                        // Get out for loop (and finish thread)
                        log::info!("Web socket writer is closing ...");
                        break;
                    }
                }
                log::info!("Web socker writer is closed");
            });

            let mut closed = ws_sender_closed.lock().unwrap();
            *closed = true;
        });

        self.ws_reader_handle = Some(ws_reader_handle);
        self.ws_sender_handle = Some(ws_sender_handle);

        Ok(())
    }

    pub async fn send(&self, event: event::ZoneEvent) {
        if let Err(_) = self.from_main_sender.send(event).await {
            log::error!("Error happen when transmit event to send though websocket")
        }
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        self.closing = true;
        self.send(event::ZoneEvent {
            event_type: event::ZoneEventType::ClientWantClose,
            event_type_name: String::from(event::CLIENT_WANT_CLOSE),
            world_row_i: 0,
            world_col_i: 0,
        })
        .await;

        let start = SystemTime::now();
        let timeout = Duration::from_secs(5);
        loop {
            let ws_sender_closed = *self.ws_sender_closed.lock().unwrap();
            let ws_reader_closed = *self.ws_reader_closed.lock().unwrap();

            if ws_sender_closed && ws_reader_closed {
                break;
            }

            if start.elapsed().unwrap() > timeout {
                log::error!(
                    "WebSockets: timeout reached, force closing (ws_sender_closed: {}, ws_reader_closed: {})",
                    ws_sender_closed, ws_reader_closed
                );
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}
