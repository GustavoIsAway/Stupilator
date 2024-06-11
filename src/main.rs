mod mat;
use mat::*;
use std::io::{self, Write};

pub const SSOMA: char = '+';
pub const SSUB: char= '-';
pub const SMULT: char = '*';
pub const SDIV: char = '/';

pub fn parser(texto: String) -> Result<f64, &'static str>{
  let lista = texto;
  
  //Checar validade dos caracteres
  for i in lista.trim().chars(){
    if i.is_numeric() || [SSOMA, SSUB, SMULT, SDIV, ' '].contains(&i){
      continue;
    }
    else{
      return Err("input contém letras que não configuram função.");
    }
  }
  
  let lista = if lista.contains(" "){
    lista.chars().filter(|&c| c != ' ').collect()
  } else {
    lista
  };

  let mut current_number = String::new();
  let mut numbers : Vec<String>= Vec::new(); 
  let mut operators: Vec<char> = Vec::new();
  let mut cache = 0.0;

  //Cria dois vetores com números e caracteres
  for i in lista.trim().chars(){
    if i.is_numeric() || i == '.'{
      current_number.push(i);
    } 
    else if [SSOMA, SSUB, SMULT, SDIV, ' '].contains(&i){
      if !current_number.is_empty() {
        numbers.push(current_number.clone());
        current_number.clear();
    }
    operators.push(i);
    }
  }
  if !current_number.is_empty() {
    numbers.push(current_number);
  }

  if lista.starts_with('+') || lista.starts_with('-'){
    operators.remove(0);
    numbers[0] = format!("{}{}", lista.chars().next().unwrap(), numbers[0]);
  }

  if numbers.len() == 1{
    return Err("não é possível operar com apenas um número");
  }

  //Opera a partir dos vetores
  for i in 0..operators.len(){
    if i == 0{
      cache = {
        match operators[0]{
          SSOMA => soma(numbers[0].trim().parse::<f64>().unwrap(), numbers[1].trim().parse::<f64>().unwrap()),
          SSUB => sub(numbers[0].trim().parse::<f64>().unwrap(), numbers[1].trim().parse::<f64>().unwrap()),
          SMULT => mult(numbers[0].trim().parse::<f64>().unwrap(), numbers[1].trim().parse::<f64>().unwrap()),
          SDIV => div(numbers[0].trim().parse::<f64>().unwrap(), numbers[1].trim().parse::<f64>().unwrap()).unwrap(),
          _ => todo!(),
        }
      };
      numbers.remove(0);
      numbers.remove(0);
    }
    else{
      cache = match operators[i]{
        SSOMA => soma(cache, numbers[0].trim().parse::<f64>().unwrap()),
        SSUB => sub(cache, numbers[0].trim().parse::<f64>().unwrap()),
        SMULT => mult(cache, numbers[0].trim().parse::<f64>().unwrap()),
        SDIV => div(cache, numbers[0].trim().parse::<f64>().unwrap()).unwrap(),
        _ => todo!(),
      };
      numbers.remove(0);
    }
  }
  Ok(cache)
}



fn main() {
  println!("          <====STUPILATOR====>");
  println!("---A MELHOR CALCULADORA DA SUA VIDA---");
  println!("|  digite seu input no espaço abaixo |");
  println!("| quantas vezes quiser e aguarde seu |");
  println!("|               resultado            |");
  println!("--------------------------------------\n\n");

  let testes = vec![
      "23 + 5", 
      "23 + 5 + 2", 
      "23 * 2", 
      "23 * 2 * 2",
      "23 * 2 / 2",
      "23 + 2 * 2 - 2"
    ];

    let respostas = vec![
      28.0,
      30.0,
      46.0,
      92.0,
      23.0,
      48.0
    ];

  for i in 0..testes.len(){
    match parser(testes[i].to_string()){
      Ok(x) => {
        if x == respostas[i]{
          println!("TESTE {} SUCESSO) Devolveu. {}", i, x);
        }
        else{
          println!("TESTE {} FRACASSOU) Devolveu {} ao invés de {}.", i, x, respostas[i]);
        }
      },

      Err(y) => println!("PARSER DEU ERRO: {}", y),
    }
  }


  /*loop{
    print!("-> ");
    io::stdout().flush().unwrap();

    /*let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    //Checando saída
    if input.trim() == "quit"{
      break;
    }*/

    

    
    println!("\n");
  }*/
}