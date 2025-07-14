use std::{env::args, fmt::Display, io::stdin, process::exit};

use getopts_macro::getopts_options;
use lambda_graph::{expr, Error, GraphCtx, Term};
use line_column::line_column;

fn main() {
    let options = getopts_options! {
        -l, --lambda        "output raw lambda";
        -s, --simple        "output simple paren lambda";
        -p, --pretty        "output pretty indent lambda";
        -n, --no-graph      "no output graph";
        -u, --unit=unit     "draw unit [default: 2 spaces]";
        -e, --func-extra=n  "after func extra units";
        -c, --call-extra=n  "after call extra units";
        -h, --help          "show help message";
        -v, --version       "show version";
    };
    let matches = match options.parse(args().skip(1)) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            exit(2)
        },
    };
    if matches.opt_present("help") {
        let desc = env!("CARGO_PKG_DESCRIPTION");
        let brief = options.short_usage(env!("CARGO_BIN_NAME"));
        let brief = format!("{brief} [expr]..\n{desc}");
        print!("{}", options.usage(&brief));
        exit(0)
    }
    if matches.opt_present("version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        exit(0)
    }

    let lambda = matches.opt_present("lambda");
    let simple = matches.opt_present("simple");
    let pretty = matches.opt_present("pretty");
    let graph = !matches.opt_present("no-graph");
    let unit = matches.opt_str("unit").unwrap_or("  ".into());
    let func_extra = matches.opt_get_default("func-extra", 0)
        .unwrap_or_else(|e| {
            let arg = matches.opt_str("func-extra").unwrap();
            eprintln!("ArgError: on arg {arg:?} {e}");
            exit(2)
        });
    let call_extra = matches.opt_get_default("call-extra", 0)
        .unwrap_or_else(|e| {
            let arg = matches.opt_str("call-extra").unwrap();
            eprintln!("ArgError: on arg {arg:?} {e}");
            exit(2)
        });

    matches.free.is_empty()
        .then(|| stdin().lines().map(Result::unwrap))
        .into_iter()
        .flatten()
        .chain(matches.free)
        .for_each(|s|
    {
        let expr = match expr(&s) {
            Ok(expr) => expr,
            Err(e) => {
                error(&s, e.location.offset, e);
                exit(3)
            },
        };
        if lambda {
            println!("{expr}")
        }
        if simple {
            println!("{expr:o}")
        }
        if pretty {
            println!("{expr:#}")
        }
        if !graph {
            return;
        }

        let mut ctx = GraphCtx::default();
        ctx.func_extra_unit = func_extra;
        ctx.call_extra_unit = call_extra;

        match ctx.foo(&expr) {
            Ok(()) => {},
            Err(Error::UndefinedTerm(Term(name, i))) => {
                let (line, col) = line_column(&s, i);
                #[allow(clippy::redundant_closure_call)]
                (|e| error(&s, i, e))(format_args!(
                    "error: undefined term `{name}` at {line}:{col}"
                ));
                exit(4)
            },
        }

        ctx.screen.print(&unit);
    });
}

fn error(s: &str, i: usize, e: impl Display) {
    let near = s[i..]
        .chars()
        .take_while(char_classes::any!(^" \t\r\n"))
        .take(5)
        .collect::<String>();
    if i >= s.len() {
        eprint!("at eof ");
    } else if !near.is_empty() {
        eprint!("near `{near}` ");
    }
    eprintln!("{e}");
}
