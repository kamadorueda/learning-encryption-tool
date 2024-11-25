use std::iter::from_fn;

pub(crate) fn try_from_fn<T, E, F>(mut f: F) -> impl Iterator<Item = Result<T, E>>
where
    F: FnMut() -> Result<Option<T>, E>,
{
    from_fn(move || f().transpose())
}
