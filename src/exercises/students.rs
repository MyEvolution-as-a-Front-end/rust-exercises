use std::io::{self, Write};

struct Student {
    name: String,
    grades: Vec<f64>,
}

fn print_bests(students: &Vec<Student>) {
    students.iter().for_each(|student| {
        let best_grades_student = student.grades.iter().filter(|grade| **grade >= 7.0);
        if best_grades_student.count() >= 2 {
            println!("{} is the best student!", student.name);
        }
    });
}

fn students() -> () {
    let mut qty_students = String::new();
    let mut students: Vec<Student> = Vec::new();

    print!("Quantity of students? ");
    io::stdout().flush().expect("Could not flush stdout");

    io::stdin()
        .read_line(&mut qty_students)
        .expect("Could not read line");

    let qty_students = qty_students
        .trim()
        .parse::<usize>()
        .expect("Could not parse");

    for _ in 0..qty_students {
        let mut name = String::new();
        let mut grades = Vec::new();

        print!("Student name? ");
        io::stdout().flush().expect("Could not flush stdout");

        io::stdin()
            .read_line(&mut name)
            .expect("Could not read line");

        for _ in 0..4 {
            let mut grade = String::new();

            print!("Grade? ");
            io::stdout().flush().expect("Could not flush stdout");

            io::stdin()
                .read_line(&mut grade)
                .expect("Could not read line");

            let grade = grade.trim().parse::<f64>().expect("Could not parse");

            grades.push(grade);
        }

        students.push(Student {
            name: name.trim().to_string(),
            grades,
        });
    }

    print_bests(&students);
}
