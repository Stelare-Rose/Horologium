SELECT id, name, status, start, end, project_id, project_name, project_color, tag_ids
FROM RecordsWithTagsAndProject
WHERE (?1 IS NULL OR end IS NULL OR end >= ?1)
  AND (?2 IS NULL OR start <= ?2)
ORDER BY start ASC, id ASC;
