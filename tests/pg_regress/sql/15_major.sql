BEGIN;
CREATE EXTENSION pg_conda;

SELECT conda_major('1.2.3');
SELECT conda_major('a');
SELECT conda_major('1a.2.3');
ROLLBACK;
