use crate::input::Input;
use std::collections::HashMap;

type EdgeBitmask = u16;
type TileId = u16;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Tile {
    id: TileId,
    /// Indexed by 0,1,3,4 = Top,Right,Bottom,Left.
    edges: [EdgeBitmask; 4],
    /// Indexed by row. Lowest bit to the right.
    /// Example: "#..#...." is stored as 0b10010000.
    body: [u8; 8],
}

impl Tile {
    fn transform_to_match(&self, x: u8, y: u8, composed_image: &HashMap<(u8, u8), Tile>) -> Self {
        let mut desired_edges = [None; 4];
        for i in 0_usize..4 {
            let (x, y) = match i {
                0 if y > 0 => (x, y - 1),
                1 => (x + 1, y),
                2 => (x, y + 1),
                3 if x > 0 => (x - 1, y),
                _ => {
                    continue;
                }
            };
            if let Some(tile) = composed_image.get(&(x, y)) {
                println!(
                    "Found edge to match (at {},{}), direction {} - that should be",
                    x, y, i
                );
                tile.debug_edge(match i {
                    0 => 2,
                    1 => 3,
                    2 => 0,
                    3 => 1,
                    _ => {
                        panic!("Invalid edge");
                    }
                });
                desired_edges[i] = Some(
                    tile.edges[match i {
                        0 => 2,
                        1 => 3,
                        2 => 0,
                        3 => 1,
                        _ => {
                            panic!("Invalid edge");
                        }
                    }],
                );
            }
        }

        let mut current = *self;
        for flip in 0..=2 {
            for _rotation in 0..=3 {
                let mut all_matches = true;
                for i in 0_usize..4 {
                    if let Some(e) = desired_edges[i] {
                        print!("Checking edge {}... ", i);
                        if e != current.edges[i] {
                            println!("NO");
                            all_matches = false;
                        } else {
                            println!("YES");
                        }
                    }
                }
                if all_matches {
                    return current;
                }
                current = current.rotate_clockwise();
            }
            if flip == 1 {
                current = current.flip_horizontal();
            } else {
                current = current.flip_vertical();
            }
        }
        panic!("transform_to_match not found");
    }

    fn debug_print(&self) {
        for i in 0..8 {
            let formatted_string = format!("{:0>8b}", self.body[i])
                .replace('1', "#")
                .replace('0', ".");
            println!("{}", formatted_string);
        }
    }

    fn debug_row(&self, i: u8) -> String {
        format!("{:0>8b}", self.body[i as usize])
            .replace('1', "#")
            .replace('0', ".")
    }

    fn debug_edge(&self, edge_idx: u8) {
        let edge = format!("  {:0>10b}", self.edges[edge_idx as usize])
            .replace('1', "#")
            .replace('0', ".");
        println!("  id={}, {}", self.id, edge);
    }

    fn rotate_clockwise(&self) -> Self {
        let rotated_edges = [self.edges[3], self.edges[0], self.edges[1], self.edges[2]];
        let mut rotated_body = [0_u8; 8];
        // abcdefgh
        // ABCDEFGH
        // =>
        // ......Aa
        // ......Bb
        // [..]
        for i in 0..8 {
            for j in 0..8 {
                rotated_body[7 - i] |= if self.body[j] & (1 << i) > 0 {
                    1 << j
                } else {
                    0
                };
            }
        }
        Self {
            id: self.id,
            edges: rotated_edges,
            body: rotated_body,
        }
    }

    fn rotate_clockwise_multiple(&self, steps: u8) -> Self {
        let mut result = *self;
        for _ in 0..steps {
            result = result.rotate_clockwise();
        }
        result
    }

    fn flip_vertical(&self) -> Self {
        let mut flipped_body = self.body;
        flipped_body.reverse();
        Self {
            id: self.id,
            edges: [
                self.edges[2],
                flip_edge(self.edges[1]),
                self.edges[0],
                flip_edge(self.edges[3]),
            ],
            body: flipped_body,
        }
    }

    fn flip_horizontal(&self) -> Self {
        let mut flipped_body = self.body;
        for b in flipped_body.iter_mut() {
            *b = b.reverse_bits();
        }
        Self {
            id: self.id,
            edges: [
                flip_edge(self.edges[0]),
                self.edges[3],
                flip_edge(self.edges[2]),
                self.edges[1],
            ],
            body: flipped_body,
        }
    }
}

fn flip_edge(number: EdgeBitmask) -> EdgeBitmask {
    // Only the first 10 bits of the edge bitmask is used.
    number.reverse_bits() >> 6
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut tiles: Vec<Tile> = Vec::new();
    // Mapped from edge bitmask to list of (tile_id, edge_direction) pairs,
    // where edge_direction is 0,1,3,4 = Top,Right,Bottom,Left.
    let mut edge_to_tile_idx = vec![Vec::new(); 1024];

    for tile_str in input.text.split("\n\n") {
        let mut tile_id = 0;
        let mut this_edges = [0 as EdgeBitmask; 4]; // Top, Right, Bottom, Left
        let mut body = [0_u8; 8];
        for (line_idx, line) in tile_str.lines().enumerate() {
            if line_idx == 0 {
                if !(line.len() == 10 && line.starts_with("Tile ") && line.ends_with(':')) {
                    return Err("Invalid tile header".to_string());
                }
                tile_id = line[5..9]
                    .parse::<u16>()
                    .map_err(|_| "Invalid tile header - cannot parse tile id")?;
            } else {
                let bytes = line.as_bytes();
                if !(bytes.len() == 10 && bytes.iter().all(|c| matches!(c, b'#' | b'.'))) {
                    return Err(
                        "Invalid tile line (not 10 in length and only '.' and '#'".to_string()
                    );
                }

                if line_idx > 1 && line_idx < 10 {
                    for i in 0..8 {
                        if bytes[i + 1] == b'#' {
                            body[line_idx - 2] |= 1 << (7 - i);
                        }
                    }
                }

                if line_idx == 1 {
                    // Top edge:
                    for i in 0..10 {
                        if bytes[i] == b'#' {
                            this_edges[0] |= 1 << (9 - i);
                        }
                    }
                } else if line_idx == 10 {
                    // Bottom edge:
                    for i in 0..10 {
                        if bytes[i] == b'#' {
                            this_edges[2] |= 1 << (9 - i);
                        }
                    }
                }
                if bytes[9] == b'#' {
                    // Right edge.
                    this_edges[1] |= 1 << (10 - line_idx);
                }
                if bytes[0] == b'#' {
                    // Left edge:
                    this_edges[3] |= 1 << (10 - line_idx);
                }
            }
        }

        /*
        println!("### Tile {}", tile_id);
        println!("  Top:    {:0>10b}", this_edges[0]);
        println!("  Right:  {:0>10b}", this_edges[1]);
        println!("  Bottom: {:0>10b}", this_edges[2]);
        println!("  Left:   {:0>10b}", this_edges[3]);
        for i in 0..8 {
            println!("  Body[{}]:   {:0>8b}", i, body[i]);
        }
        println!();
         */

        edge_to_tile_idx[this_edges[0] as usize].push(tile_id);
        edge_to_tile_idx[this_edges[1] as usize].push(tile_id);
        edge_to_tile_idx[this_edges[2] as usize].push(tile_id);
        edge_to_tile_idx[this_edges[3] as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[0]) as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[1]) as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[2]) as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[3]) as usize].push(tile_id);

        tiles.push(Tile {
            id: tile_id,
            edges: this_edges,
            body,
            //matching_edges_bitmask: 0,
        });
    }

    // The composed image is square:
    let composed_image_width = (tiles.len() as f64).sqrt() as u8;
    println!("Composed image width: {}", composed_image_width);

    let mut top_left_corner = None;
    let mut corners = Vec::new();

    for &this_tile in tiles.iter() {
        let mut matching_edges_bitmask = 0_u64;
        //for other_tile in tiles.iter() {
        //if this_tile.id != other_tile.id {
        for (this_edge_idx, &this_edge) in this_tile.edges.iter().enumerate() {
            let edge_match = &edge_to_tile_idx[this_edge as usize];
            //let flipped_edge_match = &edge_to_tile_idx[flip_edge(this_edge) as usize];
            let normal_match =
                edge_match.len() > 1 || (edge_match.len() == 1 && edge_match[0] != this_tile.id);
            //let flipped_match = flipped_edge_match.len() > 1
            //|| (flipped_edge_match.len() == 1 && flipped_edge_match[0] != this_tile.id);
            if normal_match {
                //|| flipped_match {
                //for &other_edge in other_tile.edges.iter() {
                //if this_edge == other_edge || this_edge == flip_edge(other_edge) {
                matching_edges_bitmask |= 1 << this_edge_idx;
            }
            //}
        }
        //}
        //}
        if matching_edges_bitmask.count_ones() == 2 {
            corners.push(this_tile);
            if matching_edges_bitmask == 0b0110 {
                top_left_corner = Some(this_tile);
            }
        }
    }

    if input.is_part_one() {
        return Ok(corners.iter().map(|tile| tile.id as u64).product());
    }

    // From (x,y) to tile at position.
    let mut composed_image: HashMap<(u8, u8), Tile> = HashMap::new();

    let top_left_corner = top_left_corner.unwrap();
    for &edge in top_left_corner.edges.iter() {
        edge_to_tile_idx[edge as usize].retain(|&e| e != top_left_corner.id);
        edge_to_tile_idx[flip_edge(edge) as usize].retain(|&e| e != top_left_corner.id);
    }
    composed_image.insert((0, 0), top_left_corner);

    println!("Initial top left corner (id={}):", top_left_corner.id);
    top_left_corner.debug_print();

    let mut stack = Vec::new();
    stack.push((0, 0, top_left_corner));
    while let Some((x, y, popped_tile)) = stack.pop() {
        // Remove popped tile from edge_to_tile_idx
        for (edge_idx, &edge) in popped_tile.edges.iter().enumerate() {
            let tiles_with_matching_edge = &edge_to_tile_idx[edge as usize];
            if tiles_with_matching_edge.len() == 1 {
                let tile_with_matching_edge = tiles
                    .iter()
                    .find(|t| t.id == tiles_with_matching_edge[0])
                    .unwrap();
                let (new_x, new_y) = match edge_idx {
                    0 if y > 0 => (x, y - 1),
                    1 => (x + 1, y),
                    2 => (x, y + 1),
                    3 if x > 0 => (x - 1, y),
                    _ => {
                        continue;
                    }
                };
                if new_x >= composed_image_width || new_y >= composed_image_width {
                    continue;
                } else if composed_image.contains_key(&(new_x, new_y)) {
                    continue;
                }
                println!(
                    "Found matching tile {} to place at x={}, y={}",
                    tile_with_matching_edge.id, new_x, new_y
                );

                let tile_with_matching_edge =
                    tile_with_matching_edge.transform_to_match(new_x, new_y, &composed_image);
                if new_x == 2 {
                    println!("Left edge of newly found");
                    tile_with_matching_edge.debug_edge(3);
                    println!("Does it really match?");
                    composed_image
                        .get(&(new_x - 1, new_y))
                        .unwrap()
                        .debug_edge(1);
                }

                for &edge in tile_with_matching_edge.edges.iter() {
                    edge_to_tile_idx[edge as usize].retain(|&e| e != tile_with_matching_edge.id);
                    edge_to_tile_idx[flip_edge(edge) as usize]
                        .retain(|&e| e != tile_with_matching_edge.id);
                }
                //println!("edge to tile id: {:?}", edge_to_tile_idx);
                composed_image.insert((new_x, new_y), tile_with_matching_edge);

                stack.push((new_x, new_y, tile_with_matching_edge));
            }
        }
    }

    // FIXME: 2473 is not rotated correctly
    println!("Placed tiles: {}", composed_image.len());
    for y in 0..composed_image_width {
        for x in 0..composed_image_width {
            let tile = composed_image.get(&(x, y)).unwrap();
            print!("{} ", tile.id);
        }
        println!();
    }

    for y in 0..composed_image_width {
        for row in 0..8 {
            for x in 0..composed_image_width {
                let tile = composed_image.get(&(x, y)).unwrap();
                print!("{}", tile.debug_row(row));
            }
            println!();
        }
    }

    let composed_image_width_pixels = composed_image_width * 8;

    let is_black_at = |direction: u8, pixel_x: u8, monster_direction: u8| {
        let (pixel_x, pixel_y) = match direction {
            1 => (monster_direction, pixel_x),
            3 => (composed_image_width_pixels - 1 - monster_direction, pixel_x),
            0 => (pixel_x, monster_direction),
            2 => (pixel_x, composed_image_width_pixels - 1 - monster_direction),
            _ => {
                panic!("Invalid direction");
            }
        };
        //println!("Checking {}, {}", pixel_x, pixel_y);
        let tile_x = pixel_x / 8;
        let tile_y = pixel_y / 8;
        let bit = pixel_x % 8;
        let row = pixel_y % 8;
        composed_image
            .get(&(tile_x as u8, tile_y as u8))
            .unwrap()
            .body[row as usize]
            & (1 << (7 - bit))
            != 0
    };

    println!();
    println!("###### Renddering anew");
    for y in 0..composed_image_width_pixels {
        for x in 0..composed_image_width_pixels {
            print!("{}", if is_black_at(1, y, x) { '#' } else { '.' });
        }
        println!();
    }

    // Search for the main body "#    ##    ##    ###",
    // of length 20, in the sea monster pattern:
    // "                  # "
    // "#    ##    ##    ###"
    // " #  #  #  #  #  #   "
    let monster_body_len = 20;
    for &direction in &[0_u8, 1, 2, 3] {
        for &flip in &[1_i8, -1] {
            let mut monster_count = 0;
            // TODO: Check boundary condition..
            for x in 1..(composed_image_width_pixels - 1) {
                for y in 0..(composed_image_width_pixels - monster_body_len + 1) {
                    if is_black_at(direction, x, y)
                        && is_black_at(direction, x, y + 5)
                        && is_black_at(direction, x, y + 6)
                        && is_black_at(direction, x, y + 11)
                        && is_black_at(direction, x, y + 12)
                        && is_black_at(direction, x, y + 17)
                        && is_black_at(direction, x, y + 18)
                        && is_black_at(direction, x, y + 19)
                        && is_black_at(direction, (x as i8 - flip * 1) as u8, y + 18)
                        && is_black_at(direction, (x as i8 + flip * 1) as u8, y + 1)
                        && is_black_at(direction, (x as i8 + flip * 1) as u8, y + 4)
                        && is_black_at(direction, (x as i8 + flip * 1) as u8, y + 7)
                        && is_black_at(direction, (x as i8 + flip * 1) as u8, y + 10)
                        && is_black_at(direction, (x as i8 + flip * 1) as u8, y + 13)
                        && is_black_at(direction, (x as i8 + flip * 1) as u8, y + 16)
                    {
                        monster_count += 1;
                        println!(
                            "############ Found sea monster. x={}, y={}, direction={}",
                            x, y, direction
                        );
                    }
                }
            }

            if monster_count != 0 {
                return Ok(tiles
                    .iter()
                    .map(|t| {
                        t.body
                            .iter()
                            .map(|row| row.count_ones() as u64)
                            .sum::<u64>()
                    })
                    .sum::<u64>()
                    - monster_count * 15);
            }
        }
    }

    Err("No sea monster found".to_string())
}

#[test]
pub fn test_rotate() {
    let tile = Tile {
        id: 0,
        edges: [1, 2, 3, 4],
        body: [0b10100000, 0b01010000, 0, 0, 0, 0, 0, 0],
    };

    let rotated_tile = tile.rotate_clockwise();
    assert_eq!(rotated_tile.id, tile.id);
    assert_eq!(rotated_tile.edges, [4, 1, 2, 3]);
    // #.#.....
    // .#.#....
    // [6 empty rows]
    //
    // =>
    //
    // .......#
    // ......#.
    // .......#
    // ......#.
    // [4 empty rows]
    assert_eq!(rotated_tile.body, [0b1, 0b10, 0b1, 0b10, 0, 0, 0, 0]);

    assert_eq!(
        rotated_tile.rotate_clockwise(),
        tile.rotate_clockwise_multiple(2)
    );

    assert_eq!(tile, tile.rotate_clockwise_multiple(0));
    assert_eq!(tile, tile.rotate_clockwise_multiple(4));
}

#[test]
pub fn test_flip() {
    let tile = Tile {
        id: 17,
        edges: [0b1, 0b10, 0b11, 0b100],
        body: [0b10100000, 0b01010000, 0, 0, 0, 0, 0, 0],
    };

    let horizontally_flipped = tile.flip_horizontal();
    assert_eq!(17, horizontally_flipped.id);
    assert_eq!(
        horizontally_flipped.edges,
        [0b1000000000, 0b100, 0b1100000000, 0b10]
    );
    assert_eq!(
        horizontally_flipped.body,
        [0b00000101, 0b00001010, 0, 0, 0, 0, 0, 0]
    );

    let vertically_flipped = tile.flip_vertical();
    assert_eq!(17, vertically_flipped.id);
    assert_eq!(
        vertically_flipped.edges,
        [0b11, 0b0100000000, 0b1, 0b0010000000]
    );
    assert_eq!(
        vertically_flipped.body,
        [0, 0, 0, 0, 0, 0, 0b01010000, 0b10100000],
    );
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    test_part_one!(example=> 20_899_048_083_289);
    test_part_two!(example => 273);

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 21_599_955_909_991);
    // test_part_two!(real_input => 0);
}
