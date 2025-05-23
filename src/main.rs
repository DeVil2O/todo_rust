#[derive(Debug)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

fn add_todo(todos: &mut Vec<Todo>, title: String) {
    let todo = Todo {
        id: (todos.len() + 1).to_string(),
        title,
        completed: false,
    };
    todos.push(todo);
}

fn get_todo(todos: &Vec<Todo>, id: String) -> Option<&Todo> {
    todos.iter().find(|todo| todo.id == id)
}

fn update_todo(todos: &mut Vec<Todo>, id: String, title: String) {
    let todo = todos.iter_mut().find(|todo| todo.id == id);
    if let Some(todo) = todo {
        todo.title = title;
    }
}

fn update_todo_status(todos: &mut Vec<Todo>, id: String, completed: bool) {
    let todo = todos.iter_mut().find(|todo| todo.id == id);
    if let Some(todo) = todo {
        todo.completed = completed;
    }
}

fn delete_todo(todos: &mut Vec<Todo>, id: String) {
    todos.retain(|todo| todo.id != id);
}

fn main() {
    let mut todos = vec![
        Todo {
            id: "1".to_string(),
            title: "Buy groceries".to_string(),
            completed: false,
        },
    ];

    println!("{:?}", todos);

    add_todo(&mut todos, "Play Games".to_string());

    println!("{:?}", todos);

    let todo = get_todo(&todos, "1".to_string());
    println!("{:?}", todo);
    
    update_todo(&mut todos, "1".to_string(), "Buy groceries and Play Games".to_string());
    println!("{:?}", todos);

    update_todo_status(&mut todos, "1".to_string(), true);
    println!("{:?}", todos);
    
    delete_todo(&mut todos, "1".to_string());
    println!("{:?}", todos);
}
