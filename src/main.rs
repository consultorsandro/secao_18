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

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel { 
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
fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
    
    // Class 295

/* fn book_for_one_night(entity: &impl Accommodation) { // Class 294
    entity.book("Piers", 1);
    // println!("{}", entity.get_description()); // This line would not work because `entity` is a trait object
    // Instead, we can use the method defined in the trait
println!("{}", entity.get_description()); */
}

fn main() {
    let mut hotel = Hotel::new("The Luxe"); // Class 294
    let mut airbnb = Airbnb::new("Peter");
    mix_and_match(&mut hotel, &mut airbnb, "Piers");
    println!("{:?}", hotel);
    println!("{:#?}", airbnb);
}

/*
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
*/
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
