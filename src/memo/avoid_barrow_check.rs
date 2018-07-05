struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

fn to_refs<T>(mut list: &mut List<T>) -> Vec<&mut T> {
  let mut result = vec![];
  loop {
    // need to assign another variable to avoid borrow check
    let list1 = list;
    result.push(&mut list1.value);
    /*
      if let Some(n) = list.next.as_mut() {
      result.push(&mut list.value);
    */
    if let Some(n) = list1.next.as_mut() {
      list = n;
    } else {
      return result;
    }
  }
}