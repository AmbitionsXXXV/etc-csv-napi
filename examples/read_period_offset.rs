use polars::prelude::*;
use std::error::Error;

const CSV_PATH: &str = "./period_offset.csv";

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let df = CsvReadOptions::default()
    .with_skip_rows(1)
    .map_parse_options(|mut opt| {
      opt.encoding = CsvEncoding::LossyUtf8;
      opt.truncate_ragged_lines = true;
      opt
    })
    .try_into_reader_with_file_path(Some(CSV_PATH.into()))?
    .finish()?;

  let df_clone = df
    .select_at_idx(0)
    .unwrap()
    .str()
    .unwrap()
    .into_iter()
    .filter_map(|x| x.map(String::from))
    .collect::<Vec<String>>();
  println!("{:?}", df_clone);

  Ok(())
}
