-- Describe the specific tables used in the audit scripts
-- This will list all columns for these tables so we can see their exact case-sensitivity and names.

SELECT table_name, column_name, data_type 
FROM information_schema.columns 
WHERE table_schema = 'public' 
  AND table_name IN (
      'user_', 
      'group_', 
      'role_', 
      'usergrouprole', 
      'users_roles', 
      'dlfileentry', 
      'dlfolder'
  )
ORDER BY table_name, ordinal_position;
