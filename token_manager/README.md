# Traccia
## TokenManager

Un applicativo software multithread fa accesso ai servizi di un server remoto, attraverso richieste di tipo HTTP.
Tali richieste devono includere un token di sicurezza che identifica l'applicativo stesso e ne autorizza l'accesso.
Per motivi di sicurezza, il token ha una validità limitata nel tempo (qualche minuto) e deve essere rinnovato alla sua scadenza.
Il token viene ottenuto attraverso una funzione (fornita esternamente e conforme al tipo `TokenAcquirer`) che restituisce alternativamente un token e la sua data di scadenza o un messaggio di errore se non è possibile fornirlo.
Poiché la emissione richiede un tempo apprezzabile (da alcune centinaia di millisecondi ad alcuni secondi), si vuole centralizzare la gestione del token, per evitare che più thread ne facciano richiesta in contemporanea.
A tale scopo deve essere implementata la struct TokenManager che si occupa di gestire il rilascio, il rinnovo e la messa a disposizione del token a chi ne abbia bisogno, secondo la logica di seguito indicata.

La struct TokenManager offre i seguenti metodi:
```rust
type TokenAcquirer = dyn Fn() => Result<(String, Instant), String> + Sync

pub fn new(acquire\_token: Box<TokenAcquirer> ) -> Self

pub fn get_token(&self) -> Result<string, string="">

pub fn try_get_token(&self) -> Option<string>
```

Al proprio interno, la `struct TokenManager` mantiene 3 possibili stati:

1. `Empty`- indica che non è ancora stato richiesto alcun token
2. `Pending` - indica che è in corso una richiesta di acquisizione del token
3. `Valid` - indica che è disponibile un token in corso di validità

Il metodo `new(...)` riceve il puntatore alla funzione in grado di acquisire il token. Essa opera in modalità pigra e si limita a creare un'istanza della struttura con le necessarie informazioni per gestire il suo successivo comportamento.

Il metodo `get_token(...)` implementa il seguente comportamento:

* Se lo stato è Empty, passa allo stato Pending e invoca la funzione per acquisire il token; se questa ritorna un risultato valido, memorizza il token e la sua scadenza, porta lo stato a Valid e restituisce copia del token stesso; se, invece, questa restituisce un errore, pone lo stato a Empty e restituisce l'errore ricevuto.
* Se lo stato è Pending, attende senza consumare cicli di CPU che questo passi ad un altro valore, dopodiché si comporta di conseguenza.
* Se lo stato è Valid e il token non risulta ancora scaduto, ne restituisce una copia; altrimenti pone lo stato ad Pending e inizia una richiesta di acquisizione, come indicato sopra.

Il metodo `try_get_token(...)` implementa il seguente comportamento:

* Se lo stato è Valid e il token non è scaduto, restituisce una copia del token opportunamente incapsulata in un oggetto di tipo `Option`.
* In tutti gli altri casi restituisce `None`.

Si implementi tale struttura nel linguaggio Rust.

A supporto della validazione del codice realizzato si considerino i seguenti test (due dei quali sono forniti con la relativa implementazione, i restanti sono solo indicati e possono essere opportunamente completati):

```rust
#[test]

fn a\_new\_manager\_contains\_no\_token() {

let a: Box<tokenacquirer> = Box::new(|| Err("failure".to\_string()));

let manager = TokenManager::new(a);

assert!(manager.try\_get\_token().is\_none());

}

#[test]

fn a\_failing\_acquirer\_always\_returns\_an\_error() {

let a: Box<tokenacquirer> = Box::new(|| Err("failure".to\_string()));

let manager = TokenManager::new(a);

assert\_eq!(manager.get\_token(), Err("failure".to\_string()));

assert\_eq!(manager.get\_token(), Err("failure".to\_string()));

}

#[test]

fn a\_successful\_acquirer\_always\_returns\_success() {

//...to be implemented

}

#[test]

   fn a\_slow\_acquirer\_causes\_other\_threads\_to\_wait() {

//...to be implemented

}
```
