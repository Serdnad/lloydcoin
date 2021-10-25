use yew::prelude::*;
//use std::sync::{Mutex, Arc};
//use std::thread::spawn;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

//extern crate ws;
//use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};

enum Msg {
    AddOne,
}

type IpAddresses = Arc<Mutex<Vec<String>>>;

fn set_ips(connections: &mut IpAddresses, ips: Vec<String>) {
    let mut list = connections.lock().unwrap();
    *list = ips;
}

fn get_ips(connections: &IpAddresses) -> Vec<String> {
    let list = connections.lock().unwrap();
    list.clone()
}


struct Model {
    link: ComponentLink<Self>,
    value: i64,
    connections: IpAddresses,
    ips: Vec<String>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            connections: Arc::new(Mutex::new(Vec::new())),
            ips: Vec::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ self.ips.clone() }</p>
                <input type="button" value="Connect to ips"/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
