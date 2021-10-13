use log::error;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        error!("please input 3 args.");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];
    println!(
        "[protocol] {} [role] {} [address] {}",
        protocol, role, address
    );
    match protocol {
        "tcp" => match role {
            "server" => {}
            "client" => {}
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {}
            "client" => {}
            _ => {
                missing_role();
            }
        },
        _ => {
            error!("args error.");
            std::process::exit(1);
        }
    }
}

fn missing_role() {
    error!("missing role.");
    std::process::exit(1);
}
