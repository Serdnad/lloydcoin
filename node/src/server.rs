use crate::node;
use crate::LC;

mod handlers;
mod server;

pub struct Server {
    pub socket: ws::Sender,
    pub node: node::Node,
}

impl Server {
    pub fn handle_data_received(&mut self, msg: ws::Message) {
        let socket = &self.socket.clone();

        let response = match msg {
            ws::Message::Text(msg) => self.handle_message(msg),
            _ => Err(String::from("message type not supported")),
        };

        if response.is_ok() {
            if let Some(data) = response.unwrap() {
                socket.send(data);
            }
        } else {
            println!("{}", response.err().unwrap());
        }
    }

    /// Determine whether a message received by this node is a new request, or a response to a
    /// request made by this node, and route it accordingly.
    fn handle_message(&mut self, msg: String) -> Result<Option<String>, String> {
        let parsed: Result<LC::Message, _> = serde_json::from_str(&msg);
        if parsed.is_err() {
            println!("ERROR PARSING JSON: {}", parsed.unwrap_err());
            return Err(String::from("ERROR"));
        }

        let parsed = parsed.unwrap();
        match parsed.typ {
            LC::MessageType::Request => self.handle_request(parsed),
            LC::MessageType::Response => self.handle_response(parsed),
            _ => Ok(Some(String::from("Unsupported type"))),
        }
    }

    /// Route a received request to the intended handler.
    fn handle_request(&mut self, request: LC::Message) -> Result<Option<String>, String> {
        if request.action.is_none() {
            return Err(String::from("action is missing"));
        }

        match request.action.as_ref().unwrap().as_ref() {
            "get_nodes" => Ok(Some(self.handle_get_nodes_request())),
            "get_block" => handlers::get_block(&self.node, &request),
            "get_balance" => handlers::get_balance(&self.node, &request),
            "transaction" => {
                let data = &request.data.unwrap();
                handlers::add_transaction(&mut self.node, data.to_string())
            }
            _ => Ok(Some(String::from("unsupported request"))),
        }
    }

    /// Handle a response to a request previously sent out by this node.
    /// TODO: A much better way (i think) to do this would be to store an ID for any request made,
    ///     with relevant data, and then complete them. that'll also make the ping/pong process
    ///     easier I'd think, as far as detecting timeouts.
    fn handle_response(&mut self, response: LC::Message) -> Result<Option<String>, String> {
        if response.action.is_none() {
            return Err(String::from("action is missing"));
        }

        match response.action.unwrap().as_ref() {
            "get_nodes" => {
                self.handle_get_nodes_response(response.data);
                Ok(None)
            }
            _ => Ok(Some(String::from("unsupported action"))),
        }
    }
}
