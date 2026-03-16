-- Liferay Storage Audit: Top 10 Largest Files (PostgreSQL)
-- This query identifies the top 10 largest files in the document library
-- across all sites, including the site name and the user who uploaded them.

SELECT 
    g.name AS sitename,
    f.name AS foldername,
    fe.title AS filename,
    ROUND(fe.size_ / 1024 / 1024, 2) AS sizemb,
    u.emailaddress AS uploadedby
FROM 
    dlfileentry fe
INNER JOIN group_ g ON fe.groupid = g.groupid
INNER JOIN dlfolder f ON fe.folderid = f.folderid
INNER JOIN user_ u ON fe.userid = u.userid
ORDER BY fe.size_ DESC
LIMIT 10;
