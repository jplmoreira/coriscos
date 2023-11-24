use std::time::Instant;

use component::caster::Caster;

mod component;
mod geometry;
mod material;
mod math;

fn main() {
    let pixel_samples = 10;
    let max_depth = 10;

    let caster = Caster::build(pixel_samples, max_depth);

    let now = Instant::now();

    caster.run("image.png");

    println!("{}s", now.elapsed().as_secs());
}
