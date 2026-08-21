package db

//go:generate go get -tool -modfile=sqlc.go.mod github.com/sqlc-dev/sqlc/cmd/sqlc@latest
//go:generate go tool -modfile=sqlc.go.mod sqlc generate

//go:generate go get -tool -modfile=tbls.go.mod github.com/k1LoW/tbls@latest
//go:generate sh -c "cd .. && cargo run --bin create_empty_db"
//go:generate go tool -modfile=tbls.go.mod tbls doc --rm-dist
