
pub enum ContainerState {Minimized, Restored}

impl ContainerState {
    fn minimize(&self) {
        if self == &Self::Minimized {
            
        }
    }

    fn restore(&self) {
        
    }
}

pub  struct  Container <T> {
    pub ent: T,

    pub state: ContainerState
}


impl Container {
    
    
}