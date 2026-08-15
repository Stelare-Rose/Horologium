SELECT path, fingerprint FROM Records
WHERE path LIKE ?1 || '/%';
