use std::marker::PhantomData;

use super::Asset;

pub struct Handle<T: Asset> {
    pub asset_id: usize,
    _marker: PhantomData<T>,
}

impl<T: Asset> Handle<T> {
    pub fn new(asset_id: usize) -> Self {
        Self {
            asset_id,
            _marker: PhantomData,
        }
    }
}