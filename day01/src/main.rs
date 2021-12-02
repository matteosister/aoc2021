fn main() {
    let values: Vec<i32> = utils::from_file("day01/input");
    println!("step 1: {}", step_1(&values));
    println!("step 2: {}", step_2(&values));
}

fn step_1(values: &Vec<i32>) -> i32 {
    values
        .iter()
        .fold((None, 0), |(previous, increases), v| {
            previous
                .map(|prev| (Some(v), increases + if prev < v { 1 } else { 0 }))
                .unwrap_or_else(|| (Some(v), 0))
        })
        .1
}

fn step_2(values: &Vec<i32>) -> i32 {
    step_1(&values.windows(3).map(|v| v.iter().sum()).collect())
}
