INSERT INTO RecordsTags (tag_id, record_id, position)
VALUES (?1, ?2, ?3)
ON CONFLICT DO NOTHING;
