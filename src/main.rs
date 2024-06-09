mod mat;
mod procs;
use mat::*;
use procs::*;
use std::io::{self, Write};

fn main() {
  println!("         <====STUIPILATOR====>");
  println!("---A MELHOR CALCULADORA DA SUA VIDA---");
  println!("|  digite seu input no espaço abaixo |");
  println!("| quantas vezes quiser e aguarde seu |");
  println!("|               resultado            |");
  println!("--------------------------------------\n\n");

  loop{
    print!("-> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    //Checando saída
    if input == "quit"{
      break;
    }

    println!("\n");
  }
}