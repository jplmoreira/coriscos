mod caster;
mod component;
mod geometry;
mod material;
mod math;
mod settings;

fn main() {
    let caster = caster::Caster::build().unwrap();

    caster.run();
}
