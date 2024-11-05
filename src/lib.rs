#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use polars::prelude::*;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

/// -- 异步读取 CSV 文件第一列的日期值，返回字符串数组
#[napi]
pub async fn get_trading_calendar(file_path: String) -> napi::Result<Vec<String>> {
  // -- 使用 tokio 的文件系统操作
  tokio::task::spawn_blocking(move || {
    let df = CsvReadOptions::default()
      .with_skip_rows(1)
      .map_parse_options(|mut opt| {
        opt.encoding = CsvEncoding::LossyUtf8;
        opt.truncate_ragged_lines = true;
        opt
      })
      .try_into_reader_with_file_path(Some(file_path.into()))
      .map_err(|e| napi::Error::from_reason(e.to_string()))?
      .finish()
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(
      df.select_at_idx(0)
        .unwrap()
        .str()
        .unwrap()
        .into_iter()
        .filter_map(|x| x.map(String::from))
        .collect::<Vec<String>>(),
    )
  })
  .await
  .map_err(|e| napi::Error::from_reason(e.to_string()))?
}
