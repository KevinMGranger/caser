use caser::Case;

fn case(string: &str) -> Option<Case> {
    Some(match string {
        "pascal" => Case::PascalCase,
        "camel" => Case::CamelCase,
        "snake" => Case::SnakeCase,
        _ => return None
    })
}

const HELP: &'static str = r"Usage: caser CASE term [terms]

Where CASE is one of pascal, camel, or snake.

For reference:

ThisIsPascalCase
thisIsCamelCase
this_is_snake_case

Terms are converted and printed one per line.

Example: `caser snake ConvertToSnakeCase` would print `convert_to_snake_case`";

fn main() {
    let mut args = std::env::args().into_iter().skip(1);

    let first_arg = args.next().unwrap();

    if first_arg == "-h" || first_arg == "--help" {
        println!("{}", HELP);
        return;
    }

    let case = case(&first_arg).unwrap_or_else(|| {
        println!("{}\nUnknown case '{}'", HELP, first_arg);
        std::process::exit(1)
    });

    for term in args {
        println!("{}", case.transform(&term));
    }
}