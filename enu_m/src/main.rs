enum MyEnum {
    TypeA(i32),
    TypeB(String),
    TypeC(bool),
}

impl MyEnum {
    fn method_a(&self) {
        if let MyEnum::TypeA(value) = self {
            // 对于 TypeA，调用特定的方法
            println!("TypeA: {}", value);
        }
    }

    fn method_b(&self) {
        if let MyEnum::TypeB(value) = self {
            // 对于 TypeB，调用特定的方法
            println!("TypeB: {}", value);
        }
    }

    fn method_c(&self) {
        if let MyEnum::TypeC(value) = self {
            // 对于 TypeC，调用特定的方法
            println!("TypeC: {}", value);
        }
    }
}

fn main() {
    let vec: Vec<MyEnum> = vec![
        MyEnum::TypeA(42),
        MyEnum::TypeB(String::from("Hello")),
        MyEnum::TypeC(true),
    ];

    for item in vec {
        item.method_a();
        item.method_b();
        item.method_c();
    }
}
