use std::{sync::Arc, borrow::Cow};

use crate::ecs::component::Component;
pub struct Entity<'a>
{
    id:u32,
    name:Cow<'a,str>,
    components: Vec<&'a dyn Component>,
    
}