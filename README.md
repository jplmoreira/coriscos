# Coriscos

A ray-tracer created in Rust using the book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

This project is strictly for learning more Rust.

## Usage

Copy the `example.coriscos.toml` file to `coriscos.toml`. If you wish to use a different file name/directory, define the environment variable `CORISCOS_CONFIG`.

Build and run the binary.

## Changes & Performance

The used scene was the last one from the book with `image_width = 1200`, `pixel_samples = 1000` and `max_ray_depth = 50`

* 1 - Single thread [~4 hours]
* 2 - Parallelized ray casting and pixel sampling [~37 minutes]
* 3 - Parallelized hit calculation for scene objects [2 hours] - ROLLED BACK
* 4 - Diffuse Light implementation [~41 minutes]
* 5 - Multiple optimizations (including using `release`) [~7 minutes]
* 6 - Use custom work stealing thread pool (removed `rayon`) [~8 minutes]

### Custom thread pool notes

The custom thread pool uses `crossbeam` to create a work stealing scheduler.

Created a custom `Future` struct that completes when a worker thread calculates a casted ray final color. This is used in a nested `Stream` that is buffered and is blocked using `futures::executor`.

Each worker thread will complete a ray cast until it's maximum depth. Also tested putting each ray bounce in the work queue, but it lead to a worse performance.

The performance isn't as good as when using `rayon` but it is comparable, will keep this implementation for possible future optimizations.
