mod api;
#[cfg(test)]
mod test_all;

use std::{panic, process};

use nvim_oxi as nvim;

#[nvim::module]
fn liboxi_tests() -> nvim::Result<()> {
    let result = panic::catch_unwind(|| {
        api::autocmd::clear_autocmds_buffer_n_patterns();
        api::autocmd::clear_autocmds_current_buf();
        api::autocmd::clear_autocmds_events();
        api::autocmd::create_augroup();
        api::autocmd::create_autocmd();
        api::autocmd::create_autocmd_buffer_n_patterns();
        api::autocmd::set_del_augroup_by_id();
        api::autocmd::set_del_augroup_by_name();
        api::autocmd::set_exec_del_autocmd();

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
        api::global::eval_statusline();
        api::global::get_chan_info();
        api::global::get_colors();
        api::global::get_context();
        api::global::get_highlights();
        api::global::get_mode();
        api::global::get_options();
        api::global::set_get_del_current_line();
        api::global::set_get_del_keymap();
        api::global::set_get_del_mark();
        api::global::set_get_del_var();
        api::global::set_get_option();
        api::global::strwidth();
    });

    process::exit(match result {
        Ok(_) => 0,

        Err(err) => {
            eprintln!("{err:?}");
            1
        },
    })
}
