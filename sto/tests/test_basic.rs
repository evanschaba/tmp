use lib::basic::*;

#[tokio::test]
async fn test_database_operations() -> Result<()> {
    let db = Database::new(Some(":memory:"))?;

    // Test CREATE and READ
    let key = "test_key".to_string();
    let resource = Resource::List(vec![
        Human {
            name: "Alice".to_string(),
        },
        Human {
            name: "Bob".to_string(),
        },
    ]);

    db.create(key.clone(), resource.clone())?;
    assert_eq!(db.read(&key)?, Some(resource.clone()));

    // Test APPEND
    let new_items = vec![Human {
        name: "Charlie".to_string(),
    }];
    db.append_to_list(&key, new_items)?;

    if let Some(Resource::List(list)) = db.read(&key)? {
        assert_eq!(list.len(), 3);
        assert_eq!(list[2].name, "Charlie");
    } else {
        panic!("Expected list resource");
    }

    // Test REMOVE
    db.remove_from_list(&key, "Bob")?;

    if let Some(Resource::List(list)) = db.read(&key)? {
        assert_eq!(list.len(), 2);
        assert!(!list.iter().any(|h| h.name == "Bob"));
    } else {
        panic!("Expected list resource");
    }

    // Test DELETE
    db.delete(&key)?;
    assert_eq!(db.read(&key)?, None);

    Ok(())
}
