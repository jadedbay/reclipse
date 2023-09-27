use std::sync::Arc;
use crate::asset::Asset;

pub struct Handle<T: Asset> {
    asset: Arc<T>,
}
