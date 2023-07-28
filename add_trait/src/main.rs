trait Add<RHS = Self> {
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    
    fn add(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    
    let result = a.add(b);
    
    println!("Result: {}", result);
}
