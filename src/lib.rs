mod api;
mod wasm;

use api::*;
use chrono::Local;
use std::error::Error;
use urlencoding::encode;
use wasm_bindgen::prelude::*;

fn compile(items: Vec<Item>) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string(&items)?;
    let str = format!("things:///json?data={}", encode(&json));
    Ok(str)
}

#[wasm_bindgen]
pub fn work_daily() -> String {
    let todos = vec![
        heading("Goals"),
        todo("Clear Things inbox + plan for the day", None),
        heading("Learning"),
        todo(
            "Learning plan for the day",
            Some("https://timothyandrew.net/learning"),
        ),
        heading("Work Goals"),
        todo_checklist(
            "Set work goals for the day",
            Some("Look through:"),
            vec![
                checklist_item("Ongoing sprint doc"),
                checklist_item("Sprint board"),
                checklist_item("Confluence comments"),
                checklist_item("Dev diary"),
                checklist_item("Notes from yesterday's meetings"),
                checklist_item("Slack/email"),
                checklist_item("Github \"participating\" view"),
            ],
        ),
    ];

    let now = Local::now();
    let day = now.format("%A");
    let date = now.format("%Y-%m-%d");

    let notes = format!("- Today is {}!\n- <calendar goes here>", day);
    let project = project(&format!("Daily: {}", date), "today", &notes, todos);

    compile(vec![project]).unwrap()
}
