
fn main(){
    std::env::set_var("CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG","true");
    lalrpop::process_root().unwrap();
}