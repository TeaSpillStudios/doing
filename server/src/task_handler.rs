use std::collections::HashMap;

#[derive(Default)]
pub struct TaskHandler {
    sections: HashMap<String, Section>,
    current_section: Option<String>,
}

#[derive(Default)]
pub struct Section {
    tasks: HashMap<String, Section>,
}

#[derive(Default)]
pub struct Task {
    description: String,
    completed: bool,
}
