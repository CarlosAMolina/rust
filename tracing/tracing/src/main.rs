use tracing::{event, instrument, span, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
}

fn main() {
    // Log all events to a rolling log file.
    //let logfile = tracing_appender::rolling::hourly("/tmp", "myapp-logs");
    let logfile = RollingFileAppender::new(Rotation::HOURLY, "/tmp", "myapp-logs.log");
    // In order to record trace events, executables have to use a Subscriber implementation compatible with tracing. A Subscriber implements a way of collecting trace data, such as by logging it to standard output.
    // Any trace events generated outside the context of a subscriber will not be collected.
    // Install global collector configured based on RUST_LOG env var.
    // Using init() calls set_global_default() so this collector will be used as the default in all threads for the remainder of the duration of the program, similar to how loggers work in the log crate.
    // Blocking
    // Log specified level and above to stdout.
    //let stdout = std::io::stdout.with_max_level(tracing::Level::TRACE);
    //tracing_subscriber::fmt()
    //// Combine the stdout and log file `MakeWriter`s into one
    //// `MakeWriter` that writes to both
    //.with_writer(stdout.and(logfile))
    //.init();
    // Non Blocking
    // WorkerGuard should be assigned in the main function or whatever the entrypoint of the program is. This will ensure that the guard will be dropped during an unwinding or when main exits successfully.
    let (non_blocking_logfile, _guard_logfile) = tracing_appender::non_blocking(logfile);
    let (non_blocking_stdout, _guard_stdout) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
    .with_writer(non_blocking_logfile.and(non_blocking_stdout))
    .init();

    // this creates a new event, outside of any spans.
    tracing::info!(
        answer = 42,
        summary = "this creates a new event, outside of any spans"
    );

    // Construct a new span named "my span" with trace log level.
    // Equals as `let span = info_span!("my_span");`.
    let span = span!(Level::WARN, "my_span"); // TODO check whty Level is not applied, maybe
                                              // tracing_appender causes interference.
    // Enter the span, returning a guard object.
    let _guard = span.enter();
    // Any trace events that occur before the guard is dropped will occur
    // within "my_span".
    // TODO try to output TRACE nad DEBUG event, i can't
    event!(Level::TRACE, "text trace"); // Not showed as span has INFO level.
    event!(Level::DEBUG, "text debug"); // Not showed as span has INFO level.
    event!(Level::INFO, "text info");
    event!(Level::WARN, "text warning");
    event!(Level::ERROR, "text error");
    // Dropping the guard will exit the span.

    my_function(3);

    // Log structs.
    let user = User {
        name: "ferris".to_string(),
        email: "ferris@rust-lang.org".to_string(),
    };
    // ?: specifies something should be recorded using its fmt::Debug implementation.
    event!(Level::INFO, text = ?user);
    // %: specifies something should be recorded using its fmt::Display implementation:
    event!(Level::INFO, text = %user.email);
}

// #[instrument]: adds tracing spans to functions
// Creates and enter a span with the function’s name every time the function is called.
// Arguments to that function will be recorded as span fields using fmt::Debug.
#[instrument]
pub fn my_function(my_arg: usize) {
    // This event will be recorded inside a span named `my_function` with the
    // field `my_arg`.
    event!(Level::INFO, "inside my_function!");
}
