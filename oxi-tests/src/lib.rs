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
        api::buffer::create_del_user_command();
        api::buffer::get_changedtick();
        api::buffer::loaded_n_valid();
        api::buffer::new_buf_delete();
        api::buffer::set_get_del_keymap();
        api::buffer::set_get_del_lines();
        api::buffer::set_get_del_mark();
        api::buffer::set_get_del_text();
        api::buffer::set_get_del_var();
        api::buffer::set_get_name();
        api::buffer::set_get_option();

        api::global::chan_send_fail();
        api::global::create_del_user_command();
        api::global::get_mode();
        api::global::set_get_del_current_line();
        api::global::set_get_del_keymap();
        api::global::set_get_del_mark();
        api::global::set_get_del_var();
    });

    process::exit(match result {
        Ok(_) => 0,

        Err(err) => {
            eprintln!("{err:?}");
            1
        },
    })
}
