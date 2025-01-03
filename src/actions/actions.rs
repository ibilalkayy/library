use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Result, BufReader, BufRead};
use crate::commands::commands::{BookDetails, BookTitle};

impl BookDetails {
    pub fn write_data(&self) -> Result<()> {
        let mut file_data = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("library_data.csv")?;

        if file_data.metadata()?.len() == 0 {
            writeln!(file_data, "Title,Author,ISBN,Checkout,Return")?;
        }
        writeln!(file_data, "{},{},{}", self.title, self.author, self.isbn)?;
        Ok(())
    }
}

impl BookTitle {
    pub fn remove_data(&self) {
        let file_data = OpenOptions::new()
            .read(true)
            .open("library_data.csv")
            .expect("Unable to open file");

        let reader = BufReader::new(file_data);

        let mut found = false;
        let mut lines_to_keep = Vec::new();

        for line_result in reader.lines() {
            let line = line_result.expect("Error reading line");
            if line.contains(&self.title) {
                found = true;
                break;
            } else {
                lines_to_keep.push(line);
            }
        }

        if found {
            let mut file = File::create("library_data.csv").expect("unable to open the file");
            for line in lines_to_keep {
                writeln!(file, "{}", line).expect("Error writing to a file");
            }
        } else {
            println!("Book with title '{}' not found.", self.title);
        }
    }
    
    pub fn search_data(&self) {
        let file_data = OpenOptions::new()
            .read(true)
            .open("library_data.csv")
            .expect("Unable to open file");

        let reader = BufReader::new(file_data);
        let mut found = false;
        
        for line_result in reader.lines() {
            let line = line_result.expect("Error reading line");
            if line.contains(&self.title) {
                println!("Title,Author,ISBN,Checkout,Return\n{}", line);
                found = true;
                break;
            }
        }

        if !found {
            println!("Book with title '{}' not found.", self.title);
        }
    }

    pub fn checkout_book(&self) {
        let file_path = "library_data.csv";

        // Open the file for reading
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect("Unable to open file for reading");
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        let mut found = false;

        // Process each line
        for line_result in reader.lines() {
            let mut line = line_result.expect("Error reading line");

            if line.contains(&self.title) {
                if !line.ends_with(",true") { // Avoid appending if already done
                    line.push_str(",true");
                }
                found = true;
            }

            lines.push(line);
        }

        if !found {
            println!("Book with title '{}' not found.", self.title);
            return;
        }

        // Open the file for writing and truncate its content
        let mut file_for_writing = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .expect("Unable to open file for writing");

        // Write all lines back to the file
        for line in lines {
            writeln!(file_for_writing, "{}", line).expect("Unable to write to file");
        }
    }

    pub fn return_book(&self) {
        let file_path = "library_data.csv";

        // Open the file for reading
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect("Unable to open file for reading");
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        let mut found = false;

        // Process each line
        for line_result in reader.lines() {
            let mut line = line_result.expect("Error reading line");

            if line.contains(&self.title) {
                // Split the line by commas to identify columns
                let mut columns: Vec<&str> = line.split(',').collect();

                if columns.len() >= 5 {
                    // Replace the "Return" column value or append it if missing
                    columns[4] = "true"; // Assuming "Return" is the 5th column
                } else {
                    // If there is no "Return" column, add it
                    columns.push("true");
                }

                // Reconstruct the modified line
                line = columns.join(",");

                found = true;
            }

            lines.push(line);
        }

        if !found {
            println!("Book with title '{}' not found.", self.title);
            return;
        }

        // Open the file for writing and truncate its content
        let mut file_for_writing = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)
            .expect("Unable to open file for writing");

        // Write all lines back to the file
        for line in lines {
            writeln!(file_for_writing, "{}", line).expect("Unable to write to file");
        }
    }
}

pub fn list_data() {
    let mut file_data = OpenOptions::new()
        .read(true)
        .open("library_data.csv").unwrap();
    
    let mut content = String::new();
    file_data.read_to_string(&mut content).unwrap();

    if content.is_empty() {
        println!("No data is available in the library");
    } else {
        print!("{}", content);
    }
}