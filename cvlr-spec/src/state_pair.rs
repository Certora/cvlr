//! State pair types for expressing postconditions.

/// A pair of contexts representing pre-state and post-state.
///
/// This type is used to express postconditions that compare the state before
/// and after an operation. It holds references to both the current (post) state
/// and the old (pre) state.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the context references
/// * `Ctx` - The type of the context
///
/// # Examples
///
/// ```
/// use cvlr_spec::StatePair;
///
/// struct Counter {
///     value: i32,
/// }
///
/// let pre = Counter { value: 5 };
/// let post = Counter { value: 10 };
/// let pair = StatePair::new(&post, &pre);
///
/// // Access current state
/// assert_eq!(pair.ctx().value, 10);
/// assert_eq!(pair.post().value, 10);
///
/// // Access old state
/// assert_eq!(pair.old().value, 5);
/// assert_eq!(pair.pre().value, 5);
/// ```
#[derive(Copy, Clone)]
pub struct StatePair<'a, Ctx> {
    /// The current (post-state) context.
    pub ctx: &'a Ctx,
    /// The old (pre-state) context.
    pub old: &'a Ctx,
}

impl<'a, Ctx> StatePair<'a, Ctx> {
    /// Creates a new `StatePair` from separate pre-state and post-state contexts.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The current/post-state context
    /// * `old` - The old/pre-state context
    pub fn new(ctx: &'a Ctx, old: &'a Ctx) -> Self {
        Self { ctx, old }
    }

    /// Creates a `StatePair` where both pre-state and post-state refer to the same context.
    ///
    /// This is useful when you need a `StatePair` but there's no meaningful distinction
    /// between pre and post states (e.g., for invariants that hold at a single point in time).
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context to use for both pre-state and post-state
    pub fn singleton(ctx: &'a Ctx) -> Self {
        Self { ctx, old: ctx }
    }

    /// Returns a reference to the current (post-state) context.
    ///
    /// This is an alias for accessing the post-state. See also [`post`](StatePair::post).
    pub fn ctx(&self) -> &'a Ctx {
        self.ctx
    }

    /// Returns a reference to the old (pre-state) context.
    ///
    /// This is an alias for accessing the pre-state. See also [`pre`](StatePair::pre).
    pub fn old(&self) -> &'a Ctx {
        self.old
    }

    /// Returns a reference to the pre-state context.
    ///
    /// This is an alias for [`old`](StatePair::old).
    pub fn pre(&self) -> &'a Ctx {
        self.old
    }

    /// Returns a reference to the post-state context.
    ///
    /// This is an alias for [`ctx`](StatePair::ctx).
    pub fn post(&self) -> &'a Ctx {
        self.ctx
    }
}

impl<Ctx> core::ops::Deref for StatePair<'_, Ctx> {
    type Target = Ctx;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}
