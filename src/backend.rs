use crate::ControlFlow;
use crate::types::{Flag, IntType, MemoryOperand, Operand, Register};

pub trait IntValue: Clone + Copy {
    fn size(&self) -> IntType;
}

pub trait BoolValue: Clone + Copy {

}

pub trait Builder {
    type IntValue: IntValue;
    type BoolValue: BoolValue;

    fn make_int_value(&self, ty: IntType, value: u64, sign_extend: bool) -> Self::IntValue;

    // TODO: implement all the variants with all the sizes
    fn make_u8(&mut self, value: u8) -> Self::IntValue {
        self.make_int_value(IntType::I8, value as u64, false)
    }
    fn make_u16(&mut self, value: u16) -> Self::IntValue {
        self.make_int_value(IntType::I16, value as u64, false)
    }
    fn make_u32(&mut self, value: u32) -> Self::IntValue {
        self.make_int_value(IntType::I32, value as u64, false)
    }
    fn make_u64(&mut self, value: u64) -> Self::IntValue {
        self.make_int_value(IntType::I64, value as u64, false)
    }

    fn load_register(&mut self, register: Register) -> Self::IntValue;
    fn store_register(&mut self, register: Register, value: Self::IntValue);

    fn load_flag(&mut self, flag: Flag) -> Self::BoolValue;
    fn store_flag(&mut self, flag: Flag, value: Self::BoolValue);

    // TODO: not everything fits into IntType box... like 80-bit floats, for example.......
    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue;
    fn store_memory(&mut self, address: Self::IntValue, value: Self::IntValue);

    fn add(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn int_neg(&mut self, val: Self::IntValue) -> Self::IntValue;
    fn bool_neg(&mut self, val: Self::BoolValue) -> Self::BoolValue;
    fn sub(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn mul(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn xor(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn or(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn shl(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn lshr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn ashr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;
    fn udiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue;

    fn zext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue;
    fn sext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue;
    fn trunc(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue;

    fn ifelse<L, R>(&mut self,
                    cond: Self::BoolValue,
                    iftrue: L,
                    iffalse: R)
        -> ControlFlow<Self>
    where
        L: FnOnce(&mut Self) -> ControlFlow<Self>,
        R: FnOnce(&mut Self) -> ControlFlow<Self>,
        Self: Sized;

    fn compute_memory_operand_address(&mut self, op: MemoryOperand) -> Self::IntValue {
        assert!(op.index.is_none());
        assert!(op.segment.is_none());

        let mut res = self.make_u32(i32::try_from(op.displacement).unwrap() as u32);

        if let Some(base) = op.base {
            let base_val = self.load_register(base);
            res = self.add(res, base_val);
        }

        res
    }

    fn load_operand(&mut self, operand: Operand) -> Self::IntValue {
        match operand {
            Operand::Register(reg) => self.load_register(reg),
            Operand::Immediate8(v) => self.make_u8(v),
            Operand::Immediate16(v) => self.make_u16(v),
            Operand::Immediate32(v) => self.make_u32(v),
            Operand::Immediate64(v) => self.make_u64(v),
            Operand::Memory(op) => {
                let addr = self.compute_memory_operand_address(op);
                self.load_memory(op.size.unwrap(), addr)
            }
            op => panic!("Unsupported load operand: {:?}", op),
        }
    }
    fn store_operand(&mut self, operand: Operand, value: Self::IntValue) {
        match operand {
            Operand::Register(reg) => self.store_register(reg, value),
            Operand::Memory(op) => {
                let addr = self.compute_memory_operand_address(op);
                assert_eq!(op.size.unwrap(), value.size());
                self.store_memory(addr, value)
            }
            op => panic!("Unsupported store operand: {:?}", op),
        }
    }

    // TODO: maybe (probably?) we will need a way to express branches here. Not the branch instructions, but conditional execution in the context of the instruction itself
}

// trait Backend {
//     type IntValue: IntValue;
//     type Builder: Builder<IntValue = Self::IntValue>;
//
//     // TODO: how do we make a builder? In LLVM it would need to create a basic block and stuff...
//     // leaving this kludge for now
//     fn make_builder(&mut self) -> Self::Builder; // TODO: lifetime?
// }
