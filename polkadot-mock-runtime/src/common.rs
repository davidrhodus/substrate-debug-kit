pub(crate) type AccountId = sp_core::crypto::AccountId32;
pub(crate) type BlockNumber = u32;
pub(crate) type Moment = u64;
pub(crate) type Balance = u128;
pub(crate) type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub(crate) type Hash = sp_core::H256;
pub(crate) type Index = u32;

pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<AccountId, (), (), ()>;
// ---------------------------------------------------------------- <Addres, Call, Signature, Extra>
pub type Block = sp_runtime::generic::Block<sp_runtime::testing::Header, UncheckedExtrinsic>;
