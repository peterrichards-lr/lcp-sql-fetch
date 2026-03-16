-- Liferay User-Site-Role Security Audit (PostgreSQL / Liferay 7.4)
-- This version uses friendlyurl for context and adds a role_type label for clarity.

(
    -- Site and Organization Roles
    SELECT 
        u.emailaddress, 
        CASE 
            WHEN r.type_ = 2 THEN 'Site: ' || g.friendlyurl
            WHEN r.type_ = 3 THEN 'Org: ' || g.friendlyurl
            ELSE 'Group: ' || g.friendlyurl
        END AS context, 
        r.name AS rolename,
        CASE 
            WHEN r.type_ = 2 THEN 'Site Role'
            WHEN r.type_ = 3 THEN 'Organization Role'
            ELSE 'Other'
        END AS role_type
    FROM 
        user_ u
    INNER JOIN usergrouprole ugr ON u.userid = ugr.userid
    INNER JOIN group_ g ON ugr.groupid = g.groupid
    INNER JOIN role_ r ON ugr.roleid = r.roleid
    WHERE u.status = 0
)
UNION
(
    -- Regular (Portal-Wide) Roles
    SELECT 
        u.emailaddress, 
        'Portal-Wide' AS context, 
        r.name AS rolename,
        'Regular Role' AS role_type
    FROM 
        user_ u
    INNER JOIN users_roles ur ON u.userid = ur.userid
    INNER JOIN role_ r ON ur.roleid = r.roleid
    WHERE u.status = 0
)
ORDER BY emailaddress, role_type, context;
