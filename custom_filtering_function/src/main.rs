fn main() {
    let collection = vec![1, 2, 3, 4, 32, 6];
    let filter_condition = FilterCondition {item: 32};

    println!("Result: {:?}", custom_filter(collection.clone(), &filter_condition));
}

struct FilterCondition {
    item: u32
}

impl FilterCondition {
    fn is_match(&self, item: &u32) -> bool {
        self.item == *item
    }
}

fn custom_filter(items: Vec<u32>, filter_condition: &FilterCondition) -> Vec<u32> {
    items
        .into_iter()
        .filter(|x| filter_condition.is_match(x))
        .collect()
}
