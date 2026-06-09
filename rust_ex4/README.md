La struttura `MultiChannel` implementa il concetto di canale con molti mittenti e molti ricevitori.

I messaggi inviati a questo tipo di canale sono composti da singoli byte che vengono recapitati a tutti i ricevitori attualmente collegati. 

Metodi:
- `new() -> Self`: crea un nuovo canale senza alcun ricevitore collegato
- `subscribe(&self) -> Receiver<u8>`: collega un nuovo ricevitore al canale: da quando questo metodo viene invocato, gli eventuali byte inviati al canale saranno recapitati al ricevitore. Se il ricevitore viene eliminato, il canale continuerà a funzionare inviando i propri dati ai ricevitori restanti (se presenti), altrimenti ritornerà un errore 
- `send(&self, data: u8) -> Result<(), SendError<u8>>`: invia a tutti i sottoscrittori un byte. Se non c'è alcun sottoscrittore, notifica l'errore indicando il byte che non è stato trasmesso 