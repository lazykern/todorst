use url;

pub static BASE_URL: &'static str = "https://api.todoist.com";
pub static AUTH_BASE_URL: &'static str = "https://todoist.com";
pub static SYNC_VERSION: &'static str = "v9";
pub static REST_VERSION: &'static str = "v2";

pub fn sync_url() -> url::Url {
    url::Url::parse(&format!("{}/sync/{}", BASE_URL, SYNC_VERSION)).unwrap()
}

pub fn rest_url() -> url::Url {
    url::Url::parse(&format!("{}/rest/{}", BASE_URL, REST_VERSION)).unwrap()
}

pub static TASKS_ENDPOINT: &'static str = "tasks";
pub static COLLABORATORS_ENDPOINT: &'static str = "collaborators";
pub static SECTIONS_ENDPOINT: &'static str = "sections";
pub static COMPOMENTS_ENDPOINT: &'static str = "components";
pub static LABELS_ENDPOINT: &'static str = "labels";
pub static SHARED_LABELS_ENDPOINT: &'static str = "shared_labels";
pub static SHARED_LABELS_RENAME_ENDPOINT: &'static str = "shared_labels/rename";
pub static SHARED_LABELS_REMOVE_ENDPOINT: &'static str = "shared_labels/remove";
pub static QUICK_ADD_ENDPOINT: &'static str = "quick/add";

pub static AUTHORIZE_ENDPOINT: &'static str = "oauth/authorize";
pub static ACCESS_TOKEN_ENDPOINT: &'static str = "oauth/access_token";
pub static REVOKE_TOKEN_ENDPOINT: &'static str = "access_tokens/revoke";

pub fn get_rest_url(endpoint: &str) -> url::Url {
    let mut url = rest_url();
    url.path_segments_mut().unwrap().push(endpoint);
    url
}

pub fn get_sync_url(endpoint: &str) -> url::Url {
    let mut url = sync_url();
    url.path_segments_mut().unwrap().push(endpoint);
    url
}

pub fn get_auth_url() -> url::Url {
    let mut url = url::Url::parse(AUTH_BASE_URL).unwrap();
    url.path_segments_mut().unwrap().push(AUTHORIZE_ENDPOINT);
    url
}
