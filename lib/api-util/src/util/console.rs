use ::tracing::info;

const SERVICE_STARTED_MSG: &str = "service started";
const WAITING_FOR_SHUTDOWN_MSG: &str = "waiting for shutdown signal...";
const SERVICE_STOPPED_MSG: &str = "service stopped";

pub fn print_banner() {
    // println!(include_str!("../../../../res/logo/banner.txt"));    
}

pub fn print_service_started(package: &str, version: &str) {
    info!("{package} v{version}");
    info!("{SERVICE_STARTED_MSG}");
    info!("{WAITING_FOR_SHUTDOWN_MSG}");
}

pub fn print_service_stopped() {
    info!("{SERVICE_STOPPED_MSG}");
}