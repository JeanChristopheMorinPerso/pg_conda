-- Test the output function
BEGIN;

CREATE EXTENSION conda;

SELECT '1.2.3'::condaversion::text;

ROLLBACK;
