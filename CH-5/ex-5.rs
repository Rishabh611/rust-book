fn build_user(email: String, username: String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}