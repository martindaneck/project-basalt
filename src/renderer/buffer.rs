pub struct Buffer {
    pub id: u32,
}

impl Buffer {
    /// create empty buffer
    pub fn new() -> Self {
        let mut id: u32 = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
        }
        Self { id }
    }

    pub fn upload<T>(&self, data: &[T], usage: u32) {
        unsafe {
            gl::NamedBufferData(
                self.id,
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const _,
                usage,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

