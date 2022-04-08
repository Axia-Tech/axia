use hex_literal::hex;
use sp_core::crypto::UncheckedInto;
use axia_primitives::v1::{AccountId, AccountPublic, AssignmentId, ValidatorId};
use sp_consensus_babe::AuthorityId as BabeId;
use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use grandpa::AuthorityId as GrandpaId;

pub fn endowed_accounts() -> Vec<AccountId> {
    vec![
		// Sankar//stash
		hex!["3233f745d0860ed64ae9c7f4ea5c0773316fc9265199f312d3f6e8ce08255c10"].into(),
		// Arun//stash
		hex!["445f574d57f768ea7e1a2f551bef4298ace99d8895d316352cfc02aececcf26c"].into(),
		// Rakhi//stash
        hex!["c2312f7f9a8190bf76db9dc40e5ef351c4c23e3ae6540932bf2c2d485289c37b"].into(),
        // Priya//stash
		hex!["688d6fa54d9ace0fa07492f3d8dfef78594130719e61c213d700c62421177c38"].into(),
    ]
}

pub fn initial_authorities() -> Vec<(
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ImOnlineId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
)> {
    vec![
		(
			// Sankar//stash
			hex!["3233f745d0860ed64ae9c7f4ea5c0773316fc9265199f312d3f6e8ce08255c10"].into(),
			// Sankar
			hex!["3edc55e451a46f7d2ec513fb40b3687b9a03fc32e16274d37f332205d6413945"].into(),
			// Sankar//babe
			hex!["b609973a7b7e1468afda62babf32844e387d31dbff7e046412ae8a18f3452971"].unchecked_into(),
			// Sankar//grandpa
			hex!["161a16204ef752deeb0002578456e77df6caa89ec463a1bf471533e93369e2b8"].unchecked_into(),
			// Sankar//im_online
			hex!["9a5d57e1433b8f667851beff9be7ed25f1d32e6f3d4c03fa8da21506e20a3c6c"].unchecked_into(),
			// Sankar//para_validator
			hex!["368f272ba94824fb2589d01074bcaf483ced022734b299f49ac66a61c2bb961b"].unchecked_into(),
			// Sankar//para_assignment
			hex!["6221352ed419451d86fda2481de68986bececd16fe59c274ec4de50ee5bf3810"].unchecked_into(),
			// Sankar//authority_discovery
			hex!["76e6cdd1f98d574da88af6920dc870ab6a356151d6a60d4e0b4bcbe0b4578c16"].unchecked_into(),
		),
		(
			// Arun//stash
			hex!["445f574d57f768ea7e1a2f551bef4298ace99d8895d316352cfc02aececcf26c"].into(),
			// Arun
			hex!["d05d1412507f428a1f426e9358eaadb73f4604e9abadf3a98c26af0422be2e17"].into(),
			// Arun//babe
			hex!["54097885aae3ab47c7eec090b7dcb672778f3362f41d318ecc10c889ec0b2652"].unchecked_into(),
			// Arun//grandpa
			hex!["14aa4f9c765360d5d408f1fd6563612bd30a9f50d1b3f37ac096d0a4671bcad7"].unchecked_into(),
			// Arun//im_online
			hex!["2a8acf53b52d4fd54aad5b05afd5e87cbbfffd42ed29459639ff91de03a4c167"].unchecked_into(),
			// Arun//para_validator
			hex!["7c4ecd0a349aa899c8fa35a7bba8225ddc0ce6922a4a480c00b2a90ef9abef07"].unchecked_into(),
			// Arun//para_assignment
			hex!["7293fda5944b0045d6e50743835baf0329275580ee2837a94a7824e20bb7d77a"].unchecked_into(),
			// Arun//authority_discovery
			hex!["f68863365282f034e86650515dd9ee1962e7f8c534287121aeaa203ee213cf18"].unchecked_into(),
		),
		// (
		// 	// Rakhi//stash
		// 	hex!["c2312f7f9a8190bf76db9dc40e5ef351c4c23e3ae6540932bf2c2d485289c37b"].into(),
		// 	// Rakhi
		// 	hex!["82c14ac892565d92d7a7f4e38dddf7edbe07295d08a6d3e65fa19585a43bdf0c"].into(),
		// 	// Rakhi//babe
		// 	hex!["ba8331b4dd6073bd0de45fcbdf6142b402ae7b1e02335b7318e5856a0e3dcd77"].unchecked_into(),
		// 	// Rakhi//grandpa
		// 	hex!["b34908f44b24b3052d2324a06d12a564350083b03441bc204c8acd8642b7c844"].unchecked_into(),
		// 	// Rakhi//im_online
		// 	hex!["14c233dfa06d9153ab014074391c659bfca62c730655409b51209f66587f2467"].unchecked_into(),
		// 	// Rakhi//para_validator
		// 	hex!["4ac7bdfc1b8b8c3d2e7f4c4bd8372077f9e9edcc95368a45d5607b7d75411c0a"].unchecked_into(),
		// 	// Rakhi//para_assignment
		// 	hex!["02167bdafcbe516376ef3c24086ccb2fc06dc80fd05e553dbd53cc7b75b70335"].unchecked_into(),
		// 	// Rakhi//authority_discovery
		// 	hex!["183c3a5c383f78805c5f6b60301939b536be4c3254c8ea3012ecb81d5e290279"].unchecked_into(),
		// ),
		// (
		// 	// Priya//stash
		// 	hex!["688d6fa54d9ace0fa07492f3d8dfef78594130719e61c213d700c62421177c38"].into(),
		// 	// Priya
		// 	hex!["b04155fab96288008150d78409961051d6e4e7d2b5d4bc7c51f6cf55699aa161"].into(),
		// 	// Priya//babe
		// 	hex!["74477e67460cab9167678aebbf746cb9a2a9d33148ae1f6bf12f7f83c179c75c"].unchecked_into(),
		// 	// Priya//grandpa
		// 	hex!["687e40323706755ab58d6a1b31aabedde5fbde9f682277df587af1b4e8616847"].unchecked_into(),
		// 	// Priya//im_online
		// 	hex!["4a37d8530ba98c21b90580e16a2d56b7e931692067ef096d990700c59ecd515f"].unchecked_into(),
		// 	// Priya//para_validator
		// 	hex!["2c7aa0bf1337b32c2edf66388c928cc07731efcca87160b2c5c213765b44255e"].unchecked_into(),
		// 	// Priya//para_assignment
		// 	hex!["ea610dcfc75cb6a33937dbbc50c0722a6e7992e5cec70096c086bc1e6e4ef83b"].unchecked_into(),
		// 	// Priya//authority_discovery
		// 	hex!["0c1ecee6c2199e514f838a5fe93ddff8722423d20aa190a3563b7bff9100fd17"].unchecked_into(),
		// )
	]
}


pub fn initial_authorities_beefy() -> Vec<(
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ImOnlineId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
    BeefyId
)> {
    vec![
		(
			// Sankar//stash
			hex!["3233f745d0860ed64ae9c7f4ea5c0773316fc9265199f312d3f6e8ce08255c10"].into(),
			// Sankar
			hex!["3edc55e451a46f7d2ec513fb40b3687b9a03fc32e16274d37f332205d6413945"].into(),
			// Sankar//babe
			hex!["b609973a7b7e1468afda62babf32844e387d31dbff7e046412ae8a18f3452971"].unchecked_into(),
			// Sankar//grandpa
			hex!["161a16204ef752deeb0002578456e77df6caa89ec463a1bf471533e93369e2b8"].unchecked_into(),
			// Sankar//im_online
			hex!["9a5d57e1433b8f667851beff9be7ed25f1d32e6f3d4c03fa8da21506e20a3c6c"].unchecked_into(),
			// Sankar//para_validator
			hex!["368f272ba94824fb2589d01074bcaf483ced022734b299f49ac66a61c2bb961b"].unchecked_into(),
			// Sankar//para_assignment
			hex!["6221352ed419451d86fda2481de68986bececd16fe59c274ec4de50ee5bf3810"].unchecked_into(),
			// Sankar//authority_discovery
			hex!["76e6cdd1f98d574da88af6920dc870ab6a356151d6a60d4e0b4bcbe0b4578c16"].unchecked_into(),
            // Sankar//beefy
            hex!["02db0cd689ff22ba985bc52cf37899a3b44d53f79fe68d241fae2c058da685e0bc"].unchecked_into()
		),
		(
			// Arun//stash
			hex!["445f574d57f768ea7e1a2f551bef4298ace99d8895d316352cfc02aececcf26c"].into(),
			// Arun
			hex!["d05d1412507f428a1f426e9358eaadb73f4604e9abadf3a98c26af0422be2e17"].into(),
			// Arun//babe
			hex!["54097885aae3ab47c7eec090b7dcb672778f3362f41d318ecc10c889ec0b2652"].unchecked_into(),
			// Arun//grandpa
			hex!["14aa4f9c765360d5d408f1fd6563612bd30a9f50d1b3f37ac096d0a4671bcad7"].unchecked_into(),
			// Arun//im_online
			hex!["2a8acf53b52d4fd54aad5b05afd5e87cbbfffd42ed29459639ff91de03a4c167"].unchecked_into(),
			// Arun//para_validator
			hex!["7c4ecd0a349aa899c8fa35a7bba8225ddc0ce6922a4a480c00b2a90ef9abef07"].unchecked_into(),
			// Arun//para_assignment
			hex!["7293fda5944b0045d6e50743835baf0329275580ee2837a94a7824e20bb7d77a"].unchecked_into(),
			// Arun//authority_discovery
			hex!["f68863365282f034e86650515dd9ee1962e7f8c534287121aeaa203ee213cf18"].unchecked_into(),
            // Arun//beefy
            hex!["024fd190ac32259487de7d82cc41b80181aa09b3eed8ccb43a572e8929cc8b284a"].unchecked_into()
		),
		// (
		// 	// Rakhi//stash
		// 	hex!["c2312f7f9a8190bf76db9dc40e5ef351c4c23e3ae6540932bf2c2d485289c37b"].into(),
		// 	// Rakhi
		// 	hex!["82c14ac892565d92d7a7f4e38dddf7edbe07295d08a6d3e65fa19585a43bdf0c"].into(),
		// 	// Rakhi//babe
		// 	hex!["ba8331b4dd6073bd0de45fcbdf6142b402ae7b1e02335b7318e5856a0e3dcd77"].unchecked_into(),
		// 	// Rakhi//grandpa
		// 	hex!["b34908f44b24b3052d2324a06d12a564350083b03441bc204c8acd8642b7c844"].unchecked_into(),
		// 	// Rakhi//im_online
		// 	hex!["14c233dfa06d9153ab014074391c659bfca62c730655409b51209f66587f2467"].unchecked_into(),
		// 	// Rakhi//para_validator
		// 	hex!["4ac7bdfc1b8b8c3d2e7f4c4bd8372077f9e9edcc95368a45d5607b7d75411c0a"].unchecked_into(),
		// 	// Rakhi//para_assignment
		// 	hex!["02167bdafcbe516376ef3c24086ccb2fc06dc80fd05e553dbd53cc7b75b70335"].unchecked_into(),
		// 	// Rakhi//authority_discovery
		// 	hex!["183c3a5c383f78805c5f6b60301939b536be4c3254c8ea3012ecb81d5e290279"].unchecked_into(),
        //     // Rakhi//beefy
        //     hex!["0206355db3b12fbd2d7bcef10c65eb302d412b91381e92aefdbe4e63779796ce8a"].unchecked_into()
		// ),
		// (
		// 	// Priya//stash
		// 	hex!["688d6fa54d9ace0fa07492f3d8dfef78594130719e61c213d700c62421177c38"].into(),
		// 	// Priya
		// 	hex!["b04155fab96288008150d78409961051d6e4e7d2b5d4bc7c51f6cf55699aa161"].into(),
		// 	// Priya//babe
		// 	hex!["74477e67460cab9167678aebbf746cb9a2a9d33148ae1f6bf12f7f83c179c75c"].unchecked_into(),
		// 	// Priya//grandpa
		// 	hex!["687e40323706755ab58d6a1b31aabedde5fbde9f682277df587af1b4e8616847"].unchecked_into(),
		// 	// Priya//im_online
		// 	hex!["4a37d8530ba98c21b90580e16a2d56b7e931692067ef096d990700c59ecd515f"].unchecked_into(),
		// 	// Priya//para_validator
		// 	hex!["2c7aa0bf1337b32c2edf66388c928cc07731efcca87160b2c5c213765b44255e"].unchecked_into(),
		// 	// Priya//para_assignment
		// 	hex!["ea610dcfc75cb6a33937dbbc50c0722a6e7992e5cec70096c086bc1e6e4ef83b"].unchecked_into(),
		// 	// Priya//authority_discovery
		// 	hex!["0c1ecee6c2199e514f838a5fe93ddff8722423d20aa190a3563b7bff9100fd17"].unchecked_into(),
        //     // Priya//beefy
        //     hex!["03878bd981cddf29afb33c40195ac69ccfe19e3aa466924f0396fa70dd8af91176"].unchecked_into()
		// )
	]
}