mod mat;
use mat::*;
use std::io::{self, Write};

const SSOMA: &str = "+";
const SSUB: &str = "-";
const SMULT: &str = "*";
const SDIV: &str = "/";

fn parser(texto: String){
  
  //let lista = xp_identifier(texto);
  let lista: Vec<&str> = texto.trim().split(" ").collect();
  let mut n1 = 0.0;
  let mut n2 = 0.0;
  let mut op = "";
  
  for i in 0..lista.len(){
    match i % 3{
      0 => {n1 = lista[i].parse::<f64>().unwrap()},
      1 => op = lista[i],
      2 => {
        n2 = lista[i].parse::<f64>().unwrap();
        match op{
          SSOMA => println!("{}",soma(n1, n2)),
          SSUB => println!("{}",sub(n1, n2)),
          SMULT => println!("{}",mult(n1, n2)),
          SDIV => println!("{}",div(n1, n2).unwrap()),
          _ => println!("Err: operação pedida não existe."),
        }
      }
      _ => {}
    }
  }
  
}

fn main() {
  println!("          <====STUPILATOR====>");
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
    
    //Checando saída
    if input.trim() == "quit"{
      break;
    }

    parser(input);

    println!("\n");
  }
}