use serde::{Serialize, Deserialize};
use serde;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Todo {
  title: String,
  notes: Option<String>,
  when: Option<String>,
  deadline: Option<String>,
  tags: Vec<String>,
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
  items: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Heading {
  title: String,
  #[serde(default)]
  archived: bool
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "attributes")]
pub enum ItemType {
  #[serde(rename = "to-do")] 
  Todo(Todo),

  #[serde(rename = "project")] 
  Project(Project),

  #[serde(rename = "heading")] 
  Heading(Heading)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
  #[serde(rename = "update")] 
  Update,
  #[serde(rename = "create")] 
  Create
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  #[serde(flatten)] 
  item_type: ItemType,
  operation: Operation,
  id: Option<String>,
}

pub fn project(title: &str, when: &str, items: Vec<Item>) -> Item {
  Item { 
    item_type: ItemType::Project(Project { 
      title: title.to_string(),
      items: items,
      when: Some(when.to_string()),
      ..Project::default()
    }),
    id: None,
    operation: Operation::Create
  }
}

pub fn todo(title: &str) -> Item {
  Item { 
    item_type: ItemType::Todo(Todo { 
      title: title.to_string(),
      ..Todo::default()
    }),
    id: None,
    operation: Operation::Create
  }
}

pub fn heading(title: &str) -> Item {
  Item { 
    item_type: ItemType::Heading(Heading { 
      title: title.to_string(),
      ..Heading::default()
    }),
    id: None,
    operation: Operation::Create
  }
}