use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

impl Universe {
  pub fn new(width: u32, height: u32) -> Universe {
    Universe {
      width: width,
      height: height,
      cells: vec![Cell::Dead; (width * height) as usize],
    }
  }

  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
    for (row, col) in cells.iter().cloned() {
      let idx = self.get_index(row, col);
      self.cells[idx] = Cell::Alive;
    }
  }

  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }
}

// this creates the display of the blocks
impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}
