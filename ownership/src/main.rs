#![allow(unused)]
fn main() {
    //"Rust"值被a变量所拥有，那么a变量就是"Rust"值的所有者
    //每个值同时只能有一个所有者
    //当所有者超出作用域时，该值将被删除
    
    let a = "Rust"; // a 进入作用域

    //a 从创建伊始就开始有效，然后有效期持续到它离开作用域为止

    let b = String::from("Rust");    
    string_ownership(b);              

    let c = 5;                       
    copy_c(c);                      
                                     // i32 是 Copy 的，在后面可继续使用 c

    //不可变引用
    /*
    let d = String::from("Rust");   
    change_d(&d);
    */   


    //可变引用
    let mut e = String::from("Rust"); // 同一作用域中只能有一个可变引用
    change_e(&mut e);

    
    //可变引用与不可变引用不能同时存在
    /* 
    let r1 = &e; // right
    let r2 = &e; // right
    let r3 = &mut e; //error
    */

                
    

} // a 移出作用域 -> b -> c -> d,c 的值被转移走了


fn string_ownership(to_string: String) { // to_string 进入作用域
    println!("{}", to_string);
} // to_string 移出作用域并调用 `drop` 函数,占用的内存被释放

fn copy_c(to_i32: i32) { 
    println!("{}", to_i32);
} 

/* 
fn change_d(some_string: &String) {  
    some_string.push_str("-world");
} 
*/

fn change_e(some_string: &mut String) { 
    some_string.push_str("-world");
} 
