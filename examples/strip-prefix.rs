use tracing::{dispatcher, Dispatch};
use tracing_glog::{Glog, GlogFields};

mod first_module;

fn main() {
    for (with_trimmed_directory, with_strip_prefix) in vec![
        (false, None),
        (true, None),
        (false, Some("exampl")),
        (false, Some("examples")),
        (false, Some(std::path::Path::new(file!()).iter().next().unwrap().to_str().unwrap())),
        (false, Some("examples/first_module")),
    ] {
        let sub = tracing_subscriber::fmt::fmt()
            .event_format(
                Glog::default()
                    .with_span_context(true)
                    .with_target(true)
                    .with_trimmed_directory(with_trimmed_directory)
                    .with_strip_prefix(with_strip_prefix),
            )
            .fmt_fields(GlogFields::default())
            .finish();
        let dispatch = Dispatch::new(sub);
        dispatcher::with_default(&dispatch, || {
            tracing::info!(
                "\n\nwith_trimmed_directory={}, with_strip_prefix={:?}",
                with_trimmed_directory, with_strip_prefix
            );
            first_module::hello();
            first_module::second_module::hello();
        });
    }
}
