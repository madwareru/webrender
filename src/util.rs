use time::precise_time_ns;
use internal_types::{ClipRectToRegionMaskResult, RenderPass};
use types::{ColorF, ImageFormat};

#[allow(dead_code)]
pub struct ProfileScope {
    name: &'static str,
    t0: u64,
}

impl ProfileScope {
    #[allow(dead_code)]
    pub fn new(name: &'static str) -> ProfileScope {
        ProfileScope {
            name: name,
            t0: precise_time_ns(),
        }
    }
}

impl Drop for ProfileScope {
    fn drop(&mut self) {
        if self.name.chars().next() != Some(' ') {
            let t1 = precise_time_ns();
            let ms = (t1 - self.t0) as f64 / 1000000f64;
            //if ms > 0.1 {
            println!("{} {}", self.name, ms);
        }
    }
}

pub fn get_render_pass(colors: &[ColorF],
                       format: ImageFormat,
                       mask_result: &Option<ClipRectToRegionMaskResult>)
                       -> RenderPass {
    if colors.iter().any(|color| color.a < 1.0) || mask_result.is_some() {
        return RenderPass::Alpha
    }

    match format {
        ImageFormat::A8 => RenderPass::Alpha,
        ImageFormat::RGBA8 => RenderPass::Alpha,
        ImageFormat::RGB8 => RenderPass::Opaque,
        ImageFormat::Invalid => unreachable!(),
    }
}
