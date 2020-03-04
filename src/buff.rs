use core::mem::size_of;

macro_rules! get_v_buffer {
    ( $this:expr, $t:ty, $se:ident ) => ({
        const SIZE: usize = size_of::<$t>();
        assert!($this.remaining() >= SIZE);
        let mut b = [0u8; SIZE];
        $this.copy_to_slice(&mut b);
        return <$t>::$se(b);
    })
}

pub trait Buffer {

    /// Total length of buffer.
    /// This value is greater than or equal to the length of the slice returned by Rust `bytes`.
    fn length(&self) -> usize;

    /// Forward the internal cursor of `Buffer`
    fn forward(&mut self, cnt: usize);

    /// Backward the internal cursor of `Buffer`
    fn backward(&self, cnt: usize);

    /// Got current position.
    fn position(&self) -> usize;

    /// Return a slice.
    fn bytes(&self) -> &[u8];

    fn remaining(&self) -> usize {
        self.length() - self.position()
    }

    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        assert!(self.remaining() >= dst.len());
        let b = self.bytes();
        dst.copy_from_slice(&b[..dst.len()]);
    }

    fn get_u8(&mut self) -> u8 {
        assert!(self.remaining() >= 1);
        let ret = self.bytes()[0];
        self.forward(1);
        ret
    }

    fn get_i8(&mut self) -> i8 {
        assert!(self.remaining() >= 1);
        let ret = self.bytes()[0] as i8;
        self.forward(1);
        ret
    }

    fn get_be_u16(&mut self) -> u16 {
        get_v_buffer!(self, u16, from_be_bytes)
    }

    fn get_le_u16(&mut self) -> u16 {
        get_v_buffer!(self, u16, from_le_bytes)
    }

    fn get_be_i16(&mut self) -> i16 {
        get_v_buffer!(self, i16, from_be_bytes)
    }

    fn get_le_i16(&mut self) -> i16 {
        get_v_buffer!(self, i16, from_le_bytes)
    }

    fn get_be_u32(&mut self) -> u32 {
        get_v_buffer!(self, u32, from_be_bytes)
    }

    fn get_le_u32(&mut self) -> u32 {
        get_v_buffer!(self, u32, from_le_bytes)
    }

    fn get_be_u64(&mut self) -> u64 {
        get_v_buffer!(self, u64, from_be_bytes)
    }

    fn get_le_u64(&mut self) -> u64 {
        get_v_buffer!(self, u64, from_le_bytes)
    }

    fn get_be_f32(&mut self) -> f32 {
        get_v_buffer!(self, f32, from_be_bytes)
    }

    fn get_le_f32(&mut self) -> f32 {
        get_v_buffer!(self, f32, from_le_bytes)
    }

    fn get_be_f64(&mut self) -> f64 {
        get_v_buffer!(self, f64, from_be_bytes)
    }

    fn get_le_f64(&mut self) -> f64 {
        get_v_buffer!(self, f64, from_le_bytes)
    }
}


