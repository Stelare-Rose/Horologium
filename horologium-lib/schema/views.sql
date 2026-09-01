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
