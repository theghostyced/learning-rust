
struct Student {
  name: String,
  locker_assignment: Option<i32>
}

fn main() {
  let  student: Student = Student {
    name: String::from("James"),
    locker_assignment: Some(12)
  };

  match student.locker_assignment {
      Some(assignment) => println!("Locker assignment is: {:?}", assignment),
      None => println!("No assignment made!")
  }

  println!("Student name: {:?}", student.name)
}