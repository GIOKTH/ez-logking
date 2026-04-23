# ez_logking
## Brief
this will help you setup logging file easier and also format the log output

## What can this library do?
this crate can help you create log file
add log trace_id into log lines
rotate log files


### Installation
add this into cargo.toml
ez_logking = "0.1.0"

## Example Usage

```rust
use tracing::{info, info_span};
use ez_logking::init_logger;
use std::time::Duration;
use std::thread;

fn main() {
    // 1. Initialize Logger ແລະ ເກັບ Guard ໄວ້
    let _log_guard = init_logger("PAYMENT-GATEWAY");

    let request_id = "REQ-888999";
    let span = info_span!("handle_payment", trace_id = %request_id);

    // ໃຊ້ Scope ຂອງ Span ໃຫ້ຊັດເຈນ
    {
        let _enter = span.enter();
        info!("Starting transaction...");
        do_work();
        info!("Transaction finished successfully.");
    } // Span ຈົບຢູ່ບ່ອນນີ້

    // 2. ສໍາຄັນ: ຖ້າເປັນ App ທີ່ເຮັດວຽກແລ້ວຈົບເລີຍ ຕ້ອງໃຫ້ເວລາມັນ Flush Log ໜ້ອຍໜຶ່ງ
    // ໃນ Production ທີ່ເປັນ Server (Actix-web) ບໍ່ຈຳເປັນຕ້ອງມີແຖວນີ້
    thread::sleep(Duration::from_millis(500));
}

fn do_work() {
    info!("Validating balance...");
}
 ## output be like
 {"timestamp":"2026-04-23T06:55:44.548365Z","level":"INFO","fields":{"message":"Starting transaction..."},"target":"usage","span":{"trace_id":"REQ-888999","name":"handle_payment"},"spans":[{"trace_id":"REQ-888999","name":"handle_payment"}]}
 {"timestamp":"2026-04-23T06:55:44.548492Z","level":"INFO","fields":{"message":"Validating balance..."},"target":"usage","span":{"trace_id":"REQ-888999","name":"handle_payment"},"spans":[{"trace_id":"REQ-888999","name":"handle_payment"}]}
 {"timestamp":"2026-04-23T06:55:44.548533Z","level":"INFO","fields":{"message":"Transaction finished successfully."},"target":"usage","span":{"trace_id":"REQ-888999","name":"handle_payment"},"spans":[{"trace_id":"REQ-888999","name":"handle_payment"}]}
 {"timestamp":"2026-04-23T06:56:02.182523Z","level":"INFO","fields":{"message":"Starting transaction..."},"target":"usage","span":{"trace_id":"REQ-888999","name":"handle_payment"},"spans":[{"trace_id":"REQ-888999","name":"handle_payment"}]}
 {"timestamp":"2026-04-23T06:56:02.182618Z","level":"INFO","fields":{"message":"Validating balance..."},"target":"usage","span":{"trace_id":"REQ-888999","name":"handle_payment"},"spans":[{"trace_id":"REQ-888999","name":"handle_payment"}]}
 {"timestamp":"2026-04-23T06:56:02.182653Z","level":"INFO","fields":{"message":"Transaction finished successfully."},"target":"usage","span":{"trace_id":"REQ-888999","name":"handle_payment"},"spans":[{"trace_id":"REQ-888999","name":"handle_payment"}]}

