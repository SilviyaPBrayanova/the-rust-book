//https://www.sheshbabu.com/posts/rust-module-system/
mod config;
mod routes;
mod models;

use routes::health_route;

fn main() {
    println!("Main!");
    config::print_config();
    health_route::print_health_route();
    routes::user_route::print_user_route();
}
