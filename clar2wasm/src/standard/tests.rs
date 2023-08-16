use std::borrow::BorrowMut;
use wasmtime::{Engine, Instance, Module, Store, Val};

#[test]
fn test_add_uint() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let add = instance.get_func(store.borrow_mut(), "add-uint").unwrap();
    let mut sum = [Val::I64(0), Val::I64(0)];

    // 0 + 0 = 0
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(0), Val::I64(0), Val::I64(0)],
        &mut sum,
    )
    .expect("call to add-uint failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(0));

    // 1 + 2 = 3
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(1), Val::I64(0), Val::I64(2)],
        &mut sum,
    )
    .expect("call to add-uint failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(3));

    // Carry
    // 0xffff_ffff_ffff_ffff + 1 = 0x1_0000_0000_0000_0000
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(-1), Val::I64(0), Val::I64(1)],
        &mut sum,
    )
    .expect("call to add-uint failed");
    assert_eq!(sum[0].i64(), Some(1));
    assert_eq!(sum[1].i64(), Some(0));

    // Overflow
    // 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff + 1 = Overflow
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(-1), Val::I64(-1), Val::I64(0), Val::I64(1)],
        &mut sum,
    )
    .expect_err("expected overflow");

    // Overflow
    // 1 + 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff = Overflow
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(-1), Val::I64(-1), Val::I64(0), Val::I64(1)],
        &mut sum,
    )
    .expect_err("expected overflow");
}

#[test]
fn test_add_int() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let add = instance.get_func(store.borrow_mut(), "add-int").unwrap();
    let mut sum = [Val::I64(0), Val::I64(0)];

    // 0 + 0 = 0
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(0), Val::I64(0), Val::I64(0)],
        &mut sum,
    )
    .expect("call to add-int failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(0));

    // 1 + 2 = 3
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(1), Val::I64(0), Val::I64(2)],
        &mut sum,
    )
    .expect("call to add-int failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(3));

    // Carry
    // 0xffff_ffff_ffff_ffff + 1 = 0x1_0000_0000_0000_0000
    add.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(-1), Val::I64(0), Val::I64(1)],
        &mut sum,
    )
    .expect("call to add-int failed");
    assert_eq!(sum[0].i64(), Some(1));
    assert_eq!(sum[1].i64(), Some(0));

    // Overflow in signed 64-bit, but fine in 128-bit
    // 0x7fff_ffff_ffff_ffff + 0x7fff_ffff_ffff_ffff = 0xffff_ffff_ffff_fffe
    add.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0),
            Val::I64(0x7fff_ffff_ffff_ffff),
            Val::I64(0),
            Val::I64(0x7fff_ffff_ffff_ffff),
        ],
        &mut sum,
    )
    .expect("call to add-int failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(-2));

    // Overflow
    // 0x7fff_ffff_ffff_ffff_ffff_ffff_ffff_ffff + 1 = Overflow
    add.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0x7fff_ffff_ffff_ffff),
            Val::I64(-1),
            Val::I64(0),
            Val::I64(1),
        ],
        &mut sum,
    )
    .expect_err("expected overflow");

    // Overflow
    // 1 + 0x7fff_ffff_ffff_ffff_ffff_ffff_ffff_ffff = Overflow
    add.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0),
            Val::I64(1),
            Val::I64(0x7fff_ffff_ffff_ffff),
            Val::I64(-1),
        ],
        &mut sum,
    )
    .expect_err("expected overflow");

    // Overflow
    // 0x8000_0000_0000_0000_0000_0000_0000_0000 + -1 = Overflow
    add.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(-9223372036854775808),
            Val::I64(0),
            Val::I64(-1),
            Val::I64(-1),
        ],
        &mut sum,
    )
    .expect_err("expected overflow");
}

#[test]
fn test_sub_uint() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let sub = instance.get_func(store.borrow_mut(), "sub-uint").unwrap();
    let mut sum = [Val::I64(0), Val::I64(0)];

    // 0 - 0 = 0
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(0), Val::I64(0), Val::I64(0)],
        &mut sum,
    )
    .expect("call to sub-uint failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(0));

    // 3 - 2 = 1
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(3), Val::I64(0), Val::I64(2)],
        &mut sum,
    )
    .expect("call to sub-uint failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(1));

    // Borrow
    // 0x1_0000_0000_0000_0000 - 1 = 0xffff_ffff_ffff_ffff
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(1), Val::I64(0), Val::I64(0), Val::I64(1)],
        &mut sum,
    )
    .expect("call to sub-uint failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(-1));

    // Signed underflow, but fine for unsigned
    // 0x8000_0000_0000_0000_0000_0000_0000_0000 - 1 = 0x7fff_ffff_ffff_ffff_ffff_ffff_ffff_ffff
    sub.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(-9223372036854775808),
            Val::I64(0),
            Val::I64(0),
            Val::I64(1),
        ],
        &mut sum,
    )
    .expect("call to sub-uint failed");
    assert_eq!(sum[0].i64(), Some(0x7fff_ffff_ffff_ffff));
    assert_eq!(sum[1].i64(), Some(-1));

    // Underflow
    // 1 - 2 = Underflow
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(1), Val::I64(0), Val::I64(2)],
        &mut sum,
    )
    .expect_err("expected underflow");
}

#[test]
fn test_sub_int() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let sub = instance.get_func(store.borrow_mut(), "sub-int").unwrap();
    let mut sum = [Val::I64(0), Val::I64(0)];

    // 0 - 0 = 0
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(0), Val::I64(0), Val::I64(0)],
        &mut sum,
    )
    .expect("call to sub-int failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(0));

    // 3 - 2 = 1
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(3), Val::I64(0), Val::I64(2)],
        &mut sum,
    )
    .expect("call to sub-int failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(1));

    // 1 - 2 = -1
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(1), Val::I64(0), Val::I64(2)],
        &mut sum,
    )
    .expect("call to sub-int failed");
    assert_eq!(sum[0].i64(), Some(-1));
    assert_eq!(sum[1].i64(), Some(-1));

    // Borrow
    // 0x1_0000_0000_0000_0000 - 1 = 0xffff_ffff_ffff_ffff
    sub.call(
        &mut store.borrow_mut(),
        &[Val::I64(1), Val::I64(0), Val::I64(0), Val::I64(1)],
        &mut sum,
    )
    .expect("call to sub-int failed");
    assert_eq!(sum[0].i64(), Some(0));
    assert_eq!(sum[1].i64(), Some(-1));

    // Underflow
    // 0x8000_0000_0000_0000_0000_0000_0000_0000 - 1 = Underflow
    sub.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(-9223372036854775808),
            Val::I64(0),
            Val::I64(0),
            Val::I64(1),
        ],
        &mut sum,
    )
    .expect_err("expected underflow");
}

#[test]
fn test_mul_uint() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let mul = instance.get_func(store.borrow_mut(), "mul-uint").unwrap();
    let mut result = [Val::I64(0), Val::I64(0)];

    // 0 * 0 = 0
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(0), Val::I64(0), Val::I64(0)],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 0 * 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210 = 0
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0),
            Val::I64(0),
            Val::I64(0x0123_4567_89ab_cdef),
            Val::I64(-81985529216486896),
        ],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210 * 0 = 0
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0x0123_4567_89ab_cdef),
            Val::I64(-81985529216486896),
            Val::I64(0),
            Val::I64(0),
        ],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 1 * 2 = 2
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(1), Val::I64(0), Val::I64(2)],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(2));

    // 0xffff_ffff_ffff_ffff * 0xffff_ffff_ffff_ffff = 0xffff_ffff_ffff_fffe_0000_0000_0000_0001
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(-1), Val::I64(0), Val::I64(-1)],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(-2));
    assert_eq!(result[1].i64(), Some(1));

    // Overflow
    // 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff * 2 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(-1), Val::I64(-1), Val::I64(0), Val::I64(2)],
        &mut result,
    )
    .expect_err("expected overflow");

    // Overflow (a2b2)
    // 0x1_0000_0000_0000_0000 * 0x1_0000_0000_0000_0000 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(1), Val::I64(0), Val::I64(1), Val::I64(0)],
        &mut result,
    )
    .expect_err("expected overflow");

    // Overflow (a3b1)
    // 0x1_0000_0000_0000_0000_0000_0000 * 0x1_0000_0000 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0x1_0000_0000),
            Val::I64(0),
            Val::I64(0),
            Val::I64(0x1_0000_0000),
        ],
        &mut result,
    )
    .expect_err("expected overflow");

    // Overflow (a1b3)
    // 0x1_0000_0000 * 0x1_0000_0000_0000_0000_0000_0000 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0),
            Val::I64(0x1_0000_0000),
            Val::I64(0x1_0000_0000),
            Val::I64(0),
        ],
        &mut result,
    )
    .expect_err("expected overflow");

    // Overflow (a3b2)
    // 0x1_0000_0000_0000_0000_0000_0000 * 0x1_0000_0000_0000_0000 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0x1_0000_0000),
            Val::I64(0),
            Val::I64(1),
            Val::I64(0),
        ],
        &mut result,
    )
    .expect_err("expected overflow");

    // Overflow (a2b3)
    // 0x1_0000_0000_0000_0000 * 0x1_0000_0000_0000_0000_0000_0000 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(1),
            Val::I64(0),
            Val::I64(0x1_0000_0000),
            Val::I64(0),
        ],
        &mut result,
    )
    .expect_err("expected overflow");

    // Overflow (a3b3)
    // 0x1_0000_0000_0000_0000_0000_0000 * 0x1_0000_0000_0000_0000_0000_0000 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0x1_0000_0000),
            Val::I64(0),
            Val::I64(0x1_0000_0000),
            Val::I64(0),
        ],
        &mut result,
    )
    .expect_err("expected overflow");
}

#[test]
fn test_mul_int() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let mul = instance.get_func(store.borrow_mut(), "mul-int").unwrap();
    let mut result = [Val::I64(0), Val::I64(0)];

    // 0 * 0 = 0
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(0), Val::I64(0), Val::I64(0)],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 0 * 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210 = 0
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0),
            Val::I64(0),
            Val::I64(0x0123_4567_89ab_cdef),
            Val::I64(-81985529216486896),
        ],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210 * 0 = 0
    mul.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(0x0123_4567_89ab_cdef),
            Val::I64(-81985529216486896),
            Val::I64(0),
            Val::I64(0),
        ],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 1 * 2 = 2
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(1), Val::I64(0), Val::I64(2)],
        &mut result,
    )
    .expect("call to mul-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(2));

    // 0xffff_ffff_ffff_ffff * 0xffff_ffff_ffff_ffff = 0xffff_ffff_ffff_fffe_0000_0000_0000_0001
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(-1), Val::I64(0), Val::I64(-1)],
        &mut result,
    )
    .expect_err("expected overflow");

    // Overflow
    // 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff * 2 = Overflow
    mul.call(
        &mut store.borrow_mut(),
        &[Val::I64(-1), Val::I64(-1), Val::I64(0), Val::I64(2)],
        &mut result,
    )
    .expect_err("expected overflow");
}

#[test]
fn test_div_uint() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let div = instance.get_func(store.borrow_mut(), "div-uint").unwrap();
    let mut result = [Val::I64(0), Val::I64(0)];

    // 4 / 2 = 2
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(4), Val::I64(0), Val::I64(2)],
        &mut result,
    )
    .expect("call to div-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(2));

    // 7 / 4 = 1
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(7), Val::I64(0), Val::I64(4)],
        &mut result,
    )
    .expect("call to div-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(1));

    // 123 / 456 = 0
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(123), Val::I64(0), Val::I64(456)],
        &mut result,
    )
    .expect("call to div-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 0 / 0x123_0000_0000_0000_0456 = 0
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(0), Val::I64(0x123), Val::I64(0x456)],
        &mut result,
    )
    .expect("call to div-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 0x123_0000_0000_0000_0456 / 0 = DivideByZero
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0x123), Val::I64(0x456), Val::I64(0), Val::I64(0)],
        &mut result,
    )
    .expect_err("expected divide by zero");

    // 0x123_0000_0000_0000_0456 / 22 = 0xd_3a2e_8ba2_e8ba_2ebe
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0x123), Val::I64(0x456), Val::I64(0), Val::I64(22)],
        &mut result,
    )
    .expect("call to div-uint failed");
    assert_eq!(result[0].i64(), Some(0xd));
    assert_eq!(result[1].i64(), Some(0x3a2e_8ba2_e8ba_2ebe));
}

#[test]
fn test_div_int() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let div = instance.get_func(store.borrow_mut(), "div-int").unwrap();
    let mut result = [Val::I64(0), Val::I64(0)];

    // 4 / 2 = 2
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(4), Val::I64(0), Val::I64(2)],
        &mut result,
    )
    .expect("call to div-int failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(2));

    // -4 / 2 = -2
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(-1), Val::I64(-4), Val::I64(0), Val::I64(2)],
        &mut result,
    )
    .expect("call to div-int failed");
    assert_eq!(result[0].i64(), Some(-1));
    assert_eq!(result[1].i64(), Some(-2));

    // 4 / -2 = -2
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(0), Val::I64(4), Val::I64(-1), Val::I64(-2)],
        &mut result,
    )
    .expect("call to div-int failed");
    assert_eq!(result[0].i64(), Some(-1));
    assert_eq!(result[1].i64(), Some(-2));

    // -4 / -2 = 2
    div.call(
        &mut store.borrow_mut(),
        &[Val::I64(-1), Val::I64(-4), Val::I64(-1), Val::I64(-2)],
        &mut result,
    )
    .expect("call to div-int failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(2));

    // 0x8000_0000_0000_0000_0000_0000_0000_0000 / -2 = 0xc000_0000_0000_0000_0000_0000_0000_0000
    div.call(
        &mut store.borrow_mut(),
        &[
            Val::I64(-9223372036854775808),
            Val::I64(0),
            Val::I64(0),
            Val::I64(2),
        ],
        &mut result,
    )
    .expect("call to div-int failed");
    assert_eq!(result[0].i64(), Some(-4611686018427387904i64));
    assert_eq!(result[1].i64(), Some(0));
}

#[test]
fn test_mod_uint() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let modulo = instance.get_func(store.borrow_mut(), "mod-uint").unwrap();
    let mut result = [Val::I64(0), Val::I64(0)];

    // 4 % 2 = 0
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0), Val::I64(4), Val::I64(0), Val::I64(2)],
            &mut result,
        )
        .expect("call to mod-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 7 % 4 = 3
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0), Val::I64(7), Val::I64(0), Val::I64(4)],
            &mut result,
        )
        .expect("call to mod-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(3));

    // 123 % 456 = 123
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0), Val::I64(123), Val::I64(0), Val::I64(456)],
            &mut result,
        )
        .expect("call to mod-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(123));

    // 0 % 0x123_0000_0000_0000_0456 = 0
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0), Val::I64(0), Val::I64(0x123), Val::I64(0x456)],
            &mut result,
        )
        .expect("call to mod-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(0));

    // 0x123_0000_0000_0000_0456 % 0 = DivideByZero
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0x123), Val::I64(0x456), Val::I64(0), Val::I64(0)],
            &mut result,
        )
        .expect_err("expected divide by zero");

    // 0x123_0000_0000_0000_0456 % 22 = 2
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0x123), Val::I64(0x456), Val::I64(0), Val::I64(22)],
            &mut result,
        )
        .expect("call to mod-uint failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(2));
}

#[test]
fn test_mod_int() {
    let standard_lib = include_str!("standard.wat");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(&engine, standard_lib).unwrap();
    let instance = Instance::new(store.borrow_mut(), &module, &[]).unwrap();
    let modulo = instance.get_func(store.borrow_mut(), "mod-int").unwrap();
    let mut result = [Val::I64(0), Val::I64(0)];

    // 7 % 4 = 3
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0), Val::I64(7), Val::I64(0), Val::I64(4)],
            &mut result,
        )
        .expect("call to mod-int failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(3));

    // -7 / 4 = -3
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(-1), Val::I64(-7), Val::I64(0), Val::I64(4)],
            &mut result,
        )
        .expect("call to mod-int failed");
    assert_eq!(result[0].i64(), Some(-1));
    assert_eq!(result[1].i64(), Some(-3));

    // 7 / -4 = 3
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0), Val::I64(7), Val::I64(-1), Val::I64(-4)],
            &mut result,
        )
        .expect("call to mod-int failed");
    assert_eq!(result[0].i64(), Some(0));
    assert_eq!(result[1].i64(), Some(3));

    // -7 / -4 = -3
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(-1), Val::I64(-7), Val::I64(-1), Val::I64(-4)],
            &mut result,
        )
        .expect("call to mod-int failed");
    assert_eq!(result[0].i64(), Some(-1));
    assert_eq!(result[1].i64(), Some(-3));

    // 0x123_0000_0000_0000_0456 % 0 = DivideByZero
    modulo
        .call(
            &mut store.borrow_mut(),
            &[Val::I64(0x123), Val::I64(0x456), Val::I64(0), Val::I64(0)],
            &mut result,
        )
        .expect_err("expected divide by zero");
}