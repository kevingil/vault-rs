mod db;
use db::*;

// crl clears the terminal screen using [23 command
fn clr() {
    print!("{}[2J", 27 as char);
}


fn main() {
    // Database connection
    let conn = init_database().expect("Failed to initialize database");
    clr();
    
    let ascii = r#"
    __     __          _ _     ____  ____  
    \ \   / /_ _ _   _| | |_  |  _ \/ ___| 
     \ \ / / _` | | | | | __| | |_) \___ \ 
      \ V / (_| | |_| | | |_  |  _ < ___) |
       \_/ \__,_|\__,_|_|\__| |_| \_\____/ 
                                           
    "#;
    println!("{ascii}");

    // Main prompt loop
    loop {
        println!("Secret Manager Menu:");
        println!("1. New");
        println!("2. List");
        println!("3. Search");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

     
        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                write_password_to_db(
                    &conn,
                    &entry.service,
                    &entry.username,
                    &entry.password,
                )
                .expect("Failed to write to the database");
                println!("Entry added successfully.");
    
            }
            "2" => {
                clr();
                let services = read_passwords_from_db(&conn).unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);    
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
    - Username : {} 
    - Password : {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" =>{
                clr();
                let search = prompt("Search by service name:");
                match search_service_by_name(&conn, &search) {
                    Ok(Some(entry)) => {
                        println!(
                            "Service = {}
                - Username : {} 
                - Password : {:?}",
                            entry.service, entry.username, entry.password
                        );
                    }
                    Ok(None) => {
                        println!("Service not found.");
                    }
                    Err(err) => {
                        eprintln!("Error searching for service: {}", err);
                    }
                }
            }

            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."), // Handle invalid input
        }
        println!("\n\n");
    }
}
