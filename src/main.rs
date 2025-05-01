use std::io::{self, BufRead};
use std::iter::Peekable;
use std::str::Chars;

/// Tipo che rappresenta un'espressione aritmetica.
///
/// `Expr` può essere un numero (`Num(f64)`), una somma (`Add(Vec<Expr>)`), o un prodotto (`Mul(Vec<Expr>)`).
#[derive(Debug)]
enum Expr {
    Num(f64),         // Numero (tipo `f64`)
    Add(Vec<Expr>),    // Somma di espressioni
    Mul(Vec<Expr>),    // Prodotto di espressioni
}

/// Parsifica un'espressione aritmetica.
///
/// Questa funzione analizza un'espressione che può essere un numero, una somma o un prodotto, racchiuso in parentesi.
///
/// # Parametri
/// - `chars`: Un iteratore mutabile su caratteri (`Peekable<Chars>`) che rappresenta l'input da parsare.
///
/// # Ritorno
/// Restituisce un risultato (`Result<Expr, String>`), dove:
/// - `Ok(Expr)`: Una struttura di tipo `Expr` che rappresenta l'espressione parsata.
/// - `Err(String)`: Un errore che descrive il motivo per cui l'espressione non è valida.
fn parse_expr(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    skip_whitespace(chars); 

    match chars.next() {  
        
        // Se il prossimo carattere é '(' analizza l'espressione interna
        Some('(') => {
            
            skip_whitespace(chars);
            
            match chars.next() {
                /*
                * Operatore ?:
                * Se il risultato é Ok(valore), restituisce il valore.
                * Se il risultato é Err(errore), esce dalla funzione restituendo l'errore.
                 */
                Some('+') => Ok(Expr::Add(parse_expr_list(chars)?)),  // Caso Somma
                Some('*') => Ok(Expr::Mul(parse_expr_list(chars)?)),  // Caso Prodotto
                
                // Altri casi che non richiedono operazione
                Some(c) if c.is_digit(10) || c == '-' || c == '.' => {
                    let mut num_str = String::new();
                    num_str.push(c);  
                    
                    // Leggi i successivi caratteri se sono numeri o decimali
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() || c == '.' {
                            num_str.push(c);  
                            chars.next();  
                        } else {
                            break;
                        }
                    }
                    // Verifica che ci sia una parentesi di chiusura
                    if chars.next() != Some(')') {
                        return Err("Expected ')' after number".into());
                    }
                    
                    // Parsa il numero da stringa a f64
                    num_str
                        .parse::<f64>()
                        .map(Expr::Num)
                        .map_err(|_| "Invalid number".into())  // Gestisce errori nel parse
                }
                _ => Err("Invalid expression".into()), // Errore per simboli non validi
            }
        }

        // Se il primo carattere non è '(' è errore
        _ => Err("Expected '('".into()),  
    }
}

/// Parsifica una lista di espressioni separate da parentesi.
///
/// Questa funzione gestisce una lista di espressioni aritmetiche che sono racchiuse tra parentesi.
/// Ogni espressione può essere una somma o un prodotto, e vengono raccolte in un vettore.
///
/// # Parametri
/// - `chars`: Un iteratore mutabile su caratteri (`Peekable<Chars>`) che rappresenta l'input da parsare.
///
/// # Ritorno
/// Restituisce un risultato (`Result<Vec<Expr>, String>`), dove:
/// - `Ok(Vec<Expr>)`: Una lista di espressioni parsate.
/// - `Err(String)`: Un errore che descrive il motivo per cui l'espressione non è valida.
fn parse_expr_list(chars: &mut Peekable<Chars>) -> Result<Vec<Expr>, String> {
    let mut exprs = Vec::new();
    loop {
        skip_whitespace(chars);  
        
        if let Some(&')') = chars.peek() {
            // Consuma la parentesi ')'
            chars.next();  
            break;
        }
        
        // Parsifica ogni espressione
        exprs.push(parse_expr(chars)?);
    }
    Ok(exprs)
}

/// Salta gli spazi bianchi nel flusso di caratteri.
///
/// Questa funzione consuma tutti gli spazi bianchi presenti tra i caratteri in modo che il parser possa continuare
/// con i caratteri significativi.
fn skip_whitespace(chars: &mut Peekable<Chars>) {
    loop {
        // Preleva il carattere
        match chars.peek() {
            Some(&c) if c.is_whitespace() => { chars.next(); } // Se é uno spazio bianco lo consuma
            _ => break, 
        }
    }
}

/// Calcola il valore numerico di un'espressione.
///
/// Questa funzione calcola il valore di un'espressione aritmetica, eseguendo operazioni di somma e prodotto
/// in modo ricorsivo, a seconda del tipo di espressione.
///
/// # Parametri
/// - `expr`: Un riferimento all'espressione (`&Expr`) che deve essere valutata.
///
/// # Ritorno
/// Restituisce un valore di tipo `f64` che rappresenta il risultato numerico dell'espressione.
fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,  // Caso base: ritorna direttamente il valore
        /*
        * children.iter() => Itera su tutti i figli dell'espressione
        * .map(eval) => Esegue eval su ogni elemento dell'iteratore
         */
        Expr::Add(children) => children.iter().map(eval).sum(),  // Somma
        Expr::Mul(children) => children.iter().map(eval).product(),  // Prodotto
    }
}

fn main() {
    println!("Enter an arithmetic expression:");

    // Ottiene un handle per lo standard input, consentendo di leggere i dati da tastiera o da file.
    let stdin = io::stdin();
    // Blocca l'accesso allo standard input per leggere la riga successiva.
    // 'lines()' restituisce un iteratore che produce ogni riga come Result<String, std::io::Error>.
    // 'next()' restituisce il primo elemento dell'iteratore (una riga), mentre 'unwrap()' estrae il valore
    // e termina il programma se si verifica un errore.
    let line = stdin.lock().lines().next().unwrap().unwrap();
    // Converte la riga in un iteratore di caratteri.
    // 'peekable()' rende l'iteratore capace di guardare il prossimo carattere
    // senza avanzare l'iteratore né consumarlo.
    let mut chars = line.chars().peekable();
    
    
    /* 
    * Espressioni verificate
    * (+(+(1.8)(6))(*(1.1)(2)))
    * (+(+(1.8)(6))(*(1.1)(-2)))
    * (*(+(2)(3))(2.5)))
    */

    // 'parse_expr' restituisce un Result: Ok(expr) se il parsing ha successo, Err(e) in caso di errore.
    match parse_expr(&mut chars) {
        // Il parsing ha avuto successo
        Ok(expr) => {
            // Espressione parsata
            println!("Parsed expression: {:?}", expr);  
            println!("Result: {}", eval(&expr));  
        }
        // Errore durante il parsing
        Err(e) => eprintln!("Error: {}", e),  
    }
}
