use crate::constant::*;
use url::Url;

pub fn rest_base_url() -> Url {
    Url::parse(&format!("{}/rest/{}", BASE_URL, REST_VERSION)).unwrap()
}

pub fn rest_url(endpoint: &str) -> Url {
    let mut url = rest_base_url();
    url.path_segments_mut().unwrap().push(endpoint);
    url
}

pub fn rest_projects_url() -> Url {
    rest_url(PROJECTS_ENDPOINT)
}

pub fn rest_project_url(project_id: &str) -> Url {
    let mut url = rest_projects_url();
    url.path_segments_mut()
        .unwrap()
        .push(project_id.to_string().as_str());
    url
}

pub fn rest_collaborators_url(project_id: &str) -> Url {
    let mut url = rest_project_url(project_id);
    url.path_segments_mut()
        .unwrap()
        .push(COLLABORATORS_ENDPOINT);
    url
}

pub fn rest_sections_url() -> Url {
    rest_url(SECTIONS_ENDPOINT)
}

// pub fn rest_project_sections_url(project_id: &str) -> Url {
//     let mut url = rest_project_url(project_id);
//     url.query_pairs_mut().append_pair("project_id", project_id);
//     url
// }

pub fn rest_section_url(section_id: &str) -> Url {
    let mut url = rest_sections_url();
    url.path_segments_mut().unwrap().push(section_id);
    url
}

pub fn rest_tasks_url() -> Url {
    rest_url(TASKS_ENDPOINT)
}

pub fn rest_task_url(task_id: &str) -> Url {
    let mut url = rest_tasks_url();
    url.path_segments_mut().unwrap().push(task_id);
    url
}

pub fn rest_task_close_url(task_id: &str) -> Url {
    let mut url = rest_task_url(task_id);
    url.path_segments_mut().unwrap().push("close");
    url
}

pub fn rest_task_reopen_url(task_id: &str) -> Url {
    let mut url = rest_task_url(task_id);
    url.path_segments_mut().unwrap().push("reopen");
    url
}

pub fn rest_comments_url() -> Url {
    rest_url(COMMENTS_ENDPOINT)
}

pub fn rest_task_comments_url(task_id: &str) -> Url {
    let mut url = rest_task_url(task_id);
    url.query_pairs_mut().append_pair("task_id", task_id);
    url
}

pub fn rest_comment_url(comment_id: &str) -> Url {
    let mut url = rest_comments_url();
    url.path_segments_mut().unwrap().push(comment_id);
    url
}

pub fn rest_labels_url() -> Url {
    rest_url(LABELS_ENDPOINT)
}

pub fn rest_label_url(label_id: &str) -> Url {
    let mut url = rest_labels_url();
    url.path_segments_mut().unwrap().push(label_id);
    url
}

pub fn rest_shared_labels_url() -> Url {
    rest_url(SHARED_LABELS_ENDPOINT)
}

pub fn rest_shared_labels_rename_url() -> Url {
    let mut url = rest_shared_labels_url();
    url.path_segments_mut().unwrap().push("rename");
    url
}

pub fn rest_shared_labels_remove_url() -> Url {
    let mut url = rest_shared_labels_url();
    url.path_segments_mut().unwrap().push("remove");
    url
}
