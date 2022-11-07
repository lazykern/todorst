use crate::models::task::Task;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct QuickAddResult {
    pub task: Option<Task>,

    pub resolved_project_name: Option<String>,

    pub resolved_assignee_name: Option<String>,

    pub resolved_label_names: Option<Vec<String>>,

    pub resolvved_section_name: Option<String>,
}
