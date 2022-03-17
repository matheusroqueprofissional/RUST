use std::io;

fn main(){

    println!("adivinhe o numero!");
    println!("digite seu palpite");
    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite).expect("falha ao ler entrada");
    println!("voce disse: {}",palpite);
}
