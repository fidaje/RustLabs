# Traccia
Il tratto ConcurrentCache definisce l'interfaccia di una cache thread-safe con scadenza automatica delle voci.    
## Descrizione  
Una cache è un archivio di dati che consente di recuperare rapidamente i dati che sono stati inseriti in precedenza.  Ogni dato è identificato da una chiave univoca e ha un tempo di validità (calcolato dal momento dell'inserimento)  corrispondente al valore definito con la creazione della cache.  Il dato viene inserito nella cache specificando una chiave ed il relativo valore (entrambi di tipo stringa).  La chiave viene in seguito utilizzata per recuperare il valore associato.  Se il valore associato alla chiave è scaduto, la cache si comporta come se la chiave non esistesse.  
Si provveda a definire la struct ConcurrentCacheImpl che implementa tale tratto, garantendo  la correttezza del funzionamento in presenza di richieste concorrenti da parte di più thread  e garantendo la corretta gestione delle risorse.    
## Requisiti  
- Comportamento thread-safe di tutti i metodi  
- Eliminazione automatica delle voci scadute  
- Corretto rilascio delle risorse quando la cache viene distrutta    
## Sicurezza rispetto alla concorrenza  
L'implementazione deve essere  thread-safe. Ciò significa che più thread devono poter  accedere e modificare la cache contemporaneamente senza causare corse critiche o dare origine  a comportamenti non definiti.    
## Eliminazione automatica delle voci scadute  
L'implementazione deve considerare scadute le voci che sono state inserite nella cache  dopo la durata indicata in fase di costruzione della cache e, in questo caso,  comportarsi come se la voce richiesta fosse assente.  Le voci scadute devono essere rimosse automaticamente dalla cache, indipendentemente dal fatto che  vengano richieste o meno.    
## Corretto rilascio delle risorse quando la cache viene distrutta  
L'implementazione deve procedere al corretto rilascio delle proprie risorse quando la cache viene distrutta.

