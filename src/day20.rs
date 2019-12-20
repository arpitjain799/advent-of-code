use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (-1, 0), (1, 0)];

struct Maze {
    cols: usize,
    array: Vec<u8>,
    portals: HashMap<(i32, i32), (i32, i32)>,
    start_location: (i32, i32),
    end_location: (i32, i32),
}

impl Maze {
    fn tile_at(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {
            return b' ';
        }
        self.array[(x as usize) + self.cols * (y as usize)]
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: u8) {
        //println!("setting x={},y={},tile={}, index={}, rows={}", x, y, tile as char, (x + self.rows*y), self.rows);
        self.array[x + self.cols * y] = tile;
    }

    fn parse(input: &str) -> Maze {
        let rows = input.chars().filter(|&c| c == '\n').count() + 1;
        let cols = input.lines().map(|line| line.len()).max().unwrap();

        let array = vec![b' '; rows * cols];
        let mut maze = Maze {
            cols,
            array,
            portals: HashMap::new(),
            end_location: (0, 0),
            start_location: (0, 0),
        };
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, tile)| {
                maze.set_tile(x, y, tile as u8);
            });
        });

        let mut current_string = String::new();
        let mut coming_from_passage = false;
        let mut portal_name_to_location: HashMap<String, (i32, i32)> = HashMap::new();

        let mut on_tile = |x: i32, y: i32, x_direction| {
            let tile = if x as usize == cols || y as usize == rows {
                b' '
            } else {
                maze.tile_at(x, y)
            };

            if tile >= b'A' && tile <= b'Z' {
                current_string.push(tile as char);
            } else {
                if current_string.len() == 2 {
                    let portal_x = if x_direction && coming_from_passage {
                        x - 3
                    } else {
                        x
                    };
                    let portal_y = if !x_direction && coming_from_passage {
                        y - 3
                    } else {
                        y
                    };

                    let current_location: (i32, i32) = (portal_x, portal_y);

                    match current_string.as_str() {
                        "AA" => {
                            maze.start_location = (portal_x, portal_y);
                        }
                        "ZZ" => {
                            maze.end_location = (portal_x, portal_y);
                        }
                        _ => {}
                    };

                    match portal_name_to_location.entry(current_string.clone()) {
                        Entry::Occupied(other_location) => {
                            let other_location = *other_location.get();
                            maze.portals.insert(current_location, other_location);
                            maze.portals.insert(other_location, current_location);
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(current_location);
                        }
                    }
                }

                coming_from_passage = tile == b'.';
                current_string.clear();
            }
        };

        for x in 0..=cols {
            for y in 0..=rows {
                on_tile(x as i32, y as i32, false);
            }
        }
        for y in 0..=rows {
            for x in 0..=cols {
                on_tile(x as i32, y as i32, true);
            }
        }

        maze
    }
}

pub fn part1(input_string: &str) -> String {
    let maze = Maze::parse(input_string);

    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();
    to_visit.push_back((maze.start_location, 0));
    visited.insert(maze.start_location);

    while let Some((visiting, distance)) = to_visit.pop_front() {
        let new_distance = distance + 1;

        for new_location in DIRECTIONS
            .iter()
            .map(|&(dx, dy)| (visiting.0 + dx, visiting.1 + dy))
            .chain(if let Some(&new_location) = maze.portals.get(&visiting) {
                Some(new_location).into_iter()
            } else {
                None.into_iter()
            })
        {
            if maze.tile_at(new_location.0, new_location.1) == b'.' && visited.insert(new_location)
            {
                if new_location == maze.end_location {
                    return new_distance.to_string();
                }
                to_visit.push_back((new_location, new_distance));
            }
        }
    }
    "No path found".to_string()
}

pub fn part2(_input_string: &str) -> String {
    String::from("")
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day20_example.txt")), "23");
    assert_eq!(part1(include_str!("day20_input.txt")), "580");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(""), "");

    // assert_eq!(part2(include_str!("day20_input.txt")), "");
}
