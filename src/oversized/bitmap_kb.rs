use std::mem::size_of;

const NUM_ELEMENTS: usize = 1024 / size_of::<usize>();
const TOTAL_BITS: u64 = (NUM_ELEMENTS as u64) * 8 * (size_of::<usize>() as u64);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct BitmapKB([usize; NUM_ELEMENTS]);

// An attempt at serialization so far, no idea how to implement deserialisation yet
//
// impl Serialize for BitmapKB {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(NUM_ELEMENTS))?;
//         for e in self.0 {
//             seq.serialize_element(&e)?;
//         }
//         seq.end()
//     }
// }
