use crate::vm::nodes::{
    NodeType,
    FIRST_CHILD_LABEL,
    LAST_CHILD_LABEL,
};
use crate::vm::heap::{
    Heap,
    HeapOperationResult,
};

const NODE_SIZE: u8 = 8;
pub const OS_SIZE_LABEL: u8 = 4;
pub const BODY_ADDRESS_LABEL: u8 = 5;
pub const ENV_LABEL: u8 = 6;
pub const ENV_EXTENSION_COUNT_LABEL: u8 = 7;

pub fn make_node(heap: &mut Heap, operand_stack_size: usize, body_address: usize, env_node: usize, env_ext_count: usize) -> HeapOperationResult<usize> {
    let node = heap.new_node(NodeType::Closure, NODE_SIZE.into())?;

    heap.update(node, FIRST_CHILD_LABEL.into(), ENV_LABEL.into())?;
    heap.update(node, LAST_CHILD_LABEL.into(), ENV_LABEL.into())?;
    heap.update(node, OS_SIZE_LABEL.into(), operand_stack_size.try_into().unwrap())?;
    heap.update(node, BODY_ADDRESS_LABEL.into(), body_address.try_into().unwrap())?;
    heap.update(node, ENV_LABEL.into(), env_node.try_into().unwrap())?;
    heap.update(node, ENV_EXTENSION_COUNT_LABEL.into(), env_ext_count.try_into().unwrap())?;

    Ok(node)
}

// TODO: SET_CLOSURE_ENV()