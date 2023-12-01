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
    let mut sum_till_now = 0;
    input.lines().enumerate().for_each(|line| {
        sum_till_now += get_number(line.1);
    });
    println!("{}", sum_till_now);
    Ok(())
}

fn get_number(line: &str) -> i32 {
    let mut digits: [i32; 2] = [-1, -1];
    for c in line.chars() {
        if c.is_digit(10) {
            digits[0] = c.to_digit(10).unwrap() as i32;
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            digits[1] = c.to_digit(10).unwrap() as i32;
            break;
        }
    }
    return digits[0] * 10 + digits[1];
}
