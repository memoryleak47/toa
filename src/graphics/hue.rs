use crate::*;

pub fn hue(tex: &Texture, hue: (u8, u8, u8)) -> SfBox<Texture> {
	let (w, h) = (tex.size().x, tex.size().y);
	let mut img = tex.copy_to_image().unwrap();

	let clip = |x: f32| if x > 255. { 255. } else { x } as u8;

	for x in 0..w {
		for y in 0..h {
            let mut c;
            unsafe {
			    c = img.pixel_at(x, y);
            }
			if c.b > std::cmp::max(c.r, c.g).saturating_add(5) {
				let blue = c.b as f32 / 255.;
				c.b = 0;
				c.r = clip(c.r as f32 + (hue.0 as f32) * blue);
				c.g = clip(c.g as f32 + (hue.1 as f32) * blue);
				c.b = clip(c.b as f32 + (hue.2 as f32) * blue);
                unsafe {
				    img.set_pixel(x, y, c);
                }
			}
		}
	}
    let mut tex = Texture::new().unwrap();
    let rect = IntRect {
        left: 0,
        top: 0,
        width: w as _,
        height: h as _,
    };
	tex.load_from_image(&img, rect).unwrap();

    tex
}
