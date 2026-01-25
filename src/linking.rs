use crate::host::*;
use crate::state::State;
use crate::Error;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Populate all host-defined functions used by `module` in the `extern` vector.
///
/// If `sudo` is enabled, some more host-defined functions are allowed to be used.
pub(crate) fn populate_externals<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    module: &wasmi::Module,
    sudo: bool,
    externs: &mut Vec<wasmi::Extern>,
) -> Result<(), Error> {
    let mut ctx = ctx;
    for import in module.imports() {
        let ctx = ctx.as_context_mut();
        let func = match import.module() {
            "graphics" => select_graphics_external(ctx, import),
            "audio" => select_audio_external(ctx, import),
            "input" => select_input_external(ctx, import),
            "menu" => select_menu_external(ctx, import),
            "fs" => select_fs_external(ctx, import),
            "net" => select_net_external(ctx, import),
            "stats" => select_stats_external(ctx, import),
            "misc" => select_misc_external(ctx, import),
            "sudo" => select_sudo_external(ctx, import, sudo),
            "wasi_snapshot_preview1" => select_wasip1_external(ctx, import),
            "g" => select_graphics_external_alias(ctx, import),
            "i" => select_input_external_alias(ctx, import),
            "n" => select_net_external_alias(ctx, import),
            "s" => select_stats_external_alias(ctx, import),
            "m" => select_misc_external_alias(ctx, import),
            _ => return Err(Error::UnknownHostFunction),
        }?;
        externs.push(wasmi::Extern::Func(func));
    }
    Ok(())
}

fn select_graphics_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "clear_screen" => host_func(ctx, graphics::clear_screen),
        "set_color" => host_func(ctx, graphics::set_color),
        "draw_point" => host_func(ctx, graphics::draw_point),
        "draw_line" => host_func(ctx, graphics::draw_line),
        "draw_rect" => host_func(ctx, graphics::draw_rect),
        "draw_rounded_rect" => host_func(ctx, graphics::draw_rounded_rect),
        "draw_circle" => host_func(ctx, graphics::draw_circle),
        "draw_ellipse" => host_func(ctx, graphics::draw_ellipse),
        "draw_triangle" => host_func(ctx, graphics::draw_triangle),
        "draw_arc" => host_func(ctx, graphics::draw_arc),
        "draw_sector" => host_func(ctx, graphics::draw_sector),
        "draw_qr" => host_func(ctx, graphics::draw_qr),
        "draw_text" => host_func(ctx, graphics::draw_text),
        "draw_image" => host_func(ctx, graphics::draw_image),
        "draw_sub_image" => host_func(ctx, graphics::draw_sub_image),
        "set_canvas" => host_func(ctx, graphics::set_canvas),
        "unset_canvas" => host_func(ctx, graphics::unset_canvas),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_audio_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "reset" => host_func(ctx, audio::reset),
        "reset_all" => host_func(ctx, audio::reset_all),
        "clear" => host_func(ctx, audio::clear),
        "add_empty" => host_func(ctx, audio::add_empty),
        "add_file" => host_func(ctx, audio::add_file),
        "add_mix" => host_func(ctx, audio::add_mix),
        "add_all_for_one" => host_func(ctx, audio::add_all_for_one),
        "add_gain" => host_func(ctx, audio::add_gain),
        "add_loop" => host_func(ctx, audio::add_loop),
        "add_concat" => host_func(ctx, audio::add_concat),
        "add_pan" => host_func(ctx, audio::add_pan),
        "add_mute" => host_func(ctx, audio::add_mute),
        "add_pause" => host_func(ctx, audio::add_pause),
        "add_track_position" => host_func(ctx, audio::add_track_position),
        "add_low_pass" => host_func(ctx, audio::add_low_pass),
        "add_high_pass" => host_func(ctx, audio::add_high_pass),
        "add_take_left" => host_func(ctx, audio::add_take_left),
        "add_take_right" => host_func(ctx, audio::add_take_right),
        "add_swap" => host_func(ctx, audio::add_swap),
        "add_clip" => host_func(ctx, audio::add_clip),
        "add_noise" => host_func(ctx, audio::add_noise),
        "add_sine" => host_func(ctx, audio::add_sine),
        "add_square" => host_func(ctx, audio::add_square),
        "add_sawtooth" => host_func(ctx, audio::add_sawtooth),
        "add_triangle" => host_func(ctx, audio::add_triangle),
        "add_zero" => host_func(ctx, audio::add_zero),
        "mod_linear" => host_func(ctx, audio::mod_linear),
        "mod_hold" => host_func(ctx, audio::mod_hold),
        "mod_sine" => host_func(ctx, audio::mod_sine),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_input_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "read_pad" => host_func(ctx, input::read_pad),
        "read_buttons" => host_func(ctx, input::read_buttons),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_menu_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "add_menu_item" => host_func(ctx, menu::add_menu_item),
        "remove_menu_item" => host_func(ctx, menu::remove_menu_item),
        "open_menu" => host_func(ctx, menu::open_menu),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_fs_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "get_rom_file_size" => host_func(ctx, fs::get_rom_file_size),
        "load_rom_file" => host_func(ctx, fs::load_rom_file),
        "get_file_size" => host_func(ctx, fs::get_file_size),
        "load_file" => host_func(ctx, fs::load_file),
        "dump_file" => host_func(ctx, fs::dump_file),
        "remove_file" => host_func(ctx, fs::remove_file),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_net_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "get_me" => host_func(ctx, net::get_me),
        "get_peers" => host_func(ctx, net::get_peers),
        "save_stash" => host_func(ctx, net::save_stash),
        "load_stash" => host_func(ctx, net::load_stash),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_stats_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "add_progress" => host_func(ctx, stats::add_progress),
        "add_score" => host_func(ctx, stats::add_score),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_misc_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "log_debug" => host_func(ctx, misc::log_debug),
        "log_error" => host_func(ctx, misc::log_error),
        "set_seed" => host_func(ctx, misc::set_seed),
        "get_random" => host_func(ctx, misc::get_random),
        "get_name" => host_func(ctx, misc::get_name),
        "restart" => host_func(ctx, misc::restart),
        "set_conn_status" => host_func(ctx, misc::set_conn_status),
        "quit" => host_func(ctx, misc::quit),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_sudo_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
    sudo: bool,
) -> Result<wasmi::Func, Error> {
    if !sudo {
        return Err(Error::UsedDisabledSudoHostFunction);
    }
    let func = match import.name() {
        "list_dirs" => host_func(ctx, sudo::list_dirs),
        "list_dirs_buf_size" => host_func(ctx, sudo::list_dirs_buf_size),
        "get_file_size" => host_func(ctx, sudo::get_file_size),
        "load_file" => host_func(ctx, sudo::load_file),
        "run_app" => host_func(ctx, sudo::run_app),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_wasip1_external<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "environ_get" => host_func(ctx, wasip1::environ_get),
        "environ_sizes_get" => host_func(ctx, wasip1::environ_sizes_get),
        "clock_time_get" => host_func(ctx, wasip1::clock_time_get),
        "fd_close" => host_func(ctx, wasip1::fd_close),
        "fd_read" => host_func(ctx, wasip1::fd_read),
        "fd_seek" => host_func(ctx, wasip1::fd_seek),
        "fd_write" => host_func(ctx, wasip1::fd_write),
        "proc_exit" => host_func(ctx, wasip1::proc_exit),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_graphics_external_alias<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "a" => host_func(ctx, graphics::draw_arc),
        "c" => host_func(ctx, graphics::draw_circle),
        "ca" => host_func(ctx, graphics::set_canvas),
        "cr" => host_func(ctx, graphics::unset_canvas),
        "cs" => host_func(ctx, graphics::clear_screen),
        "e" => host_func(ctx, graphics::draw_ellipse),
        "i" => host_func(ctx, graphics::draw_image),
        "l" => host_func(ctx, graphics::draw_line),
        "p" => host_func(ctx, graphics::draw_point),
        "r" => host_func(ctx, graphics::draw_rect),
        "rr" => host_func(ctx, graphics::draw_rounded_rect),
        "s" => host_func(ctx, graphics::draw_sector),
        "sc" => host_func(ctx, graphics::set_color),
        "si" => host_func(ctx, graphics::draw_sub_image),
        "t" => host_func(ctx, graphics::draw_triangle),
        "x" => host_func(ctx, graphics::draw_text),
        "q" => host_func(ctx, graphics::draw_qr),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_input_external_alias<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "p" => host_func(ctx, input::read_pad),
        "b" => host_func(ctx, input::read_buttons),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_net_external_alias<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "l" => host_func(ctx, net::load_stash),
        "m" => host_func(ctx, net::get_me),
        "p" => host_func(ctx, net::get_peers),
        "s" => host_func(ctx, net::save_stash),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_stats_external_alias<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "p" => host_func(ctx, stats::add_progress),
        "s" => host_func(ctx, stats::add_score),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

fn select_misc_external_alias<'a>(
    ctx: impl wasmi::AsContextMut<Data = Box<State<'a>>>,
    import: wasmi::ImportType<'_>,
) -> Result<wasmi::Func, Error> {
    let func = match import.name() {
        "d" => host_func(ctx, misc::log_debug),
        "e" => host_func(ctx, misc::log_error),
        "n" => host_func(ctx, misc::get_name),
        "q" => host_func(ctx, misc::quit),
        "r" => host_func(ctx, misc::get_random),
        "s" => host_func(ctx, misc::set_seed),
        _ => return Err(Error::UnknownHostFunction),
    };
    Ok(func)
}

/// Utility function to wrap host functions without Wasmi imports.
#[inline]
fn host_func<T, P, R>(
    ctx: impl wasmi::AsContextMut<Data = T>,
    func: impl wasmi::IntoFunc<T, P, R>,
) -> wasmi::Func {
    wasmi::Func::wrap(ctx, func)
}

/// Register all host-defined functions in the linker.
pub(crate) fn link(linker: &mut wasmi::Linker<Box<State>>, sudo: bool) -> Result<(), wasmi::Error> {
    linker.func_wrap("graphics", "clear_screen", graphics::clear_screen)?;
    linker.func_wrap("graphics", "set_color", graphics::set_color)?;
    linker.func_wrap("graphics", "draw_point", graphics::draw_point)?;
    linker.func_wrap("graphics", "draw_line", graphics::draw_line)?;
    linker.func_wrap("graphics", "draw_rect", graphics::draw_rect)?;
    linker.func_wrap("graphics", "draw_rounded_rect", graphics::draw_rounded_rect)?;
    linker.func_wrap("graphics", "draw_circle", graphics::draw_circle)?;
    linker.func_wrap("graphics", "draw_ellipse", graphics::draw_ellipse)?;
    linker.func_wrap("graphics", "draw_triangle", graphics::draw_triangle)?;
    linker.func_wrap("graphics", "draw_arc", graphics::draw_arc)?;
    linker.func_wrap("graphics", "draw_sector", graphics::draw_sector)?;
    linker.func_wrap("graphics", "draw_qr", graphics::draw_qr)?;
    linker.func_wrap("graphics", "draw_text", graphics::draw_text)?;
    linker.func_wrap("graphics", "draw_image", graphics::draw_image)?;
    linker.func_wrap("graphics", "draw_sub_image", graphics::draw_sub_image)?;
    linker.func_wrap("graphics", "set_canvas", graphics::set_canvas)?;
    linker.func_wrap("graphics", "unset_canvas", graphics::unset_canvas)?;

    linker.func_wrap("audio", "reset", audio::reset)?;
    linker.func_wrap("audio", "reset_all", audio::reset_all)?;
    linker.func_wrap("audio", "clear", audio::clear)?;
    linker.func_wrap("audio", "add_empty", audio::add_empty)?;
    linker.func_wrap("audio", "add_file", audio::add_file)?;
    linker.func_wrap("audio", "add_mix", audio::add_mix)?;
    linker.func_wrap("audio", "add_all_for_one", audio::add_all_for_one)?;
    linker.func_wrap("audio", "add_gain", audio::add_gain)?;
    linker.func_wrap("audio", "add_loop", audio::add_loop)?;
    linker.func_wrap("audio", "add_concat", audio::add_concat)?;
    linker.func_wrap("audio", "add_pan", audio::add_pan)?;
    linker.func_wrap("audio", "add_mute", audio::add_mute)?;
    linker.func_wrap("audio", "add_pause", audio::add_pause)?;
    linker.func_wrap("audio", "add_track_position", audio::add_track_position)?;
    linker.func_wrap("audio", "add_low_pass", audio::add_low_pass)?;
    linker.func_wrap("audio", "add_high_pass", audio::add_high_pass)?;
    linker.func_wrap("audio", "add_take_left", audio::add_take_left)?;
    linker.func_wrap("audio", "add_take_right", audio::add_take_right)?;
    linker.func_wrap("audio", "add_swap", audio::add_swap)?;
    linker.func_wrap("audio", "add_clip", audio::add_clip)?;
    linker.func_wrap("audio", "add_noise", audio::add_noise)?;
    linker.func_wrap("audio", "add_sine", audio::add_sine)?;
    linker.func_wrap("audio", "add_square", audio::add_square)?;
    linker.func_wrap("audio", "add_sawtooth", audio::add_sawtooth)?;
    linker.func_wrap("audio", "add_triangle", audio::add_triangle)?;
    linker.func_wrap("audio", "add_zero", audio::add_zero)?;
    linker.func_wrap("audio", "mod_linear", audio::mod_linear)?;
    linker.func_wrap("audio", "mod_hold", audio::mod_hold)?;
    linker.func_wrap("audio", "mod_sine", audio::mod_sine)?;

    linker.func_wrap("input", "read_pad", input::read_pad)?;
    linker.func_wrap("input", "read_buttons", input::read_buttons)?;

    linker.func_wrap("menu", "add_menu_item", menu::add_menu_item)?;
    linker.func_wrap("menu", "remove_menu_item", menu::remove_menu_item)?;
    linker.func_wrap("menu", "open_menu", menu::open_menu)?;

    linker.func_wrap("fs", "get_rom_file_size", fs::get_rom_file_size)?; // deprecated
    linker.func_wrap("fs", "load_rom_file", fs::load_rom_file)?; // deprecated
    linker.func_wrap("fs", "get_file_size", fs::get_file_size)?;
    linker.func_wrap("fs", "load_file", fs::load_file)?;
    linker.func_wrap("fs", "dump_file", fs::dump_file)?;
    linker.func_wrap("fs", "remove_file", fs::remove_file)?;

    linker.func_wrap("net", "get_me", net::get_me)?;
    linker.func_wrap("net", "get_peers", net::get_peers)?;
    linker.func_wrap("net", "save_stash", net::save_stash)?;
    linker.func_wrap("net", "load_stash", net::load_stash)?;

    linker.func_wrap("stats", "add_progress", stats::add_progress)?;
    linker.func_wrap("stats", "add_score", stats::add_score)?;

    linker.func_wrap("misc", "log_debug", misc::log_debug)?;
    linker.func_wrap("misc", "log_error", misc::log_error)?;
    linker.func_wrap("misc", "set_seed", misc::set_seed)?;
    linker.func_wrap("misc", "get_random", misc::get_random)?;
    linker.func_wrap("misc", "get_name", misc::get_name)?;
    linker.func_wrap("misc", "restart", misc::restart)?;
    linker.func_wrap("misc", "set_conn_status", misc::set_conn_status)?;
    linker.func_wrap("misc", "quit", misc::quit)?;

    if sudo {
        linker.func_wrap("sudo", "list_dirs", sudo::list_dirs)?;
        linker.func_wrap("sudo", "list_dirs_buf_size", sudo::list_dirs_buf_size)?;
        linker.func_wrap("sudo", "get_file_size", sudo::get_file_size)?;
        linker.func_wrap("sudo", "load_file", sudo::load_file)?;
        linker.func_wrap("sudo", "run_app", sudo::run_app)?;
    }

    // WASI preview 1
    const M: &str = "wasi_snapshot_preview1";
    linker.func_wrap(M, "environ_get", wasip1::environ_get)?;
    linker.func_wrap(M, "environ_sizes_get", wasip1::environ_sizes_get)?;
    linker.func_wrap(M, "clock_time_get", wasip1::clock_time_get)?;
    linker.func_wrap(M, "fd_close", wasip1::fd_close)?;
    linker.func_wrap(M, "fd_read", wasip1::fd_read)?;
    linker.func_wrap(M, "fd_seek", wasip1::fd_seek)?;
    linker.func_wrap(M, "fd_write", wasip1::fd_write)?;
    linker.func_wrap(M, "proc_exit", wasip1::proc_exit)?;

    link_aliases(linker)
}

// Link short aliases for when the wasm binary size is paramount (code golfing).
fn link_aliases(linker: &mut wasmi::Linker<Box<State>>) -> Result<(), wasmi::Error> {
    linker.func_wrap("g", "a", graphics::draw_arc)?;
    linker.func_wrap("g", "c", graphics::draw_circle)?;
    linker.func_wrap("g", "ca", graphics::set_canvas)?;
    linker.func_wrap("g", "cr", graphics::unset_canvas)?;
    linker.func_wrap("g", "cs", graphics::clear_screen)?;
    linker.func_wrap("g", "e", graphics::draw_ellipse)?;
    linker.func_wrap("g", "i", graphics::draw_image)?;
    linker.func_wrap("g", "l", graphics::draw_line)?;
    linker.func_wrap("g", "p", graphics::draw_point)?;
    linker.func_wrap("g", "r", graphics::draw_rect)?;
    linker.func_wrap("g", "rr", graphics::draw_rounded_rect)?;
    linker.func_wrap("g", "s", graphics::draw_sector)?;
    linker.func_wrap("g", "sc", graphics::set_color)?;
    linker.func_wrap("g", "si", graphics::draw_sub_image)?;
    linker.func_wrap("g", "t", graphics::draw_triangle)?;
    linker.func_wrap("g", "x", graphics::draw_text)?;
    linker.func_wrap("g", "q", graphics::draw_qr)?;

    linker.func_wrap("i", "p", input::read_pad)?;
    linker.func_wrap("i", "b", input::read_buttons)?;

    linker.func_wrap("n", "l", net::load_stash)?;
    linker.func_wrap("n", "m", net::get_me)?;
    linker.func_wrap("n", "p", net::get_peers)?;
    linker.func_wrap("n", "s", net::save_stash)?;

    linker.func_wrap("s", "p", stats::add_progress)?;
    linker.func_wrap("s", "s", stats::add_score)?;

    linker.func_wrap("m", "d", misc::log_debug)?;
    linker.func_wrap("m", "e", misc::log_error)?;
    linker.func_wrap("m", "n", misc::get_name)?;
    linker.func_wrap("m", "q", misc::quit)?;
    linker.func_wrap("m", "r", misc::get_random)?;
    linker.func_wrap("m", "s", misc::set_seed)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::link;
    use crate::state::State;

    #[test]
    fn smoke_test_linking() {
        let engine = wasmi::Engine::default();
        let mut linker = <wasmi::Linker<Box<State>>>::new(&engine);
        link(&mut linker, true).unwrap();
    }
}
