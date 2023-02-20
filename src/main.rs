mod process;

#[tokio::main]
async fn main() {
    let routes = process::filters::processbrowser();
    warp::serve(routes).run(([127, 0, 0, 1], 3001)).await;
}
