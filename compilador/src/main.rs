mod r#main;

fn main() {
    println!("Hello, world!");
    let fib_number = fibbonacci(20);
    println!("El número 8 de la sucesión de Fibonacci es: {fib_number}");
}

fn fibbonacci(n: u32) -> u32 {
    let mut number_aux = 0;
    let mut other_aux = 1;
    for _iteration in 0..n {
        other_aux += number_aux;
        number_aux = other_aux - number_aux;
        println!("El número de Fibonacci actual es: {number_aux}");
    }
    return number_aux;
}

