pub mod print_A_z {
    pub mod print_A_z_1{
        pub fn print(){
            for i in b'A' ..= b'z' {
                println!("{}",i as char);
            }
        }
    }
    
}
fn main() {
    crate::print_A_z::print_A_z_1::print();
}

