-- name: InsertOrUpdateItem :one
INSERT INTO item(item, base_score, create_time, update_time)
VALUES (?, ?, ?, ?)
ON CONFLICT (item) DO UPDATE
SET
  item = excluded.item
RETURNING id;

-- name: InsertIntoAccessLog :exec
INSERT INTO access_log(item_id, access_time) VALUES (?, ?);

-- name: QuerySelectFromItemFrecency :many
SELECT
  i.item,
  CAST(i.base_score
  + COALESCE(
    SUM(1.0 / (1.0 + MAX(sqlc.arg(now) - l.access_time, 0) / 604800.0)),
    0
  ) AS REAL) AS frecency_score
FROM item AS i
LEFT JOIN access_log AS l ON l.item_id = i.id
WHERE i.item LIKE sqlc.arg(prefix) || '%'
GROUP BY
  i.id,
  i.item,
  i.base_score
ORDER BY frecency_score DESC
LIMIT sqlc.arg(limit);
