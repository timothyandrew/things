mod api;
mod wasm;

use api::*;
use chrono::{Datelike,Weekday,Duration,Local};
use std::error::Error;
use urlencoding::encode;
use wasm_bindgen::prelude::*;

// Need this because you can't pass a `chrono` thing
// into a WASM-accessible function (for good reason).
#[wasm_bindgen]
pub enum ThingsDate {
    Today,
    Tomorrow
}

pub trait WeekdayExt {
    fn is_weekend(&self) -> bool;
}

impl WeekdayExt for Weekday {
    fn is_weekend(&self) -> bool {
        match self {
            Weekday::Sat | Weekday::Sun => true,
            _ => false
        }
    }
}

fn compile(items: Vec<Item>) -> Result<String, Box<dyn Error>> {
    let json = serde_json::to_string(&items)?;
    let str = format!("things:///json?data={}", encode(&json));
    Ok(str)
}

#[wasm_bindgen]
pub fn work_daily(date: Option<ThingsDate>) -> String {
    let date = match date {
        Some(ThingsDate::Today) => Local::today(),
        Some(ThingsDate::Tomorrow) => Local::today() + Duration::days(1),
        None => Local::today()
    };

    let mut todos = vec![
        heading("Goals"),
        todo("Clear Things inbox + plan for the day", None),
        heading("Learning"),
        todo(
            "Learning plan for the day",
            Some("https://timothyandrew.net/learning"),
        )
    ];

    if !date.weekday().is_weekend() {
        todos.extend(vec![
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
                ]
            )
        ]);
    };

    let day = date.format("%A");
    let iso = date.format("%Y-%m-%d");

    let notes = match date.weekday() {
        Weekday::Fri | Weekday::Sat | Weekday::Sun => format!("- Today is {}!", day),
        _ => format!("- Today is {}!\n- <calendar goes here>", day)
    };


    let project = project(&format!("Daily: {}", iso), "today", &notes, todos);

    compile(vec![project]).unwrap()
}
