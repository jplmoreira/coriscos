use std::time::Instant;

mod caster;
mod component;
mod geometry;
mod material;
mod math;
mod settings;

fn main() {
    println!("start time - {:?}", chrono::offset::Local::now());

    let now = Instant::now();
    let caster = caster::Caster::build().unwrap();

    println!("build duration - {}ms", now.elapsed().as_millis());

    let now = Instant::now();
    caster.run();

    let elapsed_secs = now.elapsed().as_secs();
    let seconds = elapsed_secs % 60;
    let minutes = (elapsed_secs / 60) % 60;
    let hours = (elapsed_secs / 60) / 60;
    println!("run duration - {hours}h:{minutes}m:{seconds}s");
}
