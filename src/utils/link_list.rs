
#[derive(Clone)]
enum Link<T> {
  None,
  Tail { item: T },
  Link { item: T, next: Box<Link<T>> }
}

#[derive(Clone)]
struct Cursor<T> {
  curr: Link<T>
}

impl <T> Link<T> where T: std::fmt::Debug, T: Copy {
  pub fn new() -> Self {
    Self::None
  }

  pub fn push(&mut self, x: T) {
    match self {
      Self::None => {
        println!("none, {:?}", x);
        self.to_tail(x)
      },
      Self::Tail { .. } => {
        println!("tail, {:?}", x);
        self.to_link(x)
      },
      Self::Link { next, .. } => {
        println!("link, {:?}", x);
        next.push(x)
      }
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    match self {
      Self::None => None,
      Self::Tail { item } => {
        let item = *item;
        self.to_none();
        Some(item)
      },
      Self::Link { item, next } => {
        let mut n = Box::new(Self::None);
        let item = *item;
        std::mem::swap(next, &mut n);
        self.to_next(*n);
        Some(item)
      }
    }
  }

  // no error even if len = 3, index = 5
  pub fn insert(&mut self, index: usize, x: T) {
    match self {
      Self::None => {
        self.to_tail(x);
      },
      Self::Tail { .. } => {
        self.to_link(x);
      },
      Self::Link { item, next } => {
        
      }
    }
  }

  fn to_next(&mut self, nxt: Link<T>) {
    *self = nxt;
  }

  fn to_none(&mut self) {
    *self = std::mem::replace(self, Link::None);
  }
  
  fn to_tail(&mut self, it: T) {
    *self = match self {
      Self::None =>
        Self::Tail { item: it },
      Self::Link { item:_, next:_ } =>
        Self::Tail { item: it },
      _ => panic!("can not to tail")
    }
  }

  fn to_link(&mut self, x: T) {
    *self = match self {
      Self::Tail { item } => {
        Self::Link {
          item: *item,
          next: Box::new(Self::Tail { item: x })
        }
      },
      _ => { panic!("can not to link"); }
    }
  }  
}

impl <T> IntoIterator for Link<T> where T: Copy {
  type Item = T;
  type IntoIter = Cursor<T>;

  fn into_iter(self) -> Self::IntoIter {
    Cursor { curr: self }
  }
}

impl<T> Iterator for Cursor<T> where T: Copy {
  type Item = T;
  
  fn next(&mut self) -> Option<T> {
    let nxt = match self.curr {
      Link::None => None,
      Link::Tail { item } => {
        self.curr = Link::None;
        Some(item)
      },
      Link::Link { item, ref mut next } => {
        let mut n = Box::new(Link::None);
        std::mem::swap(next, &mut n);
        self.curr = *n;
        Some(item)
      }
    };
    nxt
  }
}

pub fn test() {
  let mut list: Link<i32> = Link::new();
  let mut list2: Link<i32> = Link::new();

  list.push(1);
  list.push(2);
  list.push(3);

  list2.push(10);
  list2.push(20);
  list2.push(30);

  println!("a {}", list2.pop().unwrap());
  println!("b {}", list2.pop().unwrap());
  println!("c {}", list2.pop().unwrap());

  for i in list.clone() {
    println!("d {}", i);
  }

  for (i, x) in list.clone().into_iter().enumerate() {
    println!("iter2: {}, {}", i, x);
  }

}
