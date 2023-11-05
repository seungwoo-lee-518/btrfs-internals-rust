use crate::structs::CSUMValue;

#[allow(dead_code)]
impl CSUMValue {
    pub fn to_hex(self) -> String {
        match self {
            CSUMValue::CRC32C(value) => {
                hex::encode(value)
            }
            CSUMValue::XXHASH(value) => {
                hex::encode(value)
            }
            CSUMValue::SHA256(value) => {
                hex::encode(value)
            }
            CSUMValue::BLAKE2B(value) => {
                hex::encode(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::CSUMValue;

    #[test]
    fn check_to_hex() {
        let mut b = [0u8; 8];
        assert!(hex::decode_to_slice("fba18262b29fc5e7", &mut b).is_ok());
        let csv = CSUMValue::XXHASH(b);
        println!("{:?}", csv.to_hex())
    }
}
