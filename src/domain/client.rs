use derive_new::new;

#[derive(new, Debug)]
pub struct Message {
    pub client_name: String,
    pub chat_id: String,
    pub data: String
}