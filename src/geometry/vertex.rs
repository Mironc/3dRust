#[derive(Clone, Copy)]
pub struct Vertex
{
    pub pos:[f32;3],/* 
    pub uv:[f32;3],
    pub normal:[f32;3], */
}
impl Vertex
{
    pub fn new(pos:[f32;3]) -> Self
    {
        Self { pos: pos}
    }
}
glium::implement_vertex!(Vertex,pos);