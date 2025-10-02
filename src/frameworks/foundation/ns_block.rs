use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::mem::ConstVoidPtr;
use crate::objc::{objc_classes, ClassExports};
use crate::Environment;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSBlock: NSObject

- (id)retain { this }
- (())release {}
- (id)autorelease { this }
- (id)copy { this }

@end

@implementation __NSGlobalBlock__: NSBlock
@end

@implementation __NSStackBlock__: NSBlock
@end

@implementation __NSMallocBlock__: NSBlock
@end

@implementation __NSAutoBlock__: NSBlock
@end

};

fn ns_concrete_block(env: &mut Environment, name: &str) -> ConstVoidPtr {
    env.objc
        .get_known_class(name, &mut env.mem)
        .cast_void()
        .cast_const()
}

fn __NSConcreteGlobalBlock(env: &mut Environment) -> ConstVoidPtr {
    ns_concrete_block(env, "__NSGlobalBlock__")
}

fn __NSConcreteStackBlock(env: &mut Environment) -> ConstVoidPtr {
    ns_concrete_block(env, "__NSStackBlock__")
}

fn __NSConcreteMallocBlock(env: &mut Environment) -> ConstVoidPtr {
    ns_concrete_block(env, "__NSMallocBlock__")
}

fn __NSConcreteAutoBlock(env: &mut Environment) -> ConstVoidPtr {
    ns_concrete_block(env, "__NSAutoBlock__")
}

pub const CONSTANTS: ConstantExports = &[
    (
        "__NSConcreteGlobalBlock",
        HostConstant::Custom(__NSConcreteGlobalBlock),
    ),
    (
        "__NSConcreteStackBlock",
        HostConstant::Custom(__NSConcreteStackBlock),
    ),
    (
        "__NSConcreteMallocBlock",
        HostConstant::Custom(__NSConcreteMallocBlock),
    ),
    (
        "__NSConcreteAutoBlock",
        HostConstant::Custom(__NSConcreteAutoBlock),
    ),
];

fn _Block_copy(_: &mut Environment, block: ConstVoidPtr) -> ConstVoidPtr {
    block
}

fn _Block_release(_: &mut Environment, _: ConstVoidPtr) {}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(_Block_copy(_)),
    export_c_func!(_Block_release(_)),
];
