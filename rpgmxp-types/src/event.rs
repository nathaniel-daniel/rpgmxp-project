use crate::EventPage;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub pages: Vec<EventPage>,
    pub name: String,
    pub y: i32,
    pub x: i32,
    pub id: i32,
}
