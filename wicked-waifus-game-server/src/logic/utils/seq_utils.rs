use std::collections::{HashMap, VecDeque};

pub trait Sequencer<T> {
    fn new() -> Self;
    fn from_data<V>(data: &HashMap<T, V>) -> Self;
    fn take_id(&mut self) -> T;
    fn give_id(&mut self, id: T);
}

pub struct SequenceGenerator<T, A> {
    recycled_ids: VecDeque<T>,
    next_id: A,
}

macro_rules! sequence_trait_impl {
    ($t:ty, $a:ty) => (
        impl Sequencer<$t> for SequenceGenerator<$t, $a> {
            fn new() -> Self {
                Self {
                    recycled_ids: Default::default(),
                    next_id: <$a>::new(1),
                }
            }

            fn from_data<V>(data: &HashMap<$t, V>) -> Self {
                let max_id = data.keys().max().copied().unwrap_or(1);
                let next_id = <$a>::new(max_id);

                let mut recycled_ids = VecDeque::new();
                for i in 1..max_id {
                    if !data.contains_key(&i) {
                        recycled_ids.push_back(i);
                    }
                }
                Self {
                    recycled_ids,
                    next_id,
                }
            }

            fn take_id(&mut self) -> $t {
                self.recycled_ids
                    .pop_front()
                    .unwrap_or_else(|| self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
            }

            fn give_id(&mut self, id: $t) {
                self.recycled_ids.push_back(id);
            }
        }
    )
}

sequence_trait_impl!(i32, std::sync::atomic::AtomicI32);
sequence_trait_impl!(i64, std::sync::atomic::AtomicI64);