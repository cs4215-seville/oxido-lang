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

type Result<T> = std::result::Result<T, Error>;
type ExecutionResult = Result<String>;

pub fn execute(bytecode: &Vec<Instruction>, heap_size: usize) -> ExecutionResult {
    let mut state = VirtualMachineState::new(Heap::new(heap_size));
    while state.is_running {
        match bytecode.get(state.registers.program_counter) {
            Some(instruction) => instruction.execute(&mut state),
            None => panic!("Something is wrong with the VM implementation"),
        };
    }

    match state.exit_state {
        ExitState::Normal => unimplemented!(),
        ExitState::DivisionError => Err(Error {
            message: String::from("Execution aborted"),
        }),
        ExitState::OutOfMemoryError => Err(Error {
            message: String::from("Memory exhausted"),
        }),
    }
}

trait Execute {
    fn execute(&self, state: &mut VirtualMachineState);
}

impl Execute for Instruction {
    fn execute(&self, state: &mut VirtualMachineState) {
        match self {
            Instruction::START => unimplemented!(),
            Instruction::LDCI(value) => unimplemented!(),
            Instruction::LDCB(value) => unimplemented!(),
            Instruction::LDCU => unimplemented!(),
            Instruction::LD(index) => unimplemented!(),
            // Binary operations.
            Instruction::PLUS => unimplemented!(),
            Instruction::MINUS => unimplemented!(),
            Instruction::TIMES => unimplemented!(),
            Instruction::DIV => unimplemented!(),
            Instruction::EQUAL => unimplemented!(),
            Instruction::GREATER => unimplemented!(),
            Instruction::GEQ => unimplemented!(),
            Instruction::LESS => unimplemented!(),
            Instruction::LEQ => unimplemented!(),
            Instruction::AND => unimplemented!(),
            Instruction::OR => unimplemented!(),
            // Unary operations.
            Instruction::NOT => unimplemented!(),
            Instruction::UMINUS => unimplemented!(),
            // Others.
            Instruction::POP => unimplemented!(),
            Instruction::GOTOR(jump_by) => unimplemented!(),
            Instruction::ASSIGN(index) => unimplemented!(),
            Instruction::LDF(max_stack_size, body_offset, scope_count) => unimplemented!(),
            Instruction::CALL(index) => unimplemented!(),
            Instruction::RTN => unimplemented!(),
            Instruction::DONE => unimplemented!(),
        }
    }
}
