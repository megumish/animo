//! animo
#![deny(missing_docs)]
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;

const LENGTH_OF_OBJECT_SIZE: usize = 4;
const LENGTH_OF_OBJECTS_SIZE: usize = 4;
const LENGTH_OF_OBJECTS_TOTAL_LENGTH: usize = 4;
const LENGTH_OF_ANIMATION_SIZE: usize = 4;
const LENGTH_OF_INSTRUCTION: usize = 1;

/// Animo Object
#[derive(Clone, Debug)]
pub enum AnimObject {
    /// string
    AnimString(String),
}

/// Animo Model
pub struct AnimationModel {
    /// Anim objects
    pub objects: Vec<AnimObject>,
}

impl AnimationModel {
    /// new with file
    pub fn load<S: Into<String>>(file_name: S) -> io::Result<Self> {
        let file = File::open(file_name.into())?;

        let mut file = BufReader::new(file).bytes().collect::<io::Result<Vec<_>>>().unwrap();
        let remain_file = file.split_off(LENGTH_OF_OBJECTS_SIZE);
        let objects_size: usize = file.iter().fold(0 as usize, |size, x| (size << 8) + *x as usize);

        let mut file = remain_file;
        let remain_file = file.split_off(LENGTH_OF_OBJECTS_TOTAL_LENGTH);
        let objects_total_length: usize = file.iter().fold(0 as usize, |size, x| (size << 8) + *x as usize);

        let mut file = remain_file;
        let remain_file = file.split_off(LENGTH_OF_ANIMATION_SIZE);
        let _animation_size: usize = file.iter().fold(0 as usize, |size, x| (size << 8) + *x as usize);

        let mut model = AnimationModel{
            objects: Vec::new(),
        };

        let mut file = remain_file;
        let _remain_file = file.split_off(objects_total_length);
        for _ in 0..objects_size {
            let (object_size, remain_file): (usize, _) = {
                let mut internal_file = &mut file;
                let remain_file = (*internal_file).split_off(LENGTH_OF_OBJECT_SIZE);
                (internal_file.iter().fold(0 as usize, |size, x| (size << 8) + *x as usize), remain_file)
            };

            let mut internal_file = remain_file;
            let remain_file = internal_file.split_off(object_size);
            let object = String::from_utf8(internal_file).unwrap();
            model.objects.push(AnimObject::AnimString(object));
            file = remain_file
        }

        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use *;
    #[test]
    fn load_no_object_and_animation() -> io::Result<()> {
        let file_name = "./test_files/no_object_and_animation";
        let anim = AnimationModel::load(file_name)?;
        assert_eq!(anim.objects.len(), 0);
        Ok(())
    }

    #[test]
    fn load_object_exists_and_no_animation() -> io::Result<()> {
        let file_name = "./test_files/object_exists_and_no_animation0";
        let anim = AnimationModel::load(file_name)?;
        assert_eq!(anim.objects.len(), 2);
        Ok(())
    }
}
