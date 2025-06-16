/// Namespace selector given to
/// [`Buffer::get_extmarks`](crate::Buffer::get_extmarks).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GetExtmarksNamespaceId {
    /// Get extmarks from all namespaces.
    All,

    /// Only get extmarks registered on the namespace with this ID.
    Specific(u32),
}

impl From<u32> for GetExtmarksNamespaceId {
    #[inline]
    fn from(namespace_id: u32) -> Self {
        Self::Specific(namespace_id)
    }
}

impl From<GetExtmarksNamespaceId> for types::Integer {
    #[inline]
    fn from(namespace_id: GetExtmarksNamespaceId) -> Self {
        match namespace_id {
            GetExtmarksNamespaceId::All => -1,
            GetExtmarksNamespaceId::Specific(id) => id as Self,
        }
    }
}
