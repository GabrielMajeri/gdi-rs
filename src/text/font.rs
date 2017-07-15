use windef::HFONT;

/// A font is a collection of characters and symbols that share a common design.
pub struct Font(HFONT);

impl_object!(Font, HFONT);
