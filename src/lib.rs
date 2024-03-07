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
	([$($arg:expr),+ $(,)?] => $logic:expr) => {
		{
			let mut logic = {$logic};
			$(logic($arg);)+
		}
	};
}

/// Run a static method on each type provided.
///
/// # Example
/// ```
/// # use libdx::foreach_static;
/// struct Foo;
/// struct Bar;
///
/// trait Quacks {
///     fn static_quack() { println!("Quack!"); }
/// }
///
/// impl Quacks for Foo {};
/// impl Quacks for Bar {};
///
/// foreach_static!([Foo, Bar] => Quacks, static_quack);
/// ```
///
#[macro_export]
macro_rules! foreach_static {
    ([$($arg:ident),+ $(,)?] => $trait:ident, $method:ident) => {
        {
            $(<$arg as $trait>::$method();)+
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
