Un `CancelableLatch` è un tratto di sincronizzazione che permette a uno o più thread di attendere, senza consumare cicli di CPU, che altri thread eseguano i propri compiti e ne segnalino l'esito.

All'atto della creazione di questa struttura viene indicato il numero di compiti da attendere. 

La struttura offre:
- il metodo `count_down()` che permette di indicare che uno dei compiti è terminato con successo: se non restano altri compiti da attendere, le attese vengono sbloccate con successo, altrimenti proseguono.
- il metodo `cancel()` permette di segnalare che uno dei compiti è fallito: in questo caso, le attese vengono subito sbloccate indicando l'avvenuta cancellazione.
- un metodo di attesa incondizionato (ovvero, l'attesa si protrae fino a che tutti i compiti sono stati terminati con successo o è stata richiesta una cancellazione)
- un metodo di attesa con timeout (in questo caso, l'attesa può terminare anche se entro il tempo indicato non si raggiungono le condizioni precedenti: in tale caso viene segnalato che il tempo è scaduto).

Si realizzi, usando il linguaggio Rust, una struttura che implementi tale tratto

```rust
#[derive(PartialEq, Eq, Debug)]
pub enum WaitResult {
    Success,
    Timeout,
    Canceled
}

pub trait CancelableLatch {
    fn new(count: usize) -> Self;
    fn count_down(&self);
    fn cancel(&self);
    fn wait(&self) -> WaitResult;
    fn wait_timeout(&self, d: Duration) -> WaitResult;
}
```