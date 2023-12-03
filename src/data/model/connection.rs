use ws::Sender;

#[derive(Clone)]
pub struct Connection {
    pub su: bool,
    pub ip: String,
    pub fingerprint: String,
    /// websocket specific data and methods
    pub ws_sender: Option<Sender>
}
