pub fn init_logger() -> Result<(), fern::InitError> {
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"{}[{}][{}] {}",
				chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S:%f]"),
				record.target(),
				record.level(),
				message
			))
		})
		.level(if cfg!(debug_assertions) { log::LevelFilter::Debug } else { log::LevelFilter::Info })
		.chain(if cfg!(debug_assertions) {
			//std::fs::OpenOptions::new().write(true).create(true).append(false).open("debug.log")?
			fern::log_file("debug.log")?
		}
		else {
			fern::log_file("logging.log")?
		})
		.apply()?;

	Ok(())
}