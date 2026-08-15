SELECT path, fingerprint FROM Tags
WHERE path LIKE ?1 || '/%';
