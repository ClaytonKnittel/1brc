use std::{
  fs::File,
  io::{BufRead, BufReader, BufWriter, Write},
  process::ExitCode,
};

use clap::Parser;
use rand_distr::Normal;

use crate::error::{BrcError, BrcResult};

use rand::{seq::IteratorRandom, Rng};

mod error;

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

#[derive(Debug)]
struct City {
  name: String,
  average_temperature: f32,
}

fn get_weather_stations() -> BrcResult<Vec<City>> {
  BufReader::new(File::open(WEATHER_STATIONS_PATH)?)
    .lines()
    .filter(|line| !line.as_ref().is_ok_and(|line| line.starts_with('#')))
    .map(|line| {
      let line = line?;
      let (name, average_temperature) = line
        .split_once(';')
        .ok_or_else(|| BrcError::new(format!("No ';' found in line \"{line}\"")))?;
      Ok(City {
        name: name.to_owned(),
        average_temperature: average_temperature.parse()?,
      })
    })
    .collect()
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

    let average_temp = city.average_temperature;
    let dist = Normal::new(average_temp, 10.)?;
    let measured_temp = rng.sample(dist).clamp(-99.9, 99.9);

    let line = format!("{};{:.1}\n", city.name, measured_temp);
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
