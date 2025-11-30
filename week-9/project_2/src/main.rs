use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() {
    let students = vec![
        Student {
            name: "Olufunke Morin".to_string(),
            matric: "ACD2018001".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Abiyo".to_string(),
            matric: "BCD2018002".to_string(),
            department: "Economics".to_string(),
            level: 200,
        },
        Student {
            name: "Jane Doe".to_string(),
            matric: "CCD2018003".to_string(),
            department: "Computer".to_string(),
            level: 400,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric: "DDD2018004".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Bassey Edemoh".to_string(),
            matric: "EED2018005".to_string(),
            department: "Mechanical".to_string(),
            level: 300,
        },
    ];

    println!("STUDENT RECORDS");
    println!("----------------------------------------------");

    for s in &students {
        println!("Name: {}", s.name);
        println!("Matric No: {}", s.matric);
        println!("Department: {}", s.department);
        println!("Level: {}", s.level);
        println!("----------------------------------------------");
    }

    let mut file = File::create("students.txt").unwrap();

    for s in &students {
        let line = format!(
            "{} | {} | {} | {}\n",
            s.name, s.matric, s.department, s.level
        );
        file.write_all(line.as_bytes()).unwrap();
    }

    println!("Saved to students.txt");
}
