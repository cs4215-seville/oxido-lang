use crate::vm::nodes::{
    NodeType,
    FIRST_CHILD_LABEL,
    LAST_CHILD_LABEL,
};
use crate::vm::heap::{
    Heap,
    HeapOperationResult,
};

const NODE_SIZE: u8 = 5;
const VALUE_LABEL: u8 = 4;

pub fn make_node(heap: &mut Heap, value: bool) -> HeapOperationResult<usize> {
    let node = heap.new_node(NodeType::Bool, NODE_SIZE.into())?;
    heap.update(node, FIRST_CHILD_LABEL.into(), 6)?;
    heap.update(node, LAST_CHILD_LABEL.into(), 5)?;
    heap.update(node, VALUE_LABEL.into(), value.into())?;
    Ok(node)
}