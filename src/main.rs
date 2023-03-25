use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::time::Instant;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}
struct Particle {
    height: f64,
    widht: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl World {
    fn new(width: f64, height: f64) -> World {
        unimplemented!()
    }
    fn add_shapes(&mut self, n: i32) {
        unimplemented!()
    }
    fn remove_shapes() {
        unimplemented!()
    }
    fn update(&self) {
        unimplemented!()
    }
}
impl Particle {
    fn new() -> Particle {
        unimplemented!()
    }
    fn update() {
        unimplemented!()
    }
}
fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.widht, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
