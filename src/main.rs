#[derive(Debug)]
struct Employee {
    id: u32,
    tasks_completed: u32,
    time_spent: u32,
    performance_score: f64,
}

impl Employee {
	
	fn new(id: u32, tasks_completed: u32, time_spent: u32, performance_score: f64) -> Self {
		Employee {
			id: id,
			tasks_completed: tasks_completed,
			time_spent: time_spent,
			performance_score: performance_score,
		}
	}

	fn calc_performance(&mut self) -> () {
		let tasks_f64 = self.tasks_completed as f64;
		let time_f64 = self.time_spent as f64;
		
		if time_f64 == 0.0{
			self.performance_score = 0.0;
			} else {
				self.performance_score = tasks_f64 / time_f64;
			}
	}	
}

fn main() {
    println!("--- Employee Performance Tracker ---");
	
	let mut employees_data_vec: Vec<Employee> = Vec::new();
	employees_data_vec.push(Employee::new(101, 45, 40, 0.0));
	employees_data_vec.push(Employee::new(102, 60, 40, 0.0));
	employees_data_vec.push(Employee::new(103, 30, 40, 0.0));
	
    calculate_scores(&mut employees_data_vec);

    let average_score = find_average_score(&employees_data_vec);

    find_most_efficient_employee(&employees_data_vec);
    
    println!("\nTotal Average Team Score: {:.2}", average_score);
}

fn calculate_scores(employees: &mut Vec<Employee>) -> () {

    for employee in employees.iter_mut() {
		employee.calc_performance();
    }
}

fn find_average_score(employees: &Vec<Employee>) -> f64 {
	let sum: f64 = employees.iter()
		.map(|employee| employee.performance_score)
		.sum();
	sum / employees.len() as f64
}

fn find_most_efficient_employee(employees: &Vec<Employee>) {
	if let Some(best_employee) = employees.iter()
	        .max_by_key(|employee| {
	            employee.performance_score.total_cmp(&0.0) 
	        })
	    {
	        println!("Most Efficient Employee ID: {}", best_employee.id);
	        println!("Highest Score: {:.2}", best_employee.performance_score);
	    } else {
	        println!("Error: Employee list is empty.");
	    }
}