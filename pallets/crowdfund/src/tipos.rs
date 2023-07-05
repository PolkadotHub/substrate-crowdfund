extern crate alloc;

use alloc::vec::Vec;
use frame_support::{traits::Currency, BoundedVec};

pub type CuentaDe<T> = <T as frame_system::Config>::AccountId;
pub type BalanceDe<T> = <<T as crate::Config>::Currency as Currency<CuentaDe<T>>>::Balance;

pub type String = Vec<u8>;
pub type BoundedString<T> = BoundedVec<u8, <T as crate::Config>::LargoMaximoNombreProyecto>;

pub type NombreProyecto<T> = BoundedString<T>;
