use dod_benchmarks::standard;

const INPUT: &str = include_str!("../../data_long.txt");

fn main() {
    let arena = bumpalo::Bump::new();
    standard::parser(&arena, criterion::black_box(INPUT));
}
