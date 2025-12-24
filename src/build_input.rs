use std::{
  fs::File,
  io::{BufRead, BufReader},
};

use crate::error::{BrcError, BrcResult};

const WEATHER_STATIONS_PATH: &str = "data/weather_stations.csv";

#[derive(Debug)]
pub struct City {
  name: String,
  average_temperature: f32,
}

impl City {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn average_temperature(&self) -> f32 {
    self.average_temperature
  }
}

pub fn get_weather_stations() -> BrcResult<Vec<City>> {
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
