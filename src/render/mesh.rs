/// Manages mesh data on behalf of a RenderManager instance.
///
/// The mesh manager creates and handles mesh resources including vertex and index buffers. It does
/// not directly create a mesh component but stores the actual mesh data so that a RenderManager
/// can take advantage of mesh instancing in an agnostic way.
pub struct MeshManager {
    
}