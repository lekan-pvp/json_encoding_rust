use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("Rustocean"),
        email: String::from("rusocean@example.com"),
        password: String::from("i4mrU$t0c34n"),
    };

    let serialized = serde_json::to_string(&user).unwrap();

    println!("serialized = {}", serialized);

    let deserialize: User = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialize);

    let users = vec![
        User {
            id: 1,
            name: String::from("Gopher"),
            email: String::from("gopher@example.com"),
            password: String::from("I4mg0ph3r"),
        },
        User {
            id: 2,
            name: String::from("Rusocean"),
            email: String::from("rustocean@example.com"),
            password: String::from("i4MrU$t0c34n"),
        }
    ];

    let serialized_vector = serde_json::to_string(&users).unwrap();

    println!("serialized vector = {}", serialized_vector);

    let deserialized_vector: Vec<User> = serde_json::from_str(&serialized_vector).unwrap();

    println!("deserialized vector = {:?}", deserialized_vector);
}
