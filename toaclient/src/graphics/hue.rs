use sfml::graphics::{Texture, Image, Color};

pub fn hue(tex: &Texture, hue: (u8, u8, u8)) -> Texture {
	let (w, h) = (tex.size().x, tex.size().y);
	let mut img = tex.copy_to_image().unwrap();
	for x in 0..w {
		for y in 0..h {
			let mut c = img.pixel_at(x, y);
			let blue = c.b;
			c.b = 0;
			c.r = c.r.saturating_add(hue.0.saturating_mul(blue));
			c.g = c.g.saturating_add(hue.1.saturating_mul(blue));
			c.b = c.b.saturating_add(hue.2.saturating_mul(blue));
			img.set_pixel(x, y, &c);
		}
	}
	Texture::from_image(&img).unwrap()
}
