mod light;
mod phong;
mod debug;

pub mod lights;

pub use self::light::*;
pub use self::phong::RayTracePhongShading;

pub use self::debug::RayTraceDebugAxisShading;
pub use self::debug::RayTraceDebugNormalShading;
pub use self::debug::RayTraceDebugNormalType;
