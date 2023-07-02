pub mod print_a_Z {
    pub fn print(){
            for i in b'Z' ..= b'a' {
                println!("{}",i as char);
            }
        }
    }

fn main() {
    crate::print_a_Z::print();
}
