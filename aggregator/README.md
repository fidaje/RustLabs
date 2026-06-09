# Traccia
Un sistema di monitoraggio all'interno di uno stabilimento industriale raccoglie misure di temperatura da più sensori. Le misure vengono raccolte in modo asincrono, sono automaticamente etichettate con l'istante temporale in cui sono comunicate e possono essere inviate da più thread contemporaneamente.

Compito del sistema è quello di aggregare le misure ricevute, calcolando la temperatura media e il numero di misure ricevute da ciascun sensore, operando un campionamento ad intervalli regolari indicati dal parametro passato alla funzione di costruzione.

In tale periodo, un sensore può inviare più misure, che devono essere tutte considerate nel calcolo della media.

Un thread, creato contestualmente alla creazione della struttura, si occupa di calcolare la media delle temperature per ciascun sensore, aggiornandola secondo il periodo di campionamento indicato.

All'atto della distruzione della struttura, il thread interno deve essere terminato in modo sicuro.

Per implementare tale sistema, si richiede di realizzare la struct Aggregator che offre i seguenti metodi thread-safe:

```rust
use std::time::Instant;

pub struct Aggregator {
// campi privati
}

pub struct Average {
    pub sensor_id: usize,
    pub reference_time: Instant, //indica l'istante temporale in cui è stata calcolata la media
    pub average_temperature: f64,
}
impl Aggregator {
    pub fn new(sample_time_millis: u64) -> Self {
    // implementazione del costruttore
    todo!()
    }

    pub fn add_measure(&self, sensor_id: usize, temperature: f64) {
    // aggiunge una misura di temperatura per il sensore con id `sensor_id`
    // e temperatura `temperature`. Le misure sono automaticamente etichettate
    // con l'istante temporale in cui sono comunicate.
    todo!()
    }

    pub fn get_averages(&self) -> Vec<Average> {
    // restituisce un vettore che riporta la temperatura media di ciascun sensore,
    // calcolata durante l'ultimo periodo di campionamento.
    // Sono presenti solo i sensori che hanno inviato almeno una misura.
    todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn when_no_measures_are_sent_an_empty_state_is_returned() {
        let aggregator = Aggregator::new(10);
        let averages = aggregator.get_averages();
        assert!(averages.is_empty());
    }

    #[test]
    fn when_a_single_measure_is_sent_it_is_returned() {
        let aggregator = Aggregator::new(20);
        std::thread::sleep(std::time::Duration::*from_millis*(1));
        aggregator.add_measure(1, 1.0);
        assert!(aggregator.get_averages().is_empty());
        std::thread::sleep(Duration::*from_millis*(25));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 1);
        assert!(matches!(averages.get(0), Some(&Average{ sensor_id:1, average_temperature:1.0, .. })));
    }

    #[test]
    fn when_two_measures_are_sent_their_average_is_returned() {
        let aggregator = Aggregator::new(100);
        aggregator.add_measure(1, 1.0);
        aggregator.add_measure(1, 2.0);
        std::thread::sleep(Duration::*from_millis*(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 1);
        assert!(matches!(averages.get(0), Some(&Average{ sensor_id:1, average_temperature:1.5, .. })));
    }

    #[test]
    fn when_two_measures_are_sent_from_different_sensors_their_average_is_returned() {
        let aggregator = Aggregator::new(100);
        aggregator.add_measure(1, 1.0);
        aggregator.add_measure(2, 2.0);
        aggregator.add_measure(2, 1.0);
        aggregator.add_measure(1, 2.0);
        std::thread::sleep(Duration::*from_millis*(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 2);
        let timestamp = averages.get(0).unwrap().reference_time;
        assert!(averages.contains(&Average{ sensor_id:1, average_temperature:1.5, reference_time: timestamp }));
        assert!(averages.contains(&Average{ sensor_id:2, average_temperature:1.5, reference_time: timestamp }));
    }

    #[test]
    fn more_threads_may_send_data() {
        let aggregator = Aggregator::new(100);
        std::thread::scope(|s| {
            s.spawn(|| {
                aggregator.add_measure(1, 1.0);
                std::thread::sleep(Duration::*from_millis*(5));
                aggregator.add_measure(1, 3.0);
            });
            s.spawn(|| {
                aggregator.add_measure(2, 2.0);
                std::thread::sleep(Duration::*from_millis*(5));
                aggregator.add_measure(2, 8.0);
            });
        });
        std::thread::sleep(Duration::*from_millis*(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 2);
        let timestamp = averages.get(0).unwrap().reference_time;
        assert!(averages.contains(&Average{ sensor_id:1, average_temperature:2.0, reference_time: timestamp }));
        assert!(averages.contains(&Average{ sensor_id:2, average_temperature:5.0, reference_time: timestamp }));
    }

    #[test]
    fn an_aggregator_shuts_down_cleanly() {
        {
        let _aggregator = Aggregator::new(10);
        }
        assert!(true);
    }
}
```