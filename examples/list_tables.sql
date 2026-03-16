-- List all tables in the public schema
-- Useful for verifying table names and case-sensitivity issues.

SELECT table_name 
FROM information_schema.tables 
WHERE table_schema = 'public'
ORDER BY table_name;
