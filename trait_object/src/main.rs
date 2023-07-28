trait Math {
    fn calculate(&self) -> i32;
}

struct Adder {
    a: i32,
    b: i32,
}

impl Math for Adder {
    fn calculate(&self) -> i32 {
        self.a + self.b
    }
}

struct Multiplier {
    a: i32,
    b: i32,
}

impl Math for Multiplier {
    fn calculate(&self) -> i32 {
        self.a * self.b
    }
}

fn main() {
    let adder: Box<dyn Math> = Box::new(Adder { a: 5, b: 10 });
    let multiplier: Box<dyn Math> = Box::new(Multiplier { a: 5, b: 10 });
    
    println!("Adder result: {}", adder.calculate());
    println!("Multiplier result: {}", multiplier.calculate());
}
