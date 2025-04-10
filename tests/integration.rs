use lassdb::LassDB;

#[test]
fn it_inserts_and_reads() {
    let mut db = LassDB::new();
    db.put("key".into(), "value".into());
    assert_eq!(db.get("key"), Some(&"value".to_string()));
}
