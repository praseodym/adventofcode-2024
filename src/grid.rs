pub fn parse_char_grid(input: &str) -> (Vec<u8>, usize) {
    let len = input.lines().next().unwrap().len();
    assert_eq!(len, input.lines().count(), "input must be square");
    let mut grid = Vec::with_capacity(len * len);
    input.lines().for_each(|line| {
        grid.extend_from_slice(line.as_bytes());
    });
    (grid, len)
}

pub fn parse_char_grid_nonsquare(input: &str) -> (Vec<u8>, (usize, usize)) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid = Vec::with_capacity(width * height);
    input.lines().for_each(|line| {
        grid.extend_from_slice(line.as_bytes());
    });
    (grid, (width, height))
}
