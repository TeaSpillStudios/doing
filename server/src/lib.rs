pub mod tasks {
    use std::collections::HashMap;

    use tracing::error;

    #[derive(Default, Debug)]
    pub struct TaskHandler<'a> {
        sections: HashMap<&'a str, Section>,
        current_section: Option<String>,
    }

    #[derive(Default, Debug)]
    pub struct Section {
        pub tasks: HashMap<String, Task>,
    }

    #[derive(Default, Debug)]
    pub struct Task {
        pub description: String,
        pub completed: bool,
    }

    impl std::fmt::Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.description)
        }
    }

    impl<'a> TaskHandler<'a> {
        pub fn add_section(&mut self, section_name: &'a str) {
            match self.sections.contains_key(section_name) {
                true => {
                    error!("Could not add section, already exists: {section_name}");
                    return;
                }

                false => self.sections.insert(section_name, Section::default()),
            };
        }

        pub fn add_and_select_section(&mut self, section_name: &'a str) {
            match self.sections.contains_key(section_name) {
                true => {
                    error!("Could not add section, already exists: {section_name}");
                    return;
                }

                false => self.sections.insert(section_name, Section::default()),
            };

            match self.sections.contains_key(section_name) {
                true => self.current_section = Some(section_name.to_string()),
                false => error!("Could not find section: {}", section_name),
            }
        }

        pub fn add_task(&mut self, task_name: String, task_description: String, completed: bool) {
            let current_section_name = match &self.current_section {
                Some(v) => v,
                None => {
                    error!("Please select a section");
                    return;
                }
            };

            let section = match self.sections.get_mut(current_section_name.as_str()) {
                Some(v) => v,
                None => {
                    error!("Could not find section: {}", current_section_name.as_str());
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
                    error!("Please select a section");
                    return;
                }
            };

            let section = match self.sections.get_mut(current_section_name.as_str()) {
                Some(v) => v,
                None => {
                    error!("Could not find section: {}", current_section_name.as_str());
                    return;
                }
            };

            let task = match section.tasks.get_mut(task_name) {
                Some(v) => v,
                None => {
                    error!("Could not find task: {task_name}");
                    return;
                }
            };

            task.completed = completed;
        }

        pub fn is_section_completed(&self) -> bool {
            let current_section_name = match &self.current_section {
                Some(v) => v,
                None => {
                    error!("Please select a section");
                    return false;
                }
            };

            let section = match self.sections.get(current_section_name.as_str()) {
                Some(v) => v,
                None => {
                    error!("Could not find section: {}", current_section_name.as_str());
                    return false;
                }
            };

            return section.tasks.values().any(|t| t.completed != true);
        }

        pub fn select_section(&mut self, section_name: &'a str) {
            match self.sections.contains_key(section_name) {
                true => self.current_section = Some(section_name.to_string()),
                false => error!("Could not find section: {}", section_name),
            }
        }

        pub fn remove_section(&mut self, section_name: &'a str) {
            let current_section_name = match &self.current_section {
                Some(v) => v,
                None => {
                    error!("Please select a section");
                    return;
                }
            };

            if self.sections.get(current_section_name.as_str()).is_none() {
                error!("Could not find section: {}", current_section_name.as_str());
                return;
            };

            self.sections.remove(section_name);
        }

        pub fn remove_task(&mut self, task_name: &'a str) {
            let current_section_name = match &self.current_section {
                Some(v) => v,
                None => {
                    error!("Please select a section");
                    return;
                }
            };

            let section = match self.sections.get_mut(current_section_name.as_str()) {
                Some(v) => v,
                None => {
                    error!("Could not find section: {}", current_section_name.as_str());
                    return;
                }
            };

            if !section.tasks.contains_key(task_name) {
                error!("Could not find task: {task_name}");
                return;
            };

            section.tasks.remove(task_name);
        }

        pub fn get_tasks(&'a self) -> Option<&HashMap<String, Task>> {
            let current_section_name = match &self.current_section {
                Some(v) => v,
                None => {
                    error!("Please select a section");
                    return None;
                }
            };

            let section = match self.sections.get(current_section_name.as_str()) {
                Some(v) => v,
                None => {
                    error!("Could not find section: {}", current_section_name.as_str());
                    return None;
                }
            };

            Some(&section.tasks)
        }
    }
}
