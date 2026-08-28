CREATE TABLE item(
  id INTEGER PRIMARY KEY NOT NULL,
  item TEXT NOT NULL UNIQUE,
  base_score INTEGER NOT NULL,
  create_time INTEGER NOT NULL,
  update_time INTEGER NOT NULL
) STRICT;

CREATE TABLE access_log(
  id INTEGER PRIMARY KEY NOT NULL,
  item_id INTEGER NOT NULL REFERENCES item(id) ON DELETE CASCADE,
  access_time INTEGER NOT NULL
) STRICT;

CREATE INDEX access_log_item_time_idx ON access_log (item_id, access_time);

CREATE VIEW item_frecency AS
SELECT
  i.id,
  i.item,
  i.base_score,
  CAST(strftime('%Y-%m-%dT%H:%M:%SZ', i.create_time, 'unixepoch') AS TEXT) AS create_datetime_rfc3339,
  CAST(strftime('%Y-%m-%dT%H:%M:%SZ', i.update_time, 'unixepoch') AS TEXT) AS update_datetime_rfc3339,
  CAST(COUNT(l.id) AS INTEGER) AS access_count,
  CAST(CASE
    WHEN MAX(l.access_time) IS NULL THEN NULL
    ELSE strftime('%Y-%m-%dT%H:%M:%SZ', MAX(l.access_time), 'unixepoch')
  END AS TEXT) AS last_access_datetime_rfc3339,
  CAST(i.base_score
  + COALESCE(
    SUM(1.0 / (1.0 + MAX(unixepoch() - l.access_time, 0) / 604800.0)),
    0
  ) AS REAL) AS frecency_score
FROM item AS i
LEFT JOIN access_log AS l ON l.item_id = i.id
GROUP BY
  i.id,
  i.item,
  i.base_score,
  i.create_time,
  i.update_time;
