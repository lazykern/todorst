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
    let rest_api = todorst.rest_api();
    let tasks = rest_api.get_tasks().await.unwrap();
    println!("{:?}", tasks);
}
