mod state;
fn main() {
    pollster::block_on(run())
}

pub async fn run() {
    env_logger::init();
}
