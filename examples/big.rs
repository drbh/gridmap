use gridmap::{GridMap, GridMapCell};
use std::time::Instant;

fn main() {
    let mut grid = GridMap::new();

    let start = Instant::now();
    for x in 0..1_000 {
        for y in 0..1_000 {
            grid.insert::<f32>(x, y, 19.0);
        }
    }
    let elapsed = start.elapsed();
    println!("Millis: {:#?}", elapsed);

    let start = Instant::now();
    let res: Vec<f32> = grid.get_range_values::<f32>((1, 1), (3, 3));
    let elapsed = start.elapsed();
    println!("Millis: {:#?}", elapsed);

    let start = Instant::now();
    let res: Vec<f32> = grid.get_range_values::<f32>((1, 1), (30000, 30000));
    let elapsed = start.elapsed();
    println!("Millis: {:#?}", elapsed);

    let start = Instant::now();
    let res: Vec<Option<&GridMapCell>> = grid.get_range((1, 1), (3, 3));
    let elapsed = start.elapsed();
    println!("Millis: {:#?}", elapsed);

    let start = Instant::now();
    let res: Vec<Option<&GridMapCell>> = grid.get_range((1, 1), (30000, 30000));
    let elapsed = start.elapsed();
    println!("Millis: {:#?}", elapsed);
}
