/*
* when the imposter is sus! 😳
* - Cherry, 4/27/2023
*/

use std::error::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;
pub type SyncResult<T, E = Box<dyn Error + Send + Sync>> = Result<T, E>;

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
