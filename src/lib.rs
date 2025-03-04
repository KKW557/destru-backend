use lazy_static::lazy_static;
use sqids::Sqids;
use thiserror::Error;
use crate::Error::{InvalidID, SqidsError};

const MAGIC_VALUE: u64 = 557;

lazy_static!{
   static ref SQIDS: Sqids = Sqids::builder()
    .alphabet("ABCDEFGHJKLMNPQRSTUVWXYZ123456789abcdefghijkmnopqrstuvwxyz".chars().collect())
    .min_length(6)
    .build()
    .expect("构建Sqids失败");
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("无效ID")]
    InvalidID,
    #[error("Sqids错误: {0:?}")]
    SqidsError(sqids::Error),
}

pub fn encode(flag: u8, value: i64) -> Result<String, Error> {
    if value < 0 {
        return Err(InvalidID)
    }

    let numbers = &[
        value as u64,
        MAGIC_VALUE + flag as u64,
    ];

    let result = SQIDS.encode(numbers);

    match result {
        Ok(s) => {
            Ok(s)
        }
        Err(e) => {
            Err(SqidsError(e))
        }
    }
}

pub fn decode(flag: u8, s: &str) -> Result<i64, Error> {
    let vec = SQIDS.decode(s);

    if vec.len() != 2 {
        return Err(InvalidID);
    }

    let (v, m) = (vec[0], vec[1]);

    if m - MAGIC_VALUE != flag as u64 {
        return Err(InvalidID);
    }

    if v > i64::MAX as u64 {
        return Err(InvalidID)
    }

    Ok(v as i64)
}