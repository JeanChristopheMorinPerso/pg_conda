-- Test the input function
BEGIN;

CREATE EXTENSION conda;

SELECT '1.2.3'::condaversion;

ROLLBACK;
