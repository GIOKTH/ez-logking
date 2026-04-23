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