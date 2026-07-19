use super::Size;

#[derive(Clone, Debug)]
pub struct PixelBuffer {
    pub(crate) buf: Box<[u32]>,
    pub(crate) size: Size,
}

impl PixelBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            buf: vec![0_u32; w * h].into_boxed_slice(),
            size: Size::new(w as u64, h as u64),
        }
    }

    pub fn fill(&mut self, color: u32) {
        for v in self.buf.iter_mut() {
            *v = color;
        }
    }

    pub fn blit(&mut self, src: &PixelBuffer, dst_x: usize, dst_y: usize) {
        let src_w = src.size.w() as usize;
        let src_h = src.size.h() as usize;
        let dst_w = self.size.w() as usize;
        let dst_h = self.size.h() as usize;

        for sy in 0..src_h {
            let dy = dst_y + sy;
            if dy >= dst_h {
                break;
            }
            let src_row_start = sy * src_w;
            let dst_row_start = dy * dst_w;
            for sx in 0..src_w {
                let dx = dst_x + sx;
                if dx >= dst_w {
                    break;
                }
                let src_idx = src_row_start + sx;
                let dst_idx = dst_row_start + dx;
                self.buf[dst_idx] = src.buf[src_idx];
            }
        }
    }

    pub fn width(&self) -> usize {
        self.size.w() as usize
    }

    pub fn height(&self) -> usize {
        self.size.h() as usize
    }
}

impl std::ops::Index<usize> for PixelBuffer {
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl std::ops::IndexMut<usize> for PixelBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}
