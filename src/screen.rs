use to_true::ToTrue;

use crate::utils::Sign;

#[derive(Debug, Default)]
pub struct Screen {
    lines: Vec<Vec<bool>>,
}

impl Screen {
    pub fn bar(&mut self, y: usize, x: usize, downs: usize) {
        for i in 0..downs {
            *self.lines.sign(y+i).sign(x) = true;
        }
    }

    pub fn line(&mut self, y: usize, x: usize, len: usize) {
        for i in 0..len {
            *self.lines.sign(y).sign(x+i) = true;
        }
    }

    pub fn rev_y(&mut self) {
        self.lines.reverse();
    }

    pub fn rev_x(&mut self) {
        let max_x = self.lines.iter().map(Vec::len).max().unwrap_or_default();
        if max_x != 0 {
            self.lines.iter_mut().for_each(|line| {
                line.sign(max_x-1);
                line.reverse();
            });
        }
    }

    pub fn print(&self, space: &str) {
        let mut color = false;

        for line in &self.lines {
            for &pos in line {
                if pos {
                    color.to_true(|| print!("\x1b[7m"));
                } else {
                    color.to_false(|| print!("\x1b[27m"));
                }

                print!("{space}")
            }

            color.to_false(|| print!("\x1b[27m"));
            println!();
        }
    }
}
