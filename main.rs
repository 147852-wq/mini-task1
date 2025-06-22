use std::io;

// Custom function to calculate average
fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    total_marks / num_subjects as f32
}

// Custom function to assign grade
fn assign_grade(avg: f32) -> char {
    if avg >= 90.0 {
        'A'
    } else if avg >= 75.0 {
        'B'
    } else if avg >= 60.0 {
        'C'
    } else {
        'D'
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks_input = String::new();
    let mut num_subjects_input = String::new();

    println!("Enter student's name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Enter total marks obtained:");
    io::stdin().read_line(&mut total_marks_input).expect("Failed to read marks");

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects_input).expect("Failed to read subjects");

    let total_marks: f32 = total_marks_input.trim().parse().expect("Please enter valid number");
    let num_subjects: u32 = num_subjects_input.trim().parse().expect("Please enter valid number");

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    println!("\nStudent: {}", name.trim());
    println!("Average Marks: {:.2}", average);
    println!("Grade: {}", grade);
}
