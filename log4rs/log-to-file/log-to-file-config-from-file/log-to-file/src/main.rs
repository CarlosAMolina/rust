use log;
use log4rs;

fn main() {
    let _handle = log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    log::error!("Goes to stderr and file");
    log::warn!("Goes to stderr and file");
    log::info!("Goes to stderr and file");
    log::debug!("Goes to file only");
    log::trace!("Goes to file only");
}

