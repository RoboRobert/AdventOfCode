#[derive(Debug)]
struct SpringRecord {
    record: String,
    spring_order: Vec<u128>,
}

pub fn puzzle1(input: &str) -> u128 {
    //Parses each line into a more useful spring_record data format
    let mut record_vec: Vec<SpringRecord> = Vec::new();
    for line in input.lines() {
        let order_vec_binding: Vec<&str> = line.split_whitespace().nth(1).unwrap().split(',').collect();
        let mut order_vec: Vec<u128> = Vec::new();
        for current in order_vec_binding {
            order_vec.push(current.parse::<u128>().unwrap());
        }

        let current_record = line.split_whitespace().nth(0).unwrap();

        let new_record: SpringRecord = SpringRecord { record: current_record.to_string(), spring_order: order_vec };
        record_vec.push(new_record);
    }

    let mut sum: u128 = 0;

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    return 0;
}