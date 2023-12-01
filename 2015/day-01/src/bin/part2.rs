use day_01::part2::process;
use miette::Context;
use std::process::exit;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let input = process(file).context("process part 2")?.trim().to_string();
    let mut current_position = 0;
    input.chars().enumerate().for_each(|c| {
        if c.1 == '(' {
            current_position += 1;
        } else {
            current_position -= 1;
        }
        if current_position == -1 {
            println!("{}", c.0 + 1);
            exit(0);
        }
    });
    Ok(())
}
