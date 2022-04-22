use crate::vm::error::Error;
use crate::vm::nodes::{TAG_LABEL, SIZE_LABEL, NodeType};

pub type HeapOperationResult<T> = Result<T, Error>;

pub struct Heap {
    memory: Vec<isize>,
    pub size: usize,
    free_at: usize,
}

impl Heap {
    pub fn new(size: usize) -> Heap {
        Heap {
            memory: vec![0; size],
            size,
            free_at: 0,
        }
    }

    pub fn deref(&self, node: usize, label: usize) -> HeapOperationResult<Option<isize>> {
        if self.is_out_of_range(node + label) {
            return Err(self.make_segfault_err())
        }
        Ok(self.memory.get(node + label).map(|v| v.clone()))
    }

    pub fn new_node(&mut self, node_type: NodeType, node_size: usize) -> HeapOperationResult<usize> {
        if self.is_out_of_range(self.free_at + node_size) {
            return Err(Error {
                message: String::from("Out of memory"),
            })
        }

        let node = self.free_at;

        self.update(node, TAG_LABEL.into(), node_type.get_tag().into())?;
        self.update(node, SIZE_LABEL.into(), node_size.try_into().unwrap())?;

        self.free_at += node_size;

        Ok(node)
    }

    pub fn update(&mut self, node: usize, label: usize, target: isize) -> HeapOperationResult<()> {
        if self.is_out_of_range(node + label) {
            return Err(self.make_segfault_err())
        }
        self.memory[node + label] = target;
        Ok(())
    }

    fn is_out_of_range(&self, addr: usize) -> bool {
        addr >= self.size
    }

    fn make_segfault_err(&self) -> Error {
        Error {
            message: String::from("A segmentation fault has occurred"),
        }
    }
}
