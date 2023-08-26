use std::collections::HashMap;
use bevy::prelude::Resource;
use crate::ChunkPos;
use crate::bloc::Bloc;
use super::pos::{ChunkPos2D, BlocPos, BlocPos2D};
use super::Col;

pub type Cols<E> = HashMap<ChunkPos2D, E>;


#[derive(Resource)]
pub struct Blocs(pub Cols<Col>);

impl Blocs {
    pub fn set_bloc(&mut self, pos: BlocPos, bloc: Bloc) {
        let (colpos, coledpos) = pos.into();
        self.0.entry(colpos).or_insert(Col::new()).set(coledpos, bloc);
    }

    pub fn set_if_empty(&mut self, pos: BlocPos, bloc: Bloc) {
        let (colpos, coledpos) = pos.into();
        self.0.entry(colpos).or_insert(Col::new()).set_if_empty(coledpos, bloc);
    }
    
    pub fn get_block(&self, pos: BlocPos) -> Bloc {
        let (colpos, coledpos) = pos.into();
        match self.0.get(&colpos) {
            None => Bloc::default(),
            Some(col) => col.get(coledpos)
        }
    }

    pub fn top_block(&self, pos: BlocPos2D) -> (Bloc, i32) {
        let (colpos, pos2d) = pos.into();
        match self.0.get(&colpos) {
            None => (Bloc::default(), 0),
            Some(col) => col.top(pos2d)
        }
    }
}