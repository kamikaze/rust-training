use tokio::runtime::Runtime;

mod functions;
mod guessing_game;
mod helloworld;
mod ownership;
mod references;
mod slices;
mod variables;
mod async_channel;


fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async { async_channel::run().await.unwrap(); });

    // helloworld::run();
    // guessing_game::run();
    // variables::run();
    // functions::run();
    // ownership::run();
    // references::run();
    // slices::run();
}
