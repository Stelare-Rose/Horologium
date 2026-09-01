CREATE VIEW IF NOT EXISTS RecordsWithTagsAndProject AS
SELECT
    r.id,
    r.name,
    r.status,
    r.start,
    r.end,
    r.project AS project_id,
    p.name AS project_name,
    p.color AS project_color,
    (
        SELECT GROUP_CONCAT(rt.tag_id, ',')
        FROM RecordsTags rt
        WHERE rt.record_id = r.id
        ORDER BY rt.position
    ) AS tag_ids
FROM Records r
LEFT JOIN Projects p ON p.id = r.project;

-- Duration Views
CREATE VIEW IF NOT EXISTS TagDurations AS
SELECT
    t.id AS tag_id,
    t.name AS tag_name,
    COALESCE(SUM(COALESCE(r.end, unixepoch()) - r.start), 0) AS total_duration
FROM Tags t
LEFT JOIN RecordsTags rt ON rt.tag_id = t.id
LEFT JOIN Records r ON r.id = rt.record_id
GROUP BY t.id, t.name;

CREATE VIEW IF NOT EXISTS ProjectDurations AS
SELECT
    p.id AS project_id,
    p.name AS project_name,
    COALESCE(SUM(COALESCE(r.end, unixepoch()) - r.start), 0) AS total_duration
FROM Projects p
LEFT JOIN Records r ON r.project = p.id
GROUP BY p.id, p.name;
