use dod_benchmarks::boxed;

const INPUT: &str = include_str!("../../data_long.txt");

fn main() {
    let arena = bumpalo::Bump::new();
    boxed::parser(&arena, criterion::black_box(INPUT));
}
