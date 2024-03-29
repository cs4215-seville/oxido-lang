use crate::vm::{
    registers::Registers,
    heap::Heap,
};

pub enum ExitState {
    Normal,
    DivisionError,
    OutOfMemoryError,
    HeapError(String),
}

pub struct VirtualMachineState {
    pub registers: Registers,
    pub runtime_stack: Vec<usize>,
    pub is_running: bool,
    pub exit_state: ExitState,
    pub heap: Heap,
}

impl VirtualMachineState {
    pub fn new(heap: Heap) -> VirtualMachineState {
        VirtualMachineState {
            registers: Registers::new(),
            runtime_stack: vec![],
            is_running: true,
            exit_state: ExitState::Normal,
            heap,
        }
    }
}