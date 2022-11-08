use todorst::rest::body;
use todorst::rest::models::Color;
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

    let body = body::CreateProjectBody::new("Test project")
        .with_color(Color::Red)
        .with_is_favorite(true);

    //// equivalent statement:
    // let mut body = body::CreateProjectBody::new("Test project");
    // body.set_color(Color::Red);
    // body.set_is_favorite(true);

    let mut project = todorst_rest.crate_project(&body).await.unwrap();

    println!("Project Structure:\n{:#?}", project);

    project.name = "Test project 2".to_string();
}
