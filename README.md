# todorst

A maybe asynchronous rust wrapper for Todoist REST and Sync API.

## Usage

### REST

```rust
use todorst::rest::{body::CreateProjectBody, models::Color};
use todorst::Todorst;

#[tokio::main]
async fn main() {
    // Create an client
    let todorst = Todorst::new("API TOKEN");
    let todorst_rest = todorst.rest_api();

    // Get all tasks
    let tasks = todorst_rest.get_tasks().await.unwrap();

    // Create a project with red color
    let body = CreateProjectBody::new("Test project").with_color(Color::Red);

    let created_project = todorst_rest.crate_project(body).await.unwrap();
}
```
