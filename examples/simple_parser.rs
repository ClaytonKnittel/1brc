use std::{
  cmp::Ordering,
  collections::HashMap,
  fmt::Display,
  fs::File,
  io::{BufRead, BufReader},
  process::ExitCode,
};

use brc::error::{BrcError, BrcResult};
use clap::Parser;
use itertools::Itertools;

struct TemperatureSummary {
  min: i32,
  max: i32,
  total: i64,
  count: u32,
}

impl TemperatureSummary {
  fn min(&self) -> f32 {
    self.min as f32 / 10.0
  }

  fn max(&self) -> f32 {
    self.max as f32 / 10.0
  }

  fn avg(&self) -> f32 {
    let rounded_total = self.total + (self.count / 2) as i64;
    rounded_total.div_euclid(self.count as i64) as f32 / 10.0
  }

  fn add_reading(&mut self, temp: f32) {
    let temp = (temp * 10.0).round() as i32;
    self.min = self.min.min(temp);
    self.max = self.max.max(temp);
    self.total += temp as i64;
    self.count += 1;
  }
}

impl Default for TemperatureSummary {
  fn default() -> Self {
    Self {
      min: i32::MAX,
      max: i32::MIN,
      total: 0,
      count: 0,
    }
  }
}

pub struct WeatherStation {
  name: String,
  summary: TemperatureSummary,
}

impl PartialEq for WeatherStation {
  fn eq(&self, other: &Self) -> bool {
    self.name.eq(&other.name)
  }
}

impl Eq for WeatherStation {}

impl PartialOrd for WeatherStation {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for WeatherStation {
  fn cmp(&self, other: &Self) -> Ordering {
    self.name.cmp(&other.name)
  }
}

impl Display for WeatherStation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}={:.1}/{:.1}/{:.1}",
      self.name,
      self.summary.min(),
      self.summary.avg(),
      self.summary.max()
    )
  }
}

pub fn temperature_reading_summaries(
  input_path: &str,
) -> BrcResult<impl Iterator<Item = WeatherStation>> {
  Ok(
    BufReader::new(
      File::open(input_path)
        .map_err(|err| BrcError::new(format!("Failed to open {input_path}: {err}")))?,
    )
    .lines()
    .try_fold(
      HashMap::<String, TemperatureSummary>::new(),
      |mut map, line| -> BrcResult<_> {
        let line = line?;
        let (station, temp) = line
          .split_once(';')
          .ok_or_else(|| BrcError::new(format!("No ';' found in \"{line}\"")))?;
        map
          .entry(station.to_owned())
          .or_default()
          .add_reading(temp.parse()?);
        Ok(map)
      },
    )?
    .into_iter()
    .map(|(station, summary)| WeatherStation {
      name: station,
      summary,
    })
    .sorted_unstable(),
  )
}

#[derive(Parser, Debug)]
struct Args {
  #[arg(long, default_value = "measurements.txt")]
  input: String,
}

fn run() -> BrcResult {
  let args = Args::try_parse()?;

  println!(
    "{{{}}}",
    temperature_reading_summaries(&args.input)?
      .map(|station| format!("{station}"))
      .join(", ")
  );
  Ok(())
}

fn main() -> ExitCode {
  if let Err(err) = run() {
    println!("{err}");
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}
