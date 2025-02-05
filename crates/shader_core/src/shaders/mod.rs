pub mod flow_wireframe;
pub mod glow;
pub mod grid;
pub mod neon;
pub mod plugins;

pub enum ShaderLoadSettings {
    /// Shader path is statically checked but it cannot be hot-reloaded
    Static,
    //Shader is hot-reloadable buts its dynamically checked and location must be provided(as lookup is relative to caller not project)
    HotReloadable(&'static str),
}
