mod asset;
// mod camera;
mod interpolator;
mod layer;
mod render;
mod vm;

fn main() {
    // old_main()
    pollster::block_on(render::run());
}