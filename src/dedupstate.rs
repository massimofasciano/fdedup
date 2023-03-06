#[cfg(any(feature = "threadpool", feature = "rayon"))]
pub mod mutex;
#[cfg(any(feature = "threadpool", feature = "rayon"))]
pub(crate) use self::mutex::DedupState;

#[cfg(not(any(feature = "threadpool", feature = "rayon")))]
pub mod single;
#[cfg(not(any(feature = "threadpool", feature = "rayon")))]
pub(crate) use self::single::DedupState;
