use std::{borrow::Borrow, io};

enum equasion_el {
    Int(i32),
    Char(char),
}

fn main()
{
    let mut equasion: Vec<equasion_el> = Vec::new();
    println!("Calculator");
    println!("Write equasion without variables and with = sign");
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    for i in inp.chars(){
        let mut num:i32 = 0;
        if "1234567890".contains(i){
            num = num * 10;
            num = num + i as i32;
        }
        else if "+-/*%^".contains(i){
            equasion.push(equasion_el::Int(num));
            equasion.push(equasion_el::Char(i));
        }
        else if i == '=' {
            equasion.push(equasion_el::Int(num));
            break;
        }
    }
    for el in &equasion {
        match el {
            equasion_el::Int(n) => print!("{} ", n),
            equasion_el::Char(c) => {
                
            },
        }
    }
}