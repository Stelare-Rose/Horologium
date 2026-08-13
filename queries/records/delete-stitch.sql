UPDATE Records
SET end = (SELECT start FROM Records WHERE start > ?1 ORDER BY start ASC LIMIT 1)
WHERE id = (SELECT id FROM Records WHERE start < ?1 ORDER BY start DESC LIMIT 1);
