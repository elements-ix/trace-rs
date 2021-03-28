# trace.rs

This is a rust implementation of a ray tracer based off of the book
[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

The accompanying blog post can be found [here](https://blog.slsniff.com/posts/path-tracer-in-rust-part-1/).

## Usage

To generate a scene of random spheres, simply clone this repository, `cd trace-rs`, and run `cargo build --release` and `cargo run --release`. An image should be available in the `images` directory. Warning: it may take a little while :)