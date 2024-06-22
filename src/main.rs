mod scrpits;
use scrpits::*;


fn main() {
  println!("          <====STUPILATOR====>");
  println!("---A MELHOR CALCULADORA DA SUA VIDA---");
  println!("|  digite seu input no espaço abaixo |");
  println!("| quantas vezes quiser e aguarde seu |");
  println!("|              resultado             |");
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
    println!("---TESTANDO {}---", i);
    match parser(testes[i].to_string()){
      Ok(x) => {
        if x == respostas[i]{
          println!("TESTE {} SUCESSO) Devolveu. {}\n", i, x);
        }
        else{
          println!("TESTE {} FRACASSOU) Devolveu {} ao invés de {}.\n", i, x, respostas[i]);
        }
      },

      Err(y) => println!("PARSER DEU ERRO: {}", y),
    }
  }

  /*
  loop{
    print!("-> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    //Checando saída
    if input.trim() == "quit"{
      break;
    }

    
    match parser(input){
      Ok(x) => println!("{}", x),
      Err(y) => println!("Err: {}", y),
    }

    
    println!("\n");
  }*/
}