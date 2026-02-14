// auto-generated: "lalrpop 0.20.2"
// sha3: 00bce022ba837b4828acb6ed5f0e397b6381eb735caafda4d446f6455858267a
use crate::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::never_loop, clippy::match_single_binding, clippy::needless_raw_string_hashes)]
mod __parse__Program {

    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Expr),
        Variant2(alloc::vec::Vec<Expr>),
        Variant3(Field),
        Variant4(alloc::vec::Vec<Field>),
        Variant5(String),
        Variant6(alloc::vec::Vec<String>),
        Variant7(Variant),
        Variant8(alloc::vec::Vec<Variant>),
        Variant9(AgentDef),
        Variant10(alloc::vec::Vec<AgentDef>),
        Variant11(Vec<Expr>),
        Variant12(Vec<Field>),
        Variant13(Vec<String>),
        Variant14(Vec<Variant>),
        Variant15(core::option::Option<Expr>),
        Variant16(core::option::Option<Field>),
        Variant17(Handler),
        Variant18(alloc::vec::Vec<Handler>),
        Variant19(core::option::Option<String>),
        Variant20(i64),
        Variant21(Program),
        Variant22(StateVar),
        Variant23(alloc::vec::Vec<StateVar>),
        Variant24(Stmt),
        Variant25(alloc::vec::Vec<Stmt>),
        Variant26(Type),
        Variant27(TypeDef),
        Variant28(alloc::vec::Vec<TypeDef>),
        Variant29(core::option::Option<Variant>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 44,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 44,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 44,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 44,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 44,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 44,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 68, 69, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 72, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 68, 69, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 76, 0, 0, 0,
        // State 19
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 44,
        // State 22
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 44,
        // State 24
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 25
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 26
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 27
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 106, 0, 0, 44,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 108, 0, 0, 44,
        // State 31
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 32
        23, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 33
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, 0, 85, 86, 44,
        // State 34
        0, 0, 0, 25, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44,
        // State 35
        23, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, -28, 85, 86, 44,
        // State 36
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 84, 0, 0, -26, 85, 86, 44,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0,
        // State 43
        -56, -56, -56, -56, -56, -56, 0, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, -56, -56, 0, 0, -56,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0, -87, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, -19,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, -65,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, -20,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, -66,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, -54, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0,
        // State 62
        0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9,
        // State 64
        0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0,
        // State 67
        0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -82, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -83, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0, 0, -55, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, -10,
        // State 74
        0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 25, 0, 26, 0, 0, 0, 0, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, -43, 27, -43, -43, -43, 0, 0, 28, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, -43,
        // State 78
        0, -80, -80, -80, -80, -80, 0, -80, -80, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -80, 0, 0, -80,
        // State 79
        0, -76, -76, -76, -76, -76, 0, -76, -76, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, -76,
        // State 80
        0, -77, -77, -77, -77, -77, 0, -77, -77, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -77, 0, 0, -77,
        // State 81
        0, -48, -48, -48, -48, -48, 0, 29, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, -48,
        // State 82
        0, -79, -79, -79, -79, -79, 0, -79, -79, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -79, 0, 0, -79,
        // State 83
        0, -78, -78, -78, -78, -78, 0, -78, -78, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -78, 0, 0, -78,
        // State 84
        0, -74, -74, -74, -74, -74, 0, -74, -74, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0, 0, -74,
        // State 85
        0, -59, -59, -59, -59, -59, 0, -59, -59, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, -59,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 94, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, -62,
        // State 90
        0, 101, 0, 25, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0,
        // State 92
        0, 0, 0, 0, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0,
        // State 93
        0, 0, 0, 0, 0, 0, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14,
        // State 95
        0, -41, 27, -41, -41, -41, 0, 0, 28, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, -41,
        // State 96
        0, -42, 27, -42, -42, -42, 0, 0, 28, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, -42,
        // State 97
        0, -46, -46, -46, -46, -46, 0, 29, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, -46,
        // State 98
        0, -47, -47, -47, -47, -47, 0, 29, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, -47,
        // State 99
        0, -75, -75, -75, -75, -75, 0, -75, -75, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, -75,
        // State 100
        0, -81, -81, -81, -81, -81, 0, -81, -81, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -81, 0, 0, -81,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, -15,
        // State 102
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0,
        // State 103
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, -72, 0, 0, -72,
        // State 105
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, -52, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, -73, 0, 0, -73,
        // State 107
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0, 0, -53, 0, 0, 0,
        // State 108
        0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        0, -25, 0, 25, 115, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0,
        // State 110
        0, 0, 0, 25, 0, 26, 0, 0, 0, 0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0,
        // State 112
        0, -27, 0, 25, 117, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0,
        // State 113
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 114
        -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, -4, 0, 0, -4, -4, -4, -4,
        // State 115
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, -67, 0, 0, -67,
        // State 116
        -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, -5, 0, 0, -5, -5, -5, -5,
        // State 117
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, -69, 0, 0, -69,
        // State 118
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 0, 0, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, -68, 0, 0, -68,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 30 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -60,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -61,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -23,
        // State 38
        -95,
        // State 39
        0,
        // State 40
        -24,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        -21,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        -22,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        0,
        // State 104
        0,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        0,
        // State 109
        0,
        // State 110
        0,
        // State 111
        0,
        // State 112
        0,
        // State 113
        0,
        // State 114
        0,
        // State 115
        0,
        // State 116
        0,
        // State 117
        0,
        // State 118
        0,
        // State 119
        0,
        // State 120
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 35,
            5 => 12,
            8 => 23,
            11 => 7,
            12 => match state {
                1 | 5 => 40,
                _ => 37,
            },
            13 => match state {
                2 => 5,
                _ => 1,
            },
            14 => match state {
                36 => 118,
                _ => 108,
            },
            15 => 56,
            16 => 87,
            17 => 47,
            18 => match state {
                31 => 34,
                19 => 76,
                22 => 90,
                33 => 110,
                35 => 112,
                _ => 109,
            },
            20 => match state {
                24 => 95,
                25 => 96,
                _ => 77,
            },
            21 => match state {
                12 => 61,
                _ => 57,
            },
            23 => match state {
                15 | 18 => 70,
                _ => 60,
            },
            24 => match state {
                14 => 18,
                _ => 15,
            },
            25 => match state {
                3 => 42,
                4 => 44,
                6..=7 => 48,
                8 | 10 => 53,
                9 | 12 => 58,
                13 | 17 => 64,
                16 => 72,
                20 => 86,
                21 => 88,
                23 => 92,
                28 => 99,
                29..=30 => 103,
                34 => 111,
                _ => 78,
            },
            27 => 79,
            28 => 38,
            29 => match state {
                10 => 59,
                _ => 54,
            },
            31 => 10,
            32 => match state {
                30 => 106,
                _ => 104,
            },
            34 => 30,
            35 => 80,
            36 => match state {
                26 => 97,
                27 => 98,
                _ => 81,
            },
            37 => match state {
                17 => 74,
                _ => 65,
            },
            38 => match state {
                2 => 41,
                _ => 39,
            },
            40 => 2,
            41 => match state {
                7 => 50,
                _ => 49,
            },
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###""*""###,
        r###""+""###,
        r###"",""###,
        r###""-""###,
        r###""->""###,
        r###"".""###,
        r###""/""###,
        r###"":""###,
        r###"";""###,
        r###""=""###,
        r###""Bool""###,
        r###""Int""###,
        r###""Ref""###,
        r###""String""###,
        r###""[""###,
        r###""]""###,
        r###""agent""###,
        r###""false""###,
        r###""on""###,
        r###""send""###,
        r###""state""###,
        r###""true""###,
        r###""type""###,
        r###""{""###,
        r###""}""###,
        r###"r#"\"[^\"]*\""#"###,
        r###"r#"[0-9]+"#"###,
        r###"r#"[a-zA-Z_][a-zA-Z0-9_]*"#"###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
    >(
        __states: &[i8],
        _: core::marker::PhantomData<(&'input ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Program;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 30 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(5, _) if true => Some(0),
            Token(6, _) if true => Some(1),
            Token(7, _) if true => Some(2),
            Token(8, _) if true => Some(3),
            Token(9, _) if true => Some(4),
            Token(10, _) if true => Some(5),
            Token(11, _) if true => Some(6),
            Token(12, _) if true => Some(7),
            Token(13, _) if true => Some(8),
            Token(14, _) if true => Some(9),
            Token(15, _) if true => Some(10),
            Token(16, _) if true => Some(11),
            Token(17, _) if true => Some(12),
            Token(18, _) if true => Some(13),
            Token(19, _) if true => Some(14),
            Token(20, _) if true => Some(15),
            Token(21, _) if true => Some(16),
            Token(22, _) if true => Some(17),
            Token(23, _) if true => Some(18),
            Token(24, _) if true => Some(19),
            Token(25, _) if true => Some(20),
            Token(26, _) if true => Some(21),
            Token(27, _) if true => Some(22),
            Token(28, _) if true => Some(23),
            Token(29, _) if true => Some(24),
            Token(30, _) if true => Some(25),
            Token(31, _) if true => Some(26),
            Token(0, _) if true => Some(27),
            Token(2, _) if true => Some(28),
            Token(3, _) if true => Some(29),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => match __token {
                Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(0, __tok0) | Token(2, __tok0) | Token(3, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 9,
                    nonterminal_produced: 12,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 13,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 14,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 14,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 15,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 16,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 17,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 20,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 21,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 22,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 23,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 9,
                    nonterminal_produced: 23,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 26,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 29,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 30,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 31,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 32,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 32,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 32,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 33,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 34,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 36,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 36,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 37,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 38,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 39,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 40,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 41,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 42,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 42,
                }
            }
            94 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl Default for ProgramParser { fn default() -> Self { Self::new() } }
    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Program, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                __reduce83(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            84 => {
                __reduce84(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            85 => {
                __reduce85(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            86 => {
                __reduce86(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            87 => {
                __reduce87(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            88 => {
                __reduce88(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            89 => {
                __reduce89(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            90 => {
                __reduce90(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            91 => {
                __reduce91(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            92 => {
                __reduce92(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            93 => {
                __reduce93(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            94 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant21(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AgentDef, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Handler, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StateVar, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Stmt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TypeDef, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Variant, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Variant>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<AgentDef>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Handler>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<StateVar>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Stmt>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<TypeDef>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant28(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Variant>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Field>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Variant>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant29(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action71::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* =  => ActionFn(69);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action69::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action80::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action81::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",") = Field, "," => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action57::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action55::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")* = (<Field> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")+ = Field, "," => ActionFn(84);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action84::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Field> ",")+ = (<Field> ",")+, Field, "," => ActionFn(85);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action85::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Ident> ",") = Ident, "," => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action64::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
    }
    fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Ident> ",")* =  => ActionFn(62);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action62::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Ident> ",")* = (<Ident> ",")+ => ActionFn(63);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action63::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Ident> ",")+ = Ident, "," => ActionFn(88);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action88::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Ident> ",")+ = (<Ident> ",")+, Ident, "," => ActionFn(89);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action89::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Variant> ",") = Variant, "," => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Variant> ",")* =  => ActionFn(50);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action50::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Variant> ",")* = (<Variant> ",")+ => ActionFn(51);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Variant> ",")+ = Variant, "," => ActionFn(92);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action92::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Variant> ",")+ = (<Variant> ",")+, Variant, "," => ActionFn(93);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action93::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 11)
    }
    fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AgentDef = "agent", Ident, "{", "state", "{", "}", Handler+, "}" => ActionFn(108);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant18(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym7.2;
        let __nt = super::__action108::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (8, 12)
    }
    fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AgentDef = "agent", Ident, "{", "state", "{", StateVar+, "}", Handler+, "}" => ActionFn(109);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant18(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant23(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym8.2;
        let __nt = super::__action109::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (9, 12)
    }
    fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AgentDef+ = AgentDef => ActionFn(42);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action42::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 13)
    }
    fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AgentDef+ = AgentDef+, AgentDef => ActionFn(43);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action43::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 13)
    }
    fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = Expr => ActionFn(96);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action96::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Expr> =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action97::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (0, 14)
    }
    fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(98);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action98::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (2, 14)
    }
    fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(99);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action99::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Field> = Field => ActionFn(100);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 15)
    }
    fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Field> =  => ActionFn(101);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action101::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 15)
    }
    fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Field> = (<Field> ",")+, Field => ActionFn(102);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action102::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (2, 15)
    }
    fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Field> = (<Field> ",")+ => ActionFn(103);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action103::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 15)
    }
    fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Ident> = Ident => ActionFn(104);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action104::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Ident> =  => ActionFn(105);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action105::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 16)
    }
    fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Ident> = (<Ident> ",")+, Ident => ActionFn(106);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action106::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 16)
    }
    fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Ident> = (<Ident> ",")+ => ActionFn(107);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action107::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Variant> = Variant => ActionFn(114);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action114::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 17)
    }
    fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Variant> =  => ActionFn(115);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action115::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 17)
    }
    fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Variant> = (<Variant> ",")+, Variant => ActionFn(116);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action116::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (2, 17)
    }
    fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Variant> = (<Variant> ",")+ => ActionFn(117);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action117::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 17)
    }
    fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "+", Factor => ActionFn(16);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 18)
    }
    fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "-", Factor => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 18)
    }
    fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 18)
    }
    fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr? = Expr => ActionFn(67);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action67::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr? =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action68::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Term => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 20)
    }
    fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Term => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 20)
    }
    fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field = Ident, ":", Type => ActionFn(4);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant26(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 21)
    }
    fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field? = Field => ActionFn(53);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 22)
    }
    fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Field? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action54::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 22)
    }
    fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Handler = "on", Ident, "{", Comma<Ident>, "}", "->", "{", "}" => ActionFn(110);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant13(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym7.2;
        let __nt = super::__action110::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (8, 23)
    }
    fn __reduce52<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Handler = "on", Ident, "{", Comma<Ident>, "}", "->", "{", Stmt+, "}" => ActionFn(111);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant25(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant13(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym8.2;
        let __nt = super::__action111::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (9, 23)
    }
    fn __reduce53<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Handler+ = Handler => ActionFn(36);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 24)
    }
    fn __reduce54<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Handler+ = Handler+, Handler => ActionFn(37);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant17(__symbols);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action37::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 24)
    }
    fn __reduce55<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action29::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 25)
    }
    fn __reduce56<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident? = Ident => ActionFn(60);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 26)
    }
    fn __reduce57<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident? =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (0, 26)
    }
    fn __reduce58<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[0-9]+"# => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 27)
    }
    fn __reduce59<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = AgentDef+ => ActionFn(112);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action112::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 28)
    }
    fn __reduce60<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDef+, AgentDef+ => ActionFn(113);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action113::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (2, 28)
    }
    fn __reduce61<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StateVar = Ident, ":", Type, "=", Expr, ";" => ActionFn(11);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant26(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (6, 29)
    }
    fn __reduce62<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StateVar* =  => ActionFn(38);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action38::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (0, 30)
    }
    fn __reduce63<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StateVar* = StateVar+ => ActionFn(39);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 30)
    }
    fn __reduce64<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StateVar+ = StateVar => ActionFn(58);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (1, 31)
    }
    fn __reduce65<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StateVar+ = StateVar+, StateVar => ActionFn(59);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant22(__symbols);
        let __sym0 = __pop_Variant23(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action59::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (2, 31)
    }
    fn __reduce66<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = Ident, "=", Expr, ";" => ActionFn(13);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (4, 32)
    }
    fn __reduce67<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = "send", Expr, Ident, "{", Comma<Expr>, "}", ";" => ActionFn(14);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant11(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (7, 32)
    }
    fn __reduce68<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt = Ident, "(", Comma<Expr>, ")", ";" => ActionFn(15);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (5, 32)
    }
    fn __reduce69<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt* =  => ActionFn(33);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action33::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 33)
    }
    fn __reduce70<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt* = Stmt+ => ActionFn(34);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 33)
    }
    fn __reduce71<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt+ = Stmt => ActionFn(65);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action65::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 34)
    }
    fn __reduce72<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Stmt+ = Stmt+, Stmt => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant24(__symbols);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (2, 34)
    }
    fn __reduce73<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Str = r#"\"[^\"]*\""# => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 35)
    }
    fn __reduce74<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", Ident => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 36)
    }
    fn __reduce75<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Num => ActionFn(23);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce76<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Str => ActionFn(24);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce77<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = "true" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce78<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = "false" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce79<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Ident => ActionFn(27);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 36)
    }
    fn __reduce80<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 36)
    }
    fn __reduce81<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = "Int" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 37)
    }
    fn __reduce82<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = "String" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 37)
    }
    fn __reduce83<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = "Bool" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 37)
    }
    fn __reduce84<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = "Ref", "[", Ident, "]" => ActionFn(8);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (4, 37)
    }
    fn __reduce85<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Type = Ident => ActionFn(9);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 37)
    }
    fn __reduce86<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDef = "type", Ident, "{", Comma<Variant>, "}" => ActionFn(2);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant14(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (5, 38)
    }
    fn __reduce87<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDef* =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action44::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (0, 39)
    }
    fn __reduce88<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDef* = TypeDef+ => ActionFn(45);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (1, 39)
    }
    fn __reduce89<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDef+ = TypeDef => ActionFn(46);
        let __sym0 = __pop_Variant27(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (1, 40)
    }
    fn __reduce90<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDef+ = TypeDef+, TypeDef => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant27(__symbols);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action47::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (2, 40)
    }
    fn __reduce91<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Variant = Ident, "{", Comma<Field>, "}" => ActionFn(3);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant12(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 41)
    }
    fn __reduce92<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Variant? = Variant => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 42)
    }
    fn __reduce93<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Variant? =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action49::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (0, 42)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Program::ProgramParser;
#[rustfmt::skip]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("(?:\"[\0-!\\#-\u{10ffff}]*\")", false),
            ("(?:(?://)[\0-\t\u{b}\u{c}\u{e}-\u{10ffff}]*[\n\r]*)", true),
            ("[0-9]+", false),
            ("(?:[A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("[\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}\u{2029}\u{202f}\u{205f}\u{3000}]*", true),
            ("\\(", false),
            ("\\)", false),
            ("\\*", false),
            ("\\+", false),
            (",", false),
            ("\\-", false),
            ("(?:\\->)", false),
            ("\\.", false),
            ("/", false),
            (":", false),
            (";", false),
            ("=", false),
            ("(?:Bool)", false),
            ("(?:Int)", false),
            ("(?:Ref)", false),
            ("(?:String)", false),
            ("\\[", false),
            ("\\]", false),
            ("(?:agent)", false),
            ("(?:false)", false),
            ("(?:on)", false),
            ("(?:send)", false),
            ("(?:state)", false),
            ("(?:true)", false),
            ("(?:type)", false),
            ("\\{", false),
            ("\\}", false),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, types, _): (usize, alloc::vec::Vec<TypeDef>, usize),
    (_, agents, _): (usize, alloc::vec::Vec<AgentDef>, usize),
) -> Program
{
    Program { types, agents }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, variants, _): (usize, Vec<Variant>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> TypeDef
{
    TypeDef { name, variants }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, fields, _): (usize, Vec<Field>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Variant
{
    Variant { name, fields }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, Type, usize),
) -> Field
{
    Field { name, ty }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::Int
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::String
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Type
{
    Type::Bool
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Type
{
    Type::Ref(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Type
{
    Type::Named(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, state, _): (usize, alloc::vec::Vec<StateVar>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, handlers, _): (usize, alloc::vec::Vec<Handler>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> AgentDef
{
    AgentDef { name, state, handlers }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, Type, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, init, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> StateVar
{
    StateVar { name, ty, init }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, variant, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, params, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, alloc::vec::Vec<Stmt>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Handler
{
    Handler { variant, params, body }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, target, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, value, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Stmt
{
    Stmt::Assign { target, value }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, target, _): (usize, Expr, usize),
    (_, msg_variant, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Stmt
{
    Stmt::Send { target, msg_variant, args }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, Vec<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Stmt
{
    Stmt::Effect { name, args }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp { op: BinOp::Add, left: Box::new(l), right: Box::new(r) }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp { op: BinOp::Sub, left: Box::new(l), right: Box::new(r) }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp { op: BinOp::Mul, left: Box::new(l), right: Box::new(r) }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::BinOp { op: BinOp::Div, left: Box::new(l), right: Box::new(r) }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, obj, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, field, _): (usize, String, usize),
) -> Expr
{
    Expr::FieldAccess { obj: Box::new(obj), field }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i64, usize),
) -> Expr
{
    Expr::Int(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Expr
{
    Expr::Str(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expr
{
    Expr::Bool(true)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expr
{
    Expr::Bool(false)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Expr
{
    Expr::Var(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i64
{
    __0.parse().unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    {
    let s = __0;
    s[1..s.len()-1].to_string()
}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, e, _): (usize, core::option::Option<Expr>, usize),
) -> Vec<Expr>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action33<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Stmt>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Stmt>, usize),
) -> alloc::vec::Vec<Stmt>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, core::option::Option<String>, usize),
) -> Vec<String>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Handler, usize),
) -> alloc::vec::Vec<Handler>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Handler>, usize),
    (_, e, _): (usize, Handler, usize),
) -> alloc::vec::Vec<Handler>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action38<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<StateVar>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<StateVar>, usize),
) -> alloc::vec::Vec<StateVar>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<Field>, usize),
    (_, e, _): (usize, core::option::Option<Field>, usize),
) -> Vec<Field>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<Variant>, usize),
    (_, e, _): (usize, core::option::Option<Variant>, usize),
) -> Vec<Variant>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, AgentDef, usize),
) -> alloc::vec::Vec<AgentDef>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<AgentDef>, usize),
    (_, e, _): (usize, AgentDef, usize),
) -> alloc::vec::Vec<AgentDef>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action44<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<TypeDef>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action45<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<TypeDef>, usize),
) -> alloc::vec::Vec<TypeDef>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action46<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TypeDef, usize),
) -> alloc::vec::Vec<TypeDef>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action47<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<TypeDef>, usize),
    (_, e, _): (usize, TypeDef, usize),
) -> alloc::vec::Vec<TypeDef>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action48<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Variant, usize),
) -> core::option::Option<Variant>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action49<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Variant>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action50<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Variant>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action51<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Variant>, usize),
) -> alloc::vec::Vec<Variant>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action52<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Variant, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Variant
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action53<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> core::option::Option<Field>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action54<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Field>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action55<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Field>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action56<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Field>, usize),
) -> alloc::vec::Vec<Field>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action57<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Field
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action58<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, StateVar, usize),
) -> alloc::vec::Vec<StateVar>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action59<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<StateVar>, usize),
    (_, e, _): (usize, StateVar, usize),
) -> alloc::vec::Vec<StateVar>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action60<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> core::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action61<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<String>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action62<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<String>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action63<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
) -> alloc::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action64<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action65<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Stmt, usize),
) -> alloc::vec::Vec<Stmt>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action66<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Stmt>, usize),
    (_, e, _): (usize, Stmt, usize),
) -> alloc::vec::Vec<Stmt>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action67<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> core::option::Option<Expr>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action68<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Expr>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action69<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Expr>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action70<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr>, usize),
) -> alloc::vec::Vec<Expr>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action71<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action72<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> alloc::vec::Vec<Expr>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action73<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> alloc::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action74<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action75<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> alloc::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action76<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> alloc::vec::Vec<Field>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action77<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Field>, usize),
    (_, e, _): (usize, Field, usize),
) -> alloc::vec::Vec<Field>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action78<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Variant, usize),
) -> alloc::vec::Vec<Variant>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action79<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Variant>, usize),
    (_, e, _): (usize, Variant, usize),
) -> alloc::vec::Vec<Variant>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action80<
    'input,
>(
    input: &'input str,
    __0: (usize, Expr, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action71(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action81<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, Expr, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action71(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action82<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action69(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action83<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, core::option::Option<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action70(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action84<
    'input,
>(
    input: &'input str,
    __0: (usize, Field, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Field>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action57(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action85<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Field>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action57(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action86<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action55(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action87<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
    __1: (usize, core::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action56(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action88<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action64(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action89<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<String>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action64(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action90<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action62(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action91<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, core::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action63(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action92<
    'input,
>(
    input: &'input str,
    __0: (usize, Variant, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Variant>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action52(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action93<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Variant>, usize),
    __1: (usize, Variant, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Variant>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action52(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action94<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<Variant>, usize),
) -> Vec<Variant>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action50(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action95<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Variant>, usize),
    __1: (usize, core::option::Option<Variant>, usize),
) -> Vec<Variant>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action51(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action96<
    'input,
>(
    input: &'input str,
    __0: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action67(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action97<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Expr>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action68(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action98<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr>, usize),
    __1: (usize, Expr, usize),
) -> Vec<Expr>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action67(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action99<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Expr>, usize),
) -> Vec<Expr>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action68(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action83(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action100<
    'input,
>(
    input: &'input str,
    __0: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action53(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action101<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Field>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action54(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action102<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action53(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action103<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action54(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action104<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action60(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action105<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action106<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
    __1: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action60(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action107<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action61(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action108<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, alloc::vec::Vec<Handler>, usize),
    __7: (usize, &'input str, usize),
) -> AgentDef
{
    let __start0 = __4.2;
    let __end0 = __5.0;
    let __temp0 = __action38(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action109<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, alloc::vec::Vec<StateVar>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, alloc::vec::Vec<Handler>, usize),
    __8: (usize, &'input str, usize),
) -> AgentDef
{
    let __start0 = __5.0;
    let __end0 = __5.2;
    let __temp0 = __action39(
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
        __8,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action110<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Vec<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
) -> Handler
{
    let __start0 = __6.2;
    let __end0 = __7.0;
    let __temp0 = __action33(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __7,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action111<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Vec<String>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, alloc::vec::Vec<Stmt>, usize),
    __8: (usize, &'input str, usize),
) -> Handler
{
    let __start0 = __7.0;
    let __end0 = __7.2;
    let __temp0 = __action34(
        input,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
        __8,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action112<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<AgentDef>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action113<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDef>, usize),
    __1: (usize, alloc::vec::Vec<AgentDef>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action45(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action114<
    'input,
>(
    input: &'input str,
    __0: (usize, Variant, usize),
) -> Vec<Variant>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action48(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action115<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Variant>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action49(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action116<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Variant>, usize),
    __1: (usize, Variant, usize),
) -> Vec<Variant>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action48(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action117<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Variant>, usize),
) -> Vec<Variant>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action49(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        input,
        __0,
        __temp0,
    )
}
#[allow(clippy::type_complexity, dead_code)]

pub  trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
