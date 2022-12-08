use std::fs;
use std::env;
use std::fmt;
use std::ops::{Index, IndexMut};

type Coord = (usize, usize);

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> where T: Clone {
    fn new_default(val: T, width: usize, height: usize) -> Self {
        Self {
            data: vec![val; width*height],
            width,
            height
        }
    }

    fn col_at(&self, x: usize) -> Vec<&T> {
        self.data.iter().skip(x).step_by(self.width).collect()
    }

    fn row_at(&self, y: usize) -> Vec<&T> {
        self.data[y*self.width..y*self.width+self.width].iter().collect()
    }
}

impl Grid<i32> {
    fn new(input: &str) -> Self {
        let data_raw = input.lines().collect::<Vec<&str>>();
        let width = data_raw[0].len();
        let height = data_raw.len();
        let data = data_raw.iter()
                            .map(|row| row.chars()
                                         .filter_map(|c| c.to_digit(10))
                                         .map(|i| i as i32)
                                         .collect::<Vec<_>>())
            .flatten().collect::<Vec<_>>();
        Self {
            data,
            width,
            height
        }
    }

    fn get_scenic_score(&self, coord: Coord) -> usize {
        let col = self.col_at(coord.0);
        let row = self.row_at(coord.1);        
        let height = self[coord];

        //println!("height: {}", height);
        let to_right = if let Some((n, _)) = row[coord.0..].iter().skip(1).enumerate().find(|(_, &h)| *h >= height) {
            n + 1
        }
        else {
            row[coord.0..].iter().skip(1).len()
        };

        let to_left = if let Some((n, _)) = row[0..coord.0].iter().rev().enumerate().find(|(_, &h)| *h >= height) {
            n + 1
        }
        else {
            row[0..coord.0].iter().len()
        };



        let to_down = if let Some((n, _)) = col[coord.1..].iter().skip(1).enumerate().find(|(_, &h)| *h >= height) {
            n + 1
        }
        else {
            row[coord.1..].iter().skip(1).len()
        };

        let to_up = if let Some((n, _)) = col[0..coord.1].iter().rev().enumerate().find(|(_, &h)| *h >= height) {
            n + 1
        }
        else {
            row[0..coord.1].iter().len()
        };

        //println!("Score for {:?} = {} * {} * {} * {}", coord, to_up, to_left, to_down, to_right);

        to_up * to_left * to_down * to_right
    }
}


impl<T> Index<Coord> for Grid<T> {
    type Output = T;
    fn index(&self, coord: Coord) -> &T {
        assert!(coord.0 < self.width && coord.1 < self.height);
        &self.data[coord.0 + coord.1 * self.width]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut T {
        assert!(coord.0 < self.width && coord.1 < self.height);
        &mut self.data[coord.0 + coord.1 * self.width]
    }
}

impl fmt::Debug for Grid<i32>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data
            .chunks(self.width)
            .try_for_each(|row| {
                write!( f, "{}\n", row.iter()
                                    .map(|c| format!("[{:^5}]", c))
                                    .collect::<Vec<String>>()
                                    .join("")
                )
            })
    }
}

impl fmt::Debug for Grid<bool>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data
            .chunks(self.width)
            .try_for_each(|row| {
                write!( f, "{}\n", row.iter()
                                    .map(|&c| if c { "🌲".to_string() } else { "⬛".to_string() })
                                    .collect::<Vec<String>>()
                                    .join("")
                )
            })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let input = fs::read_to_string(&args[1]).unwrap();

    let tree_grid = Grid::new(input.as_str());
    let mut visibility: Grid<bool> = Grid::new_default(false, tree_grid.width, tree_grid.height);
    println!("{:?}", tree_grid);

    println!("{:?}", visibility);


    for x in 0..tree_grid.width {
        let col = tree_grid.col_at(x);
        // top to bottom
        let mut max: i32 = -1;
        col.iter().enumerate().for_each(|(y, &height)| {
            if *height > max { 
                visibility[(x,y)] = true;
                max = *height;
            }
        });
        // bottom to top
        max = -1;
        col.iter().rev().enumerate().for_each(|(y, &height)| {
            if *height > max { 
                visibility[(x,tree_grid.height-y-1)] = true;
                max = *height;
            }
        });
    }

    for y in 0..tree_grid.height {
        let row = tree_grid.row_at(y);
        let mut max: i32 = -1;
        row.iter().enumerate().for_each(|(x, &height)| {
            if *height > max { 
                visibility[(x,y)] = true;
                max = *height;
            }
        });
        max = -1;
        row.iter().rev().enumerate().for_each(|(x, &height)| {
            if *height > max { 
                visibility[(tree_grid.width-1 - x,y)] = true;
                max = *height;
            }
        });
    }

    println!("{:?}", visibility);
    println!("{}", visibility.data.iter().map(|t| *t as u32).sum::<u32>());

    let mut scenic_score: Grid<i32> = Grid::new_default(1, tree_grid.width, tree_grid.height);


    for x in 0..tree_grid.width {
        for y in 0..tree_grid.height {
            scenic_score[(x, y)] = tree_grid.get_scenic_score((x,y)) as i32;
        }
    }
    
    //for x in 0..tree_grid.width {
    //    let tallest_coords = visibility.col_at(x)
    //        .iter()
    //        .enumerate()
    //        .filter(|&(_, &val)| *val)
    //        .map(|(y, _)| (x, y))
    //        .collect::<Vec<Coord>>();

    //    tallest_coords.windows(2).for_each(|coords| {
    //        let diff_y = (coords[1].1 - coords[0].1) as i32;
    //        scenic_score[coords[0]] *= diff_y;
    //        scenic_score[coords[1]] *= diff_y;
    //    })
    //}

    //for y in 0..tree_grid.height {
    //    let tallest_coords = visibility.row_at(y)
    //        .iter()
    //        .enumerate()
    //        .filter(|&(_, &val)| *val)
    //        .map(|(x, _)| (x, y))
    //        .collect::<Vec<Coord>>();

    //    tallest_coords.windows(2).for_each(|coords| {
    //        let diff_x = (coords[1].0 - coords[0].0) as i32;
    //        scenic_score[coords[0]] *= diff_x;
    //        scenic_score[coords[1]] *= diff_x;
    //    })
    //}
    println!("{:?}", scenic_score.data.iter().max().unwrap());
}
