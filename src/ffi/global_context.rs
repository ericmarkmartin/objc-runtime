use std::sync::LazyLock;
use std::sync::RwLock;

pub(crate) use crate::runtime::context::Context;

pub(crate) static CONTEXT: LazyLock<RwLock<Context>> =
    LazyLock::new(|| RwLock::new(Context::new()));
