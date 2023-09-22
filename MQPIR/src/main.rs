
mod client;
mod server_offline;
mod globals;

fn main() {
    let num_of_hints = 10;
    let block_size = 32;
    let key_size = 32;
    // Get the terminal arguments
    let args: Vec<String> = std::env::args().collect();
    // Check if the argument is "client"
    if args.len() > 1 && args[1] == "client" {
        // Call the client() function
        client::preprocess(num_of_hints, block_size, key_size);
        return;
    }
    // Check if the argument is "serverOffline"
    if args.len() > 1 && args[1] == "serverOffline" {
        // Call the serverOffline() function
        server_offline::main();
        return;
    }
    // // Call the preprocess() function
    // client::preprocess(num_of_hints, block_size, key_size);
    // serverOffline::main();
    // // test global variables in config.toml
    // let config: toml::Value = from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
    // let ip_address = config["ip_address"]["address"].as_str().unwrap();
    // let port = config["port"]["value"].as_str().unwrap();

    // println!("ip_address: {}", ip_address);
    // println!("port: {}", port);

}