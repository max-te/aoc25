use divan::{Bencher, black_box};

fn load_input(day: u8) -> String {
    let input_file_path = format!("input/2025/day{day}.txt");
    if let Ok(input) = std::fs::read_to_string(input_file_path) {
        return input;
    }

    let session = std::env::var("AOC_SESSION")
        .expect("AOC_SESSION should be set when inputs are not stored locally");
    let client = ureq::Agent::new_with_defaults();
    let url = format!("https://adventofcode.com/2025/day/{day}/input");

    let mut response = client
        .get(&url)
        .header("Cookie", &format!("session={}", session))
        .call()
        .expect("should be able to send request");
    assert_eq!(response.status(), 200);
    response
        .body_mut()
        .read_to_string()
        .expect("should be able to read response body")
}

macro_rules! benchmark_days {
    ($($day:literal),*) => {
        paste::paste! {
            $(
            #[divan::bench]
            fn [<day_ $day _part_1>](bencher: Bencher) {
                use aoc25::[<day $day>]::{part1};
                bencher.with_inputs(|| {
                    load_input($day)
                })
                .bench_refs(|input|
                    part1(black_box(&input))
                );
            }

            #[divan::bench]
            fn [<day_ $day _part_2>](bencher: Bencher) {
                use aoc25::[<day $day>]::{part2};
                bencher.with_inputs(|| {
                    load_input($day)
                })
                .bench_refs(|input|
                    part2(black_box(&input))
                );
            }
            )*

        }
    };
}

benchmark_days!(1, 2, 3, 4, 5);

fn main() {
    // Run registered benchmarks.
    divan::main();
}
