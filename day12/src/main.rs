use std::fs;
use std::env;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::collections::HashSet;

type Coord = (usize, usize);

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
    start: Coord,
    end: Coord
}

impl<T> Grid<T> where T: Clone {
    fn new_default(val: T, width: usize, height: usize) -> Self {
        Self {
            data: vec![val; width*height],
            width,
            height,
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl<T> Grid<T> {
    fn coord_at(&self, i: usize) -> Coord {
        (i%self.width, i/self.width)
    }

    fn index_of(&self, coord: Coord) -> usize {
        coord.0 + coord.1 * self.width
    }


    fn neighours_at(&self, i:usize) -> Vec<usize> {
        let coord_u = self.coord_at(i);    
        let coord = (coord_u.0 as i32, coord_u.1 as i32);

        [(coord.0 - 1, coord.1), (coord.0 + 1, coord.1),
         (coord.0, coord.1 - 1), (coord.0, coord.1 + 1)]
         .iter()
         .filter_map(|coord| {
             if coord.0 >= 0 && coord.0 < self.width as i32 && coord.1 >= 0 && coord.1 < self.height as i32 {
                Some(self.index_of((coord.0 as usize, coord.1 as usize)))
             }
             else {
                 None
             }
         }).collect()
    }

}

impl Grid<i32> {
    fn new(input: &str) -> Self {
        let data_raw = input.lines().collect::<Vec<&str>>();
        let width = data_raw[0].len();
        let height = data_raw.len();

        let mut data = data_raw.iter()
                            .flat_map(|row| row.chars()
                                         .map(|c| c as i32)
                                         .map(|i| i - 'a' as i32)
                                         .collect::<Vec<_>>())
                             .collect::<Vec<_>>();

        let coord_at = |i| -> Coord {
            (i%width, i/width)
        };

        let start_pos = data.iter().position(|&i| i == ('S' as i32 - 'a' as i32)).unwrap();
        data[start_pos] = 0;

        let end_pos = data.iter().position(|&i| i == ('E' as i32 - 'a' as i32)).unwrap();
        data[end_pos] = 'z' as i32  - 'a' as i32;

        let start = coord_at(start_pos);
        let end = coord_at(end_pos);

        Self {
            data,
            width,
            height,
            start,
            end
        }
    }


    fn dist_between(&self, from: usize, to: usize) -> u32 {
        if self.data[to] <= self.data[from] + 1 {
            1 // assuming neighbours
        }
        else {
            u32::MAX
        }
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;
    fn index(&self, coord: Coord) -> &T {
        assert!(coord.0 < self.width && coord.1 < self.height);
        &self.data[self.index_of(coord)]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, coord: Coord) -> &mut T {
        assert!(coord.0 < self.width && coord.1 < self.height);
        &mut self.data[coord.0 + coord.1 * self.width]
    }
}

impl<T> fmt::Debug for Grid<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data
            .chunks(self.width)
            .try_for_each(|row| {
                writeln!( f, "{}", row.iter()
                                    .map(|c| format!("[{:^2}]", c))
                                    //.map(|c| format!("[{:?}]", c))
                                    .collect::<Vec<String>>()
                                    .join("")
                )
            })
    }
}

fn dijkstra(graph: &Grid<i32>) {
    let mut dist = Grid::new_default(u32::MAX, graph.width, graph.height);
    let mut prev: Grid<Option<usize>> = Grid::new_default(None, graph.width, graph.height);
    let mut unvisited: HashSet<usize> = (0..graph.data.len()).collect();

    dist[graph.start] = 0;

    while !unvisited.is_empty() {
        let min_dist = unvisited.iter().map(|i| (i, dist.data[*i])).min_by_key(|(_, d)| *d).unwrap();
        let min_dist_idx = *min_dist.0;
        let min_dist_val = min_dist.1;
        unvisited.remove(&min_dist_idx);

        if graph.coord_at(min_dist_idx) == graph.end {
            break
        }

        let neighbours: Vec<usize> = graph.neighours_at(min_dist_idx)
                                          .into_iter()
                                          .filter(|v| unvisited.contains(v)).collect();
        

        for neighbour in neighbours {
            let alt = min_dist_val.checked_add(graph.dist_between(min_dist_idx, neighbour)).unwrap_or(u32::MAX);
            if alt < dist.data[neighbour] {
                dist.data[neighbour] = alt;
                prev.data[neighbour] = Some(min_dist_idx);
            }
        }
    }

    let mut sequence: Vec<usize> = Vec::new();
    let mut u = graph.index_of(graph.end);
    while let Some(next) = prev.data[u] {
        sequence.push(next);
        u = next
    }

    println!("Steps: {}", sequence.len());
}

fn reverse_dijkstra(graph: &Grid<i32>) {
    let mut dist = Grid::new_default(u32::MAX, graph.width, graph.height);
    let mut prev: Grid<Option<usize>> = Grid::new_default(None, graph.width, graph.height);
    let mut unvisited: HashSet<usize> = (0..graph.data.len()).collect();

    dist[graph.end] = 0;

    let mut end = 0;

    while !unvisited.is_empty() {
        let min_dist = unvisited.iter().map(|i| (i, dist.data[*i])).min_by_key(|(_, d)| *d).unwrap();
        let min_dist_idx = *min_dist.0;
        let min_dist_val = min_dist.1;
        unvisited.remove(&min_dist_idx);

        if graph.data[min_dist_idx] == 0 {
            end = min_dist_idx;
            break;
        }

        let neighbours: Vec<usize> = graph.neighours_at(min_dist_idx)
                                          .into_iter()
                                          .filter(|v| unvisited.contains(v)).collect();
        

        for neighbour in neighbours {
            let alt = min_dist_val.checked_add(graph.dist_between(neighbour, min_dist_idx)).unwrap_or(u32::MAX);
            if alt < dist.data[neighbour] {
                dist.data[neighbour] = alt;
                prev.data[neighbour] = Some(min_dist_idx);
            }
        }
    }

    let mut sequence: Vec<usize> = Vec::new();
    let mut u = end;
    while let Some(next) = prev.data[u] {
        sequence.push(next);
        u = next
    }

    println!("Steps: {}", sequence.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let input = fs::read_to_string(&args[1]).unwrap();

    let map = Grid::new(input.as_str());

    //println!("{:?}", map);
    dijkstra(&map);
    reverse_dijkstra(&map);
}
