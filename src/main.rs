extern crate nix;

use std::io;

use nix::unistd::{fork, ForkResult, execvp, chdir};
use nix::sys::wait;
use std::ffi::CString;

fn main() {

    // Load config files if any exist

    // Run command line
    lsh_loop();
    
    // Perform shutdown/cleanup
}

fn lsh_loop() {
    loop {
        
        print!(":> ");
        io::stdout().flush().unwrap();

        // Read line
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        
        // Parse arguments
        let args = parse(line);

        execute(args);
    }
}

fn parse(line: String) -> Vec<String> {
    let iter = line.split_whitespace();
    let mut args = Vec::new();
    for arg in iter {
        args.push(arg.to_string());
    }
    args
}

fn execute(args: Vec<String>) {
    
    if args[0] == "cd" {
        change_dir(&args);
        return;
    }
    
    launch(args);
}

fn launch(args: Vec<String>) {
    io::stdout().flush().unwrap();

    // Fork the process
    match fork().expect("Fork failed") {
        // If we are in the parent process wait for the child to finish
        ForkResult::Parent{child} => {
            // println!("Child is{}", child);
            loop {
                let wait_stat = wait::waitpid(child, Some(wait::WaitPidFlag::WUNTRACED)).unwrap();
                match wait_stat {
                    wait::WaitStatus::Exited(_pid, _s) => {
                        break;
                    }

                    wait::WaitStatus::Signaled(_pid, _signal, _) => {
                        break;
                    }

                    _ => {
                        ;
                    }
                }
            }
        }

        // Replace the program
        ForkResult::Child => {
            let c_filename = CString::new(args[0].as_str()).unwrap();
            let c_args: Vec<_> = args.iter().map(|arg| CString::new(arg.as_str()).unwrap()).collect();
            // Execution failed 
            match execvp(&c_filename, &c_args) {
                Ok(_t) => {}
                Err(e) => {
                    println!("{}", e);
                }
            }
            std::process::exit(-1);
        }
    }
}

fn change_dir(args: &Vec<String>) {
    if args.len() <= 1 {
        println!("cd expected argument to \"cd\"");
    }

    match chdir(args[1].as_str()) {
        Err(e) => {
            println!("{}", e);
        }

        _ => {}
    }
}