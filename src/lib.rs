use std::path::Path;
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, prelude::*, registry::LookupSpan};

pub struct DevFormatter {
    pub app_name: String,
}

// ຕົວຊ່ວຍໃນການດຶງ Trace ID ຈາກ Fields ຂອງ Span
struct TraceIdVisitor(String);
impl tracing::field::Visit for TraceIdVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "trace_id" {
            self.0 = value.to_string();
        }
    }
    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {}
}

impl<S> tracing_subscriber::Layer<S> for DevFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<S>) {
        let metadata = event.metadata();
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let file_name = Path::new(metadata.file().unwrap_or("unknown"))
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        // --- ດຶງ Trace ID ຈາກ Span Field ---
        let mut trace_id = "N/A".to_string();
        if let Some(scope) = ctx.lookup_current() {
            for span in scope.scope() {
                let extensions = span.extensions();
                if let Some(fields) = extensions.get::<tracing_subscriber::fmt::FormattedFields<S>>() {
                    // ດຶງຂໍ້ມູນຈາກ fields ທີ່ຖືກ format ໄວ້ແລ້ວ
                    if !fields.is_empty() {
                        // ໃນບົດນີ້ເຮົາຈະໃຊ້ visitor ຜ່ານ span metadata ຖ້າຕ້ອງການຄວາມຊັດເຈນ
                    }
                }
                // ວິທີທີ່ແນ່ນອນທີ່ສຸດ: ດຶງຈາກ field ໂດຍກົງ
                let mut visitor = TraceIdVisitor("N/A".into());
                event.record(&mut visitor);
                if visitor.0 != "N/A" { trace_id = visitor.0; }
            }
        }

        // ສ້າງ Header ສວຍໆແບບ Log4j
        print!(
            "[{}] {:<5} [{}] [TID:{}] [{}:{}] - ",
            timestamp, metadata.level(), self.app_name, trace_id, file_name, metadata.line().unwrap_or(0)
        );

        // ດຶງ Message ຂອງ Log
        struct MsgVisitor;
        impl tracing::field::Visit for MsgVisitor {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                if field.name() == "message" { print!("{:?}", value); }
            }
        }
        let mut msg_visitor = MsgVisitor;
        event.record(&mut msg_visitor);
        println!();
    }
}

pub fn init_logger(app_name: &str) -> tracing_appender::non_blocking::WorkerGuard {
    let log_dir = "./logs";
    let _ = std::fs::create_dir_all(log_dir);

    // ວິທີການຈັດການຊື່ໄຟລ໌ໃຫ້ໄດ້ແບບ: app-name-YYYY-MM-DD.log
    // ພວກເຮົາຈະໃຊ້ RollingFileAppender::builder() ຖ້າ version ຮອງຮັບ
    // ຫຼື ວິທີທີ່ແນ່ນອນທີ່ສຸດແມ່ນການໃຊ້ prefix ທີ່ລົງທ້າຍດ້ວຍ "-"
    // ແລ້ວໃຫ້ system ຈັດການ rotation ເອງ

    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix(app_name)
        .filename_suffix("log") // ກຳນົດ suffix ເປັນ log
        .build(log_dir)
        .expect("Failed to create rolling file appender");

    let (non_blocking_writer, guard) = tracing_appender::non_blocking::NonBlockingBuilder::default()
        .lossy(false)
        .buffered_lines_limit(10_000)
        .finish(file_appender);

    // --- ສ່ວນທີ່ເຫຼືອຄືເກົ່າ ---
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(tracing::Level::INFO.into());

    tracing_subscriber::registry()
        .with(filter)
        .with(DevFormatter { app_name: app_name.to_string() })
        .with(tracing_subscriber::fmt::layer()
            .json()
            .with_writer(non_blocking_writer)
            .with_current_span(true))
        .init();

    guard
}