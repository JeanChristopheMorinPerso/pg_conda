BEGIN;
CREATE EXTENSION pg_conda;

SELECT conda_has_epoch('1.0.0') = FALSE;
SELECT conda_has_epoch('1.0.0'::condaversion) = FALSE;
SELECT conda_has_epoch('2!1.0.0') = TRUE;
SELECT conda_has_epoch('2!1.0.0'::condaversion) = TRUE;
ROLLBACK;
