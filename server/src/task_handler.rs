use std::collections::HashMap;

#[derive(Default)]
pub struct TaskHandler<'a> {
    sections: HashMap<&'a str, Section<'a>>,
    current_section: Option<String>,
}

#[derive(Default)]
pub struct Section<'a> {
    tasks: HashMap<&'a str, Task>,
}

#[derive(Default)]
pub struct Task {
    description: String,
    completed: bool,
}

impl<'a> TaskHandler<'a> {
    pub fn add_section(&mut self, section_name: &'a str) {
        self.sections.insert(section_name, Section::default());
    }
    // pub fn add_section(&mut self, task_name: &str, task_description: &str, completed_bool) {
}
