// macro_rules! impl_closure {
//     ($fn_trait:ident, $from_fn:ident) => {
//         impl<A, R, F> ToObject for F
//         where
//             A: lua::LuaPoppable + 'static,
//             R: lua::LuaPushable + 'static,
//             F: $fn_trait(A) -> crate::Result<R> + 'static,
//         {
//             fn to_obj(self) -> Object {
//                 lua::LuaFun::$from_fn(self).to_obj()
//             }
//         }
//     };
// }

// impl_closure!(FnMut, from_fn_mut);
// impl_closure!(FnOnce, from_fn_once);

macro_rules! impl_boxed_closure {
    ($fn_trait:ident, $from_fn:ident) => {
        impl<A, R> ToObject
            for Box<dyn $fn_trait(A) -> crate::Result<R> + 'static>
        where
            A: lua::LuaPoppable + 'static,
            R: lua::LuaPushable + 'static,
        {
            fn to_obj(self) -> Object {
                lua::LuaFun::$from_fn(self).to_obj()
            }
        }
    };
}

impl_boxed_closure!(FnMut, from_fn_mut);
impl_boxed_closure!(FnOnce, from_fn_once);
