use base::DMTest;
mod base;

macro_rules! test_dm {
    ($file:ident $(+ $($additional:literal),+)?) => {
        #[test]
        fn $file() {
            DMTest::new(stringify!($file).to_string())
                .run_hook_and_assert_result(
                    vec!(
                        $($($additional),+,)?
                        concat!(stringify!($file), ".dm")
                    )
                );
        }
    };
}

test_dm!(test_total_moles);
test_dm!(test_jz);
test_dm!(test_args);
test_dm!(push_val);
test_dm!(test_cmp);
test_dm!(test_deopt);
test_dm!(test_mul);
test_dm!(test_div);
test_dm!(test_add);
test_dm!(test_call_global);
test_dm!(test_call_static);
test_dm!(test_call_dynamic);
test_dm!(test_abs);
test_dm!(test_if_bool_ops);
test_dm!(test_sub);
test_dm!(test_aug_op);
test_dm!(test_is);
test_dm!(test_ref_count);
test_dm!(test_local_var);
test_dm!(test_null_cmp);
test_dm!(test_call_args);
test_dm!(test_round);
test_dm!(test_gas_mixture_share);