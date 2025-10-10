use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_anytype::AnytypePlugin;

fn main() {
    serve_plugin(&AnytypePlugin::new(), MsgPackSerializer {})
}
