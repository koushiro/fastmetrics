use fastmetrics::{
    derive::*,
    error::Error,
    metrics::{counter::ConstCounter, family::Family},
    registry::{Register, Registry},
};

#[derive(Clone, Debug)]
pub struct ReleaseInfoMetrics {
    release_info: Family<ReleaseInfoLabels, ConstCounter>,
}

impl Default for ReleaseInfoMetrics {
    fn default() -> Self {
        Self {
            release_info: Family::<ReleaseInfoLabels, ConstCounter>::new(|| ConstCounter::new(1)),
        }
    }
}

impl Register for ReleaseInfoMetrics {
    fn register(&self, registry: &mut Registry) -> Result<(), Error> {
        registry.register("release_info", "Release information", self.release_info.clone())?;
        let labels = ReleaseInfoLabels::default();
        self.release_info.with_or_new(&labels, |_| {});
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, EncodeLabelSet, LabelSetSchema)]
struct ReleaseInfoLabels {
    version: &'static str,
    commit: &'static str,
    rustc_version: &'static str,
}

impl Default for ReleaseInfoLabels {
    fn default() -> Self {
        let version = build_info::PKG_VERSION;
        let commit = build_info::GIT_COMMIT_HASH_SHORT.unwrap_or("unknown");
        let rustc_version = build_info::RUSTC_VERSION;
        Self { version, commit, rustc_version }
    }
}

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
