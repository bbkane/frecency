/* name: QueryItems :many */
SELECT *
FROM item_frecency
WHERE instr(item, sqlc.arg(term)) > 0
ORDER BY frecency_score DESC;