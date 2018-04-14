/// 1. Declare lazily evaluated constant
/// Declares a lazily evaluated constant HashMap. The HashMap
/// will be evaluated once and stored behind a global static
/// reference

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

fn main() {
    let access = PRIVILEGES.get("James");
    println!("James: {:?}", access);
}

/// 2. Maintain global mutable state
/// Declares some global state using lazy_static. Since lazy_static creates
/// a globally available static ref we also need to wrap our state in a Mutex
/// to allow mutation. The Mutex ensures the state cannot be simultaneously
/// accessed by multiple threads, preventing race conditions. A MutexGuard
/// must be acquired to read or mutate the value stored in a Mutex

#[macro_use]
extern crate error_chain;

use std::sync::Mutex;

error_chain! {
    foreign_links {
    }
}

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn run() -> Result<()> {
    insert("apple")?;
    insert("banana")?;
    insert("orange")?;
    {
        // acquire access
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;

        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
        // release access
    }
    insert("grape")?;
    Ok(())
}

/// 3. Verify and extract login from an email address
/// Validates that an email address is formatted correctly, and extracts everything
/// before the @ symbol

extern crate regex;

use regex::Regex;

fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
        ").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("login").map(|login| login.as_str())
    })
}
