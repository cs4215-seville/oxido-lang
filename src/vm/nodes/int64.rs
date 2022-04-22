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

const NODE_SIZE: u8 = 5;
const VALUE_LABEL: u8 = 4;

pub fn make_node(heap: &mut Heap, value: i64) -> HeapOperationResult<usize> {
    let node = heap.new_node(NodeType::Int64, NODE_SIZE.into())?;
    heap.update(node, FIRST_CHILD_LABEL.into(), 6)?;
    heap.update(node, LAST_CHILD_LABEL.into(), 5)?;
    heap.update(node, VALUE_LABEL.into(), value.try_into().unwrap())?;
    Ok(node)
}

pub fn get_value(heap: &Heap, int64_node: usize) -> HeapOperationResult<i64> {
    let value = heap.deref(int64_node, VALUE_LABEL.into())?;
    match value {
        Some(value) => match i64::try_from(value) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error {
                message: String::from("Given node has no int64 value"),
            }),
        },
        None => Err(Error {
            message: String::from("Given node has no int64 value"),
        }),
    }
}