use colored::*;

#[macro_export]
macro_rules! color_println {
    ($color_count:expr, $fmt:expr, $($arg:tt)*) => (
        let output = format!($fmt, $($arg)*);
        println!("{}", coloring(&output, $color_count))
    );
}

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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn coloring_return_blue_when_the_surplus_of_count_divided_by_5_is_0() {
        assert_eq!(coloring("test", 0), "test".blue());
        assert_eq!(coloring("test", 5), "test".blue());
    }

    #[test]
    fn coloring_return_red_when_the_surplus_of_count_divided_by_5_is_1() {
        assert_eq!(coloring("test", 1), "test".red());
        assert_eq!(coloring("test", 6), "test".red());
    }

    #[test]
    fn coloring_return_green_when_the_surplus_of_count_divided_by_5_is_2() {
        assert_eq!(coloring("test", 2), "test".green());
        assert_eq!(coloring("test", 7), "test".green());
    }

    #[test]
    fn coloring_return_yellow_when_the_surplus_of_count_divided_by_5_is_3() {
        assert_eq!(coloring("test", 3), "test".yellow());
        assert_eq!(coloring("test", 8), "test".yellow());
    }

    #[test]
    fn coloring_return_white_when_the_surplus_of_count_divided_by_5_is_4() {
        assert_eq!(coloring("test", 4), "test".white());
        assert_eq!(coloring("test", 9), "test".white());
    }

}
