# GridMap

<img src="https://giphy.com/gifs/loop-vaporwave-oSYflamt3IEjm/fullscreen" width="300px"/>

GridMap is a small, pragmatic and simple data structure for storing data in a X, Y grid.

A GridMap is a two key table that represents data in a grid. Grids are useful because you can query them for longitudinal and laterally meaning you can ask for row, columns and even specific X, Y coordinate values.

```rust
use gridmap::GridMap;
let mut grid = GridMap::new();
```

## Store
```rust
// we'll insert a float at x=2 y=2
grid.insert::<f32>(2, 2, 1253785.32131);
```
 
## Access
```rust 
// we'll access all the existing float values in the 
// range x=1 y=1 to x=3 y=3
let res: Vec<f32> = grid.get_range_values::<f32>(
	(1, 1), 
	(3, 3)
);
``` 

# Advantages

Grids are useful in a handful of situations. You can reduce graphs to grids. Represent data over coordinates, fast and memory efficent access to bi-key'd data. 


## Under the hood

GripMap represents the table as a sparse 1d array. Ensuring that we only store data of populated cells and no resources are wasted on empty cells. This allows us to handle really long tables - or large gaps between populate cells in a memory efficient way.

Cells have x, y coordinates, an index in the 1d array a cell type and hold either a numerical, boolean or string value. You can read, write and query the grid. All of the queries are converted to indexes in the 1d array or if the table is sufficently small then we iterate over the known cells.


# Roadmap
1. Optimize range calls
2. Nearest neighbors
3. Masking input
4. Simplify row, col API

