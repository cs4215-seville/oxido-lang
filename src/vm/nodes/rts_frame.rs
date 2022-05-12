use crate::vm::nodes::{
    NodeType,
    FIRST_CHILD_LABEL,
    LAST_CHILD_LABEL,
};
use crate::vm::heap::{
    Heap,
    HeapOperationResult,
};

const NODE_SIZE: u8 = 7;
const PC_LABEL: u8 = 4;
const ENV_LABEL: u8 = 5;
const OS_LABEL: u8 = 6;

pub fn make_node(heap: &mut Heap, program_counter: usize, env_node: usize, operand_stack_node: usize) -> HeapOperationResult<usize> {
    let node = heap.new_node(NodeType::RuntimeStackFrame, NODE_SIZE.into())?;

    heap.update(node, FIRST_CHILD_LABEL.into(), ENV_LABEL.into())?;
    heap.update(node, LAST_CHILD_LABEL.into(), OS_LABEL.into())?;
    heap.update(node, ENV_LABEL.into(), env_node.try_into().unwrap())?;
	// TODO: Check +2? Might need to change.
    heap.update(node, PC_LABEL.into(), (program_counter + 2).try_into().unwrap())?;
    heap.update(node, OS_LABEL.into(), operand_stack_node.try_into().unwrap())?;

    Ok(node)
}