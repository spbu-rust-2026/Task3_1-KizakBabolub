pub fn mirror_double(numbers: &[i32]) -> Vec<i32> {
    numbers.into_iter().map(|&x| x * 2).rev().collect()
}
