mod api;

use api::*;

pub fn daily() -> Vec<Item> {
  let todos = vec![
    todo("first!"),
    heading("first!"),
    todo("second!")
  ];

  let project = project("Foo!", "today", todos);
  vec![project]
}