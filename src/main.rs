


fn display_value(item: Vec<u32>) {

    for index in item {
        println!("{}", index);
    }
}

fn main() {

    let item = vec![1, 2, 3];

    display_value(item);
    display_value(item);

}
