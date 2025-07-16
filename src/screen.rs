use core::fmt;
use std::borrow::Cow;

use to_true::ToTrue;
use unicode_width::UnicodeWidthStr;

use crate::utils::Sign;

#[derive(Debug, Default)]
pub struct Screen {
    lines: Vec<Vec<bool>>,
}

pub struct OutputCtx<'a, W> {
    pub writer: W,
    pub has_color: bool,
    pub space: Option<&'a str>,
    pub solid: &'a str,
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

    pub fn print<W: fmt::Write>(&self, ctx: &mut OutputCtx<'_, W>) {
        let OutputCtx {
            writer: ref mut w,
            has_color,
            space,
            solid,
        } = *ctx;

        let mut color = false;
        let space = space.map(Cow::Borrowed)
            .unwrap_or_else(|| " ".repeat(solid.width()).into());

        for line in &self.lines {
            for &pos in line {
                if pos {
                    if has_color { color.to_true(|| write!(w, "\x1b[7m")); }
                    write!(w, "{solid}").unwrap();
                } else {
                    if has_color { color.to_false(|| write!(w, "\x1b[27m")); }
                    write!(w, "{space}").unwrap();
                }
            }

            color.to_false(|| write!(w, "\x1b[27m"));
            writeln!(w).unwrap();
        }
    }
}
