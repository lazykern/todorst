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
    let tasks = todorst.get_tasks().await.unwrap();
    println!("{:?}", tasks);
}
