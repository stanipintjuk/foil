pub fn all_ok<O, E>(o: O) -> Option<Result<O, E>> {
    Some(Ok(o))
}
