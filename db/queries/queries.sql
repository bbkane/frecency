-- name: InsertOrUpdateItem :one
INSERT INTO item(item, base_score, create_time, update_time)
VALUES (?, ?, ?, ?)
ON CONFLICT (item) DO UPDATE
SET
  item = excluded.item
RETURNING id;

-- name: InsertIntoAccessLog :exec
INSERT INTO access_log(item_id, access_time) VALUES (?, ?);

-- name: UpdateItem :one
UPDATE item
SET
  item = COALESCE(sqlc.narg(new_key), item),
  base_score = COALESCE(sqlc.narg(base_score), base_score),
  update_time = sqlc.arg(update_time)
WHERE item = sqlc.arg(key)
RETURNING id;

-- name: DeleteItem :one
DELETE FROM item
WHERE item = sqlc.arg(key)
RETURNING id;

-- name: SelectPruneCandidates :many
WITH candidates AS (
  SELECT
    i.id,
    i.item,
    CAST(COUNT(l.id) AS INTEGER) AS access_count,
    CAST(COALESCE(MAX(l.access_time), i.create_time) AS INTEGER) AS last_access_time,
    CAST(i.base_score
    + COALESCE(
      SUM(1.0 / (1.0 + MAX(sqlc.arg(now) - l.access_time, 0) / 604800.0)),
      0
    ) AS REAL) AS frecency_score,
    i.create_time,
    i.update_time
  FROM item AS i
  LEFT JOIN access_log AS l ON l.item_id = i.id
  GROUP BY
    i.id,
    i.item,
    i.base_score,
    i.create_time,
    i.update_time
)
SELECT id, item, access_count
FROM candidates
WHERE (sqlc.narg(score_below) IS NULL OR frecency_score < sqlc.narg(score_below))
  AND (
    CAST(sqlc.narg(prefix) AS TEXT) IS NULL
    OR substr(item, 1, length(CAST(sqlc.narg(prefix) AS TEXT))) = CAST(sqlc.narg(prefix) AS TEXT)
  )
  AND (
    CAST(sqlc.narg(last_access_before) AS INTEGER) IS NULL
    OR last_access_time <= CAST(sqlc.narg(last_access_before) AS INTEGER)
  )
  AND (
    CAST(sqlc.narg(created_before) AS INTEGER) IS NULL
    OR create_time < CAST(sqlc.narg(created_before) AS INTEGER)
  )
  AND (
    CAST(sqlc.narg(updated_before) AS INTEGER) IS NULL
    OR update_time < CAST(sqlc.narg(updated_before) AS INTEGER)
  )
ORDER BY item;

-- name: DeleteItemById :exec
DELETE FROM item WHERE id = sqlc.arg(id);

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
