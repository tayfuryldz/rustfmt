// rustfmt-wrap_comments: true
// rustfmt-comment_width: 100

unsafe impl<T> Sync for PinnedOnceLock<T>
where
    // Shared access to the PinnedOnceLock<T> also allows for creating T on a thread that may be
    // different from the one that later destroys the PinnedOnceLock<T>. So T also needs to be Send.
    T: Send,
    // Shared access to the PinnedOnceLock<T> allows for shared access to T, so T needs to be Sync
    // in order for the PinnedOnceLock<T> to be Sync.
    T: Sync,
{
}
