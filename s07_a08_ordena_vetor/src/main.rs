/*
	Ordenação de um Vector


Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


use std::cmp::Ordering;


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Pessoa {
    nome: String,
    idade: u32,
}


// Compara dois números f64 e retorna o enum Ordering, NAN é menor que todos
fn meu_cmp_f64(a: &f64, b: &f64) -> Ordering {
	if a.is_nan() {
		return Ordering::Less;
	} else if b.is_nan() {
		return Ordering::Greater;
	}
	if a < b {
		return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}




fn main() {

	// 1. Inteiros

	let mut numeros: Vec<i32> = Vec::new();
	numeros.push( 3 );
	numeros.push( 7 );
	numeros.push( 6 );
	numeros.push( 2 );
	numeros.push( 1 );
	numeros.push( 4 );
	numeros.push( 5 );
	
	println!("INTEIROS ORIGINAL");
	println!("{:?}", numeros);

	println!("\nINTEIROS ORDEM NATURAL");
	numeros.sort();
	numeros.sort_unstable();		// Mais rápido, mas não preserva a ordem dos iguais (neste caso tanto faz)
	println!("{:?}", numeros);

	println!("\nINTEIROS ORDEM NATURAL REVERSA");
	numeros.reverse();
	println!("{:?}", numeros);

	// 2. Floats

	let mut floats: Vec<f64> = Vec::new();
	floats.push( 3.3 );
	floats.push( 7.7 );
	floats.push( 6.6 );
	floats.push( 2.2 );
	floats.push( 1.1 );
	floats.push( 4.4 );
	floats.push( 5.5 );
	
	println!("\n\nFLOATS ORIGINAL");
	println!("{:?}", floats);

	println!("\nFLOATS SEM NAN");
//	floats.sort();
//	the trait `Ord` is not implemented for `f64`

	let x = 9.9;
	let y = f64::NAN;
	if x > y {
		println!("9.9 é maior que NAN");
	} else if x < y {
		println!("9.9 é menor que NAN");
	} else if x == y {
		println!("9.9 é igual a NAN");
	} else {
		println!("Sei lá")
	}

	// Sabendo que não existe NAN
	//floats.push( f64::NAN );
	floats.sort_by(|a, b| a.partial_cmp(b).expect("Não pode ter NAN!!!") );

	//floats.sort_by(|a, b| b.partial_cmp(a).unwrap() );
	//floats.sort_unstable_by(|a, b| b.pos_atual.partial_cmp(&a.pos_atual).unwrap() );
	println!("{:?}", floats);

	// Se existe NAN precisa criar uma função de comparação própria
	println!("\nFLOATS COM NAN");
	floats[5] = f64::NAN;
	println!("{:?}", floats);
	floats.sort_by(|a, b| meu_cmp_f64(a,b) );
	println!("{:?}", floats);


	// 3. Tuplas

	// https://www.embrapa.br/manual-de-referenciacao/anexo-cidades-homonimas
	let mut populacoes: Vec<(String,String)> = Vec::new();
	populacoes.push( ("Wenceslau Braz".to_string(),"PR".to_string()) );
	populacoes.push( ("Wenceslau Braz".to_string(),"MG".to_string()) );
	populacoes.push( ("São Carlos".to_string(),"SC".to_string()) );
	populacoes.push( ("São Carlos".to_string(),"SP".to_string()) );
	populacoes.push( ("São Domingos".to_string(),"GO".to_string()) );
	populacoes.push( ("São Domingos".to_string(),"SC".to_string()) );
	populacoes.push( ("São Domingos".to_string(),"BA".to_string()) );
	populacoes.push( ("São Domingos".to_string(),"SE".to_string()) );
	populacoes.push( ("São Francisco".to_string(),"PB".to_string()) );
	populacoes.push( ("São Francisco".to_string(),"SP".to_string()) );
	populacoes.push( ("São Francisco".to_string(),"MG".to_string()) );
	populacoes.push( ("São Francisco".to_string(),"SE".to_string()) );
	populacoes.push( ("São Francisco de Paula".to_string(),"RS".to_string()) );
	populacoes.push( ("São Francisco de Paula".to_string(),"MG".to_string()) );

	println!("\n\n (\"São Carlos\", \"SC\") < (\"São Carlos\", \"SP\") = {}\n",
					("São Carlos","SC") < ("São Carlos", "SP") );

	println!("\nTUPLAS ORIGINAL:");
	for x in populacoes.iter() {
		println!("{:?}", x);
	}

	println!("\nTUPLAS ORDENAÇÃO NATURAL: Usa os elementos da tupla da esquerda para a direita");
	populacoes.sort();
	for x in populacoes.iter() {
		println!("{:?}", x);
	}

	// Como ordenar pelos estados ?
	println!("\nTUPLAS ORDENAÇÃO PELOS ESTADOS: Usa elementos da tupla da *direita* para a *esquerda*");
	populacoes.sort_by(|a, b| if a.1!=b.1 
																			{ a.1.cmp(&b.1) } 
																			else { a.0.cmp(&b.0) } );
	for x in populacoes.iter() {
		println!("{:?}", x);
	}


	// E se fosse com semântica copy ???
	let mut numeros: Vec<(i32,i32)> = Vec::new();
	numeros.push( (3,555) );
	numeros.push( (7,444) );
	numeros.push( (6,111) );
	numeros.push( (2,222) );
	numeros.push( (1,666) );
	numeros.push( (4,333) );
	numeros.push( (5,777) );

	println!("\nTUPLAS COM SEMÂNTICA COPY: Cria nova tupla com a ordem certa para ordenar");
	//numeros.sort_by_key(|t| (t.1,t.0));
	for x in numeros.iter() {
		println!("{:?}", x);
	}
	//populacoes.sort_by_key(|t| (t.1,t.0));



	// 4. Structs

	let mut pessoas: Vec<Pessoa> = Vec::new();
	pessoas.push( Pessoa{ nome: "João".to_string(), idade:6} );
	pessoas.push( Pessoa{ nome: "Maria".to_string(), idade:15} );
	pessoas.push( Pessoa{ nome: "José".to_string(), idade:28} );
	pessoas.push( Pessoa{ nome: "Ana".to_string(), idade:22} );
	pessoas.push( Pessoa{ nome: "Ana".to_string(), idade:19} );

	// Ordenação natural
	println!("\n\nSTRUCT ORDENAÇÃO NATURAL: Precisa derivar traits 'Eq', 'Ord', 'PartialEq' e 'PartialOrd'");
	pessoas.sort();
	for x in pessoas.iter() {
		println!("{}  {}", x.nome, x.idade);
	}

	// Ordenação qualquer através de closures
	println!("\nSTRUCT ORDENAÇÃO QUALQUER: Usa closure");
	pessoas.sort_by(|a, b| a.idade.cmp(&b.idade));
	for x in pessoas.iter() {
		println!("{}  {}", x.idade, x.nome);
	}


}


