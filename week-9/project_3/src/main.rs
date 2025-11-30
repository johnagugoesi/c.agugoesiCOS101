use std::fs::File;
use std::io::Write;

fn main() {
    let names = vec![
        "Adejogun Alaranwu Oladipo".to_string(),
        "Mustafa Akbar Bamidele".to_string(),
        "Okwuosa Collins Ogbonna".to_string(),
        "Adenrele Jemoh Adewsi".to_string(),
        "Ogunwusi Fatai Olayiwola".to_string(),
    ];

    let ministries = vec![
        "Internal Affairs".to_string(),
        "Justice".to_string(),
        "Defense".to_string(),
        "Power & Steel".to_string(),
        "Petroleum".to_string(),
    ];

    let zones = vec![
        "South West".to_string(),
        "North East".to_string(),
        "South South".to_string(),
        "South West".to_string(),
        "South East".to_string(),
    ];

    println!("MERGED COMMISSIONERS LIST");
    println!("---------------------------------------------------");

    let mut file = File::create("commissioners.txt").unwrap();

    for i in 0..names.len() {
        let name = &names[i];
        let ministry = &ministries[i];
        let zone = &zones[i];

        // Print
        println!("Name: {}", name);
        println!("Ministry: {}", ministry);
        println!("Zone: {}", zone);
        println!("---------------------------------------------------");

        // Build line manually WITHOUT format!
        let mut line = String::new();
        line.push_str(name);
        line.push_str(" | ");
        line.push_str(ministry);
        line.push_str(" | ");
        line.push_str(zone);
        line.push_str("\n");

        file.write_all(line.as_bytes()).unwrap();
    }

    println!("Saved to commissioners.txt");
}
