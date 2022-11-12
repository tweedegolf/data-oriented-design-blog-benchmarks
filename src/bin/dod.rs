use dod_benchmarks::dod;

const INPUT: &str = include_str!("../../data_long.txt");

fn main() {
    let arena = bumpalo::Bump::new();
    dod::parser(&arena, criterion::black_box(INPUT));
}
