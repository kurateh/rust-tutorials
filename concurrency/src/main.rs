mod concurrency;
mod deadlock;
mod mutex;
fn main() {
    concurrency::concurrency_tutorial();
    mutex::mutex_tutorial();
    // deadlock::deadlock_tutorial();
}
