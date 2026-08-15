SELECT path, fingerprint FROM Projects
WHERE path LIKE ?1 || '/%';
