use std::thread;
use std::time::Duration;

pub const SSOMA: char = '+';
pub const SSUB: char= '-';
pub const SMULT: char = '*';
pub const SDIV: char = '/';
static TIPO: [&str; 2] = ["df","rg"];
static RG_LINKS: [&str; 10] = [
  "https://i.imgur.com/lDhE0qp.jpeg",
  "https://i.imgur.com/ToDES2l.jpeg",
  "https://i.imgur.com/8G61MCL.jpeg",
  "https://i.imgur.com/TtrQlXH.jpeg",
  "https://i.imgur.com/83eS6vj.jpg",
  "https://i.imgur.com/bL6dJlm.jpg",
  "https://i.imgur.com/jlzfHM9.jpg",
  "https://i.imgur.com/jiMqsMm.jpg",
  "https://i.imgur.com/Hq6GNd8.jpg",
  "https://i.imgur.com/zpnVSFu.jpg"
];
static OP_MATHS: [char; 4] = [SSOMA, SSUB, SDIV, SMULT];


//Operações Matemáticas
pub fn div(a: f64, b: f64) -> Result<f64, &'static str>{
  if b == 0.0{
    Err("indefinido")
  }
  else{
    Ok(a / b)
  }
}  


struct Dados{
  input_processado: String,
  tipo: String,
}


fn validar(entrada: String) -> Result<Dados, &'static str>{
  let mut input = entrada;
  let mut tipo_temp = String::new();
  
  
  {
    let mut buffer = input.chars();
    let c1 = buffer.next().unwrap();
    let c2 = buffer.next().unwrap();

    if c1.is_alphabetic() && c2.is_alphabetic(){
      tipo_temp.push(c1);
      tipo_temp.push(c2);
      input.remove(0);
      input.remove(0);
    }
    else if (c1.is_numeric() && c2.is_numeric()) || (OP_MATHS.contains(&c1) && c2.is_numeric()){
      tipo_temp.push_str("df");
    }
    else{
      return Err("caracteres iniciais não batem com a sintaxe conhecida.");
    }
  }

  if !(TIPO.contains(&tipo_temp.trim())){
    return Err("Err: tipo especificado não existe.");
  }

  for i in input.clone().trim().chars(){
    if i.is_alphabetic(){
      return Err("Err: input possui caracteres não suportados.");
    }
  }

  Ok(
    Dados { input_processado: input, tipo: tipo_temp }
  )

}




fn calculadora(entrada: String) -> f64{
  let mut numeros: Vec<f64> = Vec::new();
  let mut operadores: Vec<char> = Vec::new();
  let expressao = entrada;
  let mut num_buffer = String::new();

  for i in expressao.trim().chars(){
    if i.is_numeric() || i == '.'{
      num_buffer.push(i);
    }
    else{
      numeros.push(num_buffer.trim().parse::<f64>().unwrap());
      num_buffer = "".to_string();
      operadores.push(i);
    }
  }

  numeros.push(num_buffer.trim().parse::<f64>().unwrap());


  let mut cache = numeros[0].clone();
  numeros.remove(0);

  for j in operadores.iter(){
    match j{
      &SSOMA => cache += numeros[0],
      &SSUB => cache -= numeros[0],
      &SDIV => cache = div(cache, numeros[0]).unwrap(),
      &SMULT => cache *= numeros[0],
      _ => todo!(),
    }
  numeros.remove(0);
  }

  cache
}


fn rg(entrada: f64){
  let resultado = entrada;
  for i in resultado.clone().to_string().chars(){
    webbrowser::open(RG_LINKS[i.to_digit(10).unwrap() as usize]).unwrap();
    thread::sleep(Duration::from_secs_f32(0.5));
  }
}



pub fn parser(entrada: String) -> f64{
  let input = entrada.chars().filter(|&c| c != ' ').collect::<String>();
  let dados = validar(input).unwrap();
  let resposta = calculadora(dados.input_processado.clone());

  match dados.tipo.trim(){
    "df" => return resposta,
    "rg" => rg(resposta),
    _ => todo!(),
  }

  resposta
}