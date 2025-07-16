use std::{fmt::Display, fs, path::Path, thread};

use dissimilar::{diff, Chunk};
use lambda_graph::{expr, GraphCtx, OutputCtx};

struct Guard<S: Display>(S);
impl<S: Display> Drop for Guard<S> {
    fn drop(&mut self) {
        if thread::panicking() {
            eprintln!("{} panic!", self.0)
        }
    }
}

#[test]
fn main() {
    let this_file = file!();
    let path = Path::new(this_file);
    let dir = path.parent().unwrap();
    let mut datas_path = dir.to_path_buf();
    datas_path.push("datas");
    let data = fs::read_to_string(datas_path).unwrap();

    data.split("\n.").skip(1)
        .map(|case| case.trim().split_once('\n').unwrap())
        .map(|(s, e)| (s.trim(), e.trim()))
        .for_each(|(src, expected)|
    {
        let _guard = Guard(src);
        let expr = expr(src).expect(src);
        let ctx = &mut GraphCtx::default();
        ctx.foo(&expr).unwrap();

        let octx = &mut OutputCtx {
            writer: String::new(),
            has_color: false,
            space: Some(" "),
            solid: "x",
        };
        ctx.screen.print(octx);
        octx.writer.truncate(octx.writer.trim_end().len());

        if expected != &octx.writer {
            eprintln!("-- expected --\n{expected}");
            eprintln!("-- output --\n{}", octx.writer);
            eprintln!("-- diff --");
            for diff_chunk in diff(expected, &octx.writer) {
                match diff_chunk {
                    Chunk::Equal(s) => eprint!("{s}"),
                    Chunk::Delete(s) => eprint!("\x1b[41m{s}\x1b[m\x1b[K"),
                    Chunk::Insert(s) => eprint!("\x1b[42m{s}\x1b[m\x1b[K"),
                }
            }
            panic!("test failed, src: {src}");
        }
    });
}
