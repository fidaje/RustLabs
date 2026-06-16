Implementare la funzione `forgettable_channel<T>()` che crea un canale
multi-produttore/ricevitore-singolo le cui semantiche estendono quelle di
`std::sync::mpsc::channel` con la possibilità di annullare un messaggio
già accodato prima che venga elaborato dal ricevitore.

Interfacce fornite
------------------
I tratti che seguono definiscono il contratto pubblico del canale; non
devono essere modificati.

    `Forgettable`
    Handle restituito da `send()`. Il metodo `forget()` tenta di annullare
    il messaggio corrispondente ancora in coda:
      - restituisce `true`  se il messaggio era ancora in attesa o il canale
        è stato chiuso prima che il messaggio fosse consegnato al ricevitore.
      - restituisce `false` se il ricevitore lo aveva già elaborato.

    `ForgettableSender<T>: Clone`
    Lato mittente. `send(t)` restituisce `Some(handle)` se il messaggio è
    stato accodato con successo, `None` se il ricevitore non esiste più.

    `ForgettableReceiver<T>`
    Lato ricevitore. `recv()` blocca finché non è disponibile un messaggio
    non annullato e restituisce `None` solo quando il canale è chiuso e la
    coda è vuota. I messaggi annullati vengono scartati silenziosamente.

Suggerimenti implementativi
--------------------------------
 1. Creare la coppia mittente/ricevitore sottostante (ad esempio tramite
    `std::sync::mpsc::channel`).
 2. Avvolgere il mittente in un tipo concreto che implementi sia
    `ForgettableSender<T>` sia `Clone`, così da supportare più produttori.
 3. Avvolgere il ricevitore in un tipo concreto che implementi
    `ForgettableReceiver<T>`.


Test di accettazione
--------------------
Il modulo `tests` in fondo al file contiene una suite completa. Tutti i
test devono passare senza modifiche.
