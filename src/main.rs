use std::io;

fn main() {
    // List of subjects we're interested in
    let subject_list: [&str; 6]= ["English", "Maths", "Kiswahili", "Science", "Social Studies", "Religion"];

    // where we will store the mark for each subject as the user inputs
    let mut subject_mark: String = String::new();

    // where the marks will be stored as arrays of integers
    let mut input_marks:[u32; 6] = [0; 6];
    
    // loop 6 times asking user to imput subject marks add add the to the input_marks array
    for round in 0..6 {
        println!("Enter mark for {}: ", subject_list[round]);

        // read the user input
        io::stdin().read_line(&mut subject_mark).expect("Failed to read subject mark");
        input_marks[round] = subject_mark.trim().parse().expect("Please enter a valid mark");

        // clear the string handling the user input
        subject_mark.clear();

    }

    // generate grades for each subject
    generate_grades(input_marks, subject_list);

    // display average grade
    let average_grade: String = generate_average(&mut input_marks);

    println!("Average grade: {}", average_grade);
}


// pass by value
fn generate_grades(marks: [u32; 6], subjects: [&str; 6]) {
    for mark in 0..6 {
        if marks[mark] >= 80 {
            println!("Grade for {} is A, score is {}", subjects[mark], marks[mark]);
        } else if marks[mark] >= 65 && marks[mark] <= 79 {
            println!("Grade for {} is B, score is {}", subjects[mark], marks[mark]);
        } else if marks[mark] >= 30 && marks[mark] <= 64 {
            println!("Grade for {} is C, score is {}", subjects[mark], marks[mark]);
        } else {
            println!("Grade for {} is D, score is {}", subjects[mark], marks[mark]);
        }
    }
}

// pass by refference
fn generate_average(marks: &mut [u32; 6]) -> String {
    let mut total: u32 = 0;
    for element in 0..6 {
        total += marks[element];
    }

    let average = total / 6;

    if average >= 80 {
        String::from("A")
    } else if average >= 65 && average <= 79 {
        String::from("B")
    } else if average >= 30 && average <= 64 {
        String::from("C")
    } else {
        String::from("D")
    }
}
