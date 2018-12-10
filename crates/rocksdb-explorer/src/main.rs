fn main() -> Result<(), rocksdb::Error> {
    let db = rocksdb::DB::open_default("/tmp/protodb/data")?;
    for (key, _) in db.iterator(rocksdb::IteratorMode::Start) {
        println!("{}", String::from_utf8_lossy(&key));
    }
    Ok(())
}
