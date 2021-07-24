//! Chain specifications.

use cumulus_primitives_core::ParaId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use shiden_runtime::{
    wasm_binary_unwrap, AccountId, AuraConfig,Balance, BalancesConfig, GenesisConfig, ParachainInfoConfig,
    Signature, SudoConfig, SystemConfig, VestingConfig, SDN,
};
use sp_core::{sr25519, Pair, Public};

use sp_runtime::traits::{IdentifyAccount, Verify};

type AccountPublic = <Signature as Verify>::Signer;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Gen chain specification for given parachain id
pub fn get_chain_spec(id: ParaId) -> ChainSpec {
    if id == ParaId::from(2007) {
        return shiden_chain_spec();
    }

    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                None,
                id,
            )
        },
        vec![],
        None,
        None,
        None,
        Extensions {
            relay_chain: "westend-dev".into(),
            para_id: id.into(),
        },
    )
}

fn shiden_chain_spec() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../res/shiden.raw.json")[..]).unwrap()
}



pub fn staging_spec(para_id: ParaId) -> ChainSpec {
    use sp_core::crypto::Ss58Codec;

    let sudo_key =
        AccountId::from_ss58check("5GrootH4UVFfSXJKLf5Rt1PtZ9HFBxGsUqnx7em9saHymCLY").unwrap();

    ChainSpec::from_genesis(
        "Shiden Shell",
        "shiden",
        ChainType::Live,
        move || {
            make_genesis(
                crate::balances::SHIDEN_HOLDERS.clone(),
                sudo_key.clone(),
                para_id.into(),
            )
        },
        vec![],
        None,
        Some("Shiden"),
        None,
        Default::default(),
    )
}



pub fn staging_network(id: ParaId) -> ChainSpec {

    let sudo_key =
    AccountId::from_ss58check("5GrootH4UVFfSXJKLf5Rt1PtZ9HFBxGsUqnx7em9saHymCLY").unwrap();

	ChainSpec::from_genesis(
        "Shiden Shell",
        "shiden",
		ChainType::Live,
		move || {
			testnet_genesis(
				sudo_key.clone(),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					
				],
				vec![
					crate::balances::SHIDEN_HOLDERS.clone(),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
				],
				id,
			)
		},
		Vec::new(),
		Some(
			TelemetryEndpoints::new(vec![(POLKADOT_TELEMETRY_URL.to_string(), 0)])
				.expect("Polkadot telemetry url is valid; qed"),
		),
		Some("Shiden"),
		None,
		Default::default()
	)
}



fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 10 << 60))
				.collect(),
		},
		sudo: SudoConfig { key: root_key },
		parachain_info: ParachainInfoConfig { parachain_id: id },
		aura: AuraConfig {
			authorities: initial_authorities,
		},
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}




