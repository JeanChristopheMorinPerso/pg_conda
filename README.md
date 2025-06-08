<div align="center">
  <img src="assets/logo.svg" alt="pg_conda logo" width="200"/>
</div>

`pg_conda` is a PostgreSQL extension that adds types and functions for the conda ecosystem to your PostgreSQL database.

Powered by [rattler](https://github.com/conda/rattler) and [pgrx](https://github.com/pgcentralfoundation/pgrx).

# Installation

RPMs are available on the GitHub Releases page at https://github.com/JeanChristopheMorinPerso/pg_conda/releases/latest.

If you want to install for another OS or Linux distro, you can build from source or you can open an issue to request a build.

# Types

## condaversion

A conda version. It supports all the same operators as a string and also supports conda's ordering. It also
supports the `max` and `min` aggregate functions.

# Aggregates

## min(condaversion) -> condaversion

Returns the minimum conda version from a set of values, using conda's version ordering rules.

```sql
SELECT min(version) FROM (
    VALUES
        ('1.5.0'::condaversion),
        ('2.0.0'::condaversion),
        ('1.0.0'::condaversion)
) AS t(version);
-- Returns: 1.0.0
```

## max(condaversion) -> condaversion

Returns the maximum conda version from a set of values, using conda's version ordering rules.

```sql
SELECT max(version) FROM (
    VALUES
        ('1.5.0'::condaversion),
        ('2.0.0'::condaversion),
        ('1.0.0'::condaversion)
) AS t(version);
-- Returns: 2.0.0
```

# Functions

## conda_is_dev(condaversion) -> boolean

Returns `true` if the version is a development version, `false` otherwise.

```sql
SELECT conda_is_dev('1.0.0.dev0'::condaversion); -- true
SELECT conda_is_dev('1.0.0'::condaversion);      -- false
```

## conda_is_post(condaversion) -> boolean

Returns `true` if the version contains a post-release component, `false` otherwise.

```sql
SELECT conda_is_post('1.0.0.post1'::condaversion); -- true
SELECT conda_is_post('1.0.0'::condaversion);       -- false
```

## conda_has_epoch(condaversion) -> boolean

Returns `true` if the version has an epoch component, `false` otherwise.

```sql
SELECT conda_has_epoch('1!1.0.0'::condaversion); -- true
SELECT conda_has_epoch('1.0.0'::condaversion);   -- false
```

## conda_has_local(condaversion) -> boolean

Returns `true` if the version has a local version identifier, `false` otherwise.

```sql
SELECT conda_has_local('1.0.0+local.1'::condaversion); -- true
SELECT conda_has_local('1.0.0'::condaversion);         -- false
```

## conda_segments(condaversion) -> text[]

Returns an array of strings representing all the components/segments of the version.

```sql
SELECT conda_segments('1.2.3.alpha1'::condaversion); -- {'1', '2', '3', 'alpha', '1'}
```

## conda_major(condaversion) -> bigint

Returns the major version number if the version has a simple numeric major component, `NULL` otherwise.

```sql
SELECT conda_major('1.2.3'::condaversion);  -- 1
SELECT conda_major('1a.2.3'::condaversion); -- NULL (non-numeric major)
SELECT conda_major('a'::condaversion);      -- 0
```

## conda_minor(condaversion) -> bigint

Returns the minor version number if the version has simple numeric major and minor components, `NULL` otherwise.

```sql
SELECT conda_minor('1.2.3'::condaversion);  -- 2
SELECT conda_minor('1.2a.3'::condaversion); -- NULL (non-numeric minor)
SELECT conda_minor('1'::condaversion);      -- NULL (no minor component)
```
