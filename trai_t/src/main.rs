trait MyTrait {
    fn my_method(&self);
}

struct TypeA {
    value: i32,
}

impl MyTrait for TypeA {
    fn my_method(&self) {
        println!("TypeA: {}", self.value);
    }
}

struct TypeB {
    value: String,
}

impl MyTrait for TypeB {
    fn my_method(&self) {
        println!("TypeB: {}", self.value);
    }
}

struct TypeC {
    value: bool,
}

impl MyTrait for TypeC {
    fn my_method(&self) {
        println!("TypeC: {}", self.value);
    }
}

fn main() {
    let vec: Vec<Box<dyn MyTrait>> = vec![
        Box::new(TypeA { value: 42 }),
        Box::new(TypeB { value: String::from("Hello") }),
        Box::new(TypeC { value: true }),
    ];

    for item in vec {
        item.my_method();
    }
}
