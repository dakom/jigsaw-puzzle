use std::rc::Rc;
use shipyard::*;
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

pub async fn connect(world:Rc<World>) -> Result<(), JsValue> {
    let ws = WebSocket::new(crate::config::websocket_url());

    Ok(())
}

//const websocket = new WebSocket('wss://websocket-example.signalnerve.workers.dev');
//websocket.addEventListener('message', event => {
  //console.log('Message received from server');
  //console.log(event.data);
//});
