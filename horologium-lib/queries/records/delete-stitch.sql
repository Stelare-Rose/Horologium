UPDATE Records
SET end = (
    SELECT start FROM Records
    WHERE (start > ?1) OR (start = ?1 AND id > ?2)
    AND id != ?2
    ORDER BY start ASC, id ASC
    LIMIT 1
)
WHERE id = (
    SELECT id FROM Records
    WHERE (start < ?1) OR (start = ?1 AND id < ?2)
    ORDER BY start DESC, id DESC
    LIMIT 1
);
