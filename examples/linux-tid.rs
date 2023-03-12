use tracing_glog::{Glog, GlogFields};

fn main() {
    tracing_subscriber::fmt::fmt()
        .event_format(Glog::default().with_span_context(true))
        .fmt_fields(GlogFields::default())
        .init();
    fn log() {
        let tid = unsafe { libc::gettid() };
        tracing::info!(%tid);
    }
    let thr = std::thread::spawn(|| {
        log();
    });
    log();
    thr.join().unwrap();
}
