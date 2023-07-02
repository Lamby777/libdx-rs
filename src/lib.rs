/*
* when the imposter is sus! 😳
* - Cherry, 4/27/2023
*/

use std::error::Error;
use std::{thread, time};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;
pub type SyncResult<T, E = Box<dyn Error + Send + Sync>> = Result<T, E>;

pub fn sleep(len: u64) {
	thread::sleep(time::Duration::from_millis(len));
}

pub fn choose_rand<T: Copy, V: AsRef<[T]>>(v: V) -> Option<T> {
	v.as_ref().choose(&mut thread_rng()).map(|value| *value)
}

#[macro_export]
macro_rules! foreach {
	($logic:expr, $($arg:expr),+) => {
		{
			let logic = {$logic};
			$(logic($arg);)+
		}
	};
}

/// Praise Shepmaster
/// https://stackoverflow.com/a/27582993/8149876
///
/// ```
/// use libdx::map;
/// let r =	map! { "name" => "Sheep" };
/// assert_eq!(r.get("name"), "Sheep");
/// ```
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
	
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}
