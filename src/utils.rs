use crate::endpoints::sync_url;

pub fn get_task_url(task_id: u64, sync_id: &str) -> url::Url {
    let mut url = sync_url();

    url.path_segments_mut().unwrap().push("showTask");

    if sync_id != "" {
        url.query_pairs_mut().append_pair("sync_id", sync_id);
    }

    url.query_pairs_mut()
        .append_pair("id", &task_id.to_string());

    url
}
