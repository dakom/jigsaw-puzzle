use std::rc::Rc;
use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::WebSocket;
use futures::channel::oneshot;

pub async fn connect(world:Rc<World>) -> Result<(), JsValue> {
    let (mut tx, mut rx) = oneshot::channel();

    let ws = WebSocket::new(crate::config::websocket_url());

    tx.send("ready");

    match rx.await {
        Err(_) => Err(JsValue::from_str("couldn't connect socket!")),
        Ok(msg) => {
            Ok(())
        }
    }
}

//const websocket = new WebSocket('wss://websocket-example.signalnerve.workers.dev');
//websocket.addEventListener('message', event => {
  //console.log('Message received from server');
  //console.log(event.data);
//});
