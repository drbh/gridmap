use core::hash::{BuildHasherDefault, Hasher};
use std::collections::HashMap;
use std::fmt::Debug;

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

fn coords_to_1d_index(x: usize, y: usize, n: usize) -> usize {
    (y - 1) * n + (x - 1)
}

fn get_cells_in_row(compute_indices: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut inferred_row: Vec<(usize, usize)> = Vec::new();
    for v in (compute_indices[0].1 + 1)..(compute_indices[1].1) {
        inferred_row.push((compute_indices[0].0, v));
    }
    inferred_row
}
fn get_cells_in_col(compute_indices: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut inferred_col: Vec<(usize, usize)> = Vec::new();
    for v in (compute_indices[0].0 + 1)..(compute_indices[1].0) {
        inferred_col.push((v, compute_indices[0].1));
    }
    inferred_col
}

pub fn expand_ranges(mut compute_indices: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    if (compute_indices[0].0 != compute_indices[1].0)
        && (compute_indices[0].1 != compute_indices[1].1)
    {
        let upper_col;
        let lower_col;
        if compute_indices[0].0 > compute_indices[1].0 {
            upper_col = compute_indices[0].0;
            lower_col = compute_indices[1].0;
        } else {
            upper_col = compute_indices[1].0;
            lower_col = compute_indices[0].0;
        }

        let upper_row;
        let lower_row;
        if compute_indices[0].1 > compute_indices[1].1 {
            upper_row = compute_indices[0].1;
            lower_row = compute_indices[1].1;
        } else {
            upper_row = compute_indices[1].1;
            lower_row = compute_indices[0].1;
        }

        // just rebuild all in rectangle
        let mut finalvec = Vec::new();
        for r in lower_row..(upper_row + 1) {
            for c in lower_col..(upper_col + 1) {
                finalvec.push((c, r))
            }
        }
        compute_indices = finalvec;
    } else if compute_indices[0].0 != compute_indices[1].0 {
        let inferred_col = get_cells_in_col(&compute_indices);
        compute_indices.extend(inferred_col)
    } else if compute_indices[0].1 != compute_indices[1].1 {
        let inferred_row = get_cells_in_row(&compute_indices);
        compute_indices.extend(inferred_row)
    }
    compute_indices
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityHasher(usize);

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("IdentityHasher only supports usize key within ncolss")
    }

    fn write_usize(&mut self, i: usize) {
        self.0 = i;
    }
}

type BuildIdentityHasher = BuildHasherDefault<IdentityHasher>;

#[derive(Debug)]
pub struct GridMap {
    pub cells: HashMap<usize, GridMapCell, BuildIdentityHasher>,
    pub ncols: usize,
    pub nrows: usize,
}

#[derive(Debug, Clone)]
pub struct GridMapCell {
    pub x: usize,
    pub y: usize,
    pub i: usize,
    pub data: CellType,
}

#[derive(Debug, Clone)]
pub enum CellType {
    STRING(String),
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    BOOL(bool),
}

impl Into<String> for CellType {
    fn into(self) -> String {
        match self {
            CellType::STRING(z) => z,
            _ => String::new(),
        }
    }
}
impl From<String> for CellType {
    fn from(item: String) -> Self {
        CellType::STRING(item)
    }
}

impl From<f32> for CellType {
    fn from(item: f32) -> Self {
        CellType::F32(item)
    }
}
impl Into<f32> for CellType {
    fn into(self) -> f32 {
        match self {
            CellType::F32(z) => z,
            _ => 0.0,
        }
    }
}

impl From<f64> for CellType {
    fn from(item: f64) -> Self {
        CellType::F64(item)
    }
}
impl Into<f64> for CellType {
    fn into(self) -> f64 {
        match self {
            CellType::F64(z) => z,
            _ => 0.0,
        }
    }
}

impl From<i32> for CellType {
    fn from(item: i32) -> Self {
        CellType::I32(item)
    }
}
impl Into<i32> for CellType {
    fn into(self) -> i32 {
        match self {
            CellType::I32(z) => z,
            _ => 0,
        }
    }
}

impl From<i64> for CellType {
    fn from(item: i64) -> Self {
        CellType::I64(item)
    }
}
impl Into<i64> for CellType {
    fn into(self) -> i64 {
        match self {
            CellType::I64(z) => z,
            _ => 0,
        }
    }
}

impl From<bool> for CellType {
    fn from(item: bool) -> Self {
        CellType::BOOL(item)
    }
}
impl Into<bool> for CellType {
    fn into(self) -> bool {
        match self {
            CellType::BOOL(z) => z,
            _ => false,
        }
    }
}

impl GridMap {
    pub fn new() -> GridMap {
        let largest_square_size_limit_by_max_usize = (std::usize::MAX as f64).sqrt() as usize;
        // println!(
        //     "Init a logical {}x{} graph",
        //     largest_square_size_limit_by_max_usize, largest_square_size_limit_by_max_usize
        // );
        GridMap {
            cells: HashMap::default(),
            ncols: largest_square_size_limit_by_max_usize,
            nrows: largest_square_size_limit_by_max_usize,
        }
    }
    pub fn custom(width: usize, height: usize) -> GridMap {
        GridMap {
            cells: HashMap::default(),
            ncols: width,
            nrows: height,
        }
    }
    pub fn custom_sqr(width: usize) -> GridMap {
        GridMap {
            cells: HashMap::default(),
            ncols: width,
            nrows: width,
        }
    }

    pub fn insert<T>(&mut self, col_val: usize, row_val: usize, value: T) -> bool
    where
        CellType: std::convert::From<T>,
    {
        let cell = GridMapCell {
            x: col_val,
            y: row_val,
            i: coords_to_1d_index(col_val, row_val, self.ncols),
            data: value.into(),
        };
        let index = coords_to_1d_index(col_val, row_val, self.ncols);
        self.cells.insert(index, cell);
        true
    }

    pub fn get(&self, coords: (usize, usize)) -> Option<GridMapCell> {
        let (col_val, row_val) = coords;
        let dex = coords_to_1d_index(col_val, row_val, self.ncols);
        match self.cells.get(&dex) {
            Some(r) => {
                let x = r.clone();
                Some(x)
            }
            None => None,
        }
    }

    pub fn get_value<T>(&self, coords: (usize, usize)) -> Option<T>
    where
        CellType: std::convert::Into<T>,
    {
        let (col_val, row_val) = coords;
        let dex = coords_to_1d_index(col_val, row_val, self.ncols);

        match self.cells.get(&dex) {
            Some(r) => Some(r.data.clone().into()),
            None => None,
        }
    }

    pub fn get_range_values<T>(&self, a: (usize, usize), b: (usize, usize)) -> Vec<T>
    where
        CellType: std::convert::Into<T>,
    {
        let cdiff = (((b.0 + 1) - a.0) as f32).abs(); // col diff
        let rdiff = (((b.1 + 1) - a.1) as f32).abs(); // row diff
        let expected_grid_size = cdiff * rdiff;
        let mut results: Vec<T> = Vec::new();
        match expected_grid_size < (self.cells.keys().len() as f32) {
            // match false {
            true => {
                println!("Expand");
                // for dense matrix
                let ranges = expand_ranges(vec![a, b]);
                for (col_val, row_val) in ranges {
                    let dex = coords_to_1d_index(col_val, row_val, self.ncols);

                    match self.cells.get(&dex) {
                        Some(r) => results.push(r.data.clone().into()),
                        None => (),
                    };
                }
                results
            }
            false => {
                println!("Iterate");
                // for sparse matrix
                for (_k, v) in self.cells.iter() {
                    if ((v.x <= b.0) & (v.y <= b.1)) & ((v.x >= a.0) & (v.y >= a.1)) {
                        results.push(v.data.clone().into())
                    }
                }
                results
            }
        }
    }

    pub fn get_range(&self, a: (usize, usize), b: (usize, usize)) -> Vec<Option<&GridMapCell>> {
        let mut results: Vec<Option<&GridMapCell>> = Vec::new();
        let cdiff = (((b.0 + 1) - a.0) as f32).abs(); // col diff
        let rdiff = (((b.1 + 1) - a.1) as f32).abs(); // row diff
        let expected_grid_size = cdiff * rdiff;
        match expected_grid_size < (self.cells.keys().len() as f32) {
            // match false {
            true => {
                // println!("Expand");
                // for dense matrix
                let ranges = expand_ranges(vec![a, b]);
                for (col_val, row_val) in ranges {
                    let dex = coords_to_1d_index(col_val, row_val, self.ncols);

                    match self.cells.get(&dex) {
                        Some(r) => results.push(Some(r)),
                        None => (),
                    };
                }
                results
            }
            false => {
                // println!("Iterate");
                // for sparse matrix
                for (_k, v) in self.cells.iter() {
                    if ((v.x <= b.0) & (v.y <= b.1)) & ((v.x >= a.0) & (v.y >= a.1)) {
                        // results.push(v.data.clone().into())
                        results.push(Some(v))
                    }
                }
                results
            }
        }
    }

    pub fn get_row_to_end(&self, start: (usize, usize)) -> Vec<Option<&GridMapCell>> {
        let mut results: Vec<Option<&GridMapCell>> = Vec::new();
        for (_k, v) in self.cells.iter() {
            if (v.x >= start.0) & (v.y == start.1) {
                results.push(Some(v))
            }
        }
        results
    }

    pub fn get_row_to_beg(&self, start: (usize, usize)) -> Vec<Option<&GridMapCell>> {
        let mut results: Vec<Option<&GridMapCell>> = Vec::new();
        for (_k, v) in self.cells.iter() {
            if (v.x <= start.0) & (v.y == start.1) {
                results.push(Some(v))
            }
        }
        results
    }

    pub fn get_col_to_end(&self, start: (usize, usize)) -> Vec<Option<&GridMapCell>> {
        let mut results: Vec<Option<&GridMapCell>> = Vec::new();
        for (_k, v) in self.cells.iter() {
            if (v.x == start.0) & (v.y >= start.1) {
                results.push(Some(v))
            }
        }
        results
    }

    pub fn get_col_to_beg(&self, start: (usize, usize)) -> Vec<Option<&GridMapCell>> {
        let mut results: Vec<Option<&GridMapCell>> = Vec::new();
        for (_k, v) in self.cells.iter() {
            if (v.x == start.0) & (v.y <= start.1) {
                results.push(Some(v))
            }
        }
        results
    }

    pub fn show(&self) {
        let mut string = String::new();

        let maxkey = self.cells.keys().max().unwrap();
        let till_row_end = self.ncols - (maxkey % self.ncols);
        let end = till_row_end + maxkey;

        for i in (0..end).step_by(self.ncols) {
            string = format!("{} | {:?}", string, i);
            for j in 0..self.ncols {
                let v = i + j;

                match self.cells.get(&v) {
                    Some(cell) => match &cell.data {
                        CellType::STRING(expr) => print!("{:>4}  ", expr),
                        CellType::F32(expr) => print!("{:>4}  ", expr),
                        CellType::F64(expr) => print!("{:>4}  ", expr),
                        CellType::I32(expr) => print!("{:>4}  ", expr),
                        CellType::I64(expr) => print!("{:>4}  ", expr),
                        CellType::BOOL(expr) => print!("{:>4}  ", expr),
                    },
                    None => print!("{:>4}  ", ""),
                }
            }

            println!("{}", "");
        }
    }
}
