use std::str::FromStr;

::pgrx::pg_module_magic!();

#[derive(
    pgrx::PostgresType,
    pgrx::PostgresEq,
    pgrx::PostgresOrd,
    pgrx::PostgresHash,
    serde::Serialize,
    serde::Deserialize,
    Debug,
)]
#[inoutfuncs]
pub struct CondaVersion {
    version: rattler_conda_types::Version,
    source: Option<Box<str>>,
}

impl std::hash::Hash for CondaVersion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}

impl PartialEq for CondaVersion {
    fn eq(&self, other: &Self) -> bool {
        // We initially tried to also compare the source, but this caused
        // the hash to not work correctly.
        self.version.eq(&other.version) //&& self.as_str().eq(&other.as_str())
    }
}

impl Eq for CondaVersion {}

impl PartialOrd for CondaVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl Ord for CondaVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First order by version then by string representation
        self.version
            .cmp(&other.version)
            .then_with(|| self.as_str().cmp(&other.as_str()))
    }
}

impl CondaVersion {
    /// Constructs a new instance from a [`Version`] and a source representation.
    pub fn new(version: rattler_conda_types::Version, source: impl ToString) -> Self {
        Self {
            version,
            source: Some(source.to_string().into_boxed_str()),
        }
    }

    /// Returns the [`Version`]
    pub fn version(&self) -> &rattler_conda_types::Version {
        &self.version
    }

    /// Returns the string representation of this instance. Either this is a reference to the source
    /// string or an owned formatted version of the stored version.
    pub fn as_str(&self) -> std::borrow::Cow<'_, str> {
        match &self.source {
            Some(source) => std::borrow::Cow::Borrowed(source.as_ref()),
            None => std::borrow::Cow::Owned(format!("{}", &self.version)),
        }
    }

    /// Convert this instance back into a [`Version`].
    pub fn into_version(self) -> rattler_conda_types::Version {
        self.version
    }
}

impl PartialEq<rattler_conda_types::Version> for CondaVersion {
    fn eq(&self, other: &rattler_conda_types::Version) -> bool {
        self.version.eq(other)
    }
}

impl PartialOrd<rattler_conda_types::Version> for CondaVersion {
    fn partial_cmp(&self, other: &rattler_conda_types::Version) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(other)
    }
}

impl From<rattler_conda_types::Version> for CondaVersion {
    fn from(version: rattler_conda_types::Version) -> Self {
        CondaVersion {
            version,
            source: None,
        }
    }
}

impl AsRef<rattler_conda_types::Version> for CondaVersion {
    fn as_ref(&self) -> &rattler_conda_types::Version {
        &self.version
    }
}

impl std::ops::Deref for CondaVersion {
    type Target = rattler_conda_types::Version;

    fn deref(&self) -> &Self::Target {
        &self.version
    }
}

impl std::fmt::Display for CondaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            Some(source) => write!(f, "{}", source.as_ref()),
            None => write!(f, "{}", &self.version),
        }
    }
}

impl pgrx::InOutFuncs for CondaVersion {
    fn input(input: &core::ffi::CStr) -> Self {
        let s = input
            .to_str()
            .unwrap_or_else(|err| panic!("failed to Cstr to str. Error: {}", err));

        let version = rattler_conda_types::Version::from_str(s).unwrap_or_else(|err| {
            panic!(
                "failed to parse version {:?}. It is probably not a valid conda version. Error: {}",
                s, err
            )
        });

        return Self {
            version,
            source: Some(s.to_owned().into_boxed_str()),
        };
    }

    fn output(&self, buffer: &mut pgrx::StringInfo) {
        buffer.push_str(self.source.as_deref().unwrap());
    }
}

// https://github.com/theory/pg-semver/blob/main/sql/semver.sql#L246-L255
#[pgrx::pg_extern]
fn _conda_smaller(a: CondaVersion, b: CondaVersion) -> CondaVersion {
    if a <= b {
        return a;
    }
    return b;
}

#[pgrx::pg_extern]
fn _conda_larger(a: CondaVersion, b: CondaVersion) -> CondaVersion {
    if a >= b {
        return a;
    }
    return b;
}

pgrx::extension_sql!(
    r#"
CREATE OR REPLACE AGGREGATE min(CondaVersion) (
    SFUNC = _conda_smaller,
    STYPE = CondaVersion,
    SORTOP = <
);
"#,
    name = "min_aggregate",
    requires = [CondaVersion, _conda_smaller],
    // Insert at the end because we need the operator to be already defined.
    finalize,
);

pgrx::extension_sql!(
    r#"
CREATE OR REPLACE AGGREGATE max(CondaVersion) (
    SFUNC = _conda_larger,
    STYPE = CondaVersion,
    SORTOP = >
);
"#,
    name = "max_aggregate",
    // Insert at the end because we need the operator to be already defined.
    // Note how we insert after min_aggregate. That's because we can't have two "finalize".
    requires = [CondaVersion, _conda_larger, "min_aggregate"],
);

#[pgrx::pg_extern]
fn conda_is_dev(version: CondaVersion) -> bool {
    return version.version.is_dev();
}

#[pgrx::pg_extern]
fn conda_is_post(version: CondaVersion) -> bool {
    return version
        .segments()
        .flat_map(|segment| segment.components())
        .any(rattler_conda_types::Component::is_post);
}

#[pgrx::pg_extern]
fn conda_has_epoch(version: CondaVersion) -> bool {
    return version.version.has_epoch();
}

#[pgrx::pg_extern]
fn conda_has_local(version: CondaVersion) -> bool {
    return version.version.has_local();
}

#[pgrx::pg_extern]
fn conda_segments(version: CondaVersion) -> Vec<String> {
    return version
        .version
        .segments()
        .map(|segment| {
            segment
                .components()
                .map(|component| component.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect();
}

#[pgrx::pg_extern]
fn conda_major(version: CondaVersion) -> Option<i64> {
    let mut segments = version.version.segments();
    let segment = segments.next()?;

    if segment.component_count() == 1 {
        return Some(
            segment
                .components()
                .next()
                .and_then(rattler_conda_types::Component::as_number)? as i64,
        );
    }
    return None;
}

#[pgrx::pg_extern]
fn conda_minor(version: CondaVersion) -> Option<i64> {
    let mut segments = version.version.segments();
    let major = segments.next()?;
    let minor = segments.next()?;

    if major.component_count() == 1 && minor.component_count() == 1 {
        return Some(
            minor
                .components()
                .next()
                .and_then(rattler_conda_types::Component::as_number)? as i64,
        );
    }
    return None;
}
