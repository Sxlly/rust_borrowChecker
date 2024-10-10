// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24


fn display_value(item: Vec<u32>) { //declare display_value variable -> takes 1 parameter -> of Vector DT -> with elements that are unsigned 32 bit values

    for index in item { // loop through vector{array} printing to terminal every index that holds a value
        println!("{}", index); //print index fed into -> string "{}"
    }
}

fn main() { //decalre main method

    let item = vec![1, 2, 3]; //declare variable item that holds a vector -> [1,2,3]

    display_value(item); //call method display_value -> with passing parameter -> item
    display_value(item); //call method display_value -> with passing parameter -> item

}
