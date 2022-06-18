mod api;
#[cfg(test)]
mod test_all;

use std::{panic, process};

use nvim_oxi as nvim;

#[nvim::module]
fn liboxi_tests() -> nvim::Result<()> {
    let result = panic::catch_unwind(|| {
        api::buffer::attach();
        api::buffer::call();
        api::buffer::create_user_command();
        api::buffer::get_changedtick();
        api::buffer::get_name();
        api::buffer::set_get_del_keymap();
        api::buffer::set_get_del_mark();
        api::buffer::set_lines();
        api::buffer::set_option();
        api::buffer::set_var();
    });

    process::exit(match result {
        Ok(_) => 0,

        Err(err) => {
            eprintln!("{err:?}");
            1
        },
    })
}
