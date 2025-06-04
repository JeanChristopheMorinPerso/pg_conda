BEGIN;
CREATE EXTENSION pg_conda;

SELECT conda_segments('1.2.3');
SELECT conda_segments('25!1.2.3.dev6+local-3');
ROLLBACK;
