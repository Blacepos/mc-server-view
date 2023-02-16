/// Does not work the same as tokio Timeout. This one retries a fast-returning operation multiple
/// times whereas Timeout cancels a slow-returning operation once.
use std::time::{Duration, Instant};
use rocket::tokio::time::sleep;

const AWAIT_DURATION_MS: u64 = 50;

#[allow(dead_code)]
pub enum Method {
    Timeout(Duration),
    Deadline(Instant),
    Retry(u64)
}

/// Call a function multiple times until a failure condition is met.
/// 
/// The function must return a future that yields a `Result` type.
/// 
/// The return type of `attempt` is the same as the return type of the `func` argument.
/// 
/// Three conditions are currently supported:
/// 1. `Timeout`: Attempt a function for a duration of time.
/// 2. `Deadline`: Attempt a function until an instant in time is reached.
/// 3. `Retry`: Attempt a function a certain number of times.
/// # Example
/// ```rs
/// // Try calling `may_fail_a_few_times` with a timeout of 10 seconds
/// attempt(
///     Method::Timeout(Duration::from_secs(10)),
///     may_fail_a_few_times
/// ).await
/// ```
/// 
/// # Not Exactly What You Want
/// This function does not cancel a future. If you set a timeout of 10 seconds, `attempt` could
/// still take longer if the future returned by `func` doesn't finish before then.
/// 
/// This function is intended for operations that either succeed or fail with little delay.
pub async fn attempt<T, E, A, F>(method: Method, func: F) -> Result<T, E>
where F: Fn() -> A,
      A: std::future::Future<Output = Result<T, E>> {
    match method {
        Method::Timeout(duration) => {
            let begin = Instant::now();

            loop {
                match func().await {
                    Ok(t) => return Ok(t),
                    Err(e) => {
                        if Instant::now() - begin > duration {
                            return Err(e);
                        }
                    },
                }

                sleep(Duration::from_millis(AWAIT_DURATION_MS)).await;
            }

        },
        Method::Deadline(deadline) => {

            loop {
                match func().await {
                    Ok(t) => return Ok(t),
                    Err(e) => {
                        if Instant::now() > deadline {
                            return Err(e);
                        }
                    },
                }

                sleep(Duration::from_millis(AWAIT_DURATION_MS)).await;
            }
        },
        Method::Retry(retries) => {
            let mut tries = 0;

            loop {
                match func().await {
                    Ok(t) => return Ok(t),
                    Err(e) => {
                        if tries > retries {
                            return Err(e);
                        }
                    },
                }

                tries += 1;
                sleep(Duration::from_millis(AWAIT_DURATION_MS)).await;
            }
        },
    }
}

