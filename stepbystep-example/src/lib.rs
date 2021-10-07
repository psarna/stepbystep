use async_std;
use std::time::Duration;
use stepbystep_export::export_async;

#[export_async]
pub async fn test_me() {
    for i in 1..10 {
        println!("i={}", i);
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
