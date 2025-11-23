use std::io;

fn main() {
    let mut profession = String::new();
    let mut role = String::new();

    println!("Enter Profession (Office Administrator, Academic, Lawyer, Teacher):");
    io::stdin().read_line(&mut profession).unwrap();

    println!("Enter Job Title:");
    io::stdin().read_line(&mut role).unwrap();

    // Convert both to Title Case so matching works
    let profession = to_title_case(profession.trim());
    let role = to_title_case(role.trim());

    let aps = get_aps_level(&profession, &role);
    println!("APS Level: {}", aps);
}

fn get_aps_level(profession: &str, role: &str) -> &'static str {

    // Arrays of APS levels
    let aps_levels = [
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    // Vectors: job titles for each profession

    let office_admin = vec![
        vec!["Intern"],                               // APS 1-2
        vec!["Administrator"],                        // APS 3-5
        vec!["Senior Administrator"],                 // APS 5-8
        vec!["Office Manager"],                       // EL1
        vec!["Director"],                             // EL2
        vec!["CEO"],                                  // SES
    ];

    let academic = vec![
        vec![],                                       // APS 1-2 (empty)
        vec!["Research Assistant"],                   // APS 3-5
        vec!["PhD Candidate"],                        // APS 5-8
        vec!["Post-Doc Researcher"],                  // EL1
        vec!["Senior Lecturer"],                      // EL2
        vec!["Dean"],                                 // SES
    ];

    let lawyer = vec![
        vec!["Paralegal"],                            // APS 1-2
        vec!["Junior Associate"],                     // APS 3-5
        vec!["Associate"],                            // APS 5-8
        vec!["Senior Associate 1-2"],                 // EL1
        vec!["Senior Associate 3-4"],                 // EL2
        vec!["Partner"],                              // SES
    ];

    let teacher = vec![
        vec!["Placement"],                            // APS 1-2
        vec!["Classroom Teacher"],                    // APS 3-5
        vec!["Snr Teacher"],                          // APS 5-8
        vec!["Leading Teacher"],                      // EL1
        vec!["Deputy Principal"],                     // EL2
        vec!["Principal"],                            // SES
    ];

    // Select the correct profession vector
    let selected = match profession {
        "Office Administrator" => office_admin,
        "Academic" => academic,
        "Lawyer" => lawyer,
        "Teacher" => teacher,
        _ => return "Invalid Profession",
    };

    // Search role in each APS level slot
    for i in 0..aps_levels.len() {
        if selected[i].contains(&role) {
            return aps_levels[i];
        }
    }

    "Role Not Found"
}


// Converts "senior lecturer" → "Senior Lecturer"

fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
