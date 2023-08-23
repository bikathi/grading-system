use std::io;

fn main() {
    // List of subjects we're interested in
    let subject_list: [&str; 6]= ["English", "Maths", "Kiswahili", "Science", "Social Studies", "Religion"];

    // Number of grades that we want to take
    let number_of_grades: u32 = 6;

    // where we will store the mark for each subject as the user inputs
    let mut subject_mark: String = String::new();

    // where the marks will be stored as arrays of integers
    let input_marks:[i32; 6] = [0; 6];
    println!("Hello, world!");
}
