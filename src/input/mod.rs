use std::{env, path::Path};

use clap::{Arg, Command};

mod menu_cli;

// this files do takes the argument from the cli
// it use clap

pub fn cli_input() {
    let matches = Command::new("gitfetch")
        .version("2.0")
        .author(r"  
         ██████╗ ██╗████████╗███████╗███████╗████████╗ ██████╗██╗  ██╗
        ██╔════╝ ██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
        ██║  ███╗██║   ██║   █████╗  █████╗     ██║   ██║     ███████║
        ██║   ██║██║   ██║   ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
        ╚██████╔╝██║   ██║   ██║     ███████╗   ██║   ╚██████╗██║  ██║
         ╚═════╝ ╚═╝   ╚═╝   ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝v.0.2.0

         A CLI application for github users, which shows the information of a particular user in a `neofetch` style\n
         Proudly build with the help of Rust.")
        .about("Neofetch but build for GitHub")
        .arg(
            Arg::new("t")
                .long("t")
                .help("Show information for a temporary user: gitfetch -t <USER>"),
        )
        .arg(
            Arg::new("o")
                .long("o")
                .help("Option to create the user or insert the github API key"),
        )
        .arg(
            Arg::new("author")
            .long("author")
            .required(false)
        )
        .get_matches();

    // for menu bar
    let arg_option = match matches.get_one::<String>("o") {
        None => "None",
        Some(val) => val,
    };

    // a temporary user
    let arg_temp = match matches.get_one::<String>("t") {
        None => "None",
        Some(val) => val,
    };
    // to show the name of the user.
    let author = match matches.get_one::<String>("author") {
        None => "None",
        Some(val) => val,
    };

    println!("{:?}", author);

    // menu_cli::menu_view();
}

fn create_user_file(arg_user: String) {
    let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
    let file_path = home_dir.into_string().unwrap() + "/gitfetch_user.txt";
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {:?}", e);
            return;
        }
    };
    std::io::Write::write_all(&mut file, arg_user.as_bytes()).unwrap();
}

fn create_api_file(arg_api: String) {
    let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
    let file_path = home_dir.into_string().unwrap() + "/gitfetch_api.txt";
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {:?}", e);
            return;
        }
    };
    std::io::Write::write_all(&mut file, arg_api.as_bytes()).unwrap();
}

pub fn show_user_info() -> (String, String) {
    let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
    let file_path = Path::new(&home_dir).join("secretkey.txt");
    let secret_key = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{:?}", e);
            "Stop".to_string()
        }
    };
    ("ArshErgon".to_string(), secret_key)
}

fn about() {
    let gitfetch_logo = format!(
        r"

         ██████╗ ██╗████████╗███████╗███████╗████████╗ ██████╗██╗  ██╗
        ██╔════╝ ██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
        ██║  ███╗██║   ██║   █████╗  █████╗     ██║   ██║     ███████║
        ██║   ██║██║   ██║   ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
        ╚██████╔╝██║   ██║   ██║     ███████╗   ██║   ╚██████╗██║  ██║
         ╚═════╝ ╚═╝   ╚═╝   ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝v.0.2.0

         A CLI application for github users, which shows the information of a particular user in a `neofetch` style\n
         Proudly build with the help of Rust.
    "
    );
}