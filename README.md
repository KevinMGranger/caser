# caser

Change text between PascalCase, camelCase, and snake_case.

Can be used as a library or a command-line program.

## CLI Usage

Usage: `caser CASE term [terms]`

Where `CASE` is one of pascal, camel, snake, or sentence.

For reference:

ThisIsPascalCase
thisIsCamelCase
this_is_snake_case
This is sentence case. It starts with a capital after each sentence-ending punctuation mark.

Terms are converted and printed one per line.

Example: `caser snake ConvertToSnakeCase` would print `convert_to_snake_case`.

## Library Usage

To use as a library, use one of the `Case` variants to convert a `&str`.

`assert_eq!(caser::Case::SnakeCase.transform("PascalToSnake"), "pascal_to_snake")`

## Intent & See Also

This program is meant to be a simple get-the-job done utility.

If you need something more substantial, you might consider [change-case](https://crates.io/crates/change-case) or, heck, [heck](https://crates.io/crates/heck).