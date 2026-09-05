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
SELECT item, frecency_score FROM item_frecency
WHERE item LIKE ? || '%'
ORDER BY frecency_score DESC
LIMIT ?;
