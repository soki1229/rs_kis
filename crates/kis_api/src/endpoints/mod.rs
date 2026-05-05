use crate::client::KisClient;

#[allow(dead_code)]
pub struct Stock(pub(crate) KisClient);
#[allow(dead_code)]
pub struct Overseas(pub(crate) KisClient);
#[allow(dead_code)]
pub struct OverseasFutureOption(pub(crate) KisClient);
#[allow(dead_code)]
pub struct DomesticBond(pub(crate) KisClient);
#[allow(dead_code)]
pub struct EtfEtn(pub(crate) KisClient);
#[allow(dead_code)]
pub struct Elw(pub(crate) KisClient);
