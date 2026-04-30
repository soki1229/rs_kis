use crate::client::KisClient;

#[allow(dead_code)]
pub struct Stock(pub(crate) KisClient);
#[allow(dead_code)]
pub struct Overseas(pub(crate) KisClient);

// src/lib.rs에서 pub mod generated로 선언됨
// src/generated/stock.rs와 overseas.rs가 이 구조체들에 대해 impl을 수행함
