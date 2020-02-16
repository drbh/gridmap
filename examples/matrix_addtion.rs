use gridmap::{GridMap, GridMapCell};

use std::time::Instant;

fn main() {
    // lets init a grid - columns will overflow after 3
    let mut grid = GridMap::custom_sqr(3);

    grid.insert::<f32>(1, 1, 11.0);
    grid.insert::<f32>(2, 1, 12.0);
    grid.insert::<f32>(3, 1, 13.0);

    grid.insert::<f32>(1, 2, 14.0);
    grid.insert::<f32>(2, 2, 15.0);
    grid.insert::<f32>(3, 2, 16.0);

    grid.insert::<f32>(1, 3, 17.0);
    grid.insert::<f32>(2, 3, 18.0);
    grid.insert::<f32>(3, 3, 19.0);

    let start = Instant::now();
    let res: f32 = grid.get_range_values::<f32>((1, 1), (3, 3)).sum();
    let elapsed = start.elapsed();
    println!("Millis: {:#?}", elapsed);
}
