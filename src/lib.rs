use std::collections::HashMap;
use std::hash::Hash;

struct Node<K> {
    key: K,
    next: Option<usize>,
}

impl<K> Node<K> {
    fn new(k: K) -> Node<K> {
        Node { key: k, next: None }
    }
}

pub struct LRUCache<K, V> {
    table: HashMap<K, (V, usize)>,
    nodes: Vec<Node<K>>,
    head: Option<usize>,
    tail: Option<usize>,
    size: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(size: usize) -> LRUCache<K, V> {
        LRUCache {
            table: HashMap::new(),
            nodes: vec![],
            head: None,
            tail: None,
            size: size,
        }
    }

    pub fn put(&mut self, k: &K, v: V) {
        // check for existing node at t
        let existing = match self.table.get(k) {
            Some((_, i)) => Some(*i),
            None => None,
        };
        if let Some(i) = existing {
            // update value
            self.table.insert(k.clone(), (v, i));
            if let Some(tail) = self.tail {
                if tail == i {
                    // if this was the tail, move the tail forward to the next node
                    self.tail = self.nodes[tail].next
                }
            }
            // matched node follows the prior head and becomes the new head
            self.nodes[self.head.unwrap()].next = Some(i);
            self.head = Some(i);
        } else {
            if self.nodes.len() < self.size {
                // haven't filled up storage yet, so just append nodes
                self.append(k, v);
                return;
            }
            let (head, tail) = (self.head.unwrap(), self.tail.unwrap());
            // remove old tail key from table
            self.table.remove(&self.nodes[tail].key);
            // prior head points to tail index
            self.nodes[head].next = self.tail;
            // tail updated to the node following prior tail
            self.tail = self.nodes[tail].next;
            // tail becomes new head, reusing slot in-place
            self.head = Some(tail);
            self.nodes[tail] = Node::new(k.clone());
            self.table.insert(k.clone(), (v, tail));
        }
    }

    fn append(&mut self, k: &K, v: V) {
        if self.nodes.is_empty() {
            self.nodes.push(Node::new(k.clone()));
            self.head = Some(0);
            self.tail = Some(0);
            self.table.insert(k.clone(), (v, 0));
        } else {
            let new_node = Node {
                key: k.clone(),
                next: None,
            };
            let new_i = self.nodes.len();
            self.nodes.push(new_node);
            self.nodes[self.head.unwrap()].next = Some(new_i);
            self.head = Some(new_i);
            self.table.insert(k.clone(), (v, new_i));
        }
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        // check if existing node at t
        let i = match self.table.get(k) {
            Some((_, i)) => *i,
            None => return None,
        };
        // update tail (if matched node was the prior tail)
        if let Some(tail) = self.tail {
            if tail == i {
                self.tail = self.nodes[tail].next
            }
        }
        // update head
        self.nodes[self.head.unwrap()].next = Some(i);
        self.head = Some(i);
        self.table.get(k).map(|(v, _)| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_get() {
        let mut c: LRUCache<String, ()> = LRUCache::new(3);
        assert_eq!(c.get(&"foo".to_string()), None)
    }

    #[test]
    fn test_put_get() {
        let mut c: LRUCache<String, ()> = LRUCache::new(3);
        c.put(&"foo".to_string(), ());
        c.put(&"bar".to_string(), ());
        c.put(&"baz".to_string(), ());
        assert_eq!(c.get(&"foo".to_string()), Some(&()));
        assert_eq!(c.get(&"bar".to_string()), Some(&()));
        assert_eq!(c.get(&"baz".to_string()), Some(&()));
        assert_eq!(c.get(&"quux".to_string()), None);
    }

    #[test]
    fn test_churn_fifo() {
        let mut c: LRUCache<String, ()> = LRUCache::new(3);
        c.put(&"foo".to_string(), ());
        c.put(&"bar".to_string(), ());
        c.put(&"baz".to_string(), ());
        c.put(&"quux".to_string(), ());
        assert_eq!(c.get(&"foo".to_string()), None);
        assert_eq!(c.get(&"bar".to_string()), Some(&()));
        assert_eq!(c.get(&"baz".to_string()), Some(&()));
        assert_eq!(c.get(&"quux".to_string()), Some(&()));
    }

    #[test]
    fn test_churn_bump() {
        let mut c: LRUCache<String, ()> = LRUCache::new(3);
        c.put(&"foo".to_string(), ());
        c.put(&"bar".to_string(), ());
        c.put(&"baz".to_string(), ());
        c.get(&"foo".to_string());
        c.put(&"quux".to_string(), ());
        assert_eq!(c.get(&"foo".to_string()), Some(&()));
        assert_eq!(c.get(&"bar".to_string()), None);
        assert_eq!(c.get(&"baz".to_string()), Some(&()));
        assert_eq!(c.get(&"quux".to_string()), Some(&()));
    }

    #[test]
    fn test_churn_bump2() {
        let mut c: LRUCache<String, ()> = LRUCache::new(3);
        c.put(&"foo".to_string(), ());
        c.put(&"bar".to_string(), ());
        c.put(&"baz".to_string(), ());
        c.get(&"foo".to_string());
        c.get(&"bar".to_string());
        c.put(&"quux".to_string(), ());
        assert_eq!(c.get(&"foo".to_string()), Some(&()));
        assert_eq!(c.get(&"bar".to_string()), Some(&()));
        assert_eq!(c.get(&"baz".to_string()), None);
        assert_eq!(c.get(&"quux".to_string()), Some(&()));
    }
}
