use std::fs;
fn main() {
    //Scan through text file and print out each line
    let file_path = "../resources/sample.txt";
    let _content = fs::read_to_string(file_path).expect("Error reading the file contents");
    println!("Content: \n");
}
