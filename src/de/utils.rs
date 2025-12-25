use core::{any::TypeId, marker::PhantomData, mem};

/// Produces type IDs that are compatible with `TypeId::of::<T>`, but without
/// `T: 'static` bound.
///
/// This function must be used with extreme discretion, as no lifetime checking
/// is done. Meaning, this function returns the same `TypeId` for `Struct<'a>`
/// and `Struct<'b>`, regardless of the relationship of `'a` and `'b`.
///
/// It is however a safe function since `TypeId` itself doesn't allow one to do
/// anything dangerous in safe code. In this crate, it's only used to specialize
/// behavior based on the type, there is no transmuting going on outside of this
/// function.
pub(crate) fn non_static_type_id<T: ?Sized>() -> TypeId {
    // Copied from the castaway crate, Copyright (c) 2021 Stephen M. Coakley

    trait NonStaticAny {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static;
    }

    impl<T: ?Sized> NonStaticAny for PhantomData<T> {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static,
        {
            TypeId::of::<T>()
        }
    }

    let phantom_data = PhantomData::<T>;
    NonStaticAny::get_type_id(unsafe {
        mem::transmute::<&dyn NonStaticAny, &(dyn NonStaticAny + 'static)>(&phantom_data)
    })
}
