use chrono::{DateTime, NaiveDate, Utc};

mod event;
mod tags;
mod project;

// Do not @ me about bad encapsulation. fix it yourself
pub struct EventId(pub String);
pub struct TagId { pub id: String, pub name: String }
pub struct ProjectId { pub id: String, pub name: String }
pub enum Event {
    Active {
        id: EventId,
        name: String,
        start: DateTime<Utc>,
        tags: Vec<TagId>,
        project: Option<ProjectId>,
        body: Option<String>
    },
    Inactive {
        id: EventId,
        name: Option<String>,
        start: DateTime<Utc>,
        body: Option<String>
    }
}

pub struct Color(String);
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Vec<Color>,
}
pub enum ProjectState {
    Active,
    Done {
        completed_date: NaiveDate 
    }
}
pub struct Project {
    pub id: String,
    pub name: String,
    pub state: ProjectState,
    pub color: Vec<Color>,
    pub body: Option<String>
}

