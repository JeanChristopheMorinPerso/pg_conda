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
fn conda_smaller(a: CondaVersion, b: CondaVersion) -> CondaVersion {
    if a <= b {
        return a;
    }
    return b;
}

#[pgrx::pg_extern]
fn conda_larger(a: CondaVersion, b: CondaVersion) -> CondaVersion {
    if a >= b {
        return a;
    }
    return b;
}

pgrx::extension_sql!(
    r#"
CREATE OR REPLACE AGGREGATE min(CondaVersion) (
    SFUNC = conda_smaller,
    STYPE = CondaVersion,
    SORTOP = <
);
"#,
    name = "min_aggregate",
    requires = [CondaVersion, conda_smaller],
    // Insert at the end because we need the operator to be already defined.
    finalize,
);

pgrx::extension_sql!(
    r#"
CREATE OR REPLACE AGGREGATE max(CondaVersion) (
    SFUNC = conda_larger,
    STYPE = CondaVersion,
    SORTOP = >
);
"#,
    name = "max_aggregate",
    // Insert at the end because we need the operator to be already defined.
    // Note how we insert after min_aggregate. That's because we can't have two "finalize".
    requires = [CondaVersion, conda_larger, "min_aggregate"],
);
