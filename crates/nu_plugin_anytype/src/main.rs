use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_anytype::AnytypePlugin;

fn main() {
    serve_plugin(&AnytypePlugin::new(), MsgPackSerializer {})
}
