use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct Aggregator {
    // buffer di misure grezze accumulate nel periodo corrente
    measures: Arc<Mutex<HashMap<usize, Vec<f64>>>>,
    // ultime medie calcolate dal thread interno (tutte con lo stesso reference_time)
    last_averages: Arc<Mutex<Vec<Average>>>,
    // flag di shutdown per il thread interno
    shutdown: Arc<AtomicBool>,
    // handle del thread interno
    handle: Option<JoinHandle<()>>,
}

#[derive(PartialEq, Clone)]
pub struct Average {
    pub sensor_id: usize,
    pub reference_time: Instant, //indica l'istante temporale in cui è stata calcolata la media
    pub average_temperature: f64,
}

impl Average {
    pub fn new(sensor_id: usize, reference_time: Instant, average_temperature: f64) -> Self {
        Average {
            sensor_id,
            reference_time,
            average_temperature,
        }
    }
}

impl Aggregator {
    pub fn new(sample_time_millis: u64) -> Self {
        let measures: Arc<Mutex<HashMap<usize, Vec<f64>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let last_averages: Arc<Mutex<Vec<Average>>> = Arc::new(Mutex::new(vec![]));
        let shutdown = Arc::new(AtomicBool::new(false));

        // cloni da passare al thread
        let measures_clone = Arc::clone(&measures);
        let last_averages_clone = Arc::clone(&last_averages);
        let shutdown_clone = Arc::clone(&shutdown);
        let period = Duration::from_millis(sample_time_millis);

        let handle = thread::spawn(move || loop {
            thread::sleep(period);

            if shutdown_clone.load(Ordering::Relaxed) {
                break;
            }

            // svuota il buffer di misure grezze
            let mut m = measures_clone.lock().unwrap();
            let snapshot: HashMap<usize, Vec<f64>> = m.drain().collect();
            drop(m);

            if snapshot.is_empty() {
                continue;
            }

            // calcola tutte le medie nello stesso istante → stesso reference_time
            let now = Instant::now();
            let new_averages: Vec<Average> = snapshot
                .into_iter()
                .map(|(sensor_id, temps)| {
                    let avg = temps.iter().sum::<f64>() / temps.len() as f64;
                    Average::new(sensor_id, now, avg)
                })
                .collect();

            *last_averages_clone.lock().unwrap() = new_averages;
        });

        Aggregator {
            measures,
            last_averages,
            shutdown,
            handle: Some(handle),
        }
    }

    pub fn add_measure(&self, sensor_id: usize, temperature: f64) {
        // aggiunge una misura grezza al buffer del periodo corrente
        let mut lock = self.measures.lock().unwrap();
        lock.entry(sensor_id).or_insert_with(Vec::new).push(temperature);
    }

    pub fn get_averages(&self) -> Vec<Average> {
        // restituisce le ultime medie calcolate dal thread interno
        self.last_averages.lock().unwrap().clone()
    }
}

impl Drop for Aggregator {
    fn drop(&mut self) {
        // segnala al thread di terminare e attende il join
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn when_no_measures_are_sent_an_empty_state_is_returned() {
        let aggregator = Aggregator::new(10);
        let averages = aggregator.get_averages();
        assert!(averages.is_empty());
    }

    #[test]
    fn when_a_single_measure_is_sent_it_is_returned() {
        let aggregator = Aggregator::new(20);
        std::thread::sleep(std::time::Duration::from_millis(1));
        aggregator.add_measure(1, 1.0);
        assert!(aggregator.get_averages().is_empty());
        std::thread::sleep(Duration::from_millis(25));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 1);
        assert!(matches!(
            averages.get(0),
            Some(&Average {
                sensor_id: 1,
                average_temperature: 1.0,
                ..
            })
        ));
    }
    #[test]
    fn when_two_measures_are_sent_their_average_is_returned() {
        let aggregator = Aggregator::new(100);
        aggregator.add_measure(1, 1.0);
        aggregator.add_measure(1, 2.0);
        std::thread::sleep(Duration::from_millis(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 1);
        assert!(matches!(
            averages.get(0),
            Some(&Average {
                sensor_id: 1,
                average_temperature: 1.5,
                ..
            })
        ));
    }
    #[test]
    fn when_two_measures_are_sent_from_different_sensors_their_average_is_returned() {
        let aggregator = Aggregator::new(100);
        aggregator.add_measure(1, 1.0);
        aggregator.add_measure(2, 2.0);
        aggregator.add_measure(2, 1.0);
        aggregator.add_measure(1, 2.0);
        std::thread::sleep(Duration::from_millis(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 2);
        let timestamp = averages.get(0).unwrap().reference_time;
        assert!(averages.contains(&Average {
            sensor_id: 1,
            average_temperature: 1.5,
            reference_time: timestamp
        }));
        assert!(averages.contains(&Average {
            sensor_id: 2,
            average_temperature: 1.5,
            reference_time: timestamp
        }));
    }

    #[test]
    fn more_threads_may_send_data() {
        let aggregator = Aggregator::new(100);
        std::thread::scope(|s| {
            s.spawn(|| {
                aggregator.add_measure(1, 1.0);
                std::thread::sleep(Duration::from_millis(5));
                aggregator.add_measure(1, 3.0);
            });
            s.spawn(|| {
                aggregator.add_measure(2, 2.0);
                std::thread::sleep(Duration::from_millis(5));
                aggregator.add_measure(2, 8.0);
            });
        });
        std::thread::sleep(Duration::from_millis(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(), 2);
        let timestamp = averages.get(0).unwrap().reference_time;
        assert!(averages.contains(&Average {
            sensor_id: 1,
            average_temperature: 2.0,
            reference_time: timestamp
        }));
        assert!(averages.contains(&Average {
            sensor_id: 2,
            average_temperature: 5.0,
            reference_time: timestamp
        }));
    }
    #[test]
    fn an_aggregator_shuts_down_cleanly() {
        {
            let _aggregator = Aggregator::new(10);
        }
        assert!(true);
    }
}
