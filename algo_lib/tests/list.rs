use algo_lib::list::List;

#[test]
fn push_and_iter() {
    let mut list = List::new();
    list.push('a');
    list.push('b');
    list.push('c');

    let collected: String = list.iter().copied().collect();
    assert_eq!(collected, "abc");
    assert_eq!(list.len(), 3);
}

#[test]
fn pop_front_updates_links() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);
    assert!(list.is_empty());
}

#[test]
fn pop_back_updates_links() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
    assert!(list.is_empty());
}

#[test]
fn append_moves_nodes() {
    let mut left = List::new();
    left.push('a');
    left.push('b');

    let mut right = List::new();
    right.push('c');
    right.push('d');

    left.append(&mut right);

    let collected: String = left.iter().copied().collect();
    assert_eq!(collected, "abcd");
    assert!(right.is_empty());
}

#[test]
fn cursor_insert_and_erase() {
    let mut list = List::new();
    let mut cursor = list.begin();

    cursor = list.insert(&cursor, 'a');
    cursor.inc();
    cursor = list.insert(&cursor, 'b');
    cursor.inc();
    cursor = list.insert(&cursor, 'c');

    let collected: String = list.iter().copied().collect();
    assert_eq!(collected, "abc");

    cursor.dec();
    let cursor = list.erase(&mut cursor);
    let collected: String = list.iter().copied().collect();
    assert_eq!(collected, "ac");

    assert_eq!(list.len(), 2);
    assert!(!cursor.is_end());
}

#[test]
fn cursor_end_insert() {
    let mut list = List::new();
    list.push('a');
    list.push('b');

    let end = list.end();
    list.insert(&end, 'c');

    let collected: String = list.iter().copied().collect();
    assert_eq!(collected, "abc");
}
