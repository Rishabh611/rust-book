fn main(){
    let mut user1 = User{
        active: true,
        username: String::from("someusename123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    }
    user1.email = String:from("anotheremail@example.com");
}