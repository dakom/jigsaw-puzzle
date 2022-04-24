use std::rc::Rc;
use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use futures::{SinkExt, channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender}, StreamExt};

pub async fn connect(world:Rc<World>) -> Result<(), JsValue> {
    let (mut tx, mut rx) = unbounded();

    let ws = WebSocket::new(crate::config::websocket_url());

    tx.send("ready").await;

    match rx.next().await {
        None => Err(JsValue::from_str("couldn't connect socket!")),
        Some(msg) => {
            Ok(())
        }
    }
}

//const websocket = new WebSocket('wss://websocket-example.signalnerve.workers.dev');
//websocket.addEventListener('message', event => {
  //console.log('Message received from server');
  //console.log(event.data);
//});
