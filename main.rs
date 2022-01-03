#![allow(unused)]
use colored::*;
use std::io;



fn main() {
    
    println!("{}{}","Made by ", "Mdr721".bright_green().bold());

    //Variable zone
  
    let mut input = String::new();
    let mut input2 = String::new();

    
    //code zone
    
    println!("enter your text that you want to be colored:");
    io::stdin()
        
        .read_line(&mut input)
        .expect("Faild to read_line");

    println!("Enter red,blue, green or for short version use r,b,g for text to be colored:");

    io::stdin()
        .read_line(&mut input2)
        .expect("Faild to read_line");


    if input2 == "red\n"  {println!("{}",input.bright_red().bold());}
    if input2 == "green\n"  {println!("{}",input.bright_green().bold());}
    if input2 == "blue\n"  {println!("{}",input.bright_blue().bold());}

    if input2 == "r\n"  {println!("{}",input.bright_red().bold());}
    if input2 == "g\n"  {println!("{}",input.bright_green().bold());}
    if input2 == "b\n"  {println!("{}",input.bright_blue().bold());}
}
