pub trait Perspective: Sealed {}

pub struct Host;
impl Sealed for Host {}
impl Perspective for Host {}
pub struct Guest;
impl Sealed for Guest {}
impl Perspective for Guest {}

trait Sealed {}
