#[cynic::schema("mina")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct BalanceQueryVariables {
    pub public_key: PublicKey,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "query", variables = "BalanceQueryVariables")]
pub struct BalanceQuery {
    #[arguments(publicKey: $public_key)]
    pub account: Option<Account>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Account {
    pub balance: AnnotatedBalance,
    pub nonce: Option<AccountNonce>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnnotatedBalance {
    pub block_height: Length,
    pub state_hash: Option<StateHash>,
    pub liquid: Option<Balance>,
    pub total: Balance,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct AccountNonce(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Balance(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct Length(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct PublicKey(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct StateHash(pub String);

