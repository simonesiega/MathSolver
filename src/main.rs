#[allow(unused)]
use std::io::{self, Write};

/// # Logging macros personalizzate per debug e release
///
/// Questo blocco definisce un set di macro di logging (`trace_log!`, `debug_log!`, `info_log!`, `warn_log!`, `error_log!`) 
/// che funzionano solo in modalità `debug` (quando `cfg(debug_assertions)` è attivo).
///
/// In modalità `release`, tutte queste macro diventano no-op (non fanno nulla),
/// riducendo overhead del logging in produzione.
// MACRO ATTIVE IN MODALITÀ DEBUG //
#[cfg(debug_assertions)]
macro_rules! trace_log { ($($arg:tt)*) => { log::trace!($($arg)*); }; }
#[cfg(debug_assertions)]
#[allow(unused)]
macro_rules! debug_log { ($($arg:tt)*) => { log::debug!($($arg)*); }; }
#[cfg(debug_assertions)]
macro_rules! info_log { ($($arg:tt)*) => { log::info!($($arg)*); }; }
#[cfg(debug_assertions)]
macro_rules! warn_log { ($($arg:tt)*) => { log::warn!($($arg)*); }; }
#[cfg(debug_assertions)]
macro_rules! error_log { ($($arg:tt)*) => { log::error!($($arg)*); }; }

// VERSIONI NO-OP IN MODALITÀ RELEASE 
#[cfg(not(debug_assertions))]
macro_rules! trace_log { ($($arg:tt)*) => {}; }
#[cfg(not(debug_assertions))]
macro_rules! debug_log { ($($arg:tt)*) => {}; }
#[cfg(not(debug_assertions))]
macro_rules! info_log { ($($arg:tt)*) => {}; }
#[cfg(not(debug_assertions))]
macro_rules! warn_log { ($($arg:tt)*) => {}; }
#[cfg(not(debug_assertions))]
macro_rules! error_log { ($($arg:tt)*) => {}; }

/// # Enum `Token`
///
/// Rappresenta i token lessicali riconosciuti.
/// Ogni variante corrisponde a un tipo di simbolo nel linguaggio aritmetico:
/// - `Number(f64)`: un numero decimale.
/// - `Plus`, `Minus`, `Multiply`, `Divide`: operatori aritmetici.
/// - `LeftParen`, `RightParen`: parentesi tonde.
/// - `Equals`: simbolo di fine espressione o assegnazione.
///
/// Derive:
/// - `Debug`: per la stampa leggibile durante debug/log.
/// - `Clone` e `Copy`: per duplicare i token, poiché sono tipi leggeri e immutabili.
/// - `PartialEq`: per confrontare i token tra loro (es parser).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    /// Numero reale (es. 3.14, 42.0)
    Number(f64),
    
    /// Operatore di somma: '+'
    Plus,       
    
    /// Operatore di sottrazione: '-'
    Minus,       
    
    /// Operatore di moltiplicazione: '*'
    Multiply,  
    
    /// Operatore di divisione: '/'
    Divide,  
    
    /// Parentesi aperta: '('
    LeftParen, 
    
    /// Parentesi chiusa: ')'
    RightParen,  
    
    /// Simbolo di fine espressione: '='
    Equals,      
}

impl Token {
    /// Crea un token a partire da un carattere specifico.
    ///
    /// Restituisce `Some(Token)` se il carattere corrisponde a un token valido,
    /// altrimenti `None`.
    ///
    /// # Parametri
    /// - `c`: Il carattere da interpretare come token.
    ///
    /// # Esempio
    /// ```
    /// assert_eq!(Token::from_char('+'), Some(Token::Plus));
    /// assert_eq!(Token::from_char('x'), None);
    /// ```
    #[inline] // Suggerisce al compilatore di inserire questa funzione inline per efficienza.
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '=' => Some(Token::Equals),
            _ => None, // carattere non riconosciuto come token
        }
    }

    /// Verifica se il token è un operatore binario (matematico).
    ///
    /// # Esempio
    /// ```
    /// assert!(Token::Plus.is_operator());
    /// assert!(!Token::LeftParen.is_operator());
    /// ```
    #[inline]
    #[allow(unused)]
    fn is_operator(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Multiply | Token::Divide)
    }
}

/// Tipi di errore che possono verificarsi durante l'esecuzione di calcoli matematici.
///
/// Viene usato nel valutatore di espressioni aritmetiche per segnalare
/// errori come divisioni per zero o limiti computazionali.
///
/// Derive:
/// - `Debug`: consente la stampa dell'errore per log o debug.
/// - `PartialEq`: confrontare errori nei test o nel flusso di controllo.
#[derive(Debug, PartialEq)]
#[allow(unused)]
enum MathError {
    /// Divisione per zero.
    DivisionByZero,

    /// Il risultato ha superato i limiti superiori rappresentabili.
    OverflowError,

    /// Il risultato è sceso sotto i limiti inferiori rappresentabili.
    UnderflowError,

    /// L'espressione contiene troppi elementi o nidificazioni.
    // Attualmente non implementato
    ExpressionTooComplex,
}

/// Tipi di errore che possono verificarsi durante la fase di tokenizzazione o parsing.
/// Usato per indicare errori di sintassi o input invalido.
///
/// Derive:
/// - `Debug`: consente la stampa dell'errore per log o debug.
/// - `PartialEq`: confrontare errori nei test o nel flusso di controllo.
#[derive(Debug, PartialEq)]
#[allow(unused)]
enum TokenError {
    /// Numero malformato o non valido (es. "1..2").
    InvalidNumber(String),

    /// L'input termina in modo inaspettato (es. Parentesi non chiusa).
    UnexpectedEnd,

    /// Espressione invalida in senso sintattico.
    InvalidExpression(String),

    /// Operatore non riconosciuto (es. '%', '^', ecc.).
    InvalidOperator(char),

    /// Parentesi chiusa senza apertura o viceversa, include carattere e posizione.
    UnmatchedParenthesis { found: char, position: usize },

    /// Token inaspettato trovato in una certa posizione del parsing.
    UnexpectedToken(Token),

    /// Errore sintattico generico, con descrizione.
    // Attualmente non implementato
    SyntaxError(String),
}

/// Implementazione del trait `Display` per `MathError`.
///
/// Permette la conversione leggibile dell'errore in una stringa,
/// utile per l'output verso l'utente o log.
///
/// Inoltre, ogni ramo logga l'errore con `error_log!`,
/// che è abilitato solo in modalità `debug_assertions`.
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => {
                error_log!("Errore: divisione per zero");
                write!(f, "Errore matematico: divisione per zero")
            },
            MathError::OverflowError => {
                error_log!("Errore: overflow numerico");
                write!(f, "Errore matematico: overflow numerico")
            },
            MathError::UnderflowError => {
                error_log!("Errore: underflow numerico");
                write!(f, "Errore matematico: underflow numerico")
            },
            MathError::ExpressionTooComplex => {
                error_log!("Errore: espressione troppo complessa");
                write!(f, "Errore: espressione troppo complessa")
            },
        }
    }
}

/// Implementazione del trait `Display` per `TokenError`.
///
/// Permette la conversione leggibile dell'errore in una stringa,
/// utile per l'output verso l'utente o log.
///
/// Inoltre, ogni ramo logga l'errore con `warn_log!`,
/// che è abilitato solo in modalità `debug_assertions`.
impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenError::InvalidNumber(msg) => {
                warn_log!("Numero non valido: {}", msg);
                write!(f, "Numero non valido: {}", msg)
            },
            TokenError::UnmatchedParenthesis { found, position } => {
                warn_log!("Parentesi mancante '{}' alla posizione {}", found, position);
                write!(f, "Errore: mancante '{}' alla posizione {}", found, position)
            },
            TokenError::UnexpectedEnd => {
                error_log!("Errore: fine espressione inaspettata");
                write!(f, "Errore: espressione terminata inaspettatamente")
            },
            TokenError::InvalidExpression(msg) => {
                error_log!("Errore: espressione non valida ({})", msg);
                write!(f, "Errore: espressione non valida - {}", msg)
            },
            TokenError::InvalidOperator(op) => {
                warn_log!("Operatore non valido: '{}'", op);
                write!(f, "Errore: operatore non valido '{}'", op)
            },
            TokenError::UnexpectedToken(token) => {
                warn_log!("Token inatteso: {:?}", token);
                write!(f, "Errore: token inatteso {:?}", token)
            },
            TokenError::SyntaxError(msg) => {
                error_log!("Errore di sintassi: {}", msg);
                write!(f, "Errore di sintassi: {}", msg)
            }
        }
    }
}

/// Implementazione del trait `Error` per `MathError`.
///
/// Consente di trattare `MathError` come un errore standard, 
/// ad esempio per l'uso con `?`.
impl std::error::Error for MathError {}

/// Implementazione del trait `Error` per `TokenError`.
///
/// Consente di trattare `TokenError` come un errore standard, 
/// ad esempio per l'uso con `?`.
impl std::error::Error for TokenError {}

/// Rappresenta un errore generico durante il calcolo.
///
/// Permette di unificare gli errori matematici (`MathError`)
/// e gli errori di tokenizzazione/parsing (`TokenError`).
///
/// - `Debug`, `PartialEq`.
#[derive(Debug, PartialEq)]
enum CalcError {
    // Errore matematico
    Math(MathError),
    // Errore durante il parsing
    Token(TokenError),
}

/// Conversione automatica da `MathError` a `CalcError`.
/// Permette di usare `?` in funzioni che restituiscono `CalcResult`.
impl From<MathError> for CalcError {
    fn from(e: MathError) -> Self {
        CalcError::Math(e)
    }
}

/// Conversione automatica da `CalcError` a `MathError`.
/// Permette di `?` in funzioni che restituiscono `CalcResult`.
impl From<TokenError> for CalcError {
    fn from(e: TokenError) -> Self {
        CalcError::Token(e)
    }
}

/// Implementazione di `Display` per `CalcError`.
///
/// Produce un messaggio leggibile combinando `MathError` e `TokenError`.
/// I messaggi dettagliati vengono delegati ai rispettivi `Display`.
impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalcError::Math(e) => write!(f, "Errore matematico: {}", e),
            CalcError::Token(e) => write!(f, "Errore di parsing: {}", e),
        }
    }
}

/// Implementazione del trait `Error` per `CalcError`.
///
/// Consente di trattare `CalcError` come un errore standard, 
/// ad esempio per l'uso con `?`.
impl std::error::Error for CalcError {}

/// Alias per il tipo di risultato restituito dalle funzioni di calcolo.
///
/// - `Ok(f64)`: rappresenta il risultato numerico del calcolo.
/// - `Err(CalcError)`: rappresenta un errore che può essere:
///   - `MathError`: errori aritmetici (es. Divisione per zero, overflow).
///   - `TokenError`: errori di sintassi o di parsing dell'espressione.
type CalcResult = Result<f64, CalcError>;


/// Struttura responsabile dell'analisi lessicale di un'espressione matematica.
///
/// Divide la stringa di input in una sequenza di token riconoscibili.
/// Tiene traccia della posizione corrente durante la scansione.
/// - `'a`: Lifetime del riferimento alla stringa di input.
/// - Utilizza un riferimento immutabile (`&'a str`) per evitare copie non necessarie della stringa.
/// - `position` tiene traccia dell'indice corrente durante la scansione dei caratteri.
struct Tokenizer<'a> {
    /// Slice immutabile della stringa di input contenente l'espressione da analizzare.
    input: &'a str,
    /// Posizione corrente nell'input, utilizzata per tracciare l'avanzamento durante la tokenizzazione.
    position: usize,
}

impl<'a> Tokenizer<'a> {
    /// Crea una nuova istanza di `Tokenizer` per una data stringa di input.
    ///
    /// # Parametri
    /// - `input`: riferimento alla stringa da analizzare.
    ///
    /// # Ritorna
    /// - Istanza inizializzata di `Tokenizer` con posizione iniziale a zero.
    fn new(input: &'a str) -> Self {
        info_log!("Tokenizer creato. Input: '{}'", input);
        Self { input, position: 0 }
    }

    /// Analizza la stringa di input e produce una sequenza di token.
    ///
    /// # Ritorna
    /// - `Ok(Vec<Token>)` in caso di successo.
    /// - `Err(TokenError)` se viene rilevato un errore di sintassi o simbolo non valido.
    fn tokenize(&mut self) -> Result<Vec<Token>, TokenError> {
        info_log!("Avvio tokenizzazione");
        let mut tokens = Vec::new();

        // Scorre ogni carattere finché non raggiunge la fine dell'input.
        while self.position < self.input.len() {
            let c = self.current_char();
            
            match c {
                // Ignora spazi bianchi.
                c if c.is_whitespace() => self.advance(),

                // Gestisce sequenze numeriche, inclusi decimali.
                c if c.is_ascii_digit() || c == '.' => {
                    let token = self.parse_number()?;
                    trace_log!("Token numero trovato: {:?}", token);
                    tokens.push(token);
                }

                // Gestisce simboli e operatori.
                c => {
                    // Gestione token riconosciuti.
                    if let Some(token) = Token::from_char(c) {
                        trace_log!("Token simbolo trovato: {:?}", token);
                        tokens.push(token);
                        self.advance();
                    } 
                    // Gestisce token non riconosciuti con InvalidOperator, c - carattere non riconosciuto.
                    else {
                        return Err(TokenError::InvalidOperator(c));
                    }
                }
            }
        }
        
        // Tokenizzazione completata, ritorna OK e il vettore di Token da parsare.
        info_log!("Tokenizzazione completata: {:?}", tokens);
        Ok(tokens)
    }

    /// Analizza e costruisce un token numerico a partire dalla posizione corrente.
    ///
    /// Supporta numeri interi e decimali. Non sono ammessi più punti decimali.
    ///
    /// # Ritorna
    /// - `Ok(Token::Number(f64))` se il parsing ha successo.
    /// - `Err(TokenError::InvalidNumber)` in caso di numero malformato.
    fn parse_number(&mut self) -> Result<Token, TokenError> {
        let start = self.position;
        let mut has_decimal = false;

        // Continua a leggere finché i caratteri fanno parte del numero.
        while self.position < self.input.len() {
            match self.current_char() {
                c if c.is_ascii_digit() => self.advance(),

                // Accetta un solo punto decimale.
                '.' if !has_decimal => {
                    has_decimal = true;
                    self.advance();
                }

                // Rifiuta numeri con più punti decimali.
                // Se viene trovato un secondo '.' in un numero già marcato come decimale, viene generato un errore.
                // Esempio non valido: "2..3"
                '.' => return Err(TokenError::InvalidNumber("Numero con più punti decimali".into())),

                // Interrompe la lettura alla prima occorrenza non numerica.
                _ => break,
            }
        }

        // Estrae la sottostringa rappresentante un numero dalla posizione iniziale fino alla posizione corrente.
        let number_str = &self.input[start..self.position];

        // Tenta la conversione della sottostringa in un valore numerico `f64`.
        // In caso di successo, restituisce un token `Token::Number(n)` contenente il valore.
        // In caso di errore nel parsing, genera un errore `TokenError::InvalidNumber` contenente la stringa non valida.
        match number_str.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err(TokenError::InvalidNumber(number_str.to_string())),
        }
    }

    /// Restituisce il carattere corrente dell'input in base alla posizione attuale.
    /// Utilizza `chars().next().unwrap()` per accedere al primo carattere rimanente,
    /// assumendo che la posizione sia sempre valida e non oltre la lunghezza dell'input.
    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    /// Avanza la posizione corrente di un'unità, spostandosi al carattere successivo dell'input.
    /// La posizione è basata sugli indici dei caratteri e presuppone che `current_char()` sia stato già valutato.
    fn advance(&mut self) {
        self.position += 1;
    }
}


/// Parser per espressioni matematiche basate su una sequenza di token.
/// Gestisce l'analisi sintattica e la valutazione delle espressioni secondo la precedenza degli operatori.
struct MathExpressionParser {
    /// Sequenza di token generati dal tokenizer.
    tokens: Vec<Token>,
    /// Posizione corrente all'interno del vettore di token.
    position: usize,
}

impl MathExpressionParser {
    /// Costruisce un nuovo parser partendo da una sequenza di token.
    ///
    /// # Parametri
    /// - `tokens`: Vettore di token pre-analizzati da valutare.
    ///
    /// # Ritorna
    /// Un'istanza inizializzata di `MathExpressionParser` con posizione iniziale a zero.
    fn new(tokens: Vec<Token>) -> Self {
        info_log!("Parser inizializzato con tokens: {:?}", tokens);
        Self { tokens, position: 0 }
    }

    /// Valuta un'espressione aritmetica completa secondo la grammatica formale.
    ///
    /// Questo metodo rappresenta l'ingresso principale per il parsing e la valutazione
    /// di una formula, seguendo la regola grammaticale:
    /// ```
    /// F → E "="
    /// ```
    ///
    /// # Comportamento
    /// - Valuta l'espressione tramite `evaluate_expression()`.
    /// - Verifica la presenza del simbolo `=` alla fine.
    /// - Restituisce il risultato della valutazione se tutto è corretto, altrimenti segnala un errore.
    ///
    /// # Ritorna
    /// - `Ok(f64)` se l'espressione è valida e terminata correttamente con `=`
    /// - `Err(CalcError)` in caso di errore sintattico (token inatteso, fine prematura) o semantico
    ///
    /// # Esempi
    /// ```
    /// let mut parser = Parser::new("2 + 3 * 4 =");
    /// let result = parser.evaluate();
    /// assert_eq!(result.unwrap(), 14.0);
    /// ```
    ///
    /// ```
    /// let mut parser = Parser::new("2 + =");
    /// let result = parser.evaluate();
    /// assert!(result.is_err()); // Errore: manca un termine dopo '+'
    /// ```
    ///
    /// # Note
    /// - Il simbolo `=` è obbligatorio come delimitatore finale, ma non partecipa al calcolo.
    /// - I log interni aiutano a tracciare lo stato della valutazione.
    fn evaluate(&mut self) -> CalcResult {
        info_log!("Inizio valutazione");
        let result = self.evaluate_expression()?; // Analizza e valuta un'espressione intera.

        // Controlla se dopo l'espressione è presente un simbolo '=' (atteso).
        match self.peek() {
            Some(&Token::Equals) => {
                info_log!("Valutazione completata con successo");
                Ok(result)
            },
            Some(token) => {
                // Errore: token inatteso dopo la fine dell'espressione.
                error_log!("Token inatteso dopo valutazione: {:?}", token);
                Err(TokenError::UnexpectedToken(*token).into())
            },
            None => {
                // Errore: espressione terminata senza '=' esplicito.
                error_log!("Espressione incompleta alla fine");
                Err(TokenError::UnexpectedEnd.into())
            }
        }
    }

    /// Valuta un'espressione aritmetica composta da termini con operazioni di somma e sottrazione.
    ///
    /// Questo metodo implementa la regola grammaticale:
    /// ```
    /// E → P
    ///    | E "+" P
    ///    | E "−" P
    /// ```
    ///
    /// # Comportamento
    /// - Valuta un primo termine tramite `evaluate_term()`.
    /// - Iterativamente gestisce operatori di somma (`+`) e sottrazione (`−`) tra termini successivi.
    /// - Ogni operazione aritmetica è verificata tramite `check_overflow` per gestire eventuali errori numerici.
    ///
    /// La valutazione termina quando non sono più presenti operatori `+` o `-` oppure quando l'input è stato completamente processato.
    ///
    /// # Ritorna
    /// - `Ok(f64)` con il risultato parziale o completo dell’espressione.
    /// - `Err(CalcError)` in caso di errore sintattico (es. operatore mancante o inatteso) o semantico.
    ///
    /// # Note
    /// - La precedenza degli operatori è rispettata grazie alla separazione delle regole tra `E`, `P` e `T`.
    /// - Gli operatori non riconosciuti (`*`, `/`, implicita, ecc.) sono gestiti da livelli grammaticali inferiori e causano l’uscita anticipata dal ciclo.
    fn evaluate_expression(&mut self) -> CalcResult {
        let mut result = self.evaluate_term()?; // Valutazione del primo termine.

        // Gestione degli operatori '+' e '-' tra i termini.
        while let Some(token) = self.peek() {
            match *token {
                
                Token::Plus => {
                    self.advance();
                    let rhs = self.evaluate_term()?; // Valuta il termine a destra dell'operatore.
                    trace_log!("Operazione: {} + {}", result, rhs);
                    result = self.check_overflow(result + rhs)?; // Verifica overflow e calcola il risultato.
                }
                
                Token::Minus => {
                    self.advance();
                    let rhs = self.evaluate_term()?; // Valuta il termine a destra dell'operatore.
                    trace_log!("Operazione: {} - {}", result, rhs);
                    result = self.check_overflow(result - rhs)?; // Verifica overflow e calcola il risultato.
                }
                
                _ => break, // Se non è un operatore di somma o sottrazione, esce dal ciclo.
            }
        }

        // Restituisce il risultato finale dell'espressione.
        Ok(result)
    }

    /// Valuta un termine aritmetico, che può includere moltiplicazioni esplicite (`*`), divisioni (`/`)
    /// o moltiplicazioni implicite (es. `2(3 + 4)`), partendo dalla posizione corrente.
    ///
    /// Questo metodo implementa la regola grammaticale:
    ///
    /// ```
    /// P → T
    ///    | P "*" T
    ///    | P "/" T
    ///    | P T
    /// ```
    ///
    /// # Comportamento
    /// - Valuta il primo fattore tramite `evaluate_factor()`.
    /// - Gestisce in modo iterativo gli operatori di moltiplicazione (`*`), divisione (`/`)
    ///   e la moltiplicazione implicita (quando un numero o una parentesi segue direttamente un termine).
    /// - Ogni operazione è verificata tramite `check_overflow` per prevenire errori numerici.
    /// - Se viene rilevata una divisione per zero, viene generato un errore specifico.
    ///
    /// # Ritorna
    /// - `Ok(f64)` con il risultato del termine valutato.
    /// - `Err(CalcError)` in caso di errore sintattico, divisione per zero o overflow aritmetico.
    ///
    /// # Note
    /// - La moltiplicazione implicita è gestita riconoscendo una sequenza `P T`
    ///   come `P * T`, dove `T` può essere un numero o un'espressione tra parentesi.
    /// - Le operazioni terminano appena viene incontrato un token che non corrisponde
    ///   a un operatore di prodotto o a un fattore immediatamente moltiplicabile.
    fn evaluate_term(&mut self) -> CalcResult {
        debug_log!("Valutazione termine alla posizione: {}", self.position);
        let mut result = self.evaluate_factor()?; // Valutazione il primo fattore
        
        // Cerca operazioni di moltiplicazione o divisione
        while let Some(token) = self.peek() {
            match *token {
                
                Token::Multiply => {
                    self.advance();
                    let factor = self.evaluate_factor()?; // Valuta il fattore successivo
                    trace_log!("Moltiplicazione: {} * {}", result, factor);
                    result = self.check_overflow(result * factor)?; // Moltiplicazione con controllo overflow
                }
                
                Token::Divide => {
                    self.advance();
                    let factor = self.evaluate_factor()?; // Valuta il fattore successivo

                    if factor == 0.0 { return Err(MathError::DivisionByZero.into()); } // Errore di divisione per zero

                    trace_log!("Divisione: {} / {}", result, factor);
                    result = self.check_overflow(result / factor)?; // Divisione con controllo overflow
                }
                
                Token::Number(_) | Token::LeftParen => {
                    let factor = self.evaluate_factor()?; // Valuta il fattore successivo (moltiplicazione implicita)
                    trace_log!("Moltiplicazione implicita: {} * {}", result, factor);
                    result = self.check_overflow(result * factor)?; // Moltiplicazione implicita con controllo overflow
                }
                
                _ => break, // Se non è un operatore di moltiplicazione o divisione, esce dal ciclo.
            }
        }
        // Restituisce il risultato del termine
        Ok(result) 
    }

    /// Valuta un fattore dell'espressione aritmetica, che può essere:
    /// - Un numero senza segno (es. `3.14`)
    /// - Un'espressione tra parentesi tonde (es. `(2 + 3)`)
    /// - Un fattore preceduto da un operatore di negazione (`-`)
    ///
    /// Questo metodo implementa la regola grammaticale:
    ///
    /// ```
    /// T → "−" T
    ///    | unsigned number
    ///    | "(" E ")"
    /// ```
    ///
    /// # Comportamento
    /// - Se il token corrente è un numero (`Token::Number`), viene restituito direttamente come valore `f64`.
    /// - Se è un operatore `-`, viene effettuata una chiamata ricorsiva a `evaluate_factor()` e il risultato viene negato.
    /// - Se è una parentesi aperta `(`, viene valutata un'espressione con `evaluate_expression()` fino a trovare la parentesi chiusa `)`.
    /// - Se la parentesi di chiusura è mancante o errata, viene restituito un errore di parentesi non corrispondente.
    /// - Qualsiasi token inatteso viene considerato errore sintattico.
    ///
    /// # Ritorna
    /// - `Ok(f64)` con il valore del fattore valutato.
    /// - `Err(CalcError)` se si verifica un errore di sintassi (parentesi non corrispondenti, token imprevisti)
    ///   o semantico (struttura non valida dell'espressione).
    ///
    /// # Esempi
    /// ```
    /// // Valuta il numero semplice
    /// let mut parser = Parser::new("3.14 =");
    /// assert_eq!(parser.evaluate_factor().unwrap(), 3.14);
    /// ```
    ///
    /// ```
    /// // Valuta la negazione di un numero
    /// let mut parser = Parser::new("-2.5 =");
    /// assert_eq!(parser.evaluate_factor().unwrap(), -2.5);
    /// ```
    ///
    /// ```
    /// // Valuta espressione tra parentesi
    /// let mut parser = Parser::new("(1 + 2) =");
    /// assert_eq!(parser.evaluate_factor().unwrap(), 3.0);
    /// ```
    fn evaluate_factor(&mut self) -> CalcResult {
        match self.next() {
            // Caso di numero: restituisce il numero come valore
            Some(Token::Number(n)) => Ok(n),

            // Caso di negazione: valuta il fattore successivo e lo nega
            Some(Token::Minus) => {
                let val = self.evaluate_factor()?; // Negazione del fattore
                trace_log!("Negazione di {}", val);
                Ok(-val)
            },

            // Caso di parentesi aperta: valuta l'espressione tra parentesi
            Some(Token::LeftParen) => {
                let result = self.evaluate_expression()?;  // Analizza l'espressione tra parentesi
                
                match self.next() {
                    // Verifica che la parentesi chiusa corrisponda alla parentesi aperta
                    Some(Token::RightParen) => Ok(result),
                    
                    // Se viene trovato un altro token invece di una parentesi chiusa, errore
                    Some(tok) => {
                        error_log!("Token inatteso invece di ')': {:?}", tok);
                        Err(TokenError::UnmatchedParenthesis { found: ')', position: self.position }.into())
                    },
                    
                    // Se non c'è un token successivo (parentesi chiusa mancante)
                    None => Err(TokenError::UnmatchedParenthesis { found: '(', position: self.position }.into()),
                }
            },

            // Caso di parentesi chiusa senza corrispondente parentesi aperta
            Some(Token::RightParen) => {
                error_log!("Parentesi chiusa senza apertura");
                Err(TokenError::UnmatchedParenthesis { found: ')', position: self.position }.into())
            },

            // Caso di errore generale: token non valido trovato
            token => {
                error_log!("Fattore non valido trovato: {:?}", token);
                Err(TokenError::InvalidExpression("Espressione non valida".into()).into())
            }
        }
    }

    /// Verifica se il valore è valido, controllando eventuali condizioni di overflow o underflow.
    ///
    /// # Ritorna
    /// - `Ok(f64)` se il valore non è né infinito né subnormale.
    /// - `Err(CalcError)` in caso di overflow (valore infinito) o underflow (valore subnormale).
    ///
    /// Questa funzione si occupa di monitorare la validità del valore calcolato, restituendo un errore in caso di:
    /// - Overflow: se il valore calcolato è infinito.
    /// - Underflow: se il valore calcolato è un numero subnormale, che può indicare una perdita di precisione o un valore troppo piccolo.
    ///
    fn check_overflow(&self, val: f64) -> Result<f64, CalcError> {
        // Infinito
        if val.is_infinite() {
            Err(MathError::OverflowError.into())
        }
        // 0    
        else if val.is_subnormal() {
            Err(MathError::UnderflowError.into())
        }
            
        else {
            Ok(val)
        }
    }

    /// Restituisce il token corrente senza avanzare nella posizione.
    ///
    /// # Ritorna
    /// - `Some(&Token)` se esiste un token alla posizione corrente.
    /// - `None` se la posizione corrente è fuori dai limiti dell'elenco di token.
    ///
    /// Permette di esaminare il token attuale senza spostare la posizione del parser. 
    /// È utile per fare previsioni sui token successivi o per determinare la posizione attuale nel flusso di token.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Restituisce e avanza alla posizione successiva nella lista di token.
    ///
    /// # Ritorna
    /// - `Some(Token)` se esiste un token alla posizione corrente e avanza la posizione.
    /// - `None` se la posizione corrente è fuori dai limiti dell'elenco di token.
    ///
    /// Questo metodo restituisce il token attuale e incrementa la posizione, spostando così il parser
    /// alla posizione successiva. È utile per l'iterazione attraverso la lista di token.
    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).copied(); // Poiché prende un riferimento '&Token', .copied() usato per copiare il valore contenuto nell' Option 
        // Se esiste un token valido
        if token.is_some() { self.advance(); }
        token
    }

    /// Avanza alla posizione successiva nella lista di token.
    fn advance(&mut self) {
        self.position += 1;
    }
}

/// Modulo di test per il parsing e la valutazione delle espressioni matematiche.
///
/// Questo modulo contiene test unitari per verificare il comportamento della logica di parsing e valutazione,
/// con particolare attenzione alla gestione degli errori e alla corretta identificazione dei token.
#[cfg(test)]
mod tests {
    use super::*; // Importa tutti i membri del modulo superiore (il codice da testare)

    /// Test che simula l'errore di parentesi non corrispondenti.
    ///
    /// Questo test verifica come il tokenizer e il parser gestiscono una espressione con
    /// parentesi mancanti, simulando una situazione di errore nella sintassi dell'espressione.
    #[test]
    fn test_unmatched_parentheses_simulated() {
        let expression = "((1+2))))) ="; 
        
        let mut tokenizer = Tokenizer::new(expression); 
        let result = tokenizer.tokenize(); 
        let tokens = result.unwrap(); 
        let mut parser = MathExpressionParser::new(tokens); 
        
        println!("{:?}", parser.evaluate()); // Esegue la valutazione e stampa il risultato
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let input = "((1+2))))) =";
    info_log!("Input espressione: {}", input);

    let mut tokenizer = Tokenizer::new(input);

    let result = match tokenizer.tokenize() {
        Ok(tokens) => {
            let mut parser = MathExpressionParser::new(tokens);
            parser.evaluate()
        }
        Err(e) => Err(CalcError::Token(e))
    };

    match result {
        Ok(value) => {
            println!("Risultato: {:.3}", value);
            Ok(())
        }
        Err(e) => {
            // println!("Errore: {}", e);  
            match e {
                CalcError::Math(math_err) => {
                    error_log!("Errore matematico: {}", math_err);
                    Err(Box::new(math_err))
                }
                CalcError::Token(token_err) => {
                    error_log!("Errore di tokenizzazione: {}", token_err);
                    Err(Box::new(token_err))
                }
            }
        }
    }
}

