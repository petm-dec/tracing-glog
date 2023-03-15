#[cfg(any(target_os = "windows", target_os = "linux"))]
fn main() {
    use tracing_glog::{Glog, GlogFields};
    tracing_subscriber::fmt::fmt()
        .event_format(Glog::default().with_span_context(true))
        .fmt_fields(GlogFields::default())
        .init();
    fn log() {
        #[cfg(target_os = "windows")]
        let tid = unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() };
        #[cfg(target_os = "linux")]
        let tid = unsafe { libc::gettid() };
        tracing::info!(%tid);
    }

    let thr = std::thread::spawn(|| {
        log();
    });
    log();
    thr.join().unwrap();
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn main() {
    println!("tid example not available for this target os");
}
