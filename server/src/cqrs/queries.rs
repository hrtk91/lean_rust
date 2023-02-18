pub trait Query<Q, T> {
    fn request(self, query_option: Q) -> T;
}
