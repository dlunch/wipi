pub fn printk(fmt: &str, args: &[*const ()]) {
    wipic_simulation::kernel::printk(fmt, args);
}

pub fn exit(code: i32) {
    wipic_simulation::kernel::exit(code);
}
