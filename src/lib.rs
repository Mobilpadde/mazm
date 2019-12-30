use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate js_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Closed = 1,
    Open = 2,
    Top = 4,
    Left = 8,
    Right = 16,
    Bottom = 32,
    Linked = 64,
}

#[wasm_bindgen]
pub struct Mazm {
    width: usize,
    height: usize,
    time: usize,
    cells: Vec<usize>,
    unvisited: Vec<usize>,
    current: usize,
}

#[wasm_bindgen]
impl Mazm {
    fn get_row_col(&self, idx: usize) -> (usize, usize) {
        ((idx / self.width) as usize, (idx % self.width) as usize)
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn get_neighbours(&self, curr: usize) -> Vec<usize> {
        let mut neighbours: Vec<usize> = Vec::new();
        let (row, col) = self.get_row_col(curr); 
        let len = self.cells.len();

        if row > 0 {
            let idx = self.get_index(row-1, col);
            if
                curr > idx &&
                self.cells[idx] & Cell::Closed as usize == 0 &&
                self.cells[idx] & Cell::Linked as usize == 0
            {
                neighbours.push(idx);
            }
        }

        if col > 0 {
            let idx = self.get_index(row, col-1);
            if
                curr > idx &&
                self.cells[idx] & Cell::Closed as usize == 0 &&
                self.cells[idx] & Cell::Linked as usize == 0
            {
                neighbours.push(idx);
            }
        }

        if col + 1 < len {
            let idx = self.get_index(row, col+1);
            if
                idx < len &&
                self.cells[idx] & Cell::Closed as usize == 0 &&
                self.cells[idx] & Cell::Linked as usize == 0
            {
                neighbours.push(idx);
            }
        }

        if row + 1 < len {
            let idx = self.get_index(row+1, col);
            if
                idx < len &&
                self.cells[idx] & Cell::Closed as usize == 0 &&
                self.cells[idx] & Cell::Linked as usize == 0
            {
                neighbours.push(idx);
            }
        }

        neighbours
    }

    fn link(&mut self, curr: usize, next: usize) {
        let (row, col) = self.get_row_col(curr);
        let (n_row, n_col) = self.get_row_col(next);

        if row > n_row {
            self.cells[curr] |= Cell::Top as usize;
            self.cells[next] |= Cell::Bottom as usize;

            self.cells[curr] |= Cell::Linked as usize;
            self.cells[next] |= Cell::Linked as usize;
        }

        if col > n_col {
            self.cells[curr] |= Cell::Left as usize;
            self.cells[next] |= Cell::Right as usize;

            self.cells[curr] |= Cell::Linked as usize;
            self.cells[next] |= Cell::Linked as usize;
        }

        if col < n_col {
            self.cells[curr] |= Cell::Right as usize;
            self.cells[next] |= Cell::Left as usize;

            self.cells[curr] |= Cell::Linked as usize;
            self.cells[next] |= Cell::Linked as usize;
        }

        if row < n_row {
            self.cells[curr] |= Cell::Bottom as usize;
            self.cells[next] |= Cell::Top as usize;

            self.cells[curr] |= Cell::Linked as usize;
            self.cells[next] |= Cell::Linked as usize;
        }
    }

    fn recursive_backtracker(&mut self) {
        if self.unvisited.len() > 0 { // self.current < self.width * self.height {
            let unvisited: Vec<usize> = self.get_neighbours(self.current);

            if unvisited.len() > 0 {
                let neighbour = unvisited[(js_sys::Math::random() * unvisited.len() as f64) as usize];
                self.link(self.current, neighbour);
                self.current = neighbour;
            } else {
                let unv = self.unvisited.pop();
                match unv {
                    Some(x) => { self.current = x as usize; },
                    None => {},
                }
                //self.current = self.width * self.height;
                
                //for idx in 0..self.cells.len() {
                    //let visited = self.get_neighbours(idx);

                    //if visited.len() > 0 {
                        //self.current = idx;
                        //let neighbour = visited[(js_sys::Math::random() * visited.len() as f64) as usize];
                        //self.link(self.current, neighbour);
                        //break;
                    //}
                //}
            }
        }
    }

    pub fn new(dat: &[u8], sz: usize) -> Mazm {
        let width = sz;
        let height = sz;

        let mut cells: Vec<usize> = (0..(width * height))
            .map(|_| { Cell::Closed as usize })
            .collect();

        let mut unvisited: Vec<usize> = (0..(width * height))
            .map(|i| i)
            .collect();

        let mut selectables: Vec<usize> = Vec::new();
        let mut idx = 0;
        for i in 0..(width * height) {
            if dat[idx + 0] as usize + dat[idx + 1] as usize + dat[idx + 2] as usize > (255 * 3) / 2 {
                cells[i] = Cell::Open as usize;
                selectables.push(i);
            }

            idx += 4;
        }

        let current = selectables[(js_sys::Math::random() * (selectables.len()) as f64) as usize];
        Mazm {
            width,
            height,
            time: 1,
            cells,
            unvisited,
            current, 
        }
    }

    pub fn tick(&mut self, spd: usize) {
        for _ in 0..spd {
            self.recursive_backtracker();
        }

        self.time += spd; 
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Mazm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let mut classes = "".to_string();
                if cell & (Cell::Closed as usize) == Cell::Closed as usize {
                    classes = "closed".into();
                }

                if cell & (Cell::Top as usize) == Cell::Top as usize {
                    classes = "t".into();
                }

                if cell & (Cell::Left as usize) == Cell::Left as usize {
                    classes.push_str(" l".into())
                }

                if cell & (Cell::Right as usize) == Cell::Right as usize {
                    classes.push_str(" r".into())
                }

                if cell & (Cell::Bottom as usize) == Cell::Bottom as usize {
                    classes.push_str(" b".into())
                }

                write!(f, "<i class=\"{}\"></i>", classes)?;
            }

            write!(f, "<br/>")?;
        }

        Ok(())
    }
}
