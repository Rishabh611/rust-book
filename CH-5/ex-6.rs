fn main(){
    let user1 = User{
        active: true,
        username: String::from("someusername"),
        email::String::from("someone@example.com"),
        sign_in_count:1,
    }
    let user2 = User{
        email: String::from("another@example.com"),
        ..user1,
    }
}