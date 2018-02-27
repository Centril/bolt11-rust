use types::{Error, U5};
use num::bigint::{BigInt, Sign};

/// Bitcoin-style signature of above (520 bits)
struct Signature {
    /// r (32 bytes)
    r: BigInt,
    /// s (32 bytes)
    s: BigInt,
    /// recovery id (1 byte)
    recovery_id: u8,
}

impl Signature {
    /// decode signature
    pub fn decode(signature: &Vec<u8>) -> Result<Signature, Error> {
        match signature.len() {
            len if len < 65 => Err(Error::InvalidLength(
                "Incorrect signature length".to_owned(),
            )),
            _ => {
                let r = BigInt::from_bytes_be(Sign::Plus, &signature[..32]);
                let s = BigInt::from_bytes_be(Sign::Plus, &signature[32..64]);
                let recovery_id = signature[64];
                Ok(Signature { r, s, recovery_id })
            }
        }
    }
    /// encode signature
    pub fn encode(&self) -> Vec<u8> {
        fn fix_size(bytes: Vec<u8>) -> Vec<u8> {
            match bytes.len() {
                32 => bytes,
                len if len < 32 => [vec![0; 32 - len], bytes].concat(),
                len => bytes[len - 32..].to_vec(),
            }
        }

        let r = fix_size(self.r.to_bytes_be().1);
        let s = fix_size(self.s.to_bytes_be().1);

        [r, s, vec![self.recovery_id]].concat()
    }
}

/// seconds-since-1970 (35 bits, big-endian)
struct Timestamp;

impl Timestamp {
    /// decode timestamp from u5 vector
    fn decode(data: &Vec<U5>) -> u64 {
        data.iter().take(7).fold(0, |a, b| a * 32u64 + *b as u64)
    }
    /// encode timestamp
    fn encode(timestamp: u64) -> Vec<U5> {
        let mut acc: Vec<U5> = Vec::new();
        let mut time_acc = timestamp;
        // 35 bits, big-endian
        while acc.len() < 7 {
            acc.push((time_acc % 32) as U5);
            time_acc /= 32;
        }
        acc.reverse();
        acc
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn timestamp() {
        let data: Vec<U5> = vec![1, 12, 18, 31, 28, 25, 2];
        let timestamp = 1496314658;

        assert_eq!(Timestamp::decode(&data), timestamp);
        assert!(data.eq(&Timestamp::encode(timestamp)));
    }

    #[test]
    fn signature() {
        let hex_str = "38ec6891345e204145be8a3a99de38e98a39d6a569434e1845c8af7205afcfcc7f425\
                       fcd1463e93c32881ead0d6e356d467ec8c02553f9aab15e5738b11f127f00";
        let Signature { r, s, recovery_id } =
            Signature::decode(&::utils::from_hex(&hex_str).unwrap()).unwrap();
        let bytes = Signature { r, s, recovery_id }.encode();
        assert_eq!(::utils::to_hex(&bytes), hex_str)
    }
}
