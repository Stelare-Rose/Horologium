INSERT INTO clocks (path, sum)
VALUES (?1, -1)
ON CONFLICT(path) DO UPDATE SET sum = -1;
