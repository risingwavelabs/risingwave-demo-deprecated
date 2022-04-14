use bytes::Bytes;

trait GenerateState {
    fn get(key: String) -> Bytes;
    fn update(key: String, value: Bytes);
}
