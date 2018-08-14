//! animo
#![deny(missing_docs)]
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;

const LENGTH_OF_OBJECT_SIZE: usize = 1;
const LENGTH_OF_ANIMATION_SIZE: usize = 1;

/// Animo Object
#[derive(Clone, Debug)]
pub enum AnimObject {
    /// string
    AnimString(String),
}

/// Animo Model
pub struct AnimationModel {
    object: AnimObject,
}

impl AnimationModel {
    /// new with file
    pub fn load<S: Into<String>>(file_name: S) -> io::Result<Self> {
        let file = File::open(file_name.into())?;
        let mut file = BufReader::new(file).bytes().collect::<io::Result<Vec<_>>>().unwrap();

        let remain_file = file.split_off(LENGTH_OF_OBJECT_SIZE);
        let object_size: usize = file.iter().fold(0 as usize, |size, x| (size << 8) + *x as usize);

        let mut file = remain_file;
        let remain_file = file.split_off(LENGTH_OF_ANIMATION_SIZE);
        let _animation_size: usize = file.iter().fold(0 as usize, |size, x| (size << 8) + *x as usize);

        let mut file = remain_file;
        let _remain_file = file.split_off(object_size);
        let object = String::from_utf8(file).unwrap();
        Ok(AnimationModel {
            object: AnimObject::AnimString(object),
        })
    }

    /// run and get animation
    pub fn run(self) -> Animation {
        Animation {
            object: self.object,
        }
    }
}

/// Animation
pub struct Animation {
    object: AnimObject,
}

impl Iterator for Animation {
    type Item = Frame;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Frame{
            object: self.object.clone(),
        })
    }
}

/// frame of animation
pub struct Frame {
    object: AnimObject,
}

impl Frame {
    /// get text
    pub fn to_text(&self) -> String{
        match self.object.clone() {
            AnimObject::AnimString(content) => content,
        }
    }
}
#[cfg(test)]
mod tests {
    use *;
    #[test]
    fn load_no_object_and_animation() -> io::Result<()> {
        let file_name = "./test_files/no_object_and_animation";
        let anim = AnimationModel::load(file_name)?;
        let mut animation = anim.run();
        assert_eq!(animation.next().unwrap().to_text(), "".to_string());
        assert_eq!(animation.next().unwrap().to_text(), "".to_string());
        Ok(())
    }

    #[test]
    fn load_object_exists_and_no_animation() -> io::Result<()> {
        let file_name = "./test_files/object_exists_and_no_animation";
        let anim = AnimationModel::load(file_name)?;
        let mut animation = anim.run();
        assert_eq!(animation.next().unwrap().to_text(), "A".to_string());
        assert_eq!(animation.next().unwrap().to_text(), "A".to_string());
        Ok(())
    }
}
