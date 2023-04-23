#[allow(dead_code)]
fn factorial(n: i128) -> i128 {
    if n > 1 {
        n * factorial(n - 1)
    } else {
        n
    }
}

#[allow(dead_code)]
#[async_recursion::async_recursion]
async fn async_factorial(n: i128) -> i128 {
    if n > 1 {
        n * async_factorial(n - 1).await
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use std::thread::available_parallelism;

    use futures::future::join_all;
    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

    use tokio::runtime;

    use super::*;

    const UPPER_LIMIT: i128 = 33;
    const RUNS: usize = 2_000_000_0;

    #[ignore]
    #[test]
    fn par_iterr() {
        let max_threads = available_parallelism().unwrap().get();
        let sets = (0..max_threads).collect::<Vec<_>>();

        sets.par_iter().for_each(|_| {
            (0..RUNS).for_each(|_| {
                let _ = factorial(UPPER_LIMIT);
            });
        });
    }

    #[ignore]
    #[test]
    fn with_threads() {
        let max_threads = available_parallelism().unwrap().get();
        let handles = (0..max_threads)
            .map(|_| {
                std::thread::Builder::new()
                    .spawn(|| {
                        (0..RUNS).for_each(|_| {
                            let _ = factorial(UPPER_LIMIT);
                        })
                    })
                    .unwrap()
            })
            .collect::<Vec<_>>();

        for h in handles {
            let _ = h.join().unwrap();
        }
    }

    #[tokio::test]
    async fn async_with_threads() {
        let max_threads = available_parallelism().unwrap().get();

        let handles = (0..max_threads)
            .map(|_| {
                std::thread::Builder::new()
                    .spawn(|| {
                        runtime::Builder::new_current_thread()
                            .build()
                            .unwrap()
                            .block_on(async move {
                                let handles = (0..RUNS).map(|_| {
                                    tokio::task::spawn_blocking(move || async {
                                        let _ = async_factorial(UPPER_LIMIT).await;
                                    })
                                });

                                let _ = join_all(handles).await;
                            })
                    })
                    .unwrap()
            })
            .collect::<Vec<_>>();

        for h in handles {
            let _ = h.join().unwrap();
        }
    }
}
