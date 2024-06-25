use std ::io;
fn convert_to_int(data_input: & String)->i32{ //função converção 
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main(){
let mut number1= String::new();//inserindo os 2 numeros 
io::stdin().read_line( &mut number1).expect("erro ao ler number1");
let mut number2= String::new();
io::stdin().read_line( &mut number2).expect("erro ao ler number1");
if convert_to_int(&number1)> convert_to_int(&number2){
    println!("o numero{} é maior  {}",number1,number2);
}
  else{
      println!("o numero {} e menor ou igual {}",number1,number2);
  }  
}
