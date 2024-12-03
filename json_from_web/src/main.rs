use json_from_web::get_instance;

#[tokio::main]
async fn main() {
    let instances = match get_instance().await {
        Ok(instances) => instances,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("Total instances: {}", instances.len());
    println!("Instance 1: {:?}", instances[0]);
}