struct Node {
    value: i32,
    min: i32,
    max: i32
}

fn main() {

    let mut input: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
    input.sort(); // After sorting: [-2, -1, 0, 0, 1, 2]

    let target: i32 = 0;
    let k_value: i32 = 4;

    println!("Input array is: {:?}", input);
    println!("Number of items in input array: {}", input.len());
    println!("Number of items in each solution: {}", k_value);

    let min = input.iter().min();
    if let Some(value) = min {
        println!("Min value is: {}", value);
    }

    let max = input.iter().max();
    if let Some(value) = max {
        println!("Max value is: {}", value);
    }

    println!("Target is: {}", target);

}
