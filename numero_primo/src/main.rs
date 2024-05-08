/* Determine se um número é primo

	0 e 1 não são primos, 2 é primo.

	Um número natural é primo se ele é maior que 1 e 
	é divisível apenas por si próprio e por 1.
*/

fn numero_primo_while(num: u32) -> bool {
	if num <= 1 {
		return false;
	}

	let limite = (num as f64).sqrt() as u32;
	let mut d = 2;
	while d <= limite {
		if num % d == 0 {
			return false;
		}
		d += 1;
	}
	true
}


fn numero_primo_for(num: u32) -> bool {
	if num <= 1 {
		return false;
	}

	let limite = (num as f64).sqrt() as u32;
	for d in 2..=limite {
		if num % d == 0 {
			return false;
		}
    }
    true
}


fn numero_primo_for_v2(num: u32) -> bool {
	if num <= 1 {
		return false;
	} else if num == 2 {
		return true;
	} else if num == 3 {
		return true;
	} else if num % 2 == 0 {
		return false;
	} else if num % 3 == 0 {
		return false;
	}

	let limite = (num as f64).sqrt() as u32;
	for d in (5..=limite).step_by(2) {
		println!("     d: {} ", d);
		if num % d == 0 {
			return false;
		}
    }
    true
}


fn numero_primo_closure(num: u32) -> bool {
	if num <= 1 {
		return false;
	}

	let limite = (num as f64).sqrt() as u32;
	(2..=limite).all(|d| num % d != 0)
}


fn main() {
    println!("\nnumero_primo_while({}) -> {}", 1, numero_primo_while(1));
    println!("numero_primo_while({}) -> {}", 2, numero_primo_while(2));
    println!("numero_primo_while({}) -> {}", 3, numero_primo_while(3));
    println!("numero_primo_while({}) -> {}", 8, numero_primo_while(8));
    println!("numero_primo_while({}) -> {}", 97, numero_primo_while(97));

    println!("\nnumero_primo_for({}) -> {}", 1, numero_primo_for(1));
    println!("numero_primo_for({}) -> {}", 2, numero_primo_for(2));
    println!("numero_primo_for({}) -> {}", 3, numero_primo_for(3));
    println!("numero_primo_for({}) -> {}", 8, numero_primo_for(8));
    println!("numero_primo_for({}) -> {}", 97, numero_primo_for(97));

    println!("\nnumero_primo_for_v2({}) -> {}", 1, numero_primo_for_v2(1));
    println!("numero_primo_for_v2({}) -> {}", 2, numero_primo_for_v2(2));
    println!("numero_primo_for_v2({}) -> {}", 3, numero_primo_for_v2(3));
    println!("numero_primo_for_v2({}) -> {}", 8, numero_primo_for_v2(8));
    println!("numero_primo_for_v2({}) -> {}", 97, numero_primo_for_v2(97));

    println!("\nnumero_primo_closure({}) -> {}", 1, numero_primo_closure(1));
    println!("numero_primo_closure({}) -> {}", 2, numero_primo_closure(2));
    println!("numero_primo_closure({}) -> {}", 3, numero_primo_closure(3));
    println!("numero_primo_closure({}) -> {}", 8, numero_primo_closure(8));
    println!("numero_primo_closure({}) -> {}", 97, numero_primo_closure(97));

}

