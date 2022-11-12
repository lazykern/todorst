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

    let projects = todorst_rest.get_projects().await.unwrap();
    println!("{:#?}", projects);
}
