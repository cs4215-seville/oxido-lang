pub type Register = usize;

pub struct Registers {
    pub program_counter: Register,
    pub environment: Register,
    pub operand_stack: Register,
    pub result: Register,
    pub top_runtime_stack: Register,
    pub temp_root: Register,
    pub a: Register,
    pub b: Register,
    pub c: Register,
    pub d: Register,
    pub e: Register,
    pub f: Register,
    pub g: Register,
    pub h: Register,
    pub i: Register,
    pub j: Register,
    pub k: Register,
    pub l: Register,
    pub n: Register,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            program_counter: 0,
            environment: 0,
            operand_stack: 0,
            result: 0,
            top_runtime_stack: 0,
            temp_root: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            i: 0,
            j: 0,
            k: 0,
            l: 0,
            n: 0,
        }
    }
}