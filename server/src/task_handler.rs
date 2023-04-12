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

#[derive(Default, Clone)]
pub struct Task {
    pub description: String,
    pub completed: bool,
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

    pub fn set_task_completion(&mut self, task_name: &'a str, completed: bool) {
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

        let task = match section.tasks.get_mut(task_name) {
            Some(v) => v,
            None => {
                log::error!("Could not find task: {task_name}");
                return;
            }
        };

        task.completed = completed;
    }

    pub fn is_section_completed(&self) -> bool {
        let current_section_name = match &self.current_section {
            Some(v) => v,
            None => {
                log::error!("Please select a section");
                return false;
            }
        };

        let section = match self.sections.get(current_section_name.as_str()) {
            Some(v) => v,
            None => {
                log::error!("Could not find section: {}", current_section_name.as_str());
                return false;
            }
        };

        return section.tasks.values().any(|t| t.completed != true);
    }
}
