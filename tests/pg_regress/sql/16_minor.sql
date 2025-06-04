BEGIN;
CREATE EXTENSION pg_conda;

SELECT conda_minor('1.2');
SELECT conda_minor('1');
SELECT conda_minor('1.2a.3'::condaversion);
ROLLBACK;
