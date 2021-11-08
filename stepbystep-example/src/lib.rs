use stepbystep::export_async;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::task::Poll;
use std::task::Context;
use std::future::Future;
use std::pin::Pin;

thread_local! {
    static PAYLOAD_MAP: std::cell::RefCell<std::collections::HashMap<usize, String>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

#[no_mangle]
pub fn put_payload(idx: usize, ptr: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    let s = cstr.to_str().unwrap().to_owned();
    PAYLOAD_MAP.with(|payload_map| {
        payload_map.borrow_mut().insert(idx, s);
    })
}

struct Yield {
    ready: bool,
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready {
            Poll::Ready(())
        } else {
            self.ready = true;
            Poll::Pending
        }
    }
}

pub async fn compute_complicated_stuff(i: usize) {
    println!("Computation performed: {}*{}={}", i, i, i*i);
    Yield{ready: false}.await
}

#[export_async]
pub async fn test_me(instance_idx: usize) {
    for i in 1..10 {
        println!("Instance {}", instance_idx);
        PAYLOAD_MAP.with(|payload_map| {
            println!("i = {}; value = {}", i, match payload_map.borrow().get(&i) {
                Some(s) => s,
                None => "<none>",
            });
        });
        compute_complicated_stuff(i).await;
    }
}
