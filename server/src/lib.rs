pub mod tasks {
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
                    log::error!("Could not add section, already exists: {section_name}");
                    return;
                }

                false => self.sections.insert(section_name, Section::default()),
            };
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

        pub fn select_section(&mut self, section_name: &'a str) {
            match self.sections.contains_key(section_name) {
                true => self.current_section = Some(section_name.to_string()),
                false => log::error!("Could not find section: {}", section_name),
            }
        }

        pub fn remove_section(&mut self, section_name: &'a str) {
            let current_section_name = match &self.current_section {
                Some(v) => v,
                None => {
                    log::error!("Please select a section");
                    return;
                }
            };

            if self.sections.get(current_section_name.as_str()).is_none() {
                log::error!("Could not find section: {}", current_section_name.as_str());
                return;
            };

            self.sections.remove(section_name);
        }

        pub fn remove_task(&mut self, task_name: &'a str) {
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

            if !section.tasks.contains_key(task_name) {
                log::error!("Could not find task: {task_name}");
                return;
            };

            section.tasks.remove(task_name);
        }

        pub fn get_tasks(&'a self) -> Option<&HashMap<&'a str, Task>> {
            let current_section_name = match &self.current_section {
                Some(v) => v,
                None => {
                    log::error!("Please select a section");
                    return None;
                }
            };

            let section = match self.sections.get(current_section_name.as_str()) {
                Some(v) => v,
                None => {
                    log::error!("Could not find section: {}", current_section_name.as_str());
                    return None;
                }
            };

            Some(&section.tasks)
        }
    }
}