use std::collections::HashMap;
use std::io;

struct Student {
    id: u32,
    name: String,
    age: u32,
}

struct StudentManager {
    students: HashMap<u32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    fn remove_student(&mut self, id: u32) {
        self.students.remove(&id);
    }

    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    fn update_student(&mut self, id: u32, name: String, age: u32) {
        if let Some(student) = self.students.get_mut(&id) {
            student.name = name;
            student.age = age;
        }
    }
}

fn main() {
    let mut manager = StudentManager::new();

    loop {
        println!("请选择操作:");
        println!("1. 添加学生");
        println!("2. 删除学生");
        println!("3. 查找学生");
        println!("4. 更新学生信息");
        println!("5. 退出");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("读取用户输入失败");

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                println!("请输入学生信息：");

                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("读取用户输入失败");
                let id = id.trim().parse::<u32>().unwrap();

                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("读取用户输入失败");
                let name = name.trim().to_string();

                let mut age = String::new();
                io::stdin().read_line(&mut age).expect("读取用户输入失败");
                let age = age.trim().parse::<u32>().unwrap();

                let student = Student { id, name, age };
                manager.add_student(student);
                println!("学生添加成功!");
            }
            Ok(2) => {
                println!("请输入要删除的学生ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("读取用户输入失败");
                let id = id.trim().parse::<u32>().unwrap();

                manager.remove_student(id);
                println!("学生删除成功!");
            }
            Ok(3) => {
                println!("请输入要查找的学生ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("读取用户输入失败");
                let id = id.trim().parse::<u32>().unwrap();

                if let Some(student) = manager.get_student(id) {
                    println!("学生信息:");
                    println!("ID: {}", student.id);
                    println!("姓名: {}", student.name);
                    println!("年龄: {}", student.age);
                } else {
                    println!("找不到该学生!");
                }
            }
            Ok(4) => {
                println!("请输入要更新的学生ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("读取用户输入失败");
                let id = id.trim().parse::<u32>().unwrap();

                println!("请输入新的学生姓名:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("读取用户输入失败");
                let name = name.trim().to_string();

                println!("请输入新的学生年龄:");
                let mut age = String::new();
                io::stdin().read_line(&mut age).expect("读取用户输入失败");
                let age = age.trim().parse::<u32>().unwrap();

                manager.update_student(id, name, age);
                println!("学生信息更新成功!");
            }
            Ok(5) => {
                println!("退出程序");
                break;
            }
            _ => {
                println!("无效的选择，请重新输入!");
            }
        }
    }
}
