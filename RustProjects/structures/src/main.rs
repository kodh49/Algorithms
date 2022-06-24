fn main() {
    let mut list_a: Link<i32> = Link::None;
    let mut list_b: Link<u32> = Link::None;
    list_a.push(1);
    list_b.push(2);
    println!("{}", list_a.pop().unwrap());
    println!("{}", list_b.pop().unwrap());
}

enum Link<T> {
    None,
    Tail {item: T},
    Link {item: T, next: Box<Link<T>>}
}

impl<T> Link<T> where T:Copy {

    // add new Link to the end of the list
    pub fn push(&mut self, x:T) {
        match self {
            Self::None => self.to_tail(x),
            Self::Tail {..} => self.to_link(x),
            Self::Link {next, ..} => next.push(x)
        };
    }

    // get the latest link from the list and delete it
    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail {item} => {
                let item = *item;
                self.to_none();
                Some(item)
            },
            Self::Link {item, next} => {
                let mut n = Box::new(Self::None);
                let item = *item;
                std::mem::swap(next, &mut n);
                self.to_next(*n);
                Some(item)
            }
        }
    }

    // convert to next-link
    fn to_next(&mut self, nxt: Link<T>) {
        *self = nxt;
    }

    // convert to none-link
    fn to_none(&mut self) {
        *self = std::mem::replace(self, Link::None);
    }

    // convert to tail-Link
    fn to_tail(&mut self, it: T) {
        *self = match self {
            Self::None => Self::Tail{item: it},

            Self::Link {item:_, next:_} => Self::Tail {item: it},

            _ => panic!("Cannot convert to Tail")
        }
    }

    // convert to link-Link
    fn to_link(&mut self, it: T) {
        *self = match self {
            Self::Tail {item} => {
                Self::Link {
                    item: *item,
                    next: Box::new(Self::Tail {item:it})
                }
            },
            _ => {panic!("Cannot convert to Link");}
        }
    }
}