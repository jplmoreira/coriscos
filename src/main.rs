use std::time::Instant;

use component::caster::Caster;

mod component;
mod geometry;
mod material;
mod math;

fn main() {
    let pixel_samples = 1000;
    let max_depth = 50;

    println!("start time - {:?}", chrono::offset::Local::now());

    let now = Instant::now();
    let caster = Caster::build(pixel_samples, max_depth);

    println!("build - {}s", now.elapsed().as_millis());

    let now = Instant::now();
    caster.run("image.png");

    println!("run - {}s", now.elapsed().as_secs());
}
