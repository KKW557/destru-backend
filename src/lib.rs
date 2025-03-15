use crate::SqidsError::{Internal, InvalidID};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use jsonwebtoken::{EncodingKey, Header};
use lazy_static::lazy_static;
use serde::Serialize;
use sqids::Sqids;
use thiserror::Error;

const MAGIC_VALUE: u64 = 557;

lazy_static!{
   static ref SQIDS: Sqids = Sqids::builder()
    .alphabet("ABCDEFGHJKLMNPQRSTUVWXYZ123456789abcdefghijkmnopqrstuvwxyz".chars().collect())
    .min_length(6)
    .build()
    .expect("构建Sqids失败");

    static ref ARGON2: Argon2<'static> = Argon2::default();

    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(b"destru");
}

#[derive(Debug, Error)]
pub enum SqidsError {
    #[error("无效ID")]
    InvalidID,
    #[error("Sqids错误: {0:?}")]
    Internal(sqids::Error),
}

pub fn encode_sqids(flag: u8, value: i64) -> Result<String, SqidsError> {
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
            Err(Internal(e))
        }
    }
}

pub fn decode_sqids(flag: u8, s: &str) -> Result<i64, SqidsError> {
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

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let result = ARGON2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(result)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let password_hash = match PasswordHash::new(hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    match ARGON2.verify_password(password.as_bytes(), &password_hash) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Serialize)]
struct Claims {
    user: String,
    expired: usize,
}

pub fn generate_jwt(user: i64, expired: DateTime<Utc>) -> String {
    let claims = Claims {
        user: encode_sqids(0, user).unwrap(),
        expired: expired.timestamp() as usize
    };
    jsonwebtoken::encode(&Header::default(), &claims, &ENCODING_KEY).unwrap()
}