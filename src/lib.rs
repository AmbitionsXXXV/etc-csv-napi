#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use polars::prelude::*;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

/// -- 异步读取 CSV 文件第一列的日期值，返回字符串数组
/// -- limit: 可选参数，若提供则返回最后 n 个值
#[napi]
pub async fn get_trading_calendar(
  file_path: String,
  limit: Option<u32>,
) -> napi::Result<Vec<String>> {
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

    let mut result = df
      .select_at_idx(0)
      .ok_or_else(|| napi::Error::from_reason("未找到第一列"))?
      .str()
      .map_err(|e| napi::Error::from_reason(format!("转换为字符串失败: {}", e)))?
      .into_iter()
      .filter_map(|x| x.map(String::from))
      .collect::<Vec<String>>();

    // -- 如果指定了 limit，则只返回最后 n 个值
    if let Some(n) = limit {
      result.truncate(result.len().max(n as usize));
      result = result.into_iter().rev().take(n as usize).rev().collect();
    }

    Ok(result)
  })
  .await
  .map_err(|e| napi::Error::from_reason(e.to_string()))?
}
