use std::iter::repeat;

pub fn count_all_infinite_tuples() -> usize {
    std::iter::repeat(()).count()
}

pub fn sum_repeating_ones() -> i32 {
    std::iter::repeat(1).sum()
}

// It's actually quite sophisticated

// for example it nows that zipping an infinite iter with a finite iterator, will not be infinite
pub fn zip_infinite_and_finite() -> usize {
    repeat(1).zip(vec![1, 2, 3]).count()
}

// but chaining will
pub fn chain_infinite_and_finite() -> usize {
    repeat(1).chain(vec![1, 2, 3]).count()
}

// it's blind to variables though
pub fn chain_infinite_and_finite_with_variable() -> usize {
    let infinite = repeat(1);

    infinite.chain(vec![1, 2, 3]).count()
}
