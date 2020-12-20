use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert(String::from("apple"), 5);
    basket.insert(String::from("strawberry"), 5);

    basket
}

fn main() {
    let basket: HashMap<String, u32> = fruit_basket();
    for (key, value) in &basket {
        println!("{}: {}", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket
            .values()
            .sum::<u32>() >= 5);
    }
}