use buffer_circolare::solution::CircularBuffer;

#[test]
pub fn test_write_size() {
    let mut a = CircularBuffer::<i32>::new(10);

    let _ = a.write(1);

    assert_eq!(a.size, 1);
    assert_eq!(a.tail, 1);

}

#[test]
pub fn test_write_read() {
    let mut a = CircularBuffer::<i32>::new(10);

    let _ = a.write(1);

    if let Some(read) = a.read() {
        assert_eq!(read, 1);
    } 

    assert_eq!(a.head, 1);
    assert_eq!(a.size, 0);

}

#[test]
pub fn test_sequence() {
    let mut a = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        let _ = a.write(i);
    } 
    
    for i in 0..5 {
        if let Some(read) = a.read() {
            assert_eq!(read, i);
            assert_eq!(a.head, if i==4 {0} else {(i+1) as usize});
        } 
    } 

    assert_eq!(a.size, 0);
}

#[test]
pub fn test_read_empty() {

    let mut a = CircularBuffer::<i32>::new(5);

    assert!(a.read().is_none());
    assert_eq!(a.head, 0);
    assert_eq!(a.tail, 0);
    assert_eq!(a.size, 0);
}

#[test]
pub fn test_clear() {
    let mut a = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        let _ = a.write(i);
    } 

    a.clear();

    let b : Box<[Option<i32>]> = vec![None;5].into_boxed_slice();

    assert_eq!(a.array, b);
    assert_eq!(a.size, 0);
}

#[test]
pub fn test_wrap_around() {
    let mut a = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        let _ = a.write(i);
    } 

    assert_eq!(a.tail, 0);
    assert_eq!(a.size, 5);
    
    for i in 0..5 {
        if let Some(read) = a.read() {
            assert_eq!(read, i);
            assert_eq!(a.head, if i==4 {0} else {(i+1) as usize});
        } 
    } 
    assert_eq!(a.head, 0);
    assert_eq!(a.size, 0);
}

#[test]
pub fn test_write_full() {
    let mut a = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        let _ = a.write(i);
    } 

    assert_eq!(a.tail, 0);
    assert_eq!(a.size, 5);

    let baco = a.write(70);

    assert!(baco.is_err());

}


#[test]
pub fn test_overwrite() {

    let mut a = CircularBuffer::<i32>::new(5);

    for i in 0..5 {
        let _ = a.write(i);
    } 

    let over = 10;

    a.overwrite(over);

    if let Some(read) = a.read() {
        assert_eq!(read, over);
    } 

}
