mod core;
use core::gust::Gust;
use winit::{
    event_loop::{EventLoop}
};

fn main() {
    let event_loop = EventLoop::new();
    let mut gust = Gust::new();
    gust.run(event_loop);
}
