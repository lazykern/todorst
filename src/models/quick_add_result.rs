use crate::models::task::Task;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct QuickAddResult {
    task: Option<Task>,
    resolved_project_name: Option<String>,
    resolved_assignee_name: Option<String>,
    resolved_label_names: Option<Vec<String>>,
    resolvved_section_name: Option<String>,
}
