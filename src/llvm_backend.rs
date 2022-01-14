use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, IntValue as LlvmIntValue, PointerValue};
use inkwell::types::{FunctionType, IntType as LlvmIntType, PointerType, StructType, VoidType};
use crate::types::{FullSizeGeneralPurposeRegister, IntType, Register};
use crate::backend::IntValue;

pub struct LlvmBuilder<'ctx, 'a> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    function: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
    types: Types<'ctx>,
    ctx_ptr: PointerValue<'ctx>,
}

#[derive(Clone, Copy)]
pub struct Types<'ctx> {
    void: VoidType<'ctx>,
    i8: LlvmIntType<'ctx>,
    i16: LlvmIntType<'ctx>,
    i32: LlvmIntType<'ctx>,
    i64: LlvmIntType<'ctx>,
    ctx: StructType<'ctx>,
    ctx_ptr: PointerType<'ctx>,
    bb_fn: FunctionType<'ctx>,
}

impl<'ctx> Types<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let void = context.void_type();

        let i8 = context.i8_type();
        let i16 = context.i16_type();
        let i32 = context.i32_type();
        let i64 = context.i64_type();

        let ctx = context.opaque_struct_type("context");
        ctx.set_body(&[
            i32.array_type(8).into() // general-purpose registers
        ], false);
        let ctx_ptr = ctx.ptr_type(AddressSpace::Generic);

        let bb_fn = void.fn_type(&[
            ctx_ptr.into()
        ], false);

        Self {
            void,
            i8,
            i16,
            i32,
            i64,
            ctx,
            ctx_ptr,
            bb_fn,
        }
    }
}

impl<'ctx, 'a> LlvmBuilder<'ctx, 'a> {
    pub fn new(context: &'ctx Context, module: &'a Module<'ctx>, types: Types<'ctx>, fn_name: &str) -> Self {

        let function = module.add_function(fn_name, types.bb_fn, None);
        let bb = context.append_basic_block(function, "entry");

        let builder = context.create_builder();
        builder.position_at_end(bb);

        let ctx_ptr = function.get_nth_param(0).unwrap().into_pointer_value();

        Self {
            context,
            module,
            function,
            builder,
            types,
            ctx_ptr,
        }
    }

    pub fn get_builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    fn build_ctx_gp_gep(&mut self, ctx_ptr: PointerValue<'ctx>, reg: FullSizeGeneralPurposeRegister) -> PointerValue<'ctx> {
        // hopefully this is safe...
        // TODO: cache the pointers at (generated) function level
        let i32_type = self.context.i32_type();
        unsafe {
            self.builder.build_gep(ctx_ptr,
                                      &[
                                          i32_type.const_zero(), // deref the pointer itself
                                          i32_type.const_zero(), // select the gep array
                                          i32_type.const_int(reg as u64, false) // then select the concrete register
                                      ],
                                      &*(reg.to_string() + "_ptr"))
        }
    }

    fn int_type(&self, ty: IntType) -> LlvmIntType<'ctx> {
        match ty {
            IntType::I8 => self.types.i8,
            IntType::I16 => self.types.i16,
            IntType::I32 => self.types.i32,
            IntType::I64 => self.types.i64,
        }
    }
}

impl IntValue for LlvmIntValue<'_> {
    fn size(&self) -> IntType {
        use IntType::*;
        match self.get_type().get_bit_width() {
            8 => I8,
            16 => I16,
            32 => I32,
            64 => I64,
            _ => unreachable!(),
        }
    }
}

impl<'ctx, 'a> crate::backend::Builder for LlvmBuilder<'ctx, 'a> {
    type IntValue = LlvmIntValue<'ctx>;

    fn make_int_value(&self, ty: IntType, value: u64, sign_extend: bool) -> Self::IntValue {
        self.int_type(ty).const_int(value, sign_extend)
    }

    fn load_register(&mut self, register: Register) -> Self::IntValue {
        if let Ok(gp) = FullSizeGeneralPurposeRegister::try_from(register) {
            let ptr = self.build_ctx_gp_gep(self.ctx_ptr, gp);
            self.builder.build_load(ptr, &*gp.to_string()).into_int_value()
        } else {
            todo!()
        }
    }

    fn store_register(&mut self, register: Register, value: Self::IntValue) {
        if let Ok(gp) = FullSizeGeneralPurposeRegister::try_from(register) {
            let ptr = self.build_ctx_gp_gep(self.ctx_ptr, gp);
            self.builder.build_store(ptr, value);
        } else {
            todo!()
        }
    }

    fn load_memory(&mut self, size: IntType, address: Self::IntValue) -> Self::IntValue {
        // TODO: actually implement memory fetching from the target address space, not the host space (as it is done now)
        let hptr = self.builder.build_int_to_ptr(address,
                                                 self.int_type(size).ptr_type(AddressSpace::Generic), "hptr");
        self.builder.build_load(hptr, "").into_int_value()
    }

    fn store_memory(&mut self, size: IntType, address: Self::IntValue, value: Self::IntValue) {
        todo!()
    }

    fn add(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_add(lhs, rhs, "")
    }

    fn neg(&mut self, val: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_neg(val, "")
    }

    fn sub(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_sub(lhs, rhs, "")
    }

    fn mul(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_mul(lhs, rhs, "")
    }

    fn xor(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_xor(lhs, rhs, "")
    }

    fn or(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_or(lhs, rhs, "")
    }

    fn shl(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_left_shift(lhs, rhs, "")
    }

    fn lshr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn ashr(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        todo!()
    }

    fn udiv(&mut self, lhs: Self::IntValue, rhs: Self::IntValue) -> Self::IntValue {
        self.builder.build_int_unsigned_div(lhs, rhs, "")
    }

    fn zext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        self.builder.build_int_z_extend(val, self.int_type(to), "")
    }

    fn sext(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        self.builder.build_int_s_extend(val, self.int_type(to), "")
    }

    fn trunc(&mut self, val: Self::IntValue, to: IntType) -> Self::IntValue {
        self.builder.build_int_truncate(val, self.int_type(to), "")
    }
}