#[allow(dead_code)]
pub mod registers;
pub mod heap;
pub mod state;
pub mod error;
pub mod nodes;

use crate::compiler::instructions::Instruction;
use error::Error;
use heap::Heap;
use state::{VirtualMachineState, ExitState};
use nodes::{
    bool,
    int64,
    unit,
    stack,
    closure,
    environment,
    rts_frame,
};

static DEFAULT_OPERAND_STACK_SIZE: u8 = 5;

type Result<T> = std::result::Result<T, Error>;
type ExecutionResult = Result<String>;

pub fn execute(bytecode: &Vec<Instruction>, heap_size: usize) -> ExecutionResult {
    let mut state = VirtualMachineState::new(Heap::new(heap_size));
    while state.is_running {
        let pc: usize = state.registers.program_counter.try_into().unwrap();
        bytecode[pc].execute(&mut state);
    }

    match state.exit_state {
        ExitState::Normal => Ok(String::from("Execution complete")),
        ExitState::DivisionError => Err(Error {
            message: String::from("Execution aborted"),
        }),
        ExitState::OutOfMemoryError => Err(Error {
            message: String::from("Memory exhausted"),
        }),
        ExitState::HeapError(message) => Err(Error {
            message,
        })
    }
}

trait Execute {
    fn execute(&self, state: &mut VirtualMachineState);
}

fn handle_heap_error(state: &mut VirtualMachineState, err: Error) {
    state.is_running = false;
    state.exit_state = ExitState::HeapError(err.message);
}

fn exec_bool_binary_op(state: &mut VirtualMachineState, op: fn(bool, bool) -> bool) {
    let mut pop_result = stack::pop(&mut state.heap, state.registers.operand_stack.try_into().unwrap());
    if pop_result.is_err() {
        return handle_heap_error(state, pop_result.unwrap_err())
    }

    let second_arg = bool::get_value(&mut state.heap, pop_result.unwrap().try_into().unwrap());
    if second_arg.is_err() {
        return handle_heap_error(state, second_arg.unwrap_err())
    }

    pop_result = stack::pop(&mut state.heap, state.registers.operand_stack.try_into().unwrap());
    if pop_result.is_err() {
        return handle_heap_error(state, pop_result.unwrap_err())
    }

    let first_arg = bool::get_value(&mut state.heap, pop_result.unwrap().try_into().unwrap());
    if first_arg.is_err() {
        return handle_heap_error(state, first_arg.unwrap_err())
    }

    let result_addr = bool::make_node(&mut state.heap, op(first_arg.unwrap(), second_arg.unwrap()));
    if result_addr.is_err() {
        return handle_heap_error(state, result_addr.unwrap_err())
    }

    match stack::push(
        &mut state.heap,
        state.registers.operand_stack.try_into().unwrap(),
        result_addr.unwrap().try_into().unwrap()
    ) {
        Ok(_) => state.registers.program_counter += 1,
        Err(err) => handle_heap_error(state, err),
    }
}

fn exec_int64_binary_op(state: &mut VirtualMachineState, op: fn(i64, i64) -> i64) {
    let mut pop_result = stack::pop(&mut state.heap, state.registers.operand_stack.try_into().unwrap());
    if pop_result.is_err() {
        return handle_heap_error(state, pop_result.unwrap_err())
    }

    let second_arg = int64::get_value(&mut state.heap, pop_result.unwrap().try_into().unwrap());
    if second_arg.is_err() {
        return handle_heap_error(state, second_arg.unwrap_err())
    }

    pop_result = stack::pop(&mut state.heap, state.registers.operand_stack.try_into().unwrap());
    if pop_result.is_err() {
        return handle_heap_error(state, pop_result.unwrap_err())
    }

    let first_arg = int64::get_value(&mut state.heap, pop_result.unwrap().try_into().unwrap());
    if first_arg.is_err() {
        return handle_heap_error(state, first_arg.unwrap_err())
    }

    let result_addr = int64::make_node(&mut state.heap, op(first_arg.unwrap(), second_arg.unwrap()));
    if result_addr.is_err() {
        return handle_heap_error(state, result_addr.unwrap_err())
    }

    match stack::push(
        &mut state.heap,
        state.registers.operand_stack.try_into().unwrap(),
        result_addr.unwrap().try_into().unwrap()
    ) {
        Ok(_) => state.registers.program_counter += 1,
        Err(err) => handle_heap_error(state, err),
    }
}

impl Execute for Instruction {
    fn execute(&self, state: &mut VirtualMachineState) {
        match self {
            Instruction::START =>
                match stack::make_node(&mut state.heap, DEFAULT_OPERAND_STACK_SIZE.into()) {
                    Ok(addr) => {
                        state.registers.operand_stack = addr.try_into().unwrap();
                        match environment::make_node(&mut state.heap, 0) {
                            Ok(addr) => {
                                state.registers.environment = addr.try_into().unwrap();
                                state.registers.program_counter += 1;
                            },
                            Err(err) => handle_heap_error(state, err),
                        }
                    },
                    Err(err) => handle_heap_error(state, err), 
                },
            Instruction::LDCI(value) =>
                match int64::make_node(&mut state.heap, value.clone()) {
                    Ok(addr) => match stack::push(
                        &mut state.heap,
                        state.registers.operand_stack.try_into().unwrap(),
                        addr.try_into().unwrap()
                    ) {
                        Ok(_) => state.registers.program_counter += 1,
                        Err(err) => handle_heap_error(state, err),
                    },
                    Err(err) => handle_heap_error(state, err),
                },
            Instruction::LDCB(value) =>
                match bool::make_node(&mut state.heap, value.clone()) {
                    Ok(addr) => match stack::push(
                        &mut state.heap,
                        state.registers.operand_stack.try_into().unwrap(),
                        addr.try_into().unwrap()
                    ) {
                        Ok(_) => state.registers.program_counter += 1,
                        Err(err) => handle_heap_error(state, err),
                    },
                    Err(err) => handle_heap_error(state, err),
                },
            Instruction::LDCU =>
                match unit::make_node(&mut state.heap) {
                    Ok(addr) => match stack::push(
                        &mut state.heap,
                        state.registers.operand_stack.try_into().unwrap(),
                        addr.try_into().unwrap()
                    ) {
                        Ok(_) => state.registers.program_counter += 1,
                        Err(err) => handle_heap_error(state, err),
                    },
                    Err(err) => handle_heap_error(state, err),
                },
            Instruction::LD(index) =>
                match environment::get_at_index(&mut state.heap, state.registers.environment.try_into().unwrap(), index.clone()) {
                    Ok(addr) => match stack::push(&mut state.heap, state.registers.operand_stack.try_into().unwrap(), addr) {
                        Ok(_) => state.registers.program_counter += 1,
                        Err(err) => handle_heap_error(state, err),
                    },
                    Err(err) => handle_heap_error(state, err),
                },
            // Binary operations.
            Instruction::PLUS => exec_int64_binary_op(state, |x, y| x + y),
            Instruction::MINUS => exec_int64_binary_op(state, |x, y| x - y),
            Instruction::TIMES => exec_int64_binary_op(state, |x, y| x * y),
            Instruction::DIV => unimplemented!(),
            Instruction::EQUAL => unimplemented!(),
            Instruction::GREATER => unimplemented!(),
            Instruction::GEQ => unimplemented!(),
            Instruction::LESS => unimplemented!(),
            Instruction::LEQ => unimplemented!(),
            Instruction::AND => exec_bool_binary_op(state, |x, y| x && y),
            Instruction::OR => exec_bool_binary_op(state, |x, y| x || y),
            // Unary operations.
            Instruction::NOT => {
                let pop_result = stack::pop(&mut state.heap, state.registers.operand_stack.try_into().unwrap());
                if pop_result.is_err() {
                    return handle_heap_error(state, pop_result.unwrap_err())
                }

                let popped_value = bool::get_value(&mut state.heap, pop_result.unwrap().try_into().unwrap());
                if popped_value.is_err() {
                    return handle_heap_error(state, popped_value.unwrap_err())
                }

                let result_addr = bool::make_node(&mut state.heap, !popped_value.unwrap());
                if result_addr.is_err() {
                    return handle_heap_error(state, result_addr.unwrap_err())
                }

                match stack::push(
                    &mut state.heap,
                    state.registers.operand_stack.try_into().unwrap(),
                    result_addr.unwrap().try_into().unwrap()
                ) {
                    Ok(_) => state.registers.program_counter += 1,
                    Err(err) => handle_heap_error(state, err),
                }
            },
            Instruction::UMINUS => {
                let pop_result = stack::pop(&mut state.heap, state.registers.operand_stack.try_into().unwrap());
                if pop_result.is_err() {
                    return handle_heap_error(state, pop_result.unwrap_err())
                }

                let popped_value = int64::get_value(&mut state.heap, pop_result.unwrap().try_into().unwrap());
                if popped_value.is_err() {
                    return handle_heap_error(state, popped_value.unwrap_err())
                }

                let result_addr = int64::make_node(&mut state.heap, -popped_value.unwrap());
                if result_addr.is_err() {
                    return handle_heap_error(state, result_addr.unwrap_err())
                }

                match stack::push(
                    &mut state.heap,
                    state.registers.operand_stack.try_into().unwrap(),
                    result_addr.unwrap().try_into().unwrap()
                ) {
                    Ok(_) => state.registers.program_counter += 1,
                    Err(err) => handle_heap_error(state, err),
                }
            },
            // Others.
            Instruction::POP => match stack::pop(&mut state.heap, state.registers.operand_stack.try_into().unwrap()) {
                Ok(_) => state.registers.program_counter += 1,
                Err(err) => handle_heap_error(state, err),
            },
            Instruction::GOTOR(jump_by) => unimplemented!(),
            Instruction::ASSIGN(index) => unimplemented!(),
            Instruction::LDF(max_stack_size, body_offset, scope_count) => unimplemented!(),
            Instruction::CALL(index) => unimplemented!(),
            Instruction::RTN => unimplemented!(),
            Instruction::DONE => state.is_running = false,
        }
    }
}
