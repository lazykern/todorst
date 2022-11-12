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
    let token = args.token.as_str();

    let todorst = Todorst::new(token);
    let todorst_rest = todorst.rest_api();

    let body = CreateProjectBody::new("Old project")
        .with_color(Color::Red)
        .with_is_favorite(true);

    let project = todorst_rest.create_project(body).await.unwrap();

    println!("Project Structure:\n{:#?}", project);

    let updated = project
        .update(UpdateProjectBody::new().with_name("Updated project"))
        .await
        .unwrap();

    println!("Updated Project Structure:\n{:#?}", updated);
}
