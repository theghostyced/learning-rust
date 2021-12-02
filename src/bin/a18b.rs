
enum EmployeeType {
    Maintanence,
    Marketing,
    Managers,
    Supervisor,
    Kitchen,
    Assembly
}

enum EmployeeStatus {
  Active,
  Terminated
}

struct Employee {
  employee_type: EmployeeType,
  status: EmployeeStatus
}

fn can_access_building(employee: &Employee) -> Result<(), String> {
    match employee.status {
      EmployeeStatus::Terminated => return Err("employee has been terminated".to_owned()),
      _ => (),
    }

    match employee.employee_type {
        EmployeeType::Managers => Ok(()),
        EmployeeType::Maintanence => Ok(()),
        EmployeeType::Marketing => Ok(()),
        _ => Err("Access denied".to_owned())
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
  can_access_building(employee)?;
  println!("Employee has access to building");

  Ok(())
}

fn main() {
  let employee = Employee {
    employee_type: EmployeeType::Managers,
    status: EmployeeStatus::Active
  };

  match print_access(&employee) {
      Err(e) => println!("Employee cant access the building because: {:?}", e),
      _ => ()
  }
}