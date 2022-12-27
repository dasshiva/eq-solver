// File: solver.rs
// Date: 27.12.2022
// Author: Shivashish Das
// Purpose: Routines for parsing the expression and solving it
// License: Check the LICENSE file for details

use std::fmt;

pub enum Answer {
    Img(String, String),
    Real(f64, f64),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::Img(s1, s2) => write!(f,"{s1}, {s2}\n(where i is the square root of -1 or iota)"),
            Answer::Real(f1, f2) => write!(f, "{f1}, {f2}")
        }
    }
}

pub fn solve(eq: &str) -> Result<Answer, &str> {
    let mut index = 0usize;
    let mut x_inds: Vec<usize> = Vec::new();
    let mut op_inds: Vec<usize> = Vec::new();
    for i in eq.chars() {
        match i {
            'x' => { x_inds.push(index); },
            '+' | '-' => { op_inds.push(index) },
            _ => {}
        }

        index += 1;
    }
    if !verify(&x_inds, &op_inds) {
        return Err("Invalid placement of operators (operators must be after an expression like 3x + 7)");
    }
    
    let coefs = get_coef(eq, &x_inds, &op_inds);
    if coefs.len() == 3 {
        let mut img = false;
        let mut d = (coefs[1] * coefs[1]) - (4.0 * coefs[0] * coefs[2]);
        if d < 0.0 { 
            d = -d;
            img = true;
        }
        let root1 = (-coefs[1] + d.sqrt()) / (2.0 * coefs[0]);
        let root2 = (-coefs[1] - d.sqrt()) / (2.0 * coefs[0]);

        if img {
            Ok(Answer::Img(root1.to_string() + "i", root2.to_string() + "i"))
        }
        else {
            Ok(Answer::Real(root1, root2))
        }
    }

    else {
        Err("Number of expression terms greater than 3 are not supported")
    }
}

fn verify(x_inds: &Vec<usize>, op_inds: &Vec<usize>) -> bool {
    if x_inds.len() != op_inds.len() {
        return false;
    }

    let mut i = 0usize;
    while i < op_inds.len() {
        if i + 1 == x_inds.len() {
            break;
        }
        if op_inds[i] < x_inds[i] || op_inds[i] >  x_inds[i+1] {
            return false;
        }
        i += 1;
    }
    true
}

fn get_coef(eq: &str, x_inds: &Vec<usize>, op_inds: &Vec<usize>) -> Vec<f64> {
    let mut coefs: Vec<f64> = Vec::new();
    let mut i = 0usize;
    let mut lcv = 0usize;
    let mut cap = 0usize;
    let mut x_cap = false;
    let mut coef = "";
    let mut coef_sign = "";
    while lcv < x_inds.len() + 2 {
        if x_cap {
            coef_sign = eq[cap..op_inds[i - 1] + 1].trim();
            //println!("sign = {coef_sign}");
            cap = op_inds[i - 1] + 1;
            x_cap = false;
        }
        else {
            coef = eq[cap..x_inds[i]].trim();
            if coef.len() == 0 {
                coef = "1.0";
            }
            //println!("coefficient = {coef}");
            cap = x_inds[i] + 2;
            x_cap = true;
        }

        if lcv == 0 || lcv % 2 == 0 {
            if coef.len() == 0 {
                coefs.push(1.0f64);
                i += 1;
                lcv += 1;
                continue;
            }
            let parse = coef.parse::<f64>();
            match parse {
                Ok(mut s) => {
                    if coef_sign.contains("-") {
                        s = -s;
                    }
                    coefs.push(s);
                },

                Err(e) => { panic!("{e}"); }
            }

            i += 1;
        }
        lcv += 1;
    }

    match eq[op_inds[op_inds.len() - 1]+ 1..eq.len()].trim().parse::<f64>(){
        Ok(mut s) => {
            if coef_sign.contains("-") {
                s = -s;
            }
            coefs.push(s);
        },

        Err(e) => { panic!("{e}"); }
    }
    // println!("{:?}", coefs);
    coefs
}
