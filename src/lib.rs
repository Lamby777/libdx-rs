/*
* when the imposter is sus! 😳
* - Cherry, 4/27/2023
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;
pub type SyncResult<T, E = Box<dyn Error + Send + Sync>> = Result<T, E>;

pub fn sleep(len: u64) {
    std::thread::sleep(std::time::Duration::from_millis(len));
}

pub fn choose_rand<T: Copy, V: AsRef<[T]>>(v: V) -> Option<T> {
    v.as_ref().choose(&mut thread_rng()).map(|value| *value)
}

/// Expands to repeat some code for each item in
/// a provided list of values.
///
/// # Example
/// ```
/// # use libdx::foreach;
/// let mut vec = Vec::new();
/// foreach!([1, 2, 3] => |x| vec.push(x));
/// assert_eq!(vec, [1, 2, 3]);
/// ```
#[macro_export]
macro_rules! foreach {
	([$($arg:expr),+] => $logic:expr) => {
		{
			let mut logic = {$logic};
			$(logic($arg);)+
		}
	};
}

/// Like `foreach!`, but allows you to "iterate" over
/// identifiers, running your logic on a `&dyn T` trait
/// object for each one.
///
/// # Example
/// ```
/// # use libdx::foreach_dyn;
/// struct Foo;
/// struct Bar;
/// struct Baz;
///
/// trait Quacks {
///     fn quack(&self) { println!("Quack!"); }
/// }
///
/// impl Quacks for Foo {};
/// impl Quacks for Bar {};
/// impl Quacks for Baz {};
///
/// foreach_dyn!([Foo, Bar, Baz] => |x: &dyn Quacks| x.quack());
/// ```
///
#[macro_export]
macro_rules! foreach_dyn {
	([$($arg:ident),+] => $logic:expr) => {
		{
			let mut logic = {$logic};
			$(logic(&$arg);)+
		}
	};
}

/// Praise Shepmaster
/// https://stackoverflow.com/a/27582993/8149876
#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};

    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}
