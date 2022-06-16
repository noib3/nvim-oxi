use nvim_types::NonOwning;

// create a variable on stack
pub(crate) trait DefineVar<T> {
    type Var;
    fn define_var(val: T) -> Self::Var;
}

// translate that variable into ffi type
// VarRef will be &Var
pub(crate) trait UseVar<VarRef> {
    fn use_var(var: VarRef) -> Self;
}

macro_rules! api_call {
    (
        $ffi_name:ident
        (
            $($arg_name:ident : $ffi_ty:ty),* $(,)?
        )
        $(-> $ffi_ret:ty)?
    ) => {{
        extern "C" {
            fn $ffi_name(
                $($arg_name : $ffi_ty),*
            ) $(-> $ffi_ret)?;
        }
        $(
        let $arg_name = <$ffi_ty as $crate::api::api_call::DefineVar<_>>::define_var($arg_name);
        )*

        unsafe {
            $ffi_name (
                $(<$ffi_ty as $crate::api::api_call::UseVar<_>>::use_var(&$arg_name)),*
            )
        }
    }};

    (#[handle_error($err_name:ident)] $($rest:tt)*) => {{
        let mut err = ::nvim_types::Error::new();
        let $err_name = &mut err as *mut _;
        let result = api_call! { $($rest)* };
        if err.is_err() {
            Err($crate::Error::NvimError(err))
        } else {
            Ok(result)
        }
    }};

    (#[lua_internal_call($chan:ident)] $($rest:tt)*) => {{
        let $chan = LUA_INTERNAL_CALL;
        api_call! { $($rest)* }
    }};
}

pub(crate) use api_call;

impl<'a, T, U> DefineVar<U> for NonOwning<'a, T>
where
    T: DefineVar<U>,
{
    type Var = T::Var;
    fn define_var(val: U) -> T::Var {
        T::define_var(val)
    }
}

impl<T, U> DefineVar<U> for *const T
where
    T: DefineVar<U>,
{
    type Var = T::Var;
    fn define_var(val: U) -> T::Var {
        T::define_var(val)
    }
}

macro_rules! impl_define_var_identity {
    ($ty:ty) => {
        impl DefineVar<$ty> for $ty {
            type Var = $ty;
            fn define_var(val: $ty) -> $ty {
                val
            }
        }
    };
}

impl_define_var_identity!(*mut nvim_types::Error);
impl_define_var_identity!(u64);
impl_define_var_identity!(bool);

macro_rules! impl_define_var_into {
    ($ty:ty) => {
        impl<T> DefineVar<T> for $ty
        where
            T: Into<$ty>,
        {
            type Var = $ty;
            fn define_var(val: T) -> $ty {
                val.into()
            }
        }
    };
}

impl_define_var_into!(nvim_types::String);
impl_define_var_into!(nvim_types::Array);
impl_define_var_into!(nvim_types::Integer);

impl<Var> UseVar<&'_ Var> for *const Var {
    fn use_var(var: &Var) -> Self {
        var
    }
}

macro_rules! impl_use_var_non_owning {
    ($ty:ty) => {
        impl<'a> UseVar<&'a $ty> for NonOwning<'a, $ty> {
            fn use_var(var: &'a $ty) -> Self {
                var.non_owning()
            }
        }
    };
}

impl_use_var_non_owning!(nvim_types::Array);
impl_use_var_non_owning!(nvim_types::Dictionary);
impl_use_var_non_owning!(nvim_types::String);
impl_use_var_non_owning!(nvim_types::Object);

macro_rules! impl_use_var_copy {
    ($ty:ty) => {
        impl UseVar<&'_ $ty> for $ty {
            fn use_var(var: &$ty) -> Self {
                *var
            }
        }
    };
}

impl_use_var_copy!(nvim_types::Integer);
impl_use_var_copy!(bool);
impl_use_var_copy!(u64);
impl_use_var_copy!(*mut nvim_types::Error);
