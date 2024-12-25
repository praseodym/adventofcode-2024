use crate::point::Point;
use std::ops::{Index, IndexMut};

pub fn parse_char_grid(input: &str) -> (Vec<u8>, usize) {
    let grid = Grid::parse_square(input);
    (grid.data, grid.width)
}

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: std::clone::Clone> Grid<T> {
    pub fn new(default: T, width: usize, height: usize) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn point_from_index(&self, index: usize) -> Point {
        Point {
            x: (index % self.width) as i32,
            y: (index / self.width) as i32,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0
            && (point.x as usize) < self.width
            && point.y >= 0
            && (point.y as usize) < self.height
    }

    pub fn position<P>(&self, predicate: P) -> Option<Point>
    where
        P: FnMut(&T) -> bool,
    {
        self.data
            .iter()
            .position(predicate)
            .map(|i| self.point_from_index(i))
    }
}

impl Grid<u8> {
    pub fn parse_nonsquare(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut data = Vec::with_capacity(width * height);
        input.lines().for_each(|line| {
            data.extend_from_slice(line.as_bytes());
        });
        Self {
            data,
            width,
            height,
        }
    }

    pub fn parse_square(input: &str) -> Self {
        let grid = Self::parse_nonsquare(input);
        assert_eq!(grid.width, grid.height, "input must be square");
        grid
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[y * self.width + x] as char);
            }
            println!();
        }
    }
}

impl Grid<bool> {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    if self.data[y * self.height + x] {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.y as usize * self.width + index.x as usize]
    }
}
