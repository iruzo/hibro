use hibro::data;

use core::time::Duration;
use std::thread;
use std::path::Path;

#[test]
fn test_sync() {
    data::config::sync_plugins();
    thread::sleep(Duration::from_millis(5000));
    assert!(Path::new(data::path::sync_file().as_str()).exists());
}
