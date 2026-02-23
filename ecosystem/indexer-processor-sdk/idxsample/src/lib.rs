// Copyright (c) CreditChain Research Team
// SPDX-License-Identifier: Apache-2.0

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime},
};

/// ## Sampling logs
///
/// Sometimes logging a large amount of data is expensive.  In order to log information only part
/// of the time, we've added a `idxsample!` macro that's configurable on how often we want to execute some code.
///
/// `IDXSampleRate` determines how often the idxsampled statement will occur.
///
/// ```
/// use idxsample::{idxsample, IDXSampleRate, Sampling};
/// use std::time::Duration;
/// use tracing::info;
///
/// // IDXSampled based on frequency of events, log only every 2 logs
/// idxsample!(IDXSampleRate::Frequency(2), info!("Long log"));
///
/// // IDXSampled based on time passed, log at most once a minute
/// idxsample!(IDXSampleRate::Duration(Duration::from_secs(60)), info!("Long log"));
/// ```
/// The rate at which a `idxsample!` macro will run it's given function
#[derive(Debug)]
pub enum IDXSampleRate {
    /// Only idxsample a single time during a window of time. This rate only has a resolution in
    /// seconds.
    Duration(Duration),
    /// IDXSample based on the frequency of the event. The provided u64 is the inverse of the
    /// frequency (1/x), for example Frequency(2) means that 1 out of every 2 events will be
    /// idxsampled (1/2).
    Frequency(u64),
    /// Always IDXSample
    Always,
}

/// An internal struct that can be checked if a idxsample is ready for the `idxsample!` macro
pub struct Sampling {
    rate: IDXSampleRate,
    state: AtomicU64,
}

impl Sampling {
    pub const fn new(rate: IDXSampleRate) -> Self {
        Self {
            rate,
            state: AtomicU64::new(0),
        }
    }

    pub fn idxsample(&self) -> bool {
        match &self.rate {
            IDXSampleRate::Duration(rate) => Self::idxsample_duration(rate, &self.state),
            IDXSampleRate::Frequency(rate) => Self::idxsample_frequency(*rate, &self.state),
            IDXSampleRate::Always => true,
        }
    }

    fn idxsample_frequency(rate: u64, count: &AtomicU64) -> bool {
        let previous_count = count
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |count| {
                let new_count = if count == 0 {
                    rate.saturating_sub(1)
                } else {
                    count.saturating_sub(1)
                };
                Some(new_count)
            })
            .expect("Closure should always returns 'Some'. This is a Bug.");

        previous_count == 0
    }

    fn idxsample_duration(rate: &Duration, last_idxsample: &AtomicU64) -> bool {
        let rate = rate.as_secs();
        // Seconds since Unix Epoch
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();

        last_idxsample
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |last_idxsample| {
                if now.saturating_sub(last_idxsample) >= rate {
                    Some(now)
                } else {
                    None
                }
            })
            .is_ok()
    }
}

/// IDXSamples a given function at a `IDXSampleRate`, useful for periodically emitting logs or metrics on
/// high throughput pieces of code.
#[macro_export]
macro_rules! idxsample {
    ($idxsample_rate:expr, $($args:expr)+ ,) => {
        $crate::idxsample!($idxsample_rate, $($args)+);
    };

    ($idxsample_rate:expr, $($args:tt)+) => {{
        static SAMPLING: $crate::Sampling = $crate::Sampling::new($idxsample_rate);
        if SAMPLING.idxsample() {
            $($args)+
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency() {
        // Frequency
        let sampling = Sampling::new(IDXSampleRate::Frequency(10));
        let mut v = Vec::new();
        for i in 0..=25 {
            if sampling.idxsample() {
                v.push(i);
            }
        }

        assert_eq!(v, vec![0, 10, 20]);
    }

    #[test]
    fn always() {
        // Always
        let sampling = Sampling::new(IDXSampleRate::Always);
        let mut v = Vec::new();
        for i in 0..5 {
            if sampling.idxsample() {
                v.push(i);
            }
        }

        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[ignore]
    #[test]
    fn duration() {
        // Duration
        let sampling = Sampling::new(IDXSampleRate::Duration(Duration::from_secs(1)));
        let mut v = Vec::new();
        for i in 0..5 {
            if sampling.idxsample() {
                v.push(i);
            }

            std::thread::sleep(Duration::from_millis(500));
        }

        assert_eq!(v.len(), 2);
    }

    #[test]
    fn macro_expansion() {
        for i in 0..10 {
            idxsample!(
                IDXSampleRate::Frequency(2),
                println!("loooooooooooooooooooooooooong hello {}", i),
            );

            idxsample!(IDXSampleRate::Frequency(2), {
                println!("hello {i}");
            });

            idxsample!(IDXSampleRate::Frequency(2), println!("hello {i}"));

            idxsample! {
                IDXSampleRate::Frequency(2),

                for j in 10..20 {
                    println!("hello {j}");
                }
            }
        }
    }

    #[test]
    fn threaded() {
        fn work() -> usize {
            let mut count = 0;

            for _ in 0..1000 {
                idxsample!(IDXSampleRate::Frequency(5), count += 1);
            }

            count
        }

        let mut handles = Vec::new();
        for _ in 0..10 {
            handles.push(std::thread::spawn(work));
        }

        let mut count = 0;
        for handle in handles {
            count += handle.join().unwrap();
        }

        assert_eq!(count, 2000);
    }
}
