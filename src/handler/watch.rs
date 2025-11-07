use notify_rust::Notification;

use crate::config;

#[derive(Debug, clap::Args)]
pub struct WatchArgs {}

pub fn watch(
    args: &WatchArgs,
    directory_config: &mut config::DirectoryConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Test");
        std::thread::sleep_ms(2000);
        Notification::new()
            .summary("This is a test.")
            .body("This is just a test.")
            .appname("todo")
            .show()?;
    }
}
