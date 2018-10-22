use crate::index::IndexAccessMethod;

pub trait IndexCatalog {
    fn index_entry(&self, name: &str);
}

pub trait IndexCatalogEntry<T> {
    fn access_method(&self) -> Box<dyn IndexAccessMethod<T>>;
}
