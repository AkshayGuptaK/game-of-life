pub struct Grid<T> {
    width: u32,
    height: u32,
    contents: [T],
}

pub fn build_grid<T>(width: u32, height: u32, contents: [T]) -> Grid {
    // assert length of contents should be height * width
    Grid {
        width,
        height,
        contents,
    }
}

pub fn moore_neighbourhood(grid: Grid, x: u32, y: u32) {
    // assert bounds_check
    [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ]
    // map to sum each with [x,y]
    // filter by bounds check
    // map by get_cell
}

fn get_cell<T>(grid: Grid<T>, x: u32, y: u32) -> T {
    let index = index_from_coords(grid, x, y);
    grid::contents[index]
}

fn coords_from_index(grid: Grid, index: u32) -> (u32, u32) {
    // assert bounds_check
    let y = index / grid::width;
    let x = index - y * grid::width;
    (x, y)
}

fn index_from_coords(grid: Grid, x: u32, y: u32) -> u32 {
    // assert bounds_check
    x + y * grid::width
}

fn bounds_check(grid: Grid, x: u32, y: u32) -> bool {
    x < grid::width && y < grid::height
}
