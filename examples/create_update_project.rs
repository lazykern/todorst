use todorst::rest::{
    body::{CreateProjectBody, UpdateProjectBody},
    models::Color,
};
use todorst::Todorst;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    token: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let todorst = Todorst::new(args.token.as_str());
    let todorst_rest = todorst.rest_api();

    let body = CreateProjectBody::new("todorst project")
        .with_color(Color::Red)
        .with_is_favorite(true);

    let mut project = todorst_rest.crate_project(&body).await.unwrap();

    println!("Project Structure:\n{:#?}", project);

    project.name = "Test project 2".to_string();

    let updated_project = todorst_rest
        .update_project(project.id.as_str(), &UpdateProjectBody::from(&project))
        .await
        .unwrap();

    println!("Updated Project Structure:\n{:#?}", updated_project);
}
