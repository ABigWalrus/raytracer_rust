use std::sync::Arc;

trait BufferObject {
    fn to_bytes(&self) -> Vec<u8>;
}

struct Buffer {
    object: Arc<dyn BufferObject>,
}

impl Buffer {
    fn new(object: Arc<dyn BufferObject>) -> Self {
        Self { object }
    }
}
