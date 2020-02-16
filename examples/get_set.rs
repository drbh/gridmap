use gridmap::GridMap;

fn main() {
    let mut grid = GridMap::new();

    grid.insert::<String>(1, 1, String::from("A1"));
    grid.insert::<String>(2, 1, String::from("B1"));
    grid.insert::<String>(3, 1, String::from("C1"));

    grid.insert::<String>(1, 2, String::from("A2"));
    grid.insert::<String>(2, 2, String::from("B2"));
    grid.insert::<String>(3, 2, String::from("C2"));

    grid.insert::<String>(1, 3, String::from("A3"));
    grid.insert::<String>(2, 3, String::from("B3"));
    grid.insert::<String>(3, 3, String::from("C3"));

    let res = grid.get((3, 3));
    println!("{:#?}", res);

    let res = grid.get_value::<String>((3, 3));
    println!("{:#?}", res);
}
