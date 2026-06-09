Esercizio 1: suddivisione lavoro fra thread
I thread sono spesso usati per suddividere il lavoro tra i core della cpu, per eseguire in
parallelo delle porzioni lavoro e unire i risultati. Questo aspetto diventa interessante quando:
● l’algoritmo è parallelizzabile, ovvero ogni thread può risolvere in modo indipendente
un pezzo di lavoro senza dover aspettare il risultato di alti thread
● le eventuali operazioni di sincronizzazione non sono troppo pesanti rispetto al lavoro
da svolgere per risolvere il problema
In questo esercizio vediamo alcuni problemi che possono essere facilmente divisi in thread
Numeri primi
Nel file file prime.rs viene fornita una versione naive di is_prime(n: us64), che verifica con
brute force se n è un numero primo. Scrivere una funzione che cerchi tutti i numeri primi tra
2 e limit, suddividendo il lavoro tra n thread:
pub fn find_primes(limit: u64, n_threads: u64) -> Vec<u64> {}

Per suddividere il lavoro avete due possibilità, implementarle entrambe e verificare la più efficiente:
● condividere una variabile counter tra ogni thread, ogni thread a turno incrementa counter e verifica se quel numero è primo, se è primo lo memorizza altrimenti lo scarta; alla fine restituisce i numeri primi che ha trovato
● non condividere nulla, ogni thread conta a partire da 2,3,4,5,...n a limit modulo n e verifica quel numero; in questo modo ogni thread proverà dei numeri differenti
(perché dividere in blocchi contigui sarebbe meno efficiente?)
Es con tre thread uno rispettivamente verificherà:
2 5 8 11 …
3 6 9 12 …
4 7 10 13 …
Provare la funzione cercando i numeri primi fino a un milione e passando da 1 a 16 thread:
il tempo diminuisce linearmente con l’aumento dei thread? Perché?
Per misurare i tempi usare: std::time::Instant::now()

**Verifica soluzioni**
Quando si ha un set di dati da filtrare o di soluzioni da verificare, se ogni elemento del set è
indipendente allora è facile suddividere il lavoro tra thread, ogni thread si occupa di un
blocco di dati in parallelo con altri.

Un esempio è questo rompicapo: dati 5 numeri casuali scelti tra 0 e 9 occorre trovare una
sequenza di operazioni aritmetiche che li utilizzi tutti, in qualsiasi ordine, e che dia come
risultato 10.

Un soluzione è verificare brute force tutte le possibili combinazioni. 
Viene quindi fornito un file game.rs con una funzione che prepara tutte le possibili permutazioni dei 5 numeri scelti (es “74648”)
pub fn prepare(s: &str) -> Vec<String>
Il risultato è un vettore di stringhe con “esplose” tutte le possibili combinazioni date dalle
permutazioni delle 5 cifre e unite alle 4 operazioni aritmetiche (5! le cifre e 4^4 e le
operazioni, essendo ammessa ripetizione).
Si leggerò quindi qualcosa del tipo:[ "7+4+6+4+8", "7+4+6+4-8", "7+4+6+4*8", "7+4+6+4/8",
...]

Scrivere una funzione verify così definita
pub fn verify(v: &[String]) -> Vec<String> {}
che verifichi una per una le stringhe se danno come risultato 10 e restituisca quelle che
hanno successo. verify() va provata su 1,2,3,..16 thread, suddividendo il vettore in opportune
slice. 

Divisioni per 0 (zero) o frazionarie vanno scartate, tenere solo operazioni che diano
come risultato interi.

Non è ammesso copiare completamente il vettore in ogni thread, il vettore è unico e i thread
ricevono uno slice.

Suggerimenti:
● suddividendo il vettore in slice non sovrapposti non c’è bisogno di sincronizzazione,
se non per raccogliere i risultati di verify()
● dovendo condividere uno slice di un vettore in diversi thread, quali problemi di
lifetime ci possono essere? Come si può risolvere?