use std::{
  fs::File,
  io::{BufRead, BufReader},
};

use rand::{seq::IteratorRandom, Rng};
use rand_distr::Normal;

use crate::error::{BrcError, BrcResult};

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

pub fn get_weather_stations(path: &str) -> BrcResult<Vec<City>> {
  BufReader::new(File::open(path)?)
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

pub fn generate_input<'a, R: Rng>(
  weather_stations: &'a [City],
  records: u64,
  unique_cities: u32,
  rng: &mut R,
) -> BrcResult<impl Iterator<Item = BrcResult<(&'a str, f32)>>> {
  let sampled_stations = weather_stations
    .iter()
    .choose_multiple(rng, unique_cities as usize);

  Ok((0..records).map(move |_| {
    let city = sampled_stations
      .iter()
      .choose(rng)
      .ok_or_else(|| BrcError::new("Unexpected empty weather_stations".to_owned()))?;

    let average_temp = city.average_temperature();
    let dist = Normal::new(average_temp, 10.)?;
    let measured_temp = rng.sample(dist).clamp(-99.9, 99.9);

    Ok((city.name(), measured_temp))
  }))
}

pub fn output_lines<R: Rng>(
  weather_stations: &[City],
  records: u64,
  unique_cities: u32,
  rng: &mut R,
) -> BrcResult<impl Iterator<Item = BrcResult<String>>> {
  generate_input(weather_stations, records, unique_cities, rng).map(|measurements| {
    measurements.map(|measurement| {
      measurement.map(|(city, measured_temp)| format!("{};{:.1}\n", city, measured_temp))
    })
  })
}
