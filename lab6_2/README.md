# Esercizio 2: producer / consumer
Un pattern concorrente molto comune è il producer/consumer, dove un thread produce dei valori e li memorizza in una struttura condivisa (di solito una coda FIFO), e un secondo thread consuma i valori che estrae dalla struttura condivisa.

In questo setup molto semplice bisogna stare attenti ad alcune cose:
- la coda condivisa non può crescere all’infinito, ma se il producer è più veloce del consumer, il producer deve fermarsi finché il consumer non svuota almeno parzialmente la coda (questo meccanismo si chiama “backpressure” e permette di non saturare la memoria: casi molto comuni sono quando il producer legge da disco o riceve richieste da rete)
- quando la coda è vuota il consumer non dovrebbe usare cpu per controllare in continuazione se ci sono nuovi valori, ma dovrebbe essere svegliato solo quando c’è qualcosa da processare e farlo il più velocemente possibile
- occorre prestare molta attenzione alle condizioni di terminazione: quando il producer ha finto ed esce, il consumer deve prima processare tutti gli elementi nella coda prima di uscire anche lui; se esce prima il consumer il producer deve smettere di inserire nuovi valori e uscire pure lui.

Per gestire correttamente la terminazione possiamo creare una coda (**VecDeque**) che ha come elementi questa enum: `Item {Value(T), Stop}`. Quando il producer ha terminato può scrivere Stop nella coda ed esce, quando il consumer trova Stop esce pure lui.

Scrivere quindi una struct MyChannel che permetta di gestire più producer e più consumer rispettando i vincoli espressi sopra. La struttura suggerita è la seguente:
```rust
impl<T> MyChannel<T> {
    pub write(item: T) -> Result<(),_> {}
    pub read() -> Result<T, _> {}
    pub close() {}
}
```

MyChannel dentro deve avere un buffer FIFO di n elementi (specificato in costruzione).

Se il buffer è pieno, la `write()` rimane appesa fino a quando non si libera dello spazio per scrivere.
Se il buffer è vuoto è la `read()` che deve rimanere bloccata finché non c’è un valore da leggere.
Quando si chiama `close()` il canale viene chiuso, da quel momento in avanti ogni `write()` fallirà, mentre le `read()` continueranno finché il buffer non è svuotato, per poi dare errore anche loro.
Implementata la struttura scrivere due thread: uno che produca N valori ad intervalli casuali e poi chiami `close()` ed esca, uno che li consumi stampandoli. Verificare che vengano tutti ricevuti.

Suggerimenti: siccome sia `read()` che `write()` possono doversi bloccare e attendere che accada “qualcosa” nell’altro thread, considerare l’uso di una **condition variable** nella soluzione.

L’alternativa sarebbe fare busy waiting, testando in loop la lunghezza della struttura per ogni read e write: potete provarla e confrontare la differenza di prestazioni e uso CPU