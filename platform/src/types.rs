//! Describes types used by the platform interface.

proc_macros::keys! {
    // normal letters
    // does both uppercase and lowercase
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    j,
    i,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    // Others
    Eq => '=',
    Hyphen => '-',
    UnderScore => '_',
}

/// Key states.
pub enum Key {
    /// The key is released.
    Released(KeyCode),
    /// The key is pressed.
    Pressed(KeyCode),
    /// The key is held down.
    Held(KeyCode),
}

/// Contains the current stackframe.
/// When firing an interrupt, sometimes it might be necessary
/// to swap stack frames.
pub struct StackFrame {}
