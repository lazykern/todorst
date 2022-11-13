use todorst::{
    rest::body::{CreateProjectBody, CreateTaskBody},
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

    let todorst = Todorst::new(&args.token);
    let client = todorst.rest_api();

    pause("List all sections");
    let projects = client.get_projects().await.unwrap();

    for project in projects {
        println!("Project: {}", project.get().name);
        let sections = client.get_sections(&project.get().id).await.unwrap();
        if sections.is_empty() {
            println!("  No sections");
        } else {
            for section in sections {
                println!("  Section: {}", section.get().name);
            }
        }
    }

    pause("Create a section in the example project");

    let example_project = client
        .create_project(CreateProjectBody::new("example project (sections)"))
        .await
        .unwrap();

    println!("{:#?}", example_project);

    let example_section = example_project
        .create_section("example section")
        .await
        .unwrap();

    pause("List all sections again");

    let projects = client.get_projects().await.unwrap();

    for project in projects {
        println!("Project: {}", project.get().name);
        let sections = client.get_sections(&project.get().id).await.unwrap();
        if sections.is_empty() {
            println!("  No sections");
        } else {
            for section in sections {
                println!("  Section: {}", section.get().name);
            }
        }
    }

    pause("Create a task in the example section");
    example_section
        .create_task(CreateTaskBody::new("example task"))
        .await
        .unwrap();

    pause("Delete the example section");
    example_section.delete().await.unwrap();

    pause("Delete the example project");
    example_project.delete().await.unwrap();
}
