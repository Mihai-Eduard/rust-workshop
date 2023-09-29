struct Person{
    name: String,
    age: usize,
}

impl Person{
    fn print_age(&self){
        println!("{} is {} years old.", self.name, self.age);
    }

    fn birthday(&mut self){
        self.age += 1;
    }

    fn to_tuple(self) -> (String, usize){
        (self.name, self.age)
    }

    fn new(name: String, age: usize) -> Person {
        Person{
            name, 
            age,
        }
    }
}

fn main() {
    // Instantiate and use the struct
    let mut p = Person{name: String::from("John"), age: 32};

    print!("{} is {} years old", p.name, p.age);
    p.age += 1;
    println!(" and next year he will be {}.", p.age);
    p.print_age();
    println!("{} is {} years old.", p.name, p.age);

    p.birthday();
    p.print_age();

    let (name, age) = p.to_tuple();
    println!("{} is {} years old.", name, age);

    let p = Person::new(String::from("John"), 32);
    p.print_age();

    let Person{name, age} = p;
    println!("{} is {} years old.", name, age);
}
