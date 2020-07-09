
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChecklistItem {
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Todo {
    title: String,
    notes: Option<String>,
    when: Option<String>,
    deadline: Option<String>,
    tags: Vec<String>,
    #[serde(rename = "checklist-items")]
    checklist_items: Vec<Item>,
    #[serde(rename = "list-id")]
    list_id: Option<String>,
    list: Option<String>,
    heading: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    title: String,
    notes: Option<String>,
    when: Option<String>,
    deadline: Option<String>,
    tags: Vec<String>,
    #[serde(rename = "area-id")]
    area_id: Option<String>,
    area: Option<String>,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Heading {
    title: String,
    #[serde(default)]
    archived: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "attributes")]
pub enum ItemType {
    #[serde(rename = "to-do")]
    Todo(Todo),

    #[serde(rename = "project")]
    Project(Project),

    #[serde(rename = "heading")]
    Heading(Heading),

    #[serde(rename = "checklist-item")]
    ChecklistItem(ChecklistItem),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "create")]
    Create,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(flatten)]
    item_type: ItemType,
    operation: Operation,
    id: Option<String>,
}

// TODO: use the builder pattern (or macros) instead

pub fn project(title: &str, when: &str, notes: &str, items: Vec<Item>) -> Item {
    Item {
        item_type: ItemType::Project(Project {
            title: title.to_string(),
            notes: Some(notes.to_string()),
            items: items,
            when: Some(when.to_string()),
            ..Project::default()
        }),
        id: None,
        operation: Operation::Create,
    }
}

pub fn todo(title: &str, note: Option<&str>) -> Item {
    Item {
        item_type: ItemType::Todo(Todo {
            title: title.to_string(),
            notes: note.map(String::from),
            ..Todo::default()
        }),
        id: None,
        operation: Operation::Create,
    }
}

pub fn todo_checklist(title: &str, note: Option<&str>, checklist: Vec<Item>) -> Item {
    Item {
        item_type: ItemType::Todo(Todo {
            title: title.to_string(),
            notes: note.map(String::from),
            checklist_items: checklist,
            ..Todo::default()
        }),
        id: None,
        operation: Operation::Create,
    }
}

pub fn checklist_item(title: &str) -> Item {
    Item {
        item_type: ItemType::ChecklistItem(ChecklistItem {
            title: title.to_string(),
        }),
        id: None,
        operation: Operation::Create,
    }
}

pub fn heading(title: &str) -> Item {
    Item {
        item_type: ItemType::Heading(Heading {
            title: title.to_string(),
            ..Heading::default()
        }),
        id: None,
        operation: Operation::Create,
    }
}
