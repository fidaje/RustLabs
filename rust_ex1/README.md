Si implementi in RUST la struct `CountDownLock` che permette a uno o più thread di attendere che un gruppo di operazioni eseguite da altri thread siano eseguite.

Essa incapsula un contatore ed offre i seguenti tre metodi thread-safe (oltre alla propria funzione costruttrice) che devono essere implementati:
- `fn new(n: usize) -> Self`: inizializza la struttura, impostando ad n il contatore
- `fn count_down(&self)`: decrementa il contatore, se maggiore di 0
- `fn wait(&self)`: blocca l'esecuzione del chiamante senza consumare cicli di cpu, finché il contatore non diventa zero
- `fn wait_timeout(&self, d: Duration) -> std::sync::WaitTimeoutResult`: blocca l'esecuzione del chiamante senza consumare cicli di cpu, in attesa cha il contatore raggiunga 0 per una durata massima pari a d, restituendo il risultato dell'attesa.