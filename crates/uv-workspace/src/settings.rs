use chrono::{DateTime, Utc};
use serde::Deserialize;

use distribution_types::{FlatIndexLocation, IndexUrl};
use install_wheel_rs::linker::LinkMode;
use uv_auth::KeyringProvider;
use uv_configuration::{ConfigSettings, IndexStrategy, PackageNameSpecifier};
use uv_normalize::PackageName;
use uv_resolver::{AnnotationStyle, PreReleaseMode, ResolutionMode};
use uv_toolchain::PythonVersion;

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct PyProjectToml {
    pub(crate) tool: Option<Tools>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct Tools {
    pub(crate) uv: Option<Options>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
// #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Options {
    quiet: Option<bool>,
    verbose: Option<bool>,
    native_tls: Option<bool>,
    pip: Option<PipOptions>,
    resolver: Option<ResolverOptions>,
    installer: Option<InstallerOptions>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
// #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PipOptions {
    system: Option<bool>,
    offline: Option<bool>,
    index_url: Option<IndexUrl>,
    extra_index_url: Option<IndexUrl>,
    no_index: Option<bool>,
    find_links: Option<Vec<FlatIndexLocation>>,
    index_strategy: Option<IndexStrategy>,
    keyring_provider: Option<KeyringProvider>,
    no_build: Option<bool>,
    no_binary: Option<Vec<PackageNameSpecifier>>,
    only_binary: Option<Vec<PackageNameSpecifier>>,
    no_build_isolation: Option<bool>,
    resolver: Option<ResolverOptions>,
    installer: Option<InstallerOptions>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
// #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ResolverOptions {
    resolution: Option<ResolutionMode>,
    prerelease: Option<PreReleaseMode>,
    no_strip_extras: Option<bool>,
    no_annotate: Option<bool>,
    no_header: Option<bool>,
    generate_hashes: Option<bool>,
    legacy_setup_py: Option<bool>,
    config_setting: Option<ConfigSettings>,
    python_version: Option<PythonVersion>,
    exclude_newer: Option<DateTime<Utc>>,
    no_emit_package: Option<Vec<PackageName>>,
    emit_index_url: Option<bool>,
    emit_find_links: Option<bool>,
    annotation_style: Option<AnnotationStyle>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
// #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct InstallerOptions {
    link_mode: Option<LinkMode>,
    compile: Option<bool>,
}
