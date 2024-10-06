# XID for SQLite

Thin glue to use [XID](https://github.com/rs/xid) in SQLite. It combines the following packages:
- [xid](https://crates.io/crates/xid)
- [sqlite-loadable](https://crates.io/crates/sqlite-loadable)

## Building

Just run:

```bash
cargo build
```

## Testing

1. Initialize the database:

```bash
sqlite3 :memory:
```

2. Load the extension:

```
.load target/debug/libsqlite_xid
```

3. Test:

```sql
select xid();
```
