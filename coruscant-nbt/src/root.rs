pub struct Root<'k, 'v, T: ?Sized> {
    pub root_name: &'k str,
    pub value: &'v T,
}

impl<'v, T: ?Sized> From<&'v T> for Root<'_, 'v, T> {
    fn from(value: &'v T) -> Self {
        Root { root_name: "", value }
    }
}

impl<'k, 'v, T: ?Sized> From<(&'k str, &'v T)> for Root<'k, 'v, T> {
    fn from((root_name, value): (&'k str, &'v T)) -> Self {
        Root { root_name, value }
    }
}
