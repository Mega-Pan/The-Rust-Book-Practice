let mut user1 = User{
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

user.email = String::from("anotheremail@example.com");