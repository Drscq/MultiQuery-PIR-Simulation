
mod client;
mod server_offline;
mod server_online;
mod globals;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Check if the argument is "client"
    if args.len() > 1 && args[1] == "client" {
        // Call the client() function
        client::preprocess();
        client::search_hint();
        return;
    }
    // Check if the argument is "serverOffline"
    if args.len() > 1 && args[1] == "serverOffline" {
        // Call the serverOffline() function
        server_offline::main();
        return;
    }
    // Check if the argument is "serverOnline"
    if args.len() > 1 && args[1] == "serverOnline" {
        // Call the serverOnline() function
        server_online::handle_client();
        return;
    }

}