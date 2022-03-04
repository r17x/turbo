use crate::asset::{Asset, AssetRef, AssetsSetRef};
use turbo_tasks_fs::{FileContentRef, FileSystemPathRef};

#[turbo_tasks::value(Asset)]
#[derive(PartialEq, Eq)]
pub struct ModuleAsset {
    pub source: AssetRef,
}

#[turbo_tasks::value_impl]
impl ModuleAsset {
    #[turbo_tasks::constructor(intern)]
    pub fn new(source: &AssetRef) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

#[turbo_tasks::value_impl]
impl Asset for ModuleAsset {
    fn path(&self) -> FileSystemPathRef {
        self.source.clone().path()
    }
    fn content(&self) -> FileContentRef {
        self.source.clone().content()
    }
    async fn references(&self) -> AssetsSetRef {
        AssetsSetRef::empty()
    }
}