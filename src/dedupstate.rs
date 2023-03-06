#[cfg(any(feature = "mutex", all(feature = "channel", feature = "refcell")))]
pub mod mutex;
#[cfg(any(feature = "mutex", all(feature = "channel", feature = "refcell")))]
pub(crate) use self::mutex::DedupState;

#[cfg(any(all(feature = "channel", not(feature = "refcell")), not(any(feature = "refcell", feature = "mutex", feature = "dashmap"))))]
pub mod single;
#[cfg(any(all(feature = "channel", not(feature = "refcell")), not(any(feature = "refcell", feature = "mutex", feature = "dashmap"))))]
pub(crate) use self::single::DedupState;

#[cfg(feature = "dashmap")]
pub mod dashmap;
#[cfg(feature = "dashmap")]
pub(crate) use self::dashmap::DedupState;
