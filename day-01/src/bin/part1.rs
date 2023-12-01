use day_01::part1::process;
use miette::Context;

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
    let input = process(file).context("process part 1")?.trim().to_string();
    let total_floors: i16 = input.len().try_into().unwrap();
    let went_up_count: i16 = input.matches("(").count().try_into().unwrap();
    let went_down_count = total_floors - went_up_count;
    println!("{}", went_up_count - went_down_count);
    Ok(())
}
