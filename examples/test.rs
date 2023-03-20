use time::{
    format_description::{self, FormatItem},
    OffsetDateTime,
};
use tracing::{dispatcher, info, Dispatch, Level, Subscriber, info_span};
use tracing_subscriber::{
    fmt::{format, FormatEvent, FormatFields},
    registry::LookupSpan,
};

fn main() {
    
    use tracing_glog::{Glog, GlogFields};
    let sub = tracing_subscriber::fmt::fmt()
        .event_format(Glog::default().with_span_context(true))
        .fmt_fields(GlogFields::default())
        .finish();
    let dispatch = Dispatch::new(sub);
    let key = 3;
    dispatcher::with_default(&dispatch, || {
        let _span = info_span!("span1", %key, other="ok").entered();
        info!("hello");
    });
    let fmat = GlogFormatter {
        format: format_description::parse(
            "[month][day] [hour]:[minute]:[second].[subsecond digits:6]",
        )
        .unwrap(),
    };
    let sub = tracing_subscriber::fmt::fmt()
        .event_format(format().compact())
        .finish();
    let dispatch = Dispatch::new(sub);
    dispatcher::with_default(&dispatch, || {
        let _span = info_span!("span1", %key, other="ok").entered();
        info!("hello");
    });
    let sub = tracing_subscriber::fmt::fmt().event_format(fmat).finish();
    let dispatch = Dispatch::new(sub);
    dispatcher::with_default(&dispatch, || {
        let _span = info_span!("span1", %key, other="ok").entered();
        info!("hello");
    });
}

struct GlogFormatter<TF = Vec<FormatItem<'static>>> {
    format: TF,
}

impl<S, N> FormatEvent<S, N> for GlogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let level = *event.metadata().level();
        let level = match level {
            Level::TRACE => "T",
            Level::DEBUG => "D",
            Level::INFO => "I",
            Level::WARN => "W",
            Level::ERROR => "E",
        };
        write!(writer, "{}", level)?;
        let now = OffsetDateTime::now_utc();
        let s = now.format(&self.format).unwrap();
        write!(writer, "{s}")?;
        writeln!(writer)?;
        Ok(())
    }
}
