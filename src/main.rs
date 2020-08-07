struct NodeValue {
    value: i32,
    min: i32,
    max: i32,
    left: Node,
    right: Node
}

enum Node {
    Value(Box<NodeValue>),
    Nil
}

use crate::Node::{Value, Nil};

impl NodeValue {
    fn to_graphviz(& self) -> String {

        let value = format!("{{ <f0> node | {} }}", self.value);
        let min = format!("{{ <f1> min | {} }}", self.min);
        let max = format!("{{ <f2> max | {} }}", self.max);
        let left = "{ <f3> left | <f4> ptr }".to_string();
        let right = "{ <f5> right | <f6> ptr}".to_string();

        let graphviz = format!(
            "{{ {} | {} | {} | {} | {} }}",
            value,
            min,
            max,
            left,
            right
        );

        return graphviz;
    }
}

impl NodeValue {
    pub fn new(value: i32, min: i32, max: i32) -> Self {
        return NodeValue {
            value: value,
            min: min,
            max: max,
            left: Nil,
            right: Nil
        };
    }
}

fn main() {

    let mut input: Vec<i32> = vec![1, 0, -1, 0, -2, 2];
    input.sort();

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

    let mut index: u32 = 0;
    for item in input.iter() {
        println!("Considering: {}th item: {}", index, item);

        let mut max_if_we_take_it: i32 = 0;
        let mut min_if_we_take_it: i32 = 0;
        let mut max_if_we_dont_take_it: i32 = 0;
        let mut min_if_we_dont_take_it: i32 = 0;

        println!(
            "If we take this item, then, \
            \nthe minimum value of the collection could be: {}, \
            \nthe maximum value of the collection could be: {}",
            min_if_we_take_it,
            max_if_we_take_it
        );

        println!(
            "If we dont take this item, then, \
            \nthe minimum value of the collection could be: {}, \
            \nthe maximum value of the collection could be: {}",
            min_if_we_dont_take_it,
            max_if_we_dont_take_it
        );

        index = index + 1;
    }

}
