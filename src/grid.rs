use std::ops::{Index, IndexMut};

type Point = (i64, i64);

#[derive(Clone, Debug)]
pub struct Grid<T> {
    items: Vec<T>,
    width: i64,
    height: i64,
}

impl<T> Grid<T> {
    pub fn new(grid : Vec<Vec<T>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Grid {
            items: grid.into_iter().flatten().collect(),
            width: width as i64,
            height: height as i64
        }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.0 >= 0 && point.0 < self.height && point.1 >= 0 && point.1 < self.width
    }

    pub fn index_iter(&self) -> impl Iterator<Item = Point> {
        (0..self.height).flat_map(|r| (0..self.width).map(move |c| (r, c)))
    }

    pub fn neighbor_indices8(&self, point: Point) -> Vec<Point> {        
        let mut neighbors : Vec<Point> = Vec::new();
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue
                }

                let new = add_points(point, (dr, dc));
                if self.in_bounds(new) {
                    neighbors.push(new);
                }
            }
        }
        neighbors
    }

    pub fn neighbors8(&self, point: Point) -> impl Iterator<Item = &T> {
        self.neighbor_indices8(point).into_iter().map(|p| &self[p])
    }
}

pub fn add_points(point1: Point, point2: Point) -> Point {
    (point1.0 + point2.0, point1.1 + point2.1)
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.items[(index.0 * self.width + index.1) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.items[(index.0 * self.width + index.1) as usize]
    }
}
