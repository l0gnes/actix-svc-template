use actix_web::get;

#[get("/ping")]
pub async fn get_ping_endpoint() -> String {
    // This ping endpoint doesn't really do anything
    "Pong!".to_string()
}

// An example of using the utoipa crate to generate docs automatically for an endpoint
#[cfg(feature = "docs")]
pub mod docs_example {

    // TODO: I removed utoipa because it was low-key terrible
    // this code will stay here until I find a better alternative.

    use std::sync::LazyLock;

    use actix_web::{get, web::Json};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default)]
    pub struct EmployeeSearchPayload {
        id: Option<u64>,
        name: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Employee {
        id: u64,
        name: String,
        age: u8,
    }

    static EMPLOYEE_LIST: LazyLock<Vec<Employee>> = LazyLock::new(|| {
        let mut employeeList: Vec<Employee> = vec![];

        employeeList.push(Employee {
            id: 1,
            name: "John Smith".to_string(),
            age: 22,
        });

        employeeList.push(Employee {
            id: 2,
            name: "Mark Roberts".to_string(),
            age: 20,
        });

        return employeeList;
    });

    #[get("/employees")]
    pub async fn search_employee_endpoint(
        payload: Option<Json<EmployeeSearchPayload>>,
    ) -> Json<Vec<Employee>> {
        let payload_unwrapped = payload.unwrap_or(Json(EmployeeSearchPayload::default()));

        let mut employees = (*EMPLOYEE_LIST).clone();

        if let Some(id) = payload_unwrapped.id {
            employees = employees
                .iter()
                .cloned()
                .filter(|e| e.id == id)
                .collect::<Vec<Employee>>();
        }

        if let Some(name) = &payload_unwrapped.name {
            employees = employees
                .iter()
                .cloned()
                .filter(|e| e.name.starts_with(name))
                .collect::<Vec<Employee>>();
        }

        return Json(employees);
    }
}
