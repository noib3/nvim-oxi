use std::fmt;
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ptr::{self, NonNull};

use super::collection::Collection;
use super::object::{Object, ObjectType};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Array = Collection<Object>;

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl TryFrom<Object> for Array {
    type Error = ();

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        if !matches!(obj.r#type, ObjectType::kObjectTypeArray) {
            return Err(());
        }

        let array = unsafe { &obj.data.array };

        let mut items = ManuallyDrop::new(Vec::with_capacity(array.size));

        unsafe {
            ptr::copy(array.items.as_ptr(), items.as_mut_ptr(), array.size);

            items.set_len(array.size);
        }

        let array = Self {
            items: unsafe { NonNull::new_unchecked(items.as_mut_ptr()) },
            size: array.size,
            capacity: array.size,
            _marker: PhantomData,
        };

        mem::forget(obj);

        Ok(array)
    }
}

impl<T: Into<Object>> FromIterator<T> for Array {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec =
            iter.into_iter().map(|item| item.into()).collect::<Vec<Object>>();

        let size = vec.len();
        let capacity = vec.capacity();
        let ptr = vec.leak() as *mut [Object] as *mut Object;

        unsafe { Self::from_raw_parts(ptr, size, capacity) }
    }
}
