use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::time::Instant;

use hashers::fx_hash::FxHasher; // fast hasher for integers

// edge size of the grid
const EDGE: usize = 300;

// number of unique square patches(subgrids) that can occur in
// a square grid of given EDGE size
const CACHE_CAPACITY: usize = (EDGE * (EDGE + 1) * (2 * EDGE + 1)) / 6;

// show execution time info for each part if true
const SHOW_TIME: bool = true;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <input>", args[0]);
        std::process::exit(1);
    }

    // get grid serial number from cli args
    let grid_serial_number = args[1]
        .parse::<usize>()
        .expect(format!("Failed to parse '{}' as Grid Serial Number", args[1]).as_str());

    let mut start = Instant::now();
    part1(grid_serial_number)?;
    let delta_1 = Instant::duration_since(&Instant::now(), start);

    start = Instant::now();
    part2(grid_serial_number)?;
    let delta_2 = Instant::duration_since(&Instant::now(), start);

    if SHOW_TIME {
        println!();
        println!("time for part-1: {:?}", delta_1);
        println!("time for part-2: {:?}", delta_2);
    }

    Ok(())
}

#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    // converts the cell coordinates to index for indexing in cell powers in Grid
    #[inline]
    fn power_index(&self) -> Option<usize> {
        let position = self.x + (self.y - 1) * EDGE; // position from the beginning of the grid
        if position <= EDGE * EDGE {
            Some(position - 1) // index in the grid
        } else {
            None
        }
    }
}

impl From<usize> for Cell {
    #[inline]
    fn from(index: usize) -> Self {
        // position from the beginning of grid
        let position = index + 1;
        let x = position % EDGE;
        let y = (position - x) / EDGE + 1;

        Cell { x, y }
    }
}

struct PatchIterator {
    edge: usize,             // edge size of patch
    top_left: Cell,          // top-left cell of the patch to be iterated upon
    next_cell: Option<Cell>, // next cell in the iterator
}

impl PatchIterator {
    // if top left cell is in a position that the patch of given edge size exists
    // then it will be Some(_) otherwise None
    fn try_new(top_left: Cell, edge: usize) -> Option<Self> {
        if top_left.x + edge - 1 > EDGE || top_left.y + edge - 1 > EDGE {
            None
        } else {
            Some(PatchIterator {
                edge,
                top_left,
                next_cell: Some(top_left),
            })
        }
    }
}

impl Iterator for PatchIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_next_cell) = self.next_cell.take() {
            // create next next_cell
            if current_next_cell.x + 1 != self.top_left.x + self.edge {
                // if not crossing vertical boundry
                self.next_cell = Some(Cell {
                    x: current_next_cell.x + 1,
                    y: current_next_cell.y,
                });
                Some(current_next_cell)
            } else if current_next_cell.x + 1 == self.top_left.x + self.edge
                && current_next_cell.y + 1 != self.top_left.y + self.edge
            {
                // if crossing only vertical boundry and moving down is okay
                self.next_cell = Some(Cell {
                    x: self.top_left.x,
                    y: current_next_cell.y + 1,
                });
                Some(current_next_cell)
            } else {
                // current_next_cell was the bottom-right cell
                self.next_cell = None;
                Some(current_next_cell)
            }
        } else {
            None
        }
    }
}

struct Grid {
    cell_powers: Vec<Power>, // individual cell powers
    // power of square patches of different sizes
    patch_power_cache: HashMap<PatchCompressed, Power, BuildHasherDefault<FxHasher>>, 
}

impl Grid {
    fn new(gsn: usize, edge: usize) -> Self {
        assert!(edge > 0, "Grid edge cannot be negative");
        let cell_powers = (0usize..edge * edge)
            .into_iter()
            .map(|index| Grid::power(&Cell::from(index), gsn))
            .collect();

        Grid {
            cell_powers,
            patch_power_cache: HashMap::with_capacity_and_hasher(
                CACHE_CAPACITY,
                BuildHasherDefault::<FxHasher>::default(),
            ),
        }
    }

    #[inline]
    /// calculate power level for a given coordinates and grid serial number
    fn power(cell: &Cell, gsn: usize) -> Power {
        let rack = cell.x + 10;
        let mut power = rack * cell.y;
        power += gsn;
        power *= rack;
        power = power % 1000 - power % 100;
        power /= 100;
        // dbg!(power);
        power as Power - 5
    }

    #[inline]
    /// give an iterator over the cells of a patch with a given edge size and
    /// top_left corner, if it exists
    fn patch_iter(&self, cell: Cell, edge: usize) -> Option<PatchIterator> {
        PatchIterator::try_new(cell, edge)
    }

    #[inline]
    /// calculate the power over a patch if the iterator over it exists, i.e. the patch
    /// exists
    fn patch_power(&self, cell: Cell, edge: usize) -> Option<Power> {
        let power_sum = self.patch_iter(cell, edge)?.fold(0i32, |acc, cell| {
            acc + self.cell_powers[cell.power_index().unwrap()]
        });

        Some(power_sum)
    }
}

// power must be a signed integer
type Power = i32;

fn part1(gsn: usize) -> Result<()> {
    let mut grid = Grid::new(gsn, EDGE);

    // minigrid where every cell is positioned to become a top-left of a patch of edge 3
    let minigrid = Grid::new(gsn, EDGE - 3 + 1);

    minigrid.patch_iter(Cell {
        x: 1,
        y: 1,
    }, 1).unwrap(/* patch of edge 1 will always exist" */).for_each(|cell| {
        let moving_patch_power = grid.patch_power(cell, 3).unwrap(
        /* patch_iter with 'edge = 3' 
           will always exist for any cell in minigrid and each cell is within a grid so cell_power
           will also work cell is within the grid */);
        assert_eq!(grid.patch_power_cache.insert(patch_accumulate((cell.x, cell.y, 3)),
        moving_patch_power), None);
    });

    let max_power = grid.patch_power_cache
        .iter()
        .max_by_key(|(_, v)| *v)
        .unwrap(/* grid cannot be empty */).1;
    println!("{max_power}");


    Ok(())
}

type PatchCompressed = u32; // integral representation of patch
type PatchUnfolded = (usize, usize, usize); // (x, y, edge) represenation of patch

#[inline]
/// desperate optimisation to pack top_left_x, top_left_y, edge of a patch in an integer.
/// all these values must be representable in 10bits for it to work.
fn patch_accumulate(patch: PatchUnfolded) -> PatchCompressed {
    // checking edge is enough coz x, y <= EDGE anyway
    assert!(EDGE < 2usize.pow(10), "EDGE must be less than 2**10");

    let x = (patch.0 as u32) << 20;
    let y = (patch.1 as u32) << 10;
    let edge = patch.2 as u32;

    let mut number = 0u32;
    number |= x;
    number |= y;
    number |= edge;

    number
}

#[inline]
/// extract top left x, y and edge value of the patch
fn patch_disassemble(patch: PatchCompressed) -> PatchUnfolded {
    let edge = patch & 0b1111111111u32;
    let y = (patch & (0b1111111111u32 << 10)) >> 10;
    let x = (patch & (0b1111111111u32 << 20)) >> 20;

    (x as usize, y as usize, edge as usize)
}

fn part2(gsn: usize) -> Result<()> {
    let mut grid = Grid::new(gsn, EDGE);

    // iterate over edge sizes to fill the remaining cache points
    for edge in 2..=EDGE {
        // create a minigrid where 'edge' size each cell can be the top left cell of a patch of
        // 'edge'size.
        let minigrid = Grid::new(gsn, EDGE - edge + 1);

        // get an iterator over the cells of this patch
        minigrid.patch_iter(Cell {
            x: 1,
            y: 1,
        }, 1).unwrap(/* patch iterator will always exist coz edge is 1 */).for_each(|cell| {
            // make each cell as the top-left of a patch of size edge and find power
            let moving_patch_power = grid.patch_power(cell, edge)
                .unwrap(
                    /* patch_iter with 'edge' will always exist for any cell in minigrid and each
                     * cell is within a grid so cell_power will also work cell is within the grid
                     * */);
            assert_eq!(grid.patch_power_cache.insert(patch_accumulate((cell.x, cell.y, edge))
                    , moving_patch_power), None);
        })
    }
    let patch_compressed_id =
        grid.patch_power_cache.iter().max_by_key(|(_, v)| *v).unwrap(/* grid cannot be empty */).0;
    let (x, y, edge) = patch_disassemble(*patch_compressed_id);

    println!("part-2: [{x}, {y}, {edge}]");

    Ok(())
}
