use crate::vm::nodes::{
    NodeType,
    FIRST_CHILD_LABEL,
    LAST_CHILD_LABEL,
};
use crate::vm::heap::{
    Heap,
    HeapOperationResult,
};

const NODE_SIZE: u8 = 4;
const VALUE_LABEL: u8 = 4;

pub fn make_node(heap: &mut Heap) -> HeapOperationResult<usize> {
    let node = heap.new_node(NodeType::Unit, NODE_SIZE.into())?;
    heap.update(node, FIRST_CHILD_LABEL.into(), 5)?;
    heap.update(node, LAST_CHILD_LABEL.into(), 4)?;
    Ok(node)
}