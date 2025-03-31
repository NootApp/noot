// use std::collections::BTreeMap;
// use std::path::PathBuf;
// use infer::Type;
// use lazy_static::lazy_static;
// 
// pub fn is_buffer_acceptable(accepts: &Type, buffer: &[u8]) -> bool {
//     let guess = infer::get(buffer);
// 
//     if let Some(mime) = guess {
//         return &mime == accepts;
//     } else {
//         return false;
//     }
// }
// 
// 
// 
// pub fn is_file_acceptable(accepts: &Type, path: PathBuf) -> bool {
//     let guess = infer::get_from_path(path);
//     
//     if let Ok(Some(mime)) = guess {
//         return &mime == accepts;
//     } else {
//         return false;
//     }
// }
// 
// lazy_static!(
//   pub static ref MIMES: BTreeMap<String, Type> = BTreeMap::from_iter([
//         ("toml".to_string(), Type::),
//     ]);
// );