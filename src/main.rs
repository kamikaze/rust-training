mod functions;
mod guessing_game;
mod helloworld;
mod ownership;
mod references;
mod slices;
mod variables;

fn main() {
    helloworld::run();
    guessing_game::run();
    variables::run();
    functions::run();
    ownership::run();
    references::run();
    slices::run();
}
