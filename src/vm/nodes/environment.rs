use crate::vm::nodes::{
    NodeType,
    SIZE_LABEL,
    FIRST_CHILD_LABEL,
    LAST_CHILD_LABEL,
};
use crate::vm::heap::{
    Heap,
    HeapOperationResult,
};
use crate::vm::error::Error;

const MIN_NODE_SIZE: usize = 4;

pub fn make_node(heap: &mut Heap, env_entries_count: usize) -> HeapOperationResult<usize> {
    let node = heap.new_node(NodeType::Environment, MIN_NODE_SIZE + env_entries_count)?;
    heap.update(node, FIRST_CHILD_LABEL.into(), MIN_NODE_SIZE.try_into().unwrap())?;
    heap.update(node, LAST_CHILD_LABEL.into(), (env_entries_count + MIN_NODE_SIZE - 1).try_into().unwrap())?;
    Ok(node)
}

pub fn extend(heap: &mut Heap, env_node: usize, extension_count: usize) -> HeapOperationResult<usize> {
    let env_node_size = heap.deref(env_node, SIZE_LABEL.into())?;
    if env_node_size.is_none() {
        return Err(Error {
            message: String::from("Unable to extend environment, the given environment node has no size"),
        })
    }
    let env_node_size: usize = env_node_size.unwrap().try_into().unwrap();
    let extended_env_entries_count = env_node_size - MIN_NODE_SIZE + extension_count;
    let extended_env_node = make_node(heap, extended_env_entries_count)?;

    let first_value_label = heap.deref(env_node, FIRST_CHILD_LABEL.into())?;
    if first_value_label.is_none() {
        return Err(Error {
            message: String::from("Unable to extend environment, the given environment node has no first child"),
        })
    }
    let first_value_label = first_value_label.unwrap();

    let last_value_label = heap.deref(env_node, LAST_CHILD_LABEL.into())?;
    if last_value_label.is_none() {
        return Err(Error {
            message: String::from("Unable to extend environment, the given environment node has no last child"),
        })
    }
    let last_value_label = last_value_label.unwrap();
    
    for value_label in first_value_label..last_value_label + 1 {
        let value = heap.deref(env_node, value_label.try_into().unwrap())?;
        if value.is_none() {
            return Err(Error {
                message: format!("Unable to extend environment, child {} is not found in the environment to extend from", value_label),
            })
        }
        heap.update(extended_env_node, value_label.try_into().unwrap(), value.unwrap())?;
    }

    Ok(extended_env_node)
}

pub fn get_at_index(heap: &Heap, env_node: usize, index: usize) -> HeapOperationResult<isize> {
    let first_child_addr = heap.deref(env_node, FIRST_CHILD_LABEL.into())?;
    if first_child_addr.is_none() {
        return Err(Error {
            message: format!("Unable to get from environment by index, the given environment has no first child"),
        })
    }
    let first_child_addr: usize = first_child_addr.unwrap().try_into().unwrap();

    let child_at_index = heap.deref(env_node, first_child_addr + index)?;
    if child_at_index.is_none() {
        return Err(Error {
            message: format!("Unable to get from environment by index, the given environment has no child with the given index"),
        })
    }

    Ok(child_at_index.unwrap())
}