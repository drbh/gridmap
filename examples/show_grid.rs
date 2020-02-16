use gridmap::GridMap;

fn main() {
    // lets init a grid - columns will overflow after 4
    let mut grid = GridMap::custom_sqr(100);

    // column, row - x, y
    grid.insert::<String>(1, 1, String::from("A1"));
    grid.insert::<String>(5, 5, String::from("E5"));
    grid.insert::<String>(10, 10, String::from("J10"));
    grid.insert::<String>(10, 1, String::from("J1"));
    grid.insert::<String>(1, 10, String::from("A10"));

    grid.show();
}
