use gridmap::GridMap;

fn main() {
    let mut grid = GridMap::new();

    for i in 0..100 {
        grid.insert::<String>(1, i, format!("A{}", i));
    }

    let res = grid.get_range_values::<String>((1, 1), (1, 100));
    println!("{:#?}", res);

    println!("{:#?}", grid);
}
