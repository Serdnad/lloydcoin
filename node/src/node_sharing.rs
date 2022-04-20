use crate::server::Server;
use crate::LC;
use serde_json::json;

impl Server {
    /// Send a request to the connection for its list of connections.
    pub fn request_nodes(&self) {
        let request = json!(LC::Message {
            typ: LC::MessageType::Request,
            action: Some("get_nodes".to_string()),
            data: None,
        });
        self.socket.send(request.to_string());
    }

    /// Send back the list of connections
    pub fn handle_get_nodes_request(&mut self) -> String {
        let connections = self.node.get_connections();
        let resp_data = json!(connections.into_iter().collect::<Vec<String>>());

        let response = json!(LC::Message {
            typ: LC::MessageType::Response,
            action: Some("get_nodes".to_string()),
            data: Some(resp_data.to_string())
        });

        response.to_string()
    }

    /// Receive the list of connections and connect to new ones.
    pub fn handle_get_nodes_response(&mut self, data: Option<String>) {
        if data.is_some() {
            let str_vec = data.unwrap();
            println!("{:?}", str_vec);

            let ips: Vec<String> = serde_json::from_str(&str_vec).unwrap();
            self.connect_to_new_ips(ips);
        }
    }

    /// Connect to a given ip if not already connected to it.
    fn connect_to_new_ips(&mut self, new_ips: Vec<String>) {
        for ip in new_ips {
            if !self.node.contains_ip(&ip) {
                let url = format!("ws://{}:9001", &ip);
                crate::network::connect_to_ip(url, self.node.clone());
            }
        }
    }
}
