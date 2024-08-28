use std::collections::{BTreeMap, HashMap, HashSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Copy, Clone)]
enum TrackKind {
    Vertical,
    Horizontal,
    TopRightCorner,
    TopLeftCorner,
    BottomRightCorner,
    BottomLeftCorner,
    Intersection,
}

#[derive(Debug, Copy, Clone)]
enum Direction { /* cart direction */
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone)]
enum Turn { /* track turn kind */
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone)]
struct Cart {
    dir: Direction,
    last_turn: Option<Turn>,
}

impl Cart {
    // Find the next index where the cart will go, based on current index, direction and 
    // map dimentions.
    fn next_index(&self, current_index: usize, width: usize, height: usize) -> usize {
        match self.dir {
            Direction::Left => {
                let (_, curr_x) = (current_index / width, current_index % width);
                assert!(curr_x > 0);
                current_index - 1
            }
            Direction::Up => {
                let (curr_y, curr_x) = (current_index / width, current_index % width);
                assert!(curr_y > 0);
                let (new_x, new_y) = (curr_x, curr_y - 1);
                new_y * width + new_x
            }
            Direction::Right => {
                let (_, curr_x) = (current_index / width, current_index % width);
                assert!(curr_x < width);
                current_index + 1
            }
            Direction::Down => {
                let (curr_y, curr_x) = (current_index / width, current_index % width);
                assert!(curr_y < height);
                let (new_x, new_y) = (curr_x, curr_y + 1);
                new_y * width + new_x
            }
        }
    }

    // Change the cart's orientation to what it will be upon jumping to the next 
    // track location.
    fn move_cart(&mut self, next_track_kind: TrackKind) {
        match self.dir {
            Direction::Left => match next_track_kind {
                TrackKind::Horizontal => {}
                TrackKind::TopLeftCorner => {
                    self.dir = Direction::Down;
                }
                TrackKind::BottomLeftCorner => {
                    self.dir = Direction::Up;
                }
                TrackKind::Intersection => self.goto_intersection(),
                _ => unreachable!(),
            },
            Direction::Up => match next_track_kind {
                TrackKind::Vertical => {}
                TrackKind::TopRightCorner => {
                    self.dir = Direction::Left;
                }
                TrackKind::TopLeftCorner => {
                    self.dir = Direction::Right;
                }
                TrackKind::Intersection => self.goto_intersection(),
                _ => unreachable!(),
            },
            Direction::Right => match next_track_kind {
                TrackKind::Horizontal => {}
                TrackKind::TopRightCorner => {
                    self.dir = Direction::Down;
                }
                TrackKind::BottomRightCorner => {
                    self.dir = Direction::Up;
                }
                TrackKind::Intersection => self.goto_intersection(),
                _ => unreachable!(),
            },
            Direction::Down => match next_track_kind {
                TrackKind::Vertical => {}
                TrackKind::BottomRightCorner => {
                    self.dir = Direction::Left;
                }
                TrackKind::BottomLeftCorner => {
                    self.dir = Direction::Right;
                }
                TrackKind::Intersection => self.goto_intersection(),
                _ => unreachable!(),
            },
        }
    }

    // Modify cart's orientation as needed upon going to an intersection block.
    fn goto_intersection(&mut self) {
        match self.last_turn {
            Some(Turn::Left) => {
                self.last_turn = Some(Turn::Straight);
            }
            Some(Turn::Straight) => {
                match self.dir {
                    Direction::Left => {
                        self.dir = Direction::Up;
                    }
                    Direction::Up => {
                        self.dir = Direction::Right;
                    }
                    Direction::Right => {
                        self.dir = Direction::Down;
                    }
                    Direction::Down => {
                        self.dir = Direction::Left;
                    }
                }
                self.last_turn = Some(Turn::Right);
            }
            Some(Turn::Right) | None => {
                match self.dir {
                    Direction::Left => {
                        self.dir = Direction::Down;
                    }
                    Direction::Up => {
                        self.dir = Direction::Left;
                    }
                    Direction::Right => {
                        self.dir = Direction::Up;
                    }
                    Direction::Down => {
                        self.dir = Direction::Right;
                    }
                }
                self.last_turn = Some(Turn::Left);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Track {
    track_kind: TrackKind,
    cart: Option<Cart>,
}

#[derive(Clone)]
struct Map {
    tracks: BTreeMap<usize, Track>,
    width: usize, /* const */
    height: usize, /* const */
}

// For printing purposes.
impl From<Direction> for char {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Left => '<',
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
        }
    }
}

impl Map {
    // Print map current state.
    fn _print(&self) {
        let mut buffer = vec![' '; self.width * self.height];
        self.tracks
            .iter()
            .for_each(|(&idx, track)| match track.track_kind {
                TrackKind::Vertical => {
                    if track.cart.is_none() {
                        buffer[idx] = '|';
                    } else {
                        buffer[idx] = track.cart.clone().unwrap().dir.into();
                    }
                }
                TrackKind::Horizontal => {
                    if track.cart.is_none() {
                        buffer[idx] = '-';
                    } else {
                        buffer[idx] = track.cart.clone().unwrap().dir.into();
                    }
                }
                TrackKind::TopRightCorner => {
                    if track.cart.is_none() {
                        buffer[idx] = '\\';
                    } else {
                        buffer[idx] = track.cart.clone().unwrap().dir.into();
                    }
                }
                TrackKind::TopLeftCorner => {
                    if track.cart.is_none() {
                        buffer[idx] = '/';
                    } else {
                        buffer[idx] = track.cart.clone().unwrap().dir.into();
                    }
                }
                TrackKind::BottomRightCorner => {
                    if track.cart.is_none() {
                        buffer[idx] = '/';
                    } else {
                        buffer[idx] = track.cart.clone().unwrap().dir.into();
                    }
                }
                TrackKind::BottomLeftCorner => {
                    if track.cart.is_none() {
                        buffer[idx] = '\\';
                    } else {
                        buffer[idx] = track.cart.clone().unwrap().dir.into();
                    }
                }
                TrackKind::Intersection => {
                    if track.cart.is_none() {
                        buffer[idx] = '+';
                    } else {
                        buffer[idx] = track.cart.clone().unwrap().dir.into();
                    }
                }
            });

        for (i, c) in buffer.iter().enumerate() {
            print!("{}", c);
            if (i + 1) % self.width == 0 {
                println!();
            }
        }
    }

    // Tick for part 1.
    //
    // Returns the required index which needs to be turned into coordinate.
    fn tick_v1(&mut self) -> Option<usize> {
        // let mut carts_to_go: HashMap<usize, (usize, Cart)> = HashMap::new();
        let mut carts_to_go: Vec<(usize, (usize, Cart))> = Vec::new();

        for (&index, track) in self.tracks.iter_mut() {
            if let Some(cart) = track.cart.take() {
                // New index where the cart will go.
                let next_index = cart.next_index(index, self.width, self.height);
                carts_to_go.push((next_index, (index, cart)));
            }
        }

        // Check for collisions.
        for (i, (new_idx, _)) in carts_to_go.iter().enumerate() {
            // Where the cart needs to go, does there already exists another cart?
            for (_, (old_idx, _)) in &carts_to_go {
                if *new_idx == *old_idx {
                    // Collision: A cart's new index is actually old index for another cart.
                    return Some(*new_idx);
                }
            }

            // Where the cart needs to go, does another cart needs to go there?
            for (new_idx_after, _) in &carts_to_go[i+1..] {
                if new_idx_after == new_idx {
                    return Some(*new_idx);
                }
            }

        }

        // Move carts to their new locations as there are no collisions.
        for (index, (_, mut cart)) in carts_to_go.into_iter() {
            cart.move_cart(self.kind_at_idx(index));
            let track = self
                .tracks
                .get_mut(&index)
                .expect("Track at new index does not exist.");

            track.cart = Some(cart);
        }

        None
    }

    // Tick for part-2.
    //
    // Returns the required index which needs to be turned into coordinate.
    fn tick_v2(&mut self) -> Option<usize> {
        let mut carts_to_go: Vec<(usize, (usize, Cart))> = Vec::new();

        for (&index, track) in self.tracks.iter_mut() {
            if let Some(cart) = track.cart.take() {
                // New index where the cart will go.
                let next_index = cart.next_index(index, self.width, self.height);
                carts_to_go.push((next_index, (index, cart)));
            }
        }

        /*
         * OneWayCollision: Cart goes where another cart already exists.
         * TwoWayCollision: Cart goes somewhere, then another cart comes there in the same tick.
         */

        // Fix all the one-way-collisions.
        //
        // For each element in to_remove, remove the first element in carts_to_go
        // which has it as its next_idx or as its old_idx.
        let mut to_remove = HashSet::new();
        for (i, (new_idx, _)) in carts_to_go.iter().enumerate() {
            for (_, (old_idx, _)) in &carts_to_go[i+1..] {
                if new_idx == old_idx {
                    to_remove.insert(*new_idx);
                }
            }
        }

        for idx in to_remove.drain() {
            // Remove first element with idx as its next_idx
            let mut index = 0;
            for (i, (next_idx, _)) in carts_to_go.iter().enumerate() {
                if *next_idx == idx {
                    index = i;
                    break;
                }
            }
            carts_to_go.remove(index);

            // Remove first element with idx as its old_idx
            let mut index = 0;
            for (i, (_, (old_idx, _))) in carts_to_go.iter().enumerate() {
                if *old_idx == idx {
                    index = i;
                    break;
                }
            }
            carts_to_go.remove(index);
        }

        // Fix all the two-way-collisions.
        let mut to_remove: HashMap<usize/* new_idx */, usize /* count */> = HashMap::new();
        for (new_idx, _) in &carts_to_go {
            to_remove.entry(*new_idx).and_modify(|count| *count += 1).or_insert(1);
        }
        // Get rid of non-collision cases.
        to_remove.retain(|_, count| *count > 1);

        let closest_lesser_even = |x:usize| {
            if x % 2 == 0 {
                x
            } else {
                x - 1
            }
        };

        to_remove.drain().for_each(|(new_idx_to_del, count)| {
            let mut n = closest_lesser_even(count);
            carts_to_go.retain(|(new_idx, _)| {
                if n > 0 && *new_idx == new_idx_to_del {
                    n -= 1;
                    false
                } else {
                    true
                }
            });
        });

        assert!(carts_to_go.len() > 0, "No last remaining cart.");

        if carts_to_go.len() == 1 {
            return Some(carts_to_go[0].0);
        }

        for (index, (_, mut cart)) in carts_to_go.into_iter() {
            cart.move_cart(self.kind_at_idx(index));
            let track = self
                .tracks
                .get_mut(&index)
                .expect("Track at new index does not exist.");

            track.cart = Some(cart);
        }

        None
    }

    // Cannot fail!
    fn kind_at_idx(&self, idx: usize) -> TrackKind {
        self.tracks.get(&idx).unwrap().track_kind
    }

    fn idx_to_loc(&self, idx: usize) -> (usize /* x */, usize) {
        let y = idx / self.width;
        let x = idx % self.width;
        (x, y)
    }
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input-filename>", args[0]);
        std::process::exit(-1);
    }

    let tracks_string = std::fs::read_to_string(&args[1])?;
    let height = tracks_string.trim().lines().count();
    let width = height;

    let mut tracks: BTreeMap<usize, Track> = BTreeMap::new();

    for (y, line) in tracks_string.lines().enumerate() {
        assert!(y < height);

        let mut previous: Option<char> = None;
        for (x, c) in line.chars().enumerate() {
            assert!(x < width);
            match c {
                ' ' => {}
                '-' => {
                    let index = y * width + x;
                    tracks.insert(
                        index,
                        Track {
                            track_kind: TrackKind::Horizontal,
                            cart: None,
                        },
                    );
                }
                '|' => {
                    let index = y * width + x;
                    tracks.insert(
                        index,
                        Track {
                            track_kind: TrackKind::Vertical,
                            cart: None,
                        },
                    );
                }
                '\\' => {
                    let index = y * width + x;
                    if previous.is_some() {
                        let c = previous.unwrap();
                        if c == '-' || c == '+' || c == '>' {
                            tracks.insert(
                                index,
                                Track {
                                    track_kind: TrackKind::TopRightCorner,
                                    cart: None,
                                },
                            );
                        } else {
                            tracks.insert(
                                index,
                                Track {
                                    track_kind: TrackKind::BottomLeftCorner,
                                    cart: None,
                                },
                            );
                        }
                    } else {
                        tracks.insert(
                            index,
                            Track {
                                track_kind: TrackKind::BottomLeftCorner,
                                cart: None,
                            },
                        );
                    }
                }
                '/' => {
                    let index = y * width + x;
                    if previous.is_some() {
                        let c = previous.unwrap();
                        if c == '-' || c == '+' || c == '>' {
                            tracks.insert(
                                index,
                                Track {
                                    track_kind: TrackKind::BottomRightCorner,
                                    cart: None,
                                },
                            );
                        } else {
                            tracks.insert(
                                index,
                                Track {
                                    track_kind: TrackKind::TopLeftCorner,
                                    cart: None,
                                },
                            );
                        }
                    } else {
                        tracks.insert(
                            index,
                            Track {
                                track_kind: TrackKind::TopLeftCorner,
                                cart: None,
                            },
                        );
                    }
                }
                '>' => {
                    let index = y * width + x;
                    tracks.insert(
                        index,
                        Track {
                            track_kind: TrackKind::Horizontal,
                            cart: Some(Cart {
                                dir: Direction::Right,
                                last_turn: None,
                            }),
                        },
                    );
                }
                '<' => {
                    let index = y * width + x;
                    tracks.insert(
                        index,
                        Track {
                            track_kind: TrackKind::Horizontal,
                            cart: Some(Cart {
                                dir: Direction::Left,
                                last_turn: None,
                            }),
                        },
                    );
                }
                '^' => {
                    let index = y * width + x;
                    tracks.insert(
                        index,
                        Track {
                            track_kind: TrackKind::Vertical,
                            cart: Some(Cart {
                                dir: Direction::Up,
                                last_turn: None,
                            }),
                        },
                    );
                }
                'v' => {
                    let index = y * width + x;
                    tracks.insert(
                        index,
                        Track {
                            track_kind: TrackKind::Vertical,
                            cart: Some(Cart {
                                dir: Direction::Down,
                                last_turn: None,
                            }),
                        },
                    );
                }

                '+' => {
                    let index = y * width + x;
                    tracks.insert(
                        index,
                        Track {
                            track_kind: TrackKind::Intersection,
                            cart: None,
                        },
                    );
                }

                _ => unreachable!("got unknown character {}", c),
            }

            if c == ' ' {
                previous = None;
            } else {
                previous = Some(c);
            }
        }
    }

    let map = Map {
        tracks,
        width: width,
        height: height,
    };

    part1(map.clone())?;
    part2(map.clone())?;

    Ok(())
}

fn part1(mut map: Map) -> Result<()> {
    let collision_idx = loop {
        match map.tick_v1() {
            None => {}
            Some(idx) => break idx,
        }
    };

    let (x, y) = map.idx_to_loc(collision_idx);
    println!("{},{}", x, y);

    Ok(())
}

fn part2(mut map: Map) -> Result<()> {
    let collision_idx = loop {
        match map.tick_v2() {
            None => {}
            Some(idx) => break idx,
        }
    };

    let (x, y) = map.idx_to_loc(collision_idx);
    println!("{},{}", x, y);

    Ok(())
}
