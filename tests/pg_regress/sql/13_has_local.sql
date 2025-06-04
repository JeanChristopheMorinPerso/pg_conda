BEGIN;
CREATE EXTENSION pg_conda;

SELECT conda_has_local('1.0.0') = FALSE;
SELECT conda_has_local('1.0.0'::condaversion) = FALSE;
SELECT conda_has_local('1.0.0+whatever') = TRUE;
SELECT conda_has_local('1.0.0+whatever'::condaversion) = TRUE;
ROLLBACK;
