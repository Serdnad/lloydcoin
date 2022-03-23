use crate::blockchain::block::Block;
use crate::node;
use crate::LC;

pub mod handlers;
mod server;

/// Keeps track of a single connection and the Node information.
pub struct Server {
    pub socket: ws::Sender,
    pub node: node::Node,
}

impl Server {
    /// Responds to any data received from the connection.
    pub fn handle_data_received(&mut self, msg: ws::Message) {
        let response = match msg {
            ws::Message::Text(msg) => self.handle_message(msg),
            _ => Err(String::from("message type not supported")),
        };

        if response.is_ok() {
            if let Some(data) = response.unwrap() {
                self.socket.send(data);
            }
        } else {
            println!("{}", response.err().unwrap());
        }
    }

    /// Dispatch message to handlers based on MessageType
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
            LC::MessageType::CreatedBlock => {
                // Synchronize blocks everytime a new one is received
                self.request_new_blocks();
                handlers::validate_and_add_block(&mut self.node, parsed.data.unwrap())
            }
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
            "get_blocks" => {
                let most_recent = request.data.unwrap();
                Ok(self.handle_get_blocks_request(most_recent))
            }
            "get_balance" => handlers::get_balance(&self.node, &request),
            "transaction" => {
                let data = &request.data.unwrap();
                handlers::validate_and_mine_transaction(&mut self.node, data.to_string())
            }
            _ => Ok(Some(String::from("unsupported request"))),
        }
    }

    /// Handle a response to a request previously sent out by this node.
    fn handle_response(&mut self, response: LC::Message) -> Result<Option<String>, String> {
        if response.action.is_none() {
            return Err(String::from("action is missing"));
        }

        match response.action.unwrap().as_ref() {
            "get_nodes" => {
                self.handle_get_nodes_response(response.data);
                Ok(None)
            }
            "get_blocks" => {
                let blocks: Vec<Block> = serde_json::from_str(&response.data.unwrap()).unwrap();
                self.handle_get_blocks_response(blocks)
            }
            _ => Ok(Some(String::from("unsupported action"))),
        }
    }
}
