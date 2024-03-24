pub fn square_of_sum(n: u32) -> u32 {
    let sum_of_n: u32 = (1..=n).sum();
    let result = sum_of_n * sum_of_n;
    result
}

pub fn sum_of_squares(n: u32) -> u32 {
    let sum_of_squares_of_n: u32 = (1..=n).map(|s| s * s).sum();
    sum_of_squares_of_n
}

pub fn difference(n: u32) -> u32 {
    let a = square_of_sum(n);
    let b = sum_of_squares(n);
    a - b
}
