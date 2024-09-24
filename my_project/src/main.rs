#[derive(Debug, Default)]
pub struct Data {
    pub value: u32,
}

impl Data {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

fn main() {
    let mut data_list = vec![];

    for i in 0..10 {
        let mut value = i + 1;

        if i % 2 == 1 {
            value += 2 + i - 2;
        }
        data_list.push(Data::new(value));
    }

    // Print original data list
    for data in data_list.iter() {
        println!("{:?}", data);
    }

    // Remove elements where index is even
    let mut to_remove = vec![]; // Store indices to remove
    for (index, data) in data_list.iter().enumerate() {
        if index % 2 == 0 {
            to_remove.push(index); // Collect indices to remove
        }
    }

    // Remove elements from the end to avoid shifting issues
    for index in to_remove.iter().rev() {
        if let Some(removed_value) = data_list.remove(*index) {
            println!("removed: {}", removed_value.value);
        }
    }

    // Print updated data list
    for data in data_list.iter() {
        println!("{:?}", data);
    }
}
