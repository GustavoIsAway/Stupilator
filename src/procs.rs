pub fn checkop(texto: String){
  let teste = texto;
  let mut num_count = 0;
  let mut num_string = String::new(); 
  let mut carac = String::new();

  for i in teste.chars(){
    match i.to_digit(10){
      Some(x) => num_string += x.to_string().trim(),
      None => {
        println!("Input não possui um número válido");
        break;
      },
    }
  }
}