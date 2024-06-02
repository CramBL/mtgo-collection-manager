use crate::Message;

/// Initializes the Ctrl+C handler to facilitate graceful shutdown on Ctrl+C
///
/// Also handles SIGTERM and SIGHUP if the `termination` feature is enabled
pub fn init_ctrlc_handler(ev_sender: fltk::app::Sender<Message>) {
    // Handles SIGINT, SIGTERM and SIGHUP (as the `termination` feature is  enabled)
    ctrlc::set_handler({
        let mut stop_sig_count = 0;
        move || {
            log::warn!(
                "Stop Ctrl+C, SIGTERM, or SIGHUP received, stopping gracefully, please wait..."
            );
            ev_sender.send(Message::Quit);
            stop_sig_count += 1;
            if stop_sig_count > 1 {
                log::warn!("Second stop signal received, ungraceful shutdown.");
                std::process::exit(1);
            }
        }
    })
    .expect("Error setting Ctrl-C handler");
}
