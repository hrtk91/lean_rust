pub trait Command<T1, T2> {
    fn exec(self, repo: &mut T1) -> T2;
}
