use colored::*;

pub fn coloring(args: &str, count: u32) -> ColoredString {
    let count_mod = count % 5;
    match count_mod {
        0 => args.blue(),
        1 => args.red(),
        2 => args.green(),
        3 => args.yellow(),
        4 => args.white(),
        _ => args.white()
    }
}

