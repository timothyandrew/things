mod api;

use api::*;
use chrono::Local;

pub fn work_daily() -> Vec<Item> {
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
                checklist_item("Yesterday's daily journal note"),
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
    let project = project(&format!("Work Daily: {}", date), "today", &notes, todos);

    vec![project]
}
