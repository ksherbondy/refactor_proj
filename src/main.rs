// Assume this code is in your main.rs file

use std::io;

// Data structure to hold employee metrics
struct Employee {
    id: u32,
    tasks_completed: u32,
    time_spent: u32, // Time in hours
    performance_score: f64,
}

fn main() {
    println!("--- Employee Performance Tracker ---");
    
    // 1. Initial Data Setup (This should be generated more efficiently)
    let employees_data: Vec<Employee> = vec![
        Employee { id: 101, tasks_completed: 45, time_spent: 40, performance_score: 0.0 },
        Employee { id: 102, tasks_completed: 60, time_spent: 40, performance_score: 0.0 },
        Employee { id: 103, tasks_completed: 30, time_spent: 40, performance_score: 0.0 },
    ];
    
    // 2. Perform Calculations (This function has efficiency and type issues)
    let mut processed_employees = calculate_scores(employees_data);

    // 3. Find Global Metrics (This function uses basic iteration)
    let average_score = find_average_score(&processed_employees);

    // 4. Find the Best Employee (This is a separate, simple loop)
    find_most_efficient_employee(&processed_employees);
    
    println!("\nTotal Average Team Score: {:.2}", average_score);
}

// ----------------------------------------------------
// Functions to Refactor
// ----------------------------------------------------

// Calculates individual efficiency (tasks / time) and updates the score field
fn calculate_scores(mut employees: Vec<Employee>) -> Vec<Employee> {
    
    let mut total_score = 0.0; // FLaw 1: This accumulating variable is misplaced here

    for employee in &mut employees {
        // FLaw 2: Division of integers that should be floating point math
        let efficiency = employee.tasks_completed / employee.time_spent; 
        
        employee.performance_score = efficiency as f64;
        total_score += employee.performance_score; // FLaw 1: Misplaced side effect
    }
    employees
}

// Calculates the average score (This should be done via iterators)
fn find_average_score(employees: &Vec<Employee>) -> f64 {
    let mut sum = 0.0;
    for employee in employees {
        sum += employee.performance_score;
    }
    // FLaw 3: Manual casting and division
    sum / employees.len() as f64
}

// Finds the employee with the highest performance score
fn find_most_efficient_employee(employees: &Vec<Employee>) {
    let mut best_id = 0;
    let mut highest_score = 0.0;
    
    // FLaw 4: Uses a simple for loop, which is fine, but can be improved with iterators
    for employee in employees {
        if employee.performance_score > highest_score {
            highest_score = employee.performance_score;
            best_id = employee.id;
        }
    }
    println!("Most Efficient Employee ID: {}", best_id);
}
