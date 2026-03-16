-- List all columns for a specific table
-- Useful for verifying exact column names and case-sensitivity.
-- Replace 'user_' with the table you want to inspect.

SELECT column_name, data_type 
FROM information_schema.columns 
WHERE table_schema = 'public' 
  AND table_name = 'user_'
ORDER BY ordinal_position;
