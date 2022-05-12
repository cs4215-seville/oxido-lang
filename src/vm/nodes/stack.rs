use crate::vm::nodes::{
    NodeType,
    FIRST_CHILD_LABEL,
    LAST_CHILD_LABEL,
};
use crate::vm::heap::{
    Heap,
    HeapOperationResult,
};
use crate::vm::error::Error;

const MIN_NODE_SIZE: usize = 4;

pub fn make_node(heap: &mut Heap, max_stack_items: usize) -> HeapOperationResult<usize> {
    let node = heap.new_node(NodeType::Stack, MIN_NODE_SIZE + max_stack_items)?;
    heap.update(node, FIRST_CHILD_LABEL.into(), 4)?;
    heap.update(node, LAST_CHILD_LABEL.into(), 3)?;
    Ok(node)
}

pub fn push(heap: &mut Heap, stack_node: usize, value: isize) -> HeapOperationResult<()> {
    let top_of_stack_addr = heap.deref(stack_node, LAST_CHILD_LABEL.into())?;
    match top_of_stack_addr {
        Some(addr) => {
            let new_top_of_stack_addr = addr + 1;
            heap.update(stack_node, LAST_CHILD_LABEL.into(), new_top_of_stack_addr)?;
            heap.update(stack_node, new_top_of_stack_addr.try_into().unwrap(), value)
        },
        None => Err(Error {
            message: String::from("Failed to push value onto stack, address of the top of the stack doesn't exist"),
        }),
    }
}

pub fn pop(heap: &mut Heap, stack_node: usize) -> HeapOperationResult<isize> {
    let top_of_stack_addr = heap.deref(stack_node, LAST_CHILD_LABEL.into())?;
    match top_of_stack_addr {
        Some(addr) => {
            heap.update(stack_node, LAST_CHILD_LABEL.into(), addr - 1)?;
            let popped_value = heap.deref(stack_node, addr.try_into().unwrap())?;
            match popped_value {
                Some(value) => Ok(value),
                None => Err(Error {
                    message: String::from("Cannot pop from an empty stack"),
                }),
            }
        },
        None => Err(Error {
            message: String::from("Failed to pop value from stack, address of the top of the stack doesn't exist"),
        }),
    }
}