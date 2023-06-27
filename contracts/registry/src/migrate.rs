use crate::*;

#[derive(BorshDeserialize, PanicOnDefault)]
pub struct OldState {
    /// Registry admin, expected to be a DAO.
    pub authority: AccountId,

    /// registry of approved SBT contracts to issue tokens
    pub sbt_issuers: UnorderedMap<AccountId, IssuerId>,
    pub issuer_id_map: LookupMap<IssuerId, AccountId>, // reverse index
    /// registry of blacklisted accounts by issuer
    pub banlist: UnorderedSet<AccountId>,

    pub(crate) supply_by_owner: LookupMap<(AccountId, IssuerId), u64>,
    pub(crate) supply_by_class: LookupMap<(IssuerId, ClassId), u64>,
    pub(crate) supply_by_issuer: LookupMap<IssuerId, u64>,
    /// maps user account to list of token source info
    pub(crate) balances: TreeMap<BalanceKey, TokenId>,
    /// maps SBT contract -> map of tokens
    pub(crate) issuer_tokens: LookupMap<IssuerTokenId, TokenData>,
    /// map of SBT contract -> next available token_id
    pub(crate) next_token_ids: LookupMap<IssuerId, TokenId>,
    pub(crate) next_issuer_id: IssuerId,
    pub(crate) ongoing_soul_tx: LookupMap<AccountId, IssuerTokenId>,
}

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate(iah_issuer: AccountId, iah_classes: Vec<ClassId>) -> Self {
        // retrieve the current state from the contract
        let old_state: OldState = env::state_read().expect("failed");
        // new field in the smart contract : pub(crate) iah_classes: (AccountId, Vec<ClassId>),

        Self {
            authority: old_state.authority.clone(),
            sbt_issuers: old_state.sbt_issuers,
            issuer_id_map: old_state.issuer_id_map,
            banlist: old_state.banlist,
            supply_by_owner: old_state.supply_by_owner,
            supply_by_class: old_state.supply_by_class,
            supply_by_issuer: old_state.supply_by_issuer,
            balances: old_state.balances,
            issuer_tokens: old_state.issuer_tokens,
            next_token_ids: old_state.next_token_ids,
            next_issuer_id: old_state.next_issuer_id,
            ongoing_soul_tx: old_state.ongoing_soul_tx,
            iah_sbts: (iah_issuer.clone(), iah_classes.clone()),
        }
    }
}
