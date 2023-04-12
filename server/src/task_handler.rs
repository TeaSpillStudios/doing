use std::collections::HashMap;

#[derive(Default)]
pub struct TaskHandler<'a> {
    sections: HashMap<&'a str, Section<'a>>,
    current_section: Option<String>,
}

#[derive(Default)]
pub struct Section<'a> {
    pub tasks: HashMap<&'a str, Task>,
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
    pub fn add_task(&mut self, task_name: &'a str, task_description: &'a str, completed: bool) {
        let current_section_name = match &self.current_section {
            Some(v) => v,
            None => {
                log::error!("Please select a section");
                return;
            }
        };

        let section = match self.sections.get_mut(current_section_name.as_str()) {
            Some(v) => v,
            None => {
                log::error!("Could not find section: {}", current_section_name.as_str());
                return;
            }
        };

        section.tasks.insert(
            task_name,
            Task {
                description: task_description.to_string(),
                completed,
            },
        );
    }
}
