use std::ops::FnOnce;
use std::time::Instant;

mod part1;
mod part2;

fn benchmark_function<T, F: FnOnce() -> T>(f: F) -> T {
    let start_time = Instant::now();

    let result = f();

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    result
}

fn main() {
    let part1 = benchmark_function(part1::main);
    println!("Part1 Answer -> {part1}");
    let part2 = benchmark_function(part2::main);
    println!("Part2 Answer -> {part2}");
}
