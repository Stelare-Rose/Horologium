INSERT INTO clocks (path, sum) VALUES (?1, ?2)
ON CONFLICT(path) DO UPDATE SET sum = excluded.sum;
