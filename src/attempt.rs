
use std::time::{Duration, Instant};
use rocket::tokio::time::sleep;

const AWAIT_DURATION_MS: u64 = 50;

pub enum Method {
    Timeout(Duration),
    Deadline(Instant),
    Retry(u64)
}

pub async fn attempt<T, E, F>(method: Method, func: fn() -> F) -> Result<T, E>
where F: std::future::Future<Output = Result<T, E>> {
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

