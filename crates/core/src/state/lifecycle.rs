/// Whether the loop should keep running.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Lifecycle {
    /// Carry on.
    #[default]
    Running,
    /// Leave after this iteration.
    Exiting,
}
