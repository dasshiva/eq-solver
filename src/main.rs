// File: main.rs
// Date: 27.12.2022
// Author: Shivashish Das
// Purpose: Entry point to the program
// License: Check the LICENSE file for details

use std::io;

mod solver;
fn main() {
    let mut exit = false;
    loop {
        print!(">> ");
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(s) => {
                if s == 0 {
                    if !exit {
                        println!("Will exit on next blank input");
                        exit = true;
                        continue;
                    }

                    else {
                        std::process::exit(0)
                    }
                }

                if exit {
                    exit = false;
                }
            },
            Err(e) => { eprintln!("{e}"); std::process::exit(1) }
        }
        
       match solver::solve(buf.trim()) {
           Ok(res) => { println!("Roots = {res}"); },
           Err(e) => { println!("{e}"); std::process::exit(1) }
       }
    }
}
