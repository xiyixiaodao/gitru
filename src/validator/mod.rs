pub mod commit_msg;
pub mod git_status;

pub struct ErrorCollector<E> {
    pub errors: Vec<E>,
}

impl<E> ErrorCollector<E> {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn push(&mut self, err: E) {
        self.errors.push(err);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl<E> Default for ErrorCollector<E> {
    fn default() -> Self {
        Self::new()
    }
}
