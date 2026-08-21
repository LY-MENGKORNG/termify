pub mod api;
pub mod auth;
pub mod config;
pub mod local;
pub mod logger;
pub mod lyric;
pub mod page;
pub mod path;
pub mod widget;

pub use api::*;
pub use auth::*;
pub use config::*;
pub use local::*;
pub use lyric::*;
pub use page::*;
pub use path::*;
pub use widget::*;

pub const BANNER: &str = "
    ████████╗███████╗██████╗ ███╗   ███╗██╗███████╗██╗   ██╗
    ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║██╔════╝╚██╗ ██╔╝
       ██║   █████╗  ██████╔╝██╔████╔██║██║█████╗   ╚████╔╝
       ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██╔══╝    ╚██╔╝
       ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║        ██║
       ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝        ╚═╝
";
