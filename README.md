PostgreSQL extension that adds types and functions for the conda ecosystem to your PostgreSQL database.

Powered by [rattler](https://github.com/conda/rattler) and [pgrx](https://github.com/pgcentralfoundation/pgrx).

# Types

## condaversion

A conda version. It supports all the same operators as a string and also supports conda's ordering. It also
supports the `max` and `min` aggregate functions.
