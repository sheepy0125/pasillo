//! Magic numbers packed at the first `LEN` bytes of a struct to identify it.

/// # Safety
/// Reads `ptr` to `ptr+LEN`
#[require_unsafe_in_body]
pub unsafe fn is_magic<const LEN: usize>(ptr: *const u8, magic: [u8; LEN]) -> bool {
    magic.iter().enumerate().all(|(idx, reference_byte)| {
        let byte = unsafe { ptr.add(idx).read() };
        byte == *reference_byte
    })
}

/// Implementation of this trait requires the first field of the struct to contain a field with a type
/// of `[u8; LEN]`.
///
/// This trait is only supported for structs due to the above implementation.
///
/// # Safety
/// Undefined behavior will occur if the struct has a packed size lower than `LEN`, as garbage memory
/// will be ran.
///
/// todo: Derive macro to implement this trait, guaranteeing the `magic` field is present
pub unsafe trait Magic<const LEN: usize> {
    const MAGIC: [u8; LEN];
    /// Determines if `self` matches `Self::MAGIC`.
    ///
    /// # Safety
    /// This calls [`is_magic`] and assumes the first `LEN` bytes of the struct are the `magic` field.
    /// Since the receiver here is `&self`, the type must have already been transmuted into `Self`;
    /// this function is thus marked as "safe" as the magic field *should* be present if this trait is
    /// implemented.
    fn is_magic(s: *const Self) -> bool {
        unsafe { is_magic::<LEN>(s as *const u8, Self::MAGIC) }
    }
}
