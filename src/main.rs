use std::{
  fs::File,
  io::{BufWriter, Write},
  process::ExitCode,
};

use clap::Parser;

use brc::{
  build_input::{get_weather_stations, output_lines},
  error::BrcResult,
};

const WEATHER_STATIONS_PATH: &str = "data/weather_stations.csv";

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
  let weather_stations = get_weather_stations(WEATHER_STATIONS_PATH)?;

  let mut output = BufWriter::new(File::create(args.output)?);
  for line in output_lines(
    &weather_stations,
    args.records,
    args.unique_cities,
    &mut rng,
  )? {
    output.write_all(line?.as_bytes())?;
  }
  output.flush()?;

  Ok(())
}

fn main() -> ExitCode {
  #[cfg(feature = "profiled")]
  let guard = pprof::ProfilerGuardBuilder::default()
    .frequency(1000)
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
