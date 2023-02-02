extern crate self as shin;

mod asset;
// mod camera;
mod adv;
mod audio;
mod fps_counter;
mod input;
mod layer;
mod render;
mod time;
mod update;
mod window;

fn main() {
    // old_main()
    pollster::block_on(window::run());
}
