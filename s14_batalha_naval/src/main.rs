/*
	Batalha Naval


	Cores Básicas:
	Light		Dark
	-----		----
	DarkGrey	Black
	Red			DarkRed
	Green		DarkGreen
	Yellow		DarkYellow
	Blue		DarkBlue
	Magenta		DarkMagenta
	Cyan		DarkCyan
	White		Grey


	Observações:

	- Para indexar o tabuleiro é usado 'usize'
	- Para endereçar o cursor é usado 'u16'

	- Cada navios é armazenado como:
		- Posição x,y da popa
		- Direção
		- Tamanho
	- Para algumas operações é construído o mapa dos oceanos
	- No mapa 'y' é linha, 'x' é coluna


	- 'crossterm' não é da biblioteca padrão, requer [dependencies] crossterm = "0.27.0"
	- 'rand' não é da biblioteca padrão, requer [dependencies] rand = "0.8.5"


*/




use std::io::{stdout,Error,Write};
use std::time::Duration;

use rand::Rng;


use crossterm::{ExecutableCommand,QueueableCommand};
use crossterm::terminal::{self,Clear};
use crossterm::cursor;
use crossterm::style::{self,Stylize,ResetColor,SetForegroundColor,SetBackgroundColor,StyledContent};
use crossterm::style::Color::{self,*};
use crossterm::event::{Event,KeyCode,KeyModifiers};


const LADO_TABULEIRO: usize = 10;
const LARGURA_MINIMA: u16 = 1+2*LADO_TABULEIRO as u16+1+2*LADO_TABULEIRO as u16+1;
const ALTURA_MINIMA: u16 = 1+LADO_TABULEIRO as u16+1+5;

const MAX_TAM_NAVIO:usize = 5;
const NUM_NAVIOS: usize = 4;

const AGUA:usize = 999;
const DESTRUIDO:usize = 888;


// https://en.wikipedia.org/wiki/List_of_Unicode_characters
const _BLOCO: char = '\u{2588}';

const BLOCO_DUPLO: &str = "\u{2588}\u{2588}";
const EXPLOSAO_DUPLO: &str = "\u{0496}\u{0496}";	// Outras opções "\u{1F525}", "##"

const CANTO_SUP_ESQ: char = '\u{250C}';
const CANTO_SUP_MEIO: char = '\u{2565}';
const CANTO_SUP_DIR: char = '\u{2510}';
const CANTO_INF_ESQ: char = '\u{2514}';
const CANTO_INF_MEIO: char = '\u{2568}';
const CANTO_INF_DIR: char = '\u{2518}';

const BARRA_HORIZONTAL: char = '\u{2500}';
const BARRA_VERTICAL: char = '\u{2502}';
const BARRA_VERTICAL_DUPLA: char = '\u{2551}';


// Existe jogador humano e jogador bot
enum Jogador {
	HUMANO,
	BOT,
}

// Direção para a qual o navio está virado
#[derive(Debug,Copy,Clone)]
enum Direcao {
	NORTE,
	SUL,
	LESTE,
	OESTE,
}

// Descrição de um navio
#[derive(Debug)]
struct Navio {
	popa_x: usize,
	popa_y: usize,
	direcao: Direcao,
	tamanho: usize,
	desenho: StyledContent<&'static str>,		// String com o desenho deste navio
}

impl Navio {
	fn new(popa_y:usize,tamanho:usize ) -> Navio {
		Navio{
			popa_x: 1,
			popa_y,
			direcao: Direcao::LESTE,
			tamanho,
			desenho: match tamanho {
				MAX_TAM_NAVIO => "PP".on_dark_blue(),
				4 => "CC".on_dark_magenta(),				
				3 => "DD".on_dark_yellow(),
				2 => "SS".on_dark_red(),
				_ => panic!("Tamanho de Navio desconhecido"),
			},
		}
	}

	// Gira a direção do navio no sentido horário
	fn gira(&mut self) {
		match self.direcao {
			Direcao::NORTE => self.direcao = Direcao::LESTE,
			Direcao::SUL =>  self.direcao = Direcao::OESTE,
			Direcao::LESTE =>  self.direcao = Direcao::SUL,
			Direcao::OESTE =>  self.direcao = Direcao::NORTE,
		};
	}

	// Retorna os extremos do navio como tupla (N,S,L,O)
	fn _extremos(&self) -> (usize,usize,usize,usize) {
		let tam = self.tamanho;
		match self.direcao {
			Direcao::NORTE => (self.popa_y-tam+1,self.popa_y,self.popa_x,self.popa_x),
			Direcao::SUL => (self.popa_y,self.popa_y+tam-1,self.popa_x,self.popa_x),
			Direcao::LESTE => (self.popa_y,self.popa_y,self.popa_x+tam-1,self.popa_x),
			Direcao::OESTE => (self.popa_y,self.popa_y,self.popa_x,self.popa_x-tam+1),
		}
	}

}



// Estrutura com as informações do jogo
struct Jogo {
	navios_humano: [Navio;NUM_NAVIOS],
	navios_bot: [Navio;NUM_NAVIOS],
}

impl Jogo {
	fn new() -> Jogo {
		Jogo {
			navios_humano: [
				Navio::new(2,MAX_TAM_NAVIO),
				Navio::new(4,MAX_TAM_NAVIO-1),
				Navio::new(6,MAX_TAM_NAVIO-2), 
				Navio::new(8,MAX_TAM_NAVIO-3), 
			],

			navios_bot: [
				Navio::new(2,MAX_TAM_NAVIO), 
				Navio::new(4,MAX_TAM_NAVIO-1), 
				Navio::new(6,MAX_TAM_NAVIO-2), 
				Navio::new(8,MAX_TAM_NAVIO-3), 
			],
		}
	}

	// Cria mapa com navios humano, no mapa 'y' é linha, 'x' é coluna
	fn mapeia_humano(&self, excecao: Option<usize>) -> [[usize;LADO_TABULEIRO];LADO_TABULEIRO] {
		let mut mapa = [[AGUA;LADO_TABULEIRO];LADO_TABULEIRO];
		for n in 0..NUM_NAVIOS {
			if excecao==None || excecao!=Some(n) {
				for i in 0 .. self.navios_humano[n].tamanho {
					let mut x = self.navios_humano[n].popa_x;
					let mut y = self.navios_humano[n].popa_y;
					match self.navios_humano[n].direcao {
						Direcao::NORTE => y -= i,
						Direcao::SUL => y += i,
						Direcao::LESTE => x += i,
						Direcao::OESTE => x -= i,
					}
					mapa[y][x] = self.navios_humano[n].tamanho;
				}
			}
		}
		mapa
	}


	// Cria mapa com navios bot, no mapa 'y' é linha, 'x' é coluna
	fn mapeia_bot(&self, excecao: Option<usize>) -> [[usize;LADO_TABULEIRO];LADO_TABULEIRO] {
		let mut mapa = [[AGUA;LADO_TABULEIRO];LADO_TABULEIRO];
		for n in 0..NUM_NAVIOS {
			if excecao==None || excecao!=Some(n) {
				for i in 0 .. self.navios_bot[n].tamanho {
					let mut x = self.navios_bot[n].popa_x;
					let mut y = self.navios_bot[n].popa_y;
					match self.navios_bot[n].direcao {
						Direcao::NORTE => y -= i,
						Direcao::SUL => y += i,
						Direcao::LESTE => x += i,
						Direcao::OESTE => x -= i,
					}
					mapa[y][x] = self.navios_bot[n].tamanho;
				}
			}
		}
		mapa
	}


	// Testa se pode colocar um navio, considera extremos do tabuleiro e outros navios
	fn pode_colocar_navio(&self,jogador:&Jogador,qual:usize) -> bool {
		let navio;
		let mapa;
		match jogador {
			Jogador::HUMANO => {
				navio = &self.navios_humano[qual];
				mapa = self.mapeia_humano(Some(qual));
			}
			Jogador::BOT => {
				navio = &self.navios_bot[qual];
				mapa = self.mapeia_bot(Some(qual));
			}
		}

		// Tenta marcar navio no mapa e observar conflitos
		for i in 0 .. navio.tamanho {
			let mut x = navio.popa_x as i64;
			let mut y = navio.popa_y as i64;
			match navio.direcao {
				Direcao::NORTE => y -= i as i64,
				Direcao::SUL => y += i as i64,
				Direcao::LESTE => x += i as i64,
				Direcao::OESTE => x -= i as i64,
			}
			// Testa limites do tabuleiro
			if y < 0 || y >= LADO_TABULEIRO as i64 || x < 0 || x >= LADO_TABULEIRO as i64 {
				return false;
			}
			// Testa outros navios
			if mapa[y as usize][x as usize] != AGUA {
				return false;
			}
		}
		true
	}


	// Testa se pode girar um navio, considera extremos do tabuleiro e outros navios
	fn pode_girar_navio(&self,jogador:&Jogador,qual:usize) -> bool {
		let navio;
		let mapa;
		match jogador {
			Jogador::HUMANO => {
				navio = &self.navios_humano[qual];
				mapa = self.mapeia_humano(Some(qual));
			}
			Jogador::BOT => {
				navio = &self.navios_bot[qual];
				mapa = self.mapeia_bot(Some(qual));
			}
		}

		// Simula giro
		let nova_direcao = match &navio.direcao {
			Direcao::NORTE => Direcao::LESTE,
			Direcao::SUL => Direcao::OESTE,
			Direcao::LESTE => Direcao::SUL,
			Direcao::OESTE => Direcao::NORTE,
		};

		// Tenta marcar navio no mapa e observar conflitos
		for i in 0 .. navio.tamanho {
			let mut x = navio.popa_x as i64;
			let mut y = navio.popa_y as i64;
			match nova_direcao {
				Direcao::NORTE => y -= i as i64,
				Direcao::SUL => y += i as i64,
				Direcao::LESTE => x += i as i64,
				Direcao::OESTE => x -= i as i64,
			}
			// Testa limites do tabuleiro
			if y < 0 || y >= LADO_TABULEIRO as i64 || x < 0 || x >= LADO_TABULEIRO as i64 {
				return false;
			}
			// Testa outros navios
			if mapa[y as usize][x as usize] != AGUA {
				return false;
			}
		}
		true
	}


	// Testa se pode mover um navio, considera extremos do tabuleiro e outros navios
	fn pode_mover_navio(&self,jogador:&Jogador,qual:usize,move_direcao:Direcao) -> bool {
		let navio;
		let mapa;
		match jogador {
			Jogador::HUMANO => {
				navio = &self.navios_humano[qual];
				mapa = self.mapeia_humano(Some(qual));
			}
			Jogador::BOT => {
				navio = &self.navios_bot[qual];
				mapa = self.mapeia_bot(Some(qual));
			}
		}
		// Simula movimento
		let mut novo_popa_x = navio.popa_x as i64;
		let mut novo_popa_y = navio.popa_y as i64;
		match move_direcao {
			Direcao::NORTE => novo_popa_y -= 1,
			Direcao::SUL => novo_popa_y += 1,
			Direcao::LESTE => novo_popa_x += 1,
			Direcao::OESTE => novo_popa_x -= 1,
		}
		// Tenta marcar navio no mapa e observar conflitos
		for i in 0 .. navio.tamanho {
			let mut x = novo_popa_x as i64;
			let mut y = novo_popa_y as i64;
			match navio.direcao {
				Direcao::NORTE => y -= i as i64,
				Direcao::SUL => y += i as i64,
				Direcao::LESTE => x += i as i64,
				Direcao::OESTE => x -= i as i64,
			}
			// Testa limites do tabuleiro
			if y < 0 || y >= LADO_TABULEIRO as i64 || x < 0 || x >= LADO_TABULEIRO as i64 {
				return false;
			}
			// Testa outros navios
			if mapa[y as usize][x as usize] != AGUA {
				return false;
			}
		}
		true
	}


}



// Limpa toda a tela, posiciona cursor no topo à esquerda
fn limpa_tela() -> Result<bool, Error> {
	let mut stdout = stdout();
	stdout
		.queue(cursor::MoveTo(0,0))?
		.queue(terminal::Clear(terminal::ClearType::All))?;
	stdout.flush()?;
	Ok(true)
}


// Terminal tem o tamanho mínimo necessário ?
fn tem_tamanho_minimo() -> Result<bool, Error> {
	let (largura,altura) = terminal::size().expect("Falha da biblioteca crossterm");
	Ok(largura >= LARGURA_MINIMA  &&  altura >= ALTURA_MINIMA)
}


// Desenha moldura com 2 tabuleiros de '10 linhas e 20 colunas' cada um
fn desenha_moldura(x_esq:u16, y_sup:u16, cor_frente:Color, cor_fundo:Color) -> Result<bool, Error> {
	let x_meio = x_esq+1+2*10;
	let x_dir = x_esq+1+2*10+1+2*10;
	let y_inf = y_sup+10+1;

	limpa_tela()?;

	// Saídas acontecem através dessa variável 
	let mut stdout = stdout();

	// Muda cor
	stdout
		.queue( SetForegroundColor(cor_frente) )?
		.queue( SetBackgroundColor(cor_fundo) )?;

	for x in x_esq .. x_dir {
		for y in y_sup .. y_inf {
			stdout
			.queue( cursor::MoveTo(x,y) )?
			.queue( style::Print(" ") )?;
		}
	}

	// Barra horizontal superior
	stdout
		.queue( cursor::MoveTo(x_esq,y_sup) )?
		.queue( style::Print(CANTO_SUP_ESQ) )?
		.queue( style::Print(String::from(BARRA_HORIZONTAL).repeat(20)) )?
		.queue( style::Print(CANTO_SUP_MEIO) )?
		.queue( style::Print(String::from(BARRA_HORIZONTAL).repeat(20)) )?
		.queue( style::Print(CANTO_SUP_DIR) )?;

	// Barra horizontal inferior
	stdout
		.queue( cursor::MoveTo(x_esq,y_inf) )?
		.queue( style::Print(CANTO_INF_ESQ) )?
		.queue( style::Print(String::from(BARRA_HORIZONTAL).repeat(20)) )?
		.queue( style::Print(CANTO_INF_MEIO) )?
		.queue( style::Print(String::from(BARRA_HORIZONTAL).repeat(20)) )?
		.queue( style::Print(CANTO_INF_DIR) )?;

	// Barras verticais
	for y in y_sup+1..=y_sup+1+9 {
		stdout
			.queue( cursor::MoveTo(x_esq,y) )?
			.queue( style::Print(BARRA_VERTICAL) )?;
	}
	for y in y_sup+1..=y_sup+1+9 {
		stdout
			.queue( cursor::MoveTo(x_meio,y) )?
			.queue( style::Print(BARRA_VERTICAL_DUPLA) )?;
	}
	for y in y_sup+1..=y_sup+1+9 {
		stdout
			.queue( cursor::MoveTo(x_dir,y) )?
			.queue( style::Print(BARRA_VERTICAL) )?;
	}

	// Volta cor padrão
		stdout
			.queue( ResetColor )?
			.queue( cursor::MoveTo(0,y_inf+1) )?;

	// Executa o lote de comandos
	stdout.flush()?;

	println!("+11223344556677889900+11223344556677889900+");
	println!("      Meus Navios           Inimigo          ");
	// Define linha das mensagens
	stdout.execute(cursor::SavePosition)?;

	Ok(true)
}



// Converte posição x do tabuleiro para posição na tela humano (são 2 caracteres)
fn x_para_tela_humano(x:usize) -> u16 {
	x as u16*2+1
}

// Converte posição y do tabuleiro para posição na tela humano
fn y_para_tela_humano(y:usize) -> u16 {
	y as u16+1
}

// Converte posição x do tabuleiro para posição na tela bot (são 2 caracteres)
fn x_para_tela_bot(x:usize) -> u16 {
	x as u16*2+1 + 1+20
}

// Converte posição y do tabuleiro para posição na tela bot
fn y_para_tela_bot(y:usize) -> u16 {
	y as u16+1
}



// Desdesenha um navio com anotações especiais
fn desdesenha_navio(navio: &Navio) -> Result<bool, Error> {
	let mut stdout = stdout();
	let desenho = BLOCO_DUPLO.cyan();

	// Desdesenha cada pedaço do navio
	for i in 0 .. navio.tamanho {
		let mut x = navio.popa_x;
		let mut y = navio.popa_y;
		match navio.direcao {
			Direcao::NORTE => y -= i,
			Direcao::SUL => y += i,
			Direcao::LESTE => x += i,
			Direcao::OESTE => x -= i,
		}
		stdout
			.queue( cursor::MoveTo(x_para_tela_humano(x),y_para_tela_humano(y)) )?
			.queue( style::PrintStyledContent(desenho) )?;
	}

	stdout.flush()?;
	Ok(true)
}


// Desenha um navio com anotações especiais
fn desenha_navio(navio: &Navio, cursor:bool) -> Result<bool, Error> {
	let mut stdout = stdout();

	// Desenha cada pedaço do navio
	for i in 0 .. navio.tamanho {
		let mut x = navio.popa_x;
		let mut y = navio.popa_y;
		match navio.direcao {
			Direcao::NORTE => y -= i,
			Direcao::SUL => y += i,
			Direcao::LESTE => x += i,
			Direcao::OESTE => x -= i,
		}
		stdout
			.queue( cursor::MoveTo(x_para_tela_humano(x),y_para_tela_humano(y)) )?
			.queue( style::PrintStyledContent(navio.desenho) )?;
	}

	if cursor {
		stdout.queue(cursor::MoveTo(x_para_tela_humano(navio.popa_x),
											y_para_tela_humano(navio.popa_y)) )?;
	} else {
		stdout.queue(cursor::RestorePosition)?;
	}

	stdout.flush()?;
	Ok(true)
}



// Arruma os navios do bot de forma aleatória
fn arruma_navios_bot(jogo: &mut Jogo) {
	for n in 0 .. NUM_NAVIOS {
		let mut colocado = false;
		while !colocado {
			let nd = rand::thread_rng().gen_range(0..4);
			let nova_direcao = match nd {
				0 => Direcao::NORTE,
				1 => Direcao::SUL,
				2 => Direcao::LESTE,
				3 => Direcao::OESTE,
				_ => Direcao::NORTE,
			};

			let novo_popa_x = rand::thread_rng().gen_range(0..LADO_TABULEIRO);
			let novo_popa_y = rand::thread_rng().gen_range(0..LADO_TABULEIRO);

			// Tenta
			let velha_direcao = jogo.navios_bot[n].direcao;
			let velho_popa_x = jogo.navios_bot[n].popa_x;
			let velho_popa_y = jogo.navios_bot[n].popa_y;
			jogo.navios_bot[n].direcao = nova_direcao;
			jogo.navios_bot[n].popa_x = novo_popa_x;
			jogo.navios_bot[n].popa_y = novo_popa_y;

			if jogo.pode_colocar_navio(&Jogador::BOT,n) {
				colocado = true;
			} else {		
				// Desfaz
				jogo.navios_bot[n].direcao = velha_direcao;
				jogo.navios_bot[n].popa_x = velho_popa_x;
				jogo.navios_bot[n].popa_y = velho_popa_y;
			}
		}
	}
}



// Deixa humano arrumar a posição dos seus navios
fn arruma_navios_humano(jogo: &mut Jogo) -> Result<bool, Error> {
	println!("Mova com as teclas {}{}{}{}, 'g' p/girar, 'm' p/mudar, 'i' p/iniciar batalha",
											 '\u{2190}', '\u{2191}', '\u{2192}', '\u{2193}');

	let mut corrente = 0;
	stdout().execute(cursor::MoveTo(x_para_tela_humano(jogo.navios_humano[corrente].popa_x),
											y_para_tela_humano(jogo.navios_humano[corrente].popa_y) ) )?;

	terminal::enable_raw_mode()?;

	loop {
		let evento = crossterm::event::read()?;
		match evento {
			Event::Key(key_event) => {
	
				match (key_event.code,key_event.modifiers) {
					(KeyCode::Char(_x),m) if m == KeyModifiers::CONTROL => {
						terminal::disable_raw_mode()?;
						return Ok(false);
					}

					(KeyCode::Char(x), _) => {
						match x {
							'g'|'G' => {
								if jogo.pode_girar_navio(&Jogador::HUMANO,corrente) {
									desdesenha_navio(&jogo.navios_humano[corrente])?;
									jogo.navios_humano[corrente].gira();
									desenha_navio(&jogo.navios_humano[corrente], true)?;
								}
							}
							'm'|'M' => {
								corrente = (corrente+1) % NUM_NAVIOS;
								stdout()
									.execute(
									cursor::MoveTo(x_para_tela_humano(jogo.navios_humano[corrente].popa_x),
														  y_para_tela_humano(jogo.navios_humano[corrente].popa_y) ))?;
							}
							'i'|'I' => {
								terminal::disable_raw_mode()?;
								break;	
							}
							_ => {}
						}
					}

					(KeyCode::Up, _) => {
						if jogo.pode_mover_navio(&Jogador::HUMANO,corrente,Direcao::NORTE) {
							desdesenha_navio(&jogo.navios_humano[corrente])?;
							jogo.navios_humano[corrente].popa_y -= 1;
							desenha_navio(&jogo.navios_humano[corrente], true)?;
						}
					}

					(KeyCode::Down, _) => {
						if jogo.pode_mover_navio(&Jogador::HUMANO,corrente,Direcao::SUL) {
							desdesenha_navio(&jogo.navios_humano[corrente])?;
							jogo.navios_humano[corrente].popa_y += 1;
							desenha_navio(&jogo.navios_humano[corrente], true)?;
						}
					}

					(KeyCode::Right, _) => {
						if jogo.pode_mover_navio(&Jogador::HUMANO,corrente,Direcao::LESTE) {
							desdesenha_navio(&jogo.navios_humano[corrente])?;
							jogo.navios_humano[corrente].popa_x += 1;
							desenha_navio(&jogo.navios_humano[corrente], true)?;
						}
					}	

					(KeyCode::Left, _) => {
						if jogo.pode_mover_navio(&Jogador::HUMANO,corrente,Direcao::OESTE) {
							desdesenha_navio(&jogo.navios_humano[corrente])?;
							jogo.navios_humano[corrente].popa_x -= 1;
							desenha_navio(&jogo.navios_humano[corrente], true)?;
						}	
					}

					_ => {

					}
				}
			}
			
			Event::FocusGained => {},
			Event::FocusLost => {},
			Event::Mouse(_mouse_event) => {},
			Event::Paste(_s) => {},
			Event::Resize(_colunas,_linhas) => {
				if !tem_tamanho_minimo().expect("Erro na biblioteca crossterm") {
					terminal::disable_raw_mode()?;
					panic!("Terminal não tem o tamanho mínimo!");
				}
			},
		}
	}

	Ok(true)
}


// Realiza a batalha naval
fn executa_batalha(jogo: &mut Jogo) -> Result<bool, Error> {
	let mut stdout = stdout();

	// Mapas
	let mut mapa_humano = jogo.mapeia_humano(None);
	let mut mapa_bot = jogo.mapeia_bot(None);

	// Quantos pedaços vivos (não destruídos) ainda existem
	let mut vivos_humano = 5+4+3+2;
	let mut vivos_bot = 5+4+3+2;
	
	// Mira do humano
	let mut mira_x_humano = 0;
	let mut mira_y_humano = 0;
									
	// Mira do bot
	let mut mira_x_bot;
	let mut mira_y_bot;

	// Gerador de números aleatórios
	let mut rng = rand::thread_rng();

	// Teclado em modo 'raw'
	terminal::enable_raw_mode()?;

	// Cada loop é uma rodada, humanos começam
	loop {

		// TIRO DO HUMANO
		loop {
			stdout.execute(cursor::MoveTo(x_para_tela_bot(mira_x_humano),
													y_para_tela_bot(mira_y_humano) ) )?;

			let evento = crossterm::event::read()?;
			match evento {
				Event::Key(key_event) => {
					match (key_event.code,key_event.modifiers) {
						(KeyCode::Char(_x),m) if m == KeyModifiers::CONTROL => {
							terminal::disable_raw_mode()?;
							return Ok(false);
						}
						(KeyCode::Char(x), _) => {
							match x {
								'f'|'F' => break,
								_ => {}
							}
						}
						(KeyCode::Up, _) =>
							if mira_y_humano > 0 {
								mira_y_humano -= 1;
							}
						(KeyCode::Down, _) => 
							if mira_y_humano < LADO_TABULEIRO-1 {
								mira_y_humano += 1;
							}
						(KeyCode::Right, _) =>
							if mira_x_humano < LADO_TABULEIRO-1 {
								mira_x_humano += 1;
							}
						(KeyCode::Left, _) => {
							if mira_x_humano > 0 {
								mira_x_humano -= 1;
							}	
						}
						_ => {}
					}
				}
				Event::FocusGained => {}
				Event::FocusLost => {}
				Event::Mouse(_mouse_event) => {}
				Event::Paste(_s) => {}
				Event::Resize(_colunas,_linhas) => {
					if !tem_tamanho_minimo().expect("Erro na biblioteca crossterm") {
						terminal::disable_raw_mode()?;
						panic!("Terminal não tem o tamanho mínimo!");
					}
				}
			}
		}
		// Tiro foi dado na mira do humano
		match mapa_bot[mira_y_humano][mira_x_humano] {
			AGUA => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_cyan()) )?;
			}
			DESTRUIDO => {
			}
			2 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_red()) )?;
				mapa_bot[mira_y_humano][mira_x_humano] = DESTRUIDO;
				vivos_bot -= 1;
			}
			3 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_yellow()) )?;
				mapa_bot[mira_y_humano][mira_x_humano] = DESTRUIDO;
				vivos_bot -= 1;
			}
			4 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_magenta()) )?;
				mapa_bot[mira_y_humano][mira_x_humano] = DESTRUIDO;
				vivos_bot -= 1;
			}
			5 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_blue()) )?;
				mapa_bot[mira_y_humano][mira_x_humano] = DESTRUIDO;
				vivos_bot -= 1;
			}			
			_ => panic!("Tamanho de Navio desconhecido"),
		}
		// Atualiza placar
		stdout.queue(cursor::RestorePosition).expect("Erro na biblioteca crossterm");
		stdout.queue(Clear(terminal::ClearType::CurrentLine)).expect("Erro na biblioteca crossterm");
		stdout.queue( style::Print(format!("Restam:    {}                  {}", vivos_humano,vivos_bot)) )?;
		stdout.flush()?;
		std::thread::sleep(Duration::from_secs(1));

		//	Terminou ?
		if vivos_bot == 0 {
			terminal::disable_raw_mode()?;
			stdout
				.execute(Clear(terminal::ClearType::CurrentLine))
				.expect("Erro na biblioteca crossterm");
			println!("\rVITÓRIA DO HUMANO!!!     ");
			return Ok(true);
		}

		// TIRO DO BOT

		// Não deixa o bot atirar em posição já tentada
		loop {
			mira_x_bot = rng.gen::<usize>() % LADO_TABULEIRO;
			mira_y_bot = rng.gen::<usize>() % LADO_TABULEIRO;
			if mapa_humano[mira_y_bot][mira_x_bot] != DESTRUIDO {
				break;
			}
		}

		stdout.execute(cursor::MoveTo(x_para_tela_humano(mira_x_bot),
												y_para_tela_humano(mira_y_bot) ) )?;

		// Tiro foi dado na mira do bot
		match mapa_humano[mira_y_bot][mira_x_bot] {
			AGUA => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_cyan()) )?;
				mapa_humano[mira_y_bot][mira_x_bot] = DESTRUIDO;
			}
			DESTRUIDO => {
			}
			2 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_red()) )?;
				mapa_humano[mira_y_bot][mira_x_bot] = DESTRUIDO;
				vivos_humano -= 1;
			}
			3 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_yellow()) )?;
				mapa_humano[mira_y_bot][mira_x_bot] = DESTRUIDO;
				vivos_humano -= 1;
			}
			4 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_magenta()) )?;
				mapa_humano[mira_y_bot][mira_x_bot] = DESTRUIDO;
				vivos_humano -= 1;
			}
			5 => {
				stdout.execute( style::PrintStyledContent(EXPLOSAO_DUPLO.on_dark_blue()) )?;
				mapa_humano[mira_y_bot][mira_x_bot] = DESTRUIDO;
				vivos_humano -= 1;
			}			
			_ => {
				println!("@@@@@@@ {} {} {}", mira_y_bot, mira_x_bot, mapa_humano[mira_y_bot][mira_x_bot]);
				panic!("Tamanho de Navio desconhecido");
			}
		}
		// Volta cursor para humano ver onde o bot atirou
		stdout.execute(cursor::MoveTo(x_para_tela_humano(mira_x_bot),
												y_para_tela_humano(mira_y_bot) ) )?;
		std::thread::sleep(Duration::from_secs(1));

		// Atualiza placar
		stdout
			.queue(cursor::RestorePosition).expect("Erro na biblioteca crossterm")
			.queue(Clear(terminal::ClearType::CurrentLine)).expect("Erro na biblioteca crossterm")
			.queue( style::Print(format!("Restam:    {}                  {}", vivos_humano,vivos_bot)) )?
			.flush()?;

		//	Terminou ?
		if vivos_humano == 0 {
			terminal::disable_raw_mode()?;
			stdout.execute(Clear(terminal::ClearType::CurrentLine)).expect("Erro na biblioteca crossterm");
			println!("\rVITÓRIA DO BOT!!!     ");
			return Ok(true);
		}
	}
}



// Estrutura para realizar o drop e normalizar o teclado em caso de pânico
struct Limpeza;
impl Drop for Limpeza {
	fn drop(&mut self) {
		terminal::disable_raw_mode().expect("Falha ao sair do raw mode");
    }
}

fn main() {
	let _limpeza = Limpeza;
	let mut stdout = stdout();

	println!("Batalha Naval");

	// Cria um novo jogo com posições iniciais dos navios
	let mut jogo = Jogo::new();

	// Testa se o terminal tem o tamanho mínimo necessário	
	if !tem_tamanho_minimo().expect("Erro na biblioteca crossterm") {
		println!("Terminal não tem o tamanho mínimo!");
		return;
	}

	// Desenha moldura do jogo
	desenha_moldura(0,0,Black, Cyan)
		.expect("Erro na biblioteca crossterm");

	// Desenha navios humano
	for n in jogo.navios_humano.iter() {
		desenha_navio(n,false).expect("Erro na biblioteca crossterm");
	}

	// Permite que o humano arrume os seus navios
	if !arruma_navios_humano(&mut jogo).expect("Erro na biblioteca crossterm") {
		stdout.queue(cursor::RestorePosition).expect("Erro na biblioteca crossterm");
		println!("\nJogo foi abortado.\n");
		return;
	}

	// Arruma navios do bot
	arruma_navios_bot(&mut jogo);

	// Passa para a fase de tiros (batalha)
	stdout.execute(cursor::RestorePosition).expect("Erro na biblioteca crossterm");
	stdout.execute(Clear(terminal::ClearType::CurrentLine)).expect("Erro na biblioteca crossterm");
	println!("Mova a mira as teclas {}{}{}{}, 'f' p/fogo", '\u{2190}', '\u{2191}', '\u{2192}', '\u{2193}');
	if executa_batalha(&mut jogo).expect("Erro na biblioteca crossterm") {
		stdout.execute(cursor::RestorePosition).expect("Erro na biblioteca crossterm");
		println!("\nFim do jogo.\n");
	} else {
		stdout.execute(cursor::RestorePosition).expect("Erro na biblioteca crossterm");
		println!("\nJogo foi abortado.\n");
	}

	// Mapa final, para depuração
	println!("Mapa Humano");
	let zzz = jogo.mapeia_humano(None);
	for i in 0..10 {
		println!("{:?}", zzz[i]);
	}
	println!("Mapa Bot");
	let zzz = jogo.mapeia_bot(None);
	for i in 0..10 {
		println!("{:?}", zzz[i]);
	}

}

