use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};


fn main() -> std::io::Result<()> {
    let file = File::open("extension.txt")?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let split_data: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        data.push(split_data);

    }
    println!("{:?}", data);
    
    let mut converted_data: Vec<Vec<String>> = Vec::new();

    for record in &data {
        let mut converted_record: Vec<String> = Vec::new();
        for (i, field) in record.iter().enumerate() {
                if let Ok(num) = field.trim().parse::<u64>() {
                    converted_record.push((num + 5).to_string()); 
                } else {
                    converted_record.push(field.clone());
                }
            }
        converted_data.push(converted_record);
    }
    for record in &converted_data {
        println!("{:?}", record);
    }

    let mut output_file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("C:/Users/Guts/Documents/extension_modificado.txt")?;

for record in &converted_data {
    let line = record.join(", ") + "\n";
    output_file.write_all(line.as_bytes())?;
}
    Ok(())
}
