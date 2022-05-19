use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ptr::{self, NonNull};

use super::collection::Collection;
use super::object::{Object, ObjectType};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Array = Collection<Object>;

impl TryFrom<Object> for Array {
    type Error = ();

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        match obj.r#type {
            ObjectType::kObjectTypeArray => {
                let array = unsafe { &obj.data.array };

                let mut items =
                    ManuallyDrop::new(Vec::with_capacity(array.size));

                unsafe {
                    ptr::copy(
                        array.items.as_ptr(),
                        items.as_mut_ptr(),
                        array.size,
                    );

                    items.set_len(array.size);
                }

                let array = Self {
                    items: unsafe {
                        NonNull::new_unchecked(items.as_mut_ptr())
                    },
                    size: array.size,
                    capacity: array.size,
                    _marker: PhantomData,
                };

                mem::forget(obj);

                Ok(array)
            },

            _ => Err(()),
        }
    }
}

impl<T: Into<Object>, Iter: IntoIterator<Item = T>> From<Iter> for Array {
    fn from(iter: Iter) -> Self {
        let mut vec = ManuallyDrop::new(
            iter.into_iter().map(|item| item.into()).collect::<Vec<Object>>(),
        );

        let size = vec.len();

        Self {
            items: unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) },
            size,
            capacity: size,
            _marker: PhantomData,
        }
    }
}

impl<T: Into<Object>> FromIterator<T> for Array {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter)
    }
}
