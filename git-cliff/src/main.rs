use clap::Parser;
use git_cliff::args::Opt;
use git_cliff::logger;
use git_cliff_core::error::Result;
use std::env;
use std::process;

#[cfg(feature = "profiler")]
mod profiler;
#[cfg(feature = "profiler")]
use profiler::{
	finish_profiling,
	start_profiling,
};

fn main() -> Result<()> {
	// Initialize the profiler guard if the feature is enabled
	let mut _profiler_guard = None;
	#[cfg(feature = "profiler")]
	{
		_profiler_guard = Some(start_profiling());
	}
	#[cfg(not(feature = "profiler"))]
	{
		_profiler_guard = Some(());
	}

	// Parse the command line arguments
	let args = Opt::parse();
	if args.verbose == 1 {
		env::set_var("RUST_LOG", "debug");
	} else if args.verbose > 1 {
		env::set_var("RUST_LOG", "trace");
	} else if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
	}
	logger::init()?;

	// Run git-cliff
	let exit_code = match git_cliff::run(args) {
		Ok(_) => 0,
		Err(e) => {
			log::error!("{}", e);
			1
		}
	};

	// Report the profiler if the feature is enabled
	#[cfg(feature = "profiler")]
	{
		finish_profiling(_profiler_guard.unwrap());
	}

	process::exit(exit_code);
}
