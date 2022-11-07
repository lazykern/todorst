use todorst::Todorst;

#[tokio::main]
async fn main() {
    let todorst = Todorst::new("619ed3f4ce444fede8fa73646416a9c5d1730fbb");
    let tasks = todorst.get_tasks().await.unwrap();
    println!("{:?}", tasks);
}
