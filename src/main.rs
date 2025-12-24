use std::{
  fs::File,
  io::{BufWriter, Write},
  process::ExitCode,
};

use clap::Parser;
use rand_distr::Normal;

use brc::{
  build_input::get_weather_stations,
  error::{BrcError, BrcResult},
};

use rand::{seq::IteratorRandom, Rng};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(long, default_value_t = 1_000_000_000)]
  records: u64,

  #[arg(long, default_value_t = 10_000)]
  unique_cities: u32,

  #[arg(short, long, default_value = "measurements.txt")]
  output: String,
}

fn run() -> BrcResult {
  let mut rng = rand::rng();
  let args = Args::try_parse()?;
  let weather_stations = get_weather_stations()?;

  let sampled_stations = weather_stations
    .iter()
    .choose_multiple(&mut rng, args.unique_cities as usize);

  let mut output = BufWriter::new(File::create(args.output)?);
  for _ in 0..args.records {
    let city = sampled_stations
      .iter()
      .choose(&mut rng)
      .ok_or_else(|| BrcError::new("Unexpected empty weather_stations".to_owned()))?;

    let average_temp = city.average_temperature();
    let dist = Normal::new(average_temp, 10.)?;
    let measured_temp = rng.sample(dist).clamp(-99.9, 99.9);

    let line = format!("{};{:.1}\n", city.name(), measured_temp);
    output.write_all(line.as_bytes())?;
  }
  output.flush()?;

  Ok(())
}

fn main() -> ExitCode {
  #[cfg(feature = "profiled")]
  let guard = pprof::ProfilerGuardBuilder::default()
    .frequency(1000)
    .blocklist(&["libc", "libgcc", "pthread", "vdso"])
    .build()
    .unwrap();

  let res = run();

  #[cfg(feature = "profiled")]
  if let Ok(report) = guard.report().build() {
    let file = std::fs::File::create("brc.svg").unwrap();
    report.flamegraph(file).unwrap();
  };

  if let Err(err) = res {
    println!("{err}");
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}
