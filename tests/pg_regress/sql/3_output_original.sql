-- Test that the version is not changed during conversions.
-- 1.01 would usually be normalized to 1.1 by rattler.
-- Our extension makes sure that this is not the case.
BEGIN;

CREATE EXTENSION conda;

SELECT '1.01'::condaversion::text;

ROLLBACK;
