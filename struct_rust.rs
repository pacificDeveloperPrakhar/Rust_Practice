struct Post {
    name: String,
    empId: i32,
    cmpId: i32,
    roles: String,
}

impl Post {
    fn company_find(&self) {
        println!("Finding company for employee: {}", self.name);
    }
}

fn main() {
    let employee = Post {
        name: String::from("Alice"),
        empId: 101,
        cmpId: 202,
        roles: String::from("Developer"),
    };

    println!(
        "Employee: {}, ID: {}, Company ID: {}, Role: {}",
        employee.name, employee.empId, employee.cmpId, employee.roles
    );

    employee.company_find();
}
