pub fn print_user_route() {
    println!("user_route");
    super::health_route::print_health_route();
    crate::models::user_model::print_user_model();
}