/*!
Available event filters.

These can be used with [Dispatcher::subscribe][Subscribe] to get a [Stream] of [Messages] filtered to this event.

See the [table]

[Subscribe]: ../client/struct.Dispatcher.html#method.subscribe
[Stream]: https://docs.rs/futures/0.3.1/futures/stream/trait.Stream.html
[Messages]: ../messages/index.html
[table]: ../client/struct.Dispatcher.html#a-table-of-mappings
*/
use super::*;
use crate::client::Event;

/// Used to get a [messages::Cap][Cap]
///
/// [Cap]: ../messages/struct.Cap.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Cap;
impl<'t> Event<'t> for Cap {
    type Parsed = messages::Cap<'t>;
}

/// Used to get a [messages::ClearChat][ClearChat]
///
/// [ClearChat]: ../messages/struct.ClearChat.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct ClearChat;
impl<'t> Event<'t> for ClearChat {
    type Parsed = messages::ClearChat<'t>;
}

/// Used to get a [messages::ClearMsg][ClearMsg]
///
/// [ClearMsg]: ../messages/struct.ClearMsg.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct ClearMsg;
impl<'t> Event<'t> for ClearMsg {
    type Parsed = messages::ClearMsg<'t>;
}

/// Used to get a [messages::GlobalUserState][GlobalUserState]
///
/// [GlobalUserState]: ../messages/struct.GlobalUserState.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct GlobalUserState;
impl<'t> Event<'t> for GlobalUserState {
    type Parsed = messages::GlobalUserState<'t>;
}

/// Used to get a [messages::HostTarget][HostTarget]
///
/// [HostTarget]: ../messages/struct.HostTarget.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct HostTarget;
impl<'t> Event<'t> for HostTarget {
    type Parsed = messages::HostTarget<'t>;
}

/// Used to get a [messages::IrcReady][IrcReady]
///
/// [IrcReady]: ../messages/struct.IrcReady.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct IrcReady;
impl<'t> Event<'t> for IrcReady {
    type Parsed = messages::IrcReady<'t>;
}

/// Used to get a [messages::Join][Join]
///
/// [Join]: ../messages/struct.Join.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Join;
impl<'t> Event<'t> for Join {
    type Parsed = messages::Join<'t>;
}

/// Used to get a [messages::Mode][Mode]
///
/// [Mode]: ../messages/struct.Mode.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Mode;
impl<'t> Event<'t> for Mode {
    type Parsed = messages::Mode<'t>;
}

/// Used to get a [messages::Names][Names]
///
/// [Names]: ../messages/struct.Names.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Names;
impl<'t> Event<'t> for Names {
    type Parsed = messages::Names<'t>;
}

/// Used to get a [messages::Notice][Notice]
///
/// [Notice]: ../messages/struct.Notice.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Notice;
impl<'t> Event<'t> for Notice {
    type Parsed = messages::Notice<'t>;
}

/// Used to get a [messages::Part][Part]
///
/// [Part]: ../messages/struct.Part.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Part;
impl<'t> Event<'t> for Part {
    type Parsed = messages::Part<'t>;
}

/// Used to get a [messages::Ping][Ping]
///
/// [Ping]: ../messages/struct.Ping.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Ping;
impl<'t> Event<'t> for Ping {
    type Parsed = messages::Ping<'t>;
}

/// Used to get a [messages::Pong][Pong]
///
/// [Pong]: ../messages/struct.Pong.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Pong;
impl<'t> Event<'t> for Pong {
    type Parsed = messages::Pong<'t>;
}

/// Used to get a [messages::Privmsg][Privmsg]
///
/// [Privmsg]: ../messages/struct.Privmsg.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Privmsg;
impl<'t> Event<'t> for Privmsg {
    type Parsed = messages::Privmsg<'t>;
}

/// Used to get a [messages::Raw][Raw]
///
/// [Raw]: ../messages/type.Raw.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Raw;
impl<'t> Event<'t> for Raw {
    type Parsed = messages::Raw<'t>;
}

/// Used to get a [messages::Ready][Ready]
///
/// [Ready]: ../messages/struct.Ready.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Ready;
impl<'t> Event<'t> for Ready {
    type Parsed = messages::Ready<'t>;
}

/// Used to get a [messages::Reconnect][Reconnect]
///
/// [Reconnect]: ../messages/struct.Reconnect.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct Reconnect;
impl<'t> Event<'t> for Reconnect {
    type Parsed = messages::Reconnect;
}

/// Used to get a [messages::RoomState][RoomState]
///
/// [RoomState]: ../messages/struct.RoomState.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct RoomState;
impl<'t> Event<'t> for RoomState {
    type Parsed = messages::RoomState<'t>;
}

/// Used to get a [messages::UserNotice][UserNotice]
///
/// [UserNotice]: ../messages/struct.UserNotice.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct UserNotice;
impl<'t> Event<'t> for UserNotice {
    type Parsed = messages::UserNotice<'t>;
}

/// Used to get a [messages::UserState][UserState]
///
/// [UserState]: ../messages/struct.UserState.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct UserState;
impl<'t> Event<'t> for UserState {
    type Parsed = messages::UserState<'t>;
}

/// Used to get a [messages::AllCommands][AllCommands]
///
/// [AllCommands]: ../messages/enum.AllCommands.html
#[non_exhaustive]
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct All;
impl<'t> Event<'t> for All {
    type Parsed = messages::AllCommands<'t>;
}
