/*
	Crie uma função que calcula o fatorial de um número
*/


// Solução clássica com for
fn fatorial_classico(n: i64) -> i64{

	let mut fatorial = 1;

	for i in 2..=n {
		fatorial *= i;
	}

	fatorial
}


// Solução recursiva, gasta muita memória
fn fatorial_recursivo(n: i64) -> i64{

	if n<=1 {
		return 1;
	}

	n * fatorial_recursivo(n-1)
}


// Solução usando iterador
fn fatorial_iterador(n: i64) -> i64 {
	(1..=n).product()
}



// Sobre overflow de inteiros ver:
// https://doc.rust-lang.org/book/ch03-02-data-types.html?highlight=overflow#integer-overflow
// Debug mode -> panic!
// Release mode -> "dá a volta" usando complemento de dois
// Pode-se lidar com o overflow explicitamente no código usando métodos da biblioteca
fn main() {
	let x: i64 = 4;

    println!("Fatorial clássico de {} é {}", x, fatorial_classico(x));
    println!("Fatorial recursivo de {} é {}", x, fatorial_recursivo(x));
    println!("Fatorial iterador de {} é {}", x, fatorial_iterador(x));
}

