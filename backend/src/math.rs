//popular math operations 
pub fn factorial(n: f64) -> f64 {
    if !is_positive_integer(n){
        return -1.
    }
    if n == 0. {
        1.
    } else {
        n * factorial(n - 1.)
    }
}

pub fn is_positive_integer(num: f64) -> bool {
    num > 0.0 && num.fract() == 0.0
}

pub fn average(nums: Vec<f64>) -> f64{
    let mut sum = 0.;
    for num in nums.clone(){
        sum += num;
    }
    sum/nums.len() as f64
}

pub fn standard_deviation(nums: Vec<f64>) -> f64{
    let mut sum_of_squares = 0.;
    let average = average(nums.clone());
    for num in nums.clone(){
        sum_of_squares += (num - average)*(num - average)
    }
    (sum_of_squares/nums.len() as f64).sqrt()
}

pub fn sum(nums: Vec<f64>) -> f64{
    let mut sum = 0.;
    for num in nums{
        sum += num;
    }
    sum
}

pub fn max(nums: Vec<f64>) -> f64 {
    let mut max = nums[0];
    for num in nums{
        if num > max{
            max = num;
        }
    }
    max
}

pub fn min(nums: Vec<f64>) -> f64 {
    let mut min = nums[0];
    for num in nums{
        if num < min{
            min = num;
        }
    }
    min
}

pub fn median(nums: Vec<f64>) -> f64 {
    let mut v = nums.clone();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len()/2]
}