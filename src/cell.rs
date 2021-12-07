use rand::thread_rng;
use rand::Rng;

use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub enum Position {
  LeftTop,
  TopSide,
  RightTop,
  LeftSide,
  Center,
  RightSide,
  LeftBottom,
  BottomSide,
  RightBottom,
}

impl Display for Position {
  fn fmt<'a>(&self, formatter: &mut Formatter<'a>) -> Result<(), FmtError> {
    let mut s = String::new();
    match self {
      Position::LeftTop => s.push_str("LeftTop    "),
      Position::TopSide => s.push_str("TopSide    "),
      Position::RightTop => s.push_str("RightTop   "),
      Position::LeftSide => s.push_str("LeftSide   "),
      Position::Center => s.push_str("Center     "),
      Position::RightSide => s.push_str("RightSide  "),
      Position::LeftBottom => s.push_str("LeftBottom "),
      Position::BottomSide => s.push_str("BottomSide "),
      Position::RightBottom => s.push_str("RightBottom"),
    }

    formatter.write_str(&s)?;
    Ok(())
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
  position: Position,
  neighbours: [Option<usize>; 8],
  alive: bool,
}

impl Cell {
  pub fn new(position: Position, neighbours: [Option<usize>; 8]) -> Self {
    Cell {
      position,
      neighbours,
      alive: false,
    }
  }

  pub fn change_state(&mut self, state: bool) {
    self.alive = state;
  }
}

pub struct Grid {
  length: usize,
  width: usize,
  cells: Vec<Cell>,
}

impl Grid {
  pub fn new(width: usize, height: usize) -> Self {
    let length = width * height;
    let mut vec: Vec<Cell> = Vec::with_capacity(length);

    for i in 0..length {
      let mut neighbours: [Option<usize>; 8] = [None; 8];
      let pos = calculate_position(i, width, height);
      match pos {
        Position::LeftTop => {
          neighbours[4] = Some(i + 1);
          neighbours[6] = Some(i + width);
          neighbours[7] = Some(i + width + 1);
        }
        Position::RightTop => {
          neighbours[3] = Some(i - 1);
          neighbours[5] = Some(i + width - 1);
          neighbours[6] = Some(i + width);
        }
        Position::LeftBottom => {
          neighbours[1] = Some(i - width);
          neighbours[2] = Some(i - width + 1);
          neighbours[4] = Some(i + 1);
        }
        Position::RightBottom => {
          neighbours[0] = Some(i - width - 1);
          neighbours[1] = Some(i - width);
          neighbours[3] = Some(i - 1);
        }
        Position::TopSide => {
          neighbours[3] = Some(i - 1);
          neighbours[4] = Some(i + 1);
          neighbours[5] = Some(i + width - 1);
          neighbours[6] = Some(i + width);
          neighbours[7] = Some(i + width + 1);
        }
        Position::BottomSide => {
          neighbours[0] = Some(i - width - 1);
          neighbours[1] = Some(i - width);
          neighbours[2] = Some(i - width + 1);
          neighbours[3] = Some(i - 1);
          neighbours[4] = Some(i + 1);
        }
        Position::LeftSide => {
          neighbours[1] = Some(i - width);
          neighbours[2] = Some(i - width + 1);
          neighbours[4] = Some(i + 1);
          neighbours[6] = Some(i + width);
          neighbours[7] = Some(i + 1);
        }
        Position::RightSide => {
          neighbours[0] = Some(i - width - 1);
          neighbours[1] = Some(i - width);
          neighbours[3] = Some(i - 1);
          neighbours[5] = Some(i + width - 1);
          neighbours[6] = Some(i + width);
        }
        Position::Center => {
          neighbours[0] = Some(i - width - 1);
          neighbours[1] = Some(i - width);
          neighbours[2] = Some(i - width + 1);
          neighbours[3] = Some(i - 1);
          neighbours[4] = Some(i + 1);
          neighbours[5] = Some(i + width - 1);
          neighbours[6] = Some(i + width);
          neighbours[7] = Some(i + width + 1);
        }
      }
      let cell = Cell::new(pos, neighbours);
      vec.push(cell);
    }

    Self {
      length,
      width,
      cells: vec,
    }
  }

  pub fn new_randomized(width: usize, height: usize) -> Self {
    let mut grid = Grid::new(width, height);
    let mut rng = thread_rng();
    for indx in 0..grid.length {
      let float: f64 = rng.gen_range(0f64..=1f64);
      if float > 0.5 {
        grid.change_state(indx, true);
      }
    }

    grid
  }

  #[inline(always)]
  pub fn change_state(&mut self, index: usize, is_alive: bool) {
    self.cells[index].alive = is_alive;
  }

  pub fn compute_next_generation(&mut self) -> Result<(), &'static str> {
    let mut new_generation: Vec<Cell> = Vec::with_capacity(self.cells.len());
    let mut next_generation_deadcount: usize = 0;
    for indx in 0..self.cells.len() {
      let mut alive = 0;
      let mut cell = self.cells[indx];
      // Check all neighbours
      for maybe_neighbour in cell.neighbours {
        if let Some(neighbour_idx) = maybe_neighbour {
          if self.cells[neighbour_idx].alive {
            alive += 1;
          }
        }
      }
      cell.change_state(alive >= 3 && alive <= 5);
      if !cell.alive {
        next_generation_deadcount += 1;
      }
      new_generation.push(cell);
    }

    self.cells = new_generation;

    if next_generation_deadcount == self.cells.len() {
      Err("Whole generation died")
    } else {
      Ok(())
    }
  }
}

impl Display for Grid {
  fn fmt<'a>(&self, formatter: &mut Formatter<'a>) -> Result<(), FmtError> {
    let mut string = String::new();

    for i in 0..self.cells.len() {
      let cell = self.cells[i];
      if i % self.width == 0 {
        string.push('\n');
      }
      if cell.alive {
        string.push('1');
      } else {
        string.push('0');
      }
      string.push(' ');
    }
    formatter.write_str(&string)?;

    Ok(())
  }
}

#[inline(always)]
fn calculate_position(index: usize, width: usize, height: usize) -> Position {
  let length = width * height;
  if index == 0 {
    Position::LeftTop
  } else if index == width - 1 {
    Position::RightTop
  } else if index == length - width {
    Position::LeftBottom
  } else if index == length - 1 {
    Position::RightBottom
  } else if index < width {
    Position::TopSide
  } else if index % width == 0 {
    Position::LeftSide
  } else if index % width == width - 1 {
    Position::RightSide
  } else if index >= length - width && index < length {
    Position::BottomSide
  } else {
    Position::Center
  }
}
