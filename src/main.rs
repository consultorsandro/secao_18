use std::collections::HashMap;
trait Accommodation { // Class 292
    fn get_description(&self) -> String {
        String::from("This is a place to stay")
    }

    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel { //Class 291
    fn get_description(&self) -> String { 
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct Airbnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl Airbnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for Airbnb {
/*     fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    } */
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights)); // push
    }
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    //println!("{}", hotel.get_description());
    println!("{}", hotel.summarize()); // Class 293
    hotel.book("Piers", 5);
    println!("{:?}", hotel);

    let mut airbnb = Airbnb::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{:#?}", airbnb);
}

/*
use std::collections::HashMap;
trait Accommodation {
    fn get_description(&self) -> String {
        String::from("This is a place to stay")
    }

    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel { //Class 291
    fn get_description(&self) -> String { 
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct Airbnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl Airbnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for Airbnb {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights)); // push
    }
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.get_description());
    hotel.book("Piers", 5);
    println!("{:?}", hotel);

    let mut airbnb = Airbnb::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{:#?}", airbnb);
}

*/
