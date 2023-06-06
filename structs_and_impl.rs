
// Structures
fn main() {
    let emp = Employees {
        name: String::from("John"),
        company: String::from("Ruah Tech"),
        age: 35
    };
    println!("{:#?}",emp);
    // accessing particular element
    println!("Name: {}",emp.name);
    println!("Company: {}",emp.company);
    println!("Age: {}",emp.age);
    println!("{}",emp.fn_details()); // accessing `self` method
    println!("{}",Employees::static_method()); // accessing static method
}

#[derive(Debug)]
struct Employees {
    name: String,
    company: String,
    age: u32
}

impl Employees {
    fn fn_details(&self) -> String {
        format!("name: {}, company: {}, age: {}",&self.name,&self.company, &self.age)
    }

    /// using static method without self
    fn static_method() -> String {
        String::from("This method is from static")
    }
}