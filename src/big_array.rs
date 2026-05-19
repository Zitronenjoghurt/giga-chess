use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
    array: &[T; N],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    array.as_slice().serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>, const N: usize>(
    deserializer: D,
) -> Result<[T; N], D::Error> {
    let vec = Vec::<T>::deserialize(deserializer)?;
    vec.try_into()
        .map_err(|v: Vec<T>| serde::de::Error::invalid_length(v.len(), &format!("{N}").as_str()))
}
