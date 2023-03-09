pub enum RayColor {
    Green,
    Orange,
    Red,
    Purple,
    Blue,
    Gray,
}

impl ToString for RayColor {
    fn to_string(&self) -> String {
        match self {
            RayColor::Green => "green",
            RayColor::Orange => "orange",
            RayColor::Red => "red",
            RayColor::Purple => "purple",
            RayColor::Blue => "blue",
            RayColor::Gray => "gray",
        }
        .to_string()
    }
}
