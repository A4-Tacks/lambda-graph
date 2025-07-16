use std::mem::replace;

use crate::{utils::MaxTo, Atom, Screen, Term};

#[derive(Debug, Clone)]
pub enum Error {
    UndefinedTerm(Term),
}

#[derive(Debug)]
struct Bar {
    name: Term,
    y: usize,
    end: usize,
}

#[derive(Debug, Default)]
pub struct GraphCtx {
    bars: Vec<Bar>,
    leaders: Vec<usize>,
    pub screen: Screen,
    offset: usize,
    offset_handle: usize,
    y: usize,
    fun_offset: Option<usize>,
    pub func_extra_unit: usize,
    pub call_extra_unit: usize,
}

impl GraphCtx {
    pub fn foo(&mut self, atom: &Atom) -> Result<(), Error> {
        match atom {
            Atom::Term(term) => {
                self.fun_offset = None;
                self.offset_handle = self.offset;
                let bar = self.bars.iter_mut()
                    .rfind(|bar| bar.name == **term)
                    .ok_or_else(|| Error::UndefinedTerm(term.clone()))?;
                let x = self.offset;
                bar.end = x+1;
                self.screen.bar(bar.y, x, self.y-bar.y);
            },
            Atom::Call(fun, arg) => {
                let base_y = self.y;

                self.foo(fun)?;
                self.offset += 2+self.call_extra_unit;
                self.fun_offset = None;

                let left_y = replace(&mut self.y, base_y);
                let left_handle = self.offset_handle;

                self.foo(arg)?;
                let right_y = self.y;
                let right_handle = self.offset_handle;

                self.y = self.y.max(left_y);
                self.screen.line(self.y, left_handle, self.offset_handle-left_handle+1);

                self.offset_handle = left_handle;

                self.screen.bar(left_y, left_handle, self.y-left_y);
                self.screen.bar(right_y, right_handle, self.y-right_y);
                self.screen.bar(self.y, left_handle, 2);
                self.y += 2;
            },
            Atom::Func(name, atom) => {
                let is_leader = self.fun_offset.is_none();
                if let Some(offset) = self.fun_offset.take() {
                    self.offset = offset;
                }

                if is_leader {
                    self.leaders.push(self.bars.len());
                }

                let y = self.y;
                let x = self.offset;
                self.add_bar(name.clone(), y);

                self.fun_offset = Some(self.offset);
                self.offset += 1;
                self.y += 2;

                self.foo(atom)?;

                self.sync_leader();
                self.ext_end_from_subfunc();

                let bar = self.bars.pop().unwrap();
                let end = bar.end.max(x+2);

                if is_leader {
                    self.leaders.pop().unwrap();
                }

                self.offset.max_to(end+self.func_extra_unit);
                self.screen.line(y, x, end-x+1);
            },
        }
        Ok(())
    }

    fn sync_leader(&mut self) {
        if let Some(&leader_i) = self.leaders.last() {
            let bar_group = &mut self.bars[leader_i..];
            let max_end = bar_group.iter()
                .map(|bar| bar.end)
                .max()
                .unwrap();
            for bar in bar_group {
                bar.end = max_end;
            }
        }
    }
    fn ext_end_from_subfunc(&mut self) {
        self.bars.iter_mut().rev().reduce(|child, parent| {
            if child.end > parent.end {
                parent.end = child.end
            }
            parent
        });
    }

    fn add_bar(&mut self, name: Term, y: usize) {
        self.bars.push(Bar {
            name,
            y,
            end: self.offset,
        });
    }
}
