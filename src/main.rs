use std::io;

fn display_menu(){
    println!("\n--- TO-DO List Meny ---");
    println!("1. View Tasks");
    println!("2. Add a new task");
    println!("3. Remove a task");
    println!("4. Exit");
    println!("Please choose an option");
}


fn main(){
    let mut tasks: Vec<String>= Vec::new();

    loop{
        display_menu();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice : i32= match choice.trim().parse(){
            Ok(num) => num, 
            Err(_)=>{
                println!("Please enter a correct number.");
                continue;
            }
        };
        
        match choice{
            1=> view_tasks(&tasks),
            2=> add_tasks(&mut tasks),
            3=> remove_taks(&mut tasks),           
            4=> {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, Please select a valid number.")
        }
    }
}


fn view_tasks(tasks: &Vec<String>){
    if tasks.is_empty(){
        println!("Your todo list is empty");
    }else{
        println!("\n Your Tasks: ");
        for (index, task) in tasks.iter().enumerate(){
            println!("{}. {}", index +1, task);
        }
    }
}

fn add_tasks(tasks: &mut Vec<String>){
    println!("Enter a new task: ");

    let mut new_task= String::new();
    io::stdin()
        .read_line(&mut new_task)
        .expect("Failed to read input");

    tasks.push(new_task.trim().to_string());
    
    println!("Task Added!");
}

fn remove_taks(tasks: &mut Vec<String>){
    if tasks.is_empty(){
        println!("Your to-do list is empty, nothing to remove");
        return; 
    }
    println!("Entert the number of the task you want to remove");

    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Failed to read inpout");

    let task_number: usize = match task_number.trim().parse(){
        Ok(num)=> num,
        Err(_)=> {
            println!("Please enter a valied number");
            return;
        }
    };

    if task_number==0 || task_number> tasks.len(){
        println!("Invalid task number");
    } else{
        tasks.remove(task_number -1);
        println!("Task removed");
    }

}

