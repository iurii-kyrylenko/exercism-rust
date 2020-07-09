use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Ord)]

// A layer represents the distance between the cell and the nearest edge of the matrix:
//
//        0   1   2   3   4   5
//      ┌───┬───┬───┬───┬───┬───┐
//   0  │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │
//      ├───┼───┼───┼───┼───┼───┤
//   1  │ 0 │ 1 │ 1 │ 1 │ 1 │ 0 │
//      ├───┼───┼───┼───┼───┼───┤
//   2  │ 0 │ 1 │ 2 │ 2 │ 1 │ 0 │
//      ├───┼───┼───┼───┼───┼───┤
//   3  │ 0 │ 1 │ 2 │ 2 │ 1 │ 0 │
//      ├───┼───┼───┼───┼───┼───┤
//   4  │ 0 │ 1 │ 1 │ 1 │ 1 │ 0 │
//      ├───┼───┼───┼───┼───┼───┤
//   5  │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │
//      └───┴───┴───┴───┴───┴───┘

struct Cell {
    row: usize,
    column: usize,
    layer: usize,
}

// This allows to sort a cell collection in the "spiral" order
//
impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.layer.partial_cmp(&other.layer) {
            Some(Ordering::Equal) => match (self.column >= self.row, other.column >= other.row) {
                (true, true) => {
                    if self.column == other.column {
                        self.row.partial_cmp(&other.row)
                    } else {
                        self.column.partial_cmp(&other.column)
                    }
                }
                (false, false) => {
                    if self.column == other.column {
                        other.row.partial_cmp(&self.row)
                    } else {
                        other.column.partial_cmp(&self.column)
                    }
                }
                (true, false) => Some(Ordering::Less),
                (false, true) => Some(Ordering::Greater),
            },
            layer_ordering => layer_ordering,
        }
    }
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }

    let half_size = size / 2;

    let to_layer = |i: usize| if i < half_size { i } else { size - i - 1 };

    // Create a cell collection
    let mut cells: Vec<Cell> = (0..size * size)
        .map(|i| {
            let row = i / size;
            let column = i % size;
            let layer = to_layer(row).min(to_layer(column));

            Cell { row, column, layer }
        })
        .collect();

    // Sort the cells in the spiral order
    cells.sort();

    // Enumerate and sort in usual linear order
    let mut ordered_cells: Vec<(usize, &Cell)> = cells.iter().enumerate().collect();
    ordered_cells.sort_by_key(|(_, cell)| cell.row * size + cell.column);

    // Return as 2D collection of numbers
    ordered_cells
        .iter()
        .map(|(i, _)| *i as u32 + 1)
        .collect::<Vec<u32>>()
        .chunks(size)
        .map(|chunk| chunk.to_vec())
        .collect()
}
