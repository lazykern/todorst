use todorst::{rest::body::CreateTaskBody, Todorst};

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

    pause("Next step: get all tasks");

    println!("{:#?}", todorst_rest.get_tasks().await.unwrap());

    pause("Next step: create today task");

    let example_task = todorst_rest
        .create_task(
            CreateTaskBody::new("example task")
                .with_description("this is an example task")
                .with_due_string("today"),
        )
        .await
        .unwrap();
    println!("{:#?}", example_task);

    pause("Next step: delete the task");
    println!("{:#?}", example_task.delete().await.unwrap());
}
