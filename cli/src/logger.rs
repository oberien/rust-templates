use std::env;

use env_logger::LogBuilder;
use time;

use error::*;

pub fn init() -> Result<()> {
    let mut builder = LogBuilder::new();
    builder.format(|record| {
        let now = time::now();
        let time = time::strftime("%Y-%m-%d %H:%M:%S", &now).unwrap();
        format!("[{},{:03}] {}", time, now.tm_nsec / 1000_000, record.args())
    });
    let rust_log = env::var("RUST_LOG");
    if let Ok(var) = rust_log {
       builder.parse(&var);
    }
    builder.init()?;
    Ok(())
}
