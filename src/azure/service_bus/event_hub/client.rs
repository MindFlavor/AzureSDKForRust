use azure::service_bus::event_hub::submit_event;

pub struct Client {
    pub namespace: String,
    pub event_hub: String,
    pub policy_name: String,
    pub key: String,
}

impl Client {
    pub fn new(namespace: &str,
        event_hub: &str,
        policy_name: &str,
        key: &str) -> Client {
            Client {
                namespace : namespace.to_owned(),
                event_hub : event_hub.to_owned(),
                policy_name : policy_name.to_owned(),
                key : key.to_owned()
            }
    }
}
