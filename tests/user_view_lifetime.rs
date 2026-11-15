mod support;

#[test]
fn user_view_cannot_outlive_username() {
    support::assert_compile_fails("user_view_outlives_username.rs", "E0597");
}
