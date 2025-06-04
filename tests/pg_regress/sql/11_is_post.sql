BEGIN;
CREATE EXTENSION pg_conda;

SELECT conda_is_post('1.0.0') = FALSE;
SELECT conda_is_post('1.0.0'::condaversion) = FALSE;
SELECT conda_is_post('1.0.0.dev1') = FALSE;
SELECT conda_is_post('1.0.0.post1') = TRUE;
SELECT conda_is_post('1.0.0.post1'::condaversion) = TRUE;
ROLLBACK;
