use todorst::{
    rest::{
        body::{CreateProjectBody, UpdateProjectBody},
        Color,
    },
    Todorst,
};

use clap::Parser;

use std::io::{stdin, stdout, Read, Write};

fn pause(message: &str) {
    let mut stdout = stdout();
    stdout.write(b"\n").unwrap();
    stdout.write(message.as_bytes()).unwrap();
    stdout.write(b"\npress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

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

    pause("Next step: get all projects");

    let projects = todorst_rest.get_projects().await.unwrap();
    println!("{:#?}", projects);

    pause("Next step: create a project");

    let example_project = todorst_rest
        .create_project(CreateProjectBody::new("example project").with_color(Color::BerryRed))
        .await
        .unwrap();
    println!("{:#?}", example_project);

    pause("Next step: update the project");

    let body = UpdateProjectBody::from(example_project.get())
        .with_color(Color::Blue)
        .with_name("updated example project");
    let updated_project = example_project.update(body).await.unwrap();
    println!("{:#?}", updated_project);

    pause("Next step: delete the project");

    updated_project.delete().await.unwrap();
}
