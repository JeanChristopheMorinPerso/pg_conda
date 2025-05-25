BEGIN;
CREATE EXTENSION pg_conda;

SELECT 'asd }{}'::condaversion;

ROLLBACK;
