use anyhow::anyhow;
use codec::{Decode, Encode};
use std::{
	collections::BTreeMap,
	fmt::Display,
	future::Future,
	pin::Pin,
	time::{Duration, Instant},
};

use beefy_gadget_rpc::BeefyApiClient;
use futures::{future, pending, ready, FutureExt, Stream, StreamExt, TryFutureExt};
use grandpa::BlockNumberOps;
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ibc_proto::google::protobuf::Any;
use polkadot::api::runtime_types::{
	pallet_grandpa, rococo_runtime, sp_finality_grandpa as polkadot_grandpa,
};
use sp_runtime::{
	generic::Era,
	traits::{Header as HeaderT, IdentifyAccount, One, Verify},
	MultiSignature, MultiSigner,
};
use subxt::{
	rpc_params,
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams, SubstrateExtrinsicParamsBuilder},
	Config,
};
use transaction_payment_rpc::TransactionPaymentApiClient;
use transaction_payment_runtime_api::RuntimeDispatchInfo;

use primitives::{Chain, IbcProvider, MisbehaviourHandler};

use super::{error::Error, signer::ExtrinsicSigner, ParachainClient};
use crate::{
	parachain,
	parachain::{api, api::runtime_types::pallet_ibc::Any as RawAny, UncheckedExtrinsic},
	polkadot, FinalityProtocol, H256,
};
use finality_grandpa_rpc::GrandpaApiClient;
use futures::future::{pending, ready};
use ibc::{
	core::{
		ics02_client::msgs::{update_client::MsgUpdateAnyClient, ClientMsg},
		ics26_routing::msgs::Ics26Envelope,
	},
	tx_msg::Msg,
};
use ics10_grandpa::client_message::{ClientMessage, Misbehaviour, RelayChainHeader};
use pallet_ibc::light_clients::AnyClientMessage;
use primitives::mock::LocalClientTypes;
use sp_core::ByteArray;

use sp_finality_grandpa::{check_message_signature, Equivocation, OpaqueKeyOwnershipProof};
use subxt::rpc::ChainBlock;
use tokio::time::{sleep, timeout};

type GrandpaJustification = grandpa_light_client_primitives::justification::GrandpaJustification<
	polkadot_core_primitives::Header,
>;

type BeefyJustification =
	beefy_primitives::SignedCommitment<u32, beefy_primitives::crypto::Signature>;

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct JustificationNotification(sp_core::Bytes);

#[async_trait::async_trait]
impl<T: Config + Send + Sync> MisbehaviourHandler for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	sp_core::H256: From<T::Hash>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
{
	async fn check_for_misbehaviour<C: Chain>(
		&self,
		counterparty: &C,
		client_message: AnyClientMessage,
	) -> Result<(), anyhow::Error> {
		use tendermint_proto::Protobuf;
		log::info!("counterparty: {}", counterparty.client_id());
		log::info!("client_msg: {}", hex::encode(client_message.encode_vec()));
		match client_message {
			AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
				let justification = GrandpaJustification::decode(
					&mut header.finality_proof.justification.as_slice(),
				)?;

				let encoded =
					GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
						&*self.relay_ws_client,
						justification.commit.target_number,
					)
					.await?
					.ok_or_else(|| {
						anyhow!(
							"No justification found for block: {:?}",
							header.finality_proof.block
						)
					})?
					.0;
				log::info!("encoded: {}", hex::encode(&encoded.0));
				// let encoded =
				// hex::decode("
				// fd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf6190d1300000000000000fd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf64b02000018fd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf64b02000033d2baa7d4efca99f10019c0706f2495e4de687b88212133bcbaf73eca57240ea57e98c6333bf37db2d8dfa2b9bf6159a28d55fb73d993b576e12e1d59d99c0b1dfe3e22cc0d45c70779c1095f7489a8ef3cf52d62fbd8c2fa38c9f1723502b5fd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf64b020000b0d5ec1f7581aa56cfb532aa560cbfddb2fc630249140248d93a80c4451362d698dd614c34c96c0534076603091197a63372342ab071aab17d2974b3b8f65a09439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234ffd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf64b020000915acd1101aa210d8c240efdc778494b47d376f77d5dc8df5e814e238d5739ab4bbd7e4ed38eea4704d69bf8b4bdd37973c1c11c4bb6c3b1d43c0a989fe71506568cb4a574c6d178feb39c27dfc8b3f789e5f5423e19c71633c748b9acf086b5fd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf64b0200006ae94519fee84d17657b917af0cf7cb5a3f18858d789efbc610245982f30838a5aff1a36a702d25a00e25587f6a94d20414f32252ca232dfbf59304bc6b7ac045e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d9fd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf64b020000b8cd9c77befd02ca1f01904213f1ccbcc663defb8aadf89be49d70921d1de00b8dccf225739ae0b68ca0152b583d6d377328a9c8c0cd6cd4eaa023a41e7e3d0788dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0eefd604b5f3fd9c851f3504185827c70006770208c16302b85625378d44ae69cf64b020000afffbead55fff06a2f9a73bb3e466e4ab9adf31c92b393196a8400a46baf727223de7b0fb2561409f408830bbdb4f8bda2bf32f59e40710ae50590b9a6433905d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69005c543383ae1df6d3d03cacecdd128bea2f93b73d17c392e49f5ee04cd42977c8ffd508d01afc810654083c0e62f5b2a3327a28ff5da17aa220f7a82ffc8dd114bf210298890684e16e4681ebd047278648ecba91e0e076e2b9e868b883f6b86a8539d80c0642414245b501030200000049298d100000000014b640bbdac1b354dc42a2f08cc5528360e27f9b2690aa221a0e566f1a8cae2bf40d9d96fe7418ce6eef70135e312634bf44b1d14f8f346651bb3775772a710f1ac6a5c4c500f90554ad0d1f78ca1c3a93640b290bad53fa179412ff5591cc0a0442454546840334678d7fa67ca87ca7170835429cb159bbf09e66df9610c45b5c5d000da51eac054241424501016096fbb9de543ab001728e55a7fb666d9e13a3a02db97ae925e56be654e5a232c18477f952b6acf074948ca4f11922f4bf87bec8d2f110d06c9fe843bc22a086d06fc5a8481929633102832430176c1a315fb949befe79729a20135bb3c13353d908c708daaec6d9be73bc2f265d0d8a63f11dc45d372f73a1780c65ae409cadf24a3866fba7dfe23c4e9ac328d3b3ad590550768c85f4c0333a65faa13d53b7344a0c0642414245b50103050000004a298d100000000098677cb88d1b0a15332f2194e26355ac28fe45a53f638ac0e411a31995d41a3a068b4073aaac49a438d25d2ab472aa6070b3f06b2bf5f13bd186f290e2083303c060d757e796a0b11cf3fc2a22fac651c076b12f2aff9e40a0b4e74b5b523c0004424545468403ff6898b3785ca01c5705997d517cbcbd0b2ad58c7489d1627af32cc3cc1d3a1805424142450101d2dd6c6d45f6b210315532d16c2d4547a63ed4ca3be54fbb2b5ef89da4c83c4a5969273614c9c8bc2213205228429d2d1334e1e9aa8c2d2a428bf2dee9d3568e6acb8676e68f66c1730befcfb46fdff41e522855e4ea57e22d0a6b8806726a43dd0856bedd2d7bbadbde78f856cdcd77b09acabfe890a557fc753ce8bc01973f041bf05284d1fdff7c20bee277a6e14342bbcb2b82a277865ea0de2ba77a5f0c5d0b0c0642414245b50101020000004b298d100000000042da33dbea88aedb714a1b4046e6f3777ffafe952e4382b4c2aca787d6e7ac7e517331abcc924869ef134b73b1a89ca22119cb32474a6bd5f9d071d30b4e5a0d85141be8cb6efed0b7ec6cf8902353f2d58df8f8de2487beb8a03c10a810980d0442454546840346b14cf8876d5cfcb2bdc854e70a5f6a1fb2a456df8f78d05efd0bf780d9f49305424142450101f4b5decf657b21d3ea8eb5ca6b6877040462f05bd1ca62da14110950416fb0758ea0d37cf04c1852354984cf6d7050063eb8c0b5bc1393cd47df826c3f7c5a841dbf56c7803f21f3083463dc834d7673d0504b6ac046d2df94d7c2f69e7bcca2e10870383ad8f2a878cf7e25269dae88539c7a02537869c0abdfe677f8bbf32b18a702524b56d3c014d229ea78a9f203a0b797ef5920b96fda896b5cf30546ac07cc0c0642414245b50103000000004c298d1000000000f49a672a12058fae11b44e6e9bdd990b0a857d9fdb4eaa2c1bf9c3cdd5e93e4bcc5f5339d1e26d253f559ce2675e64577db90eeee46aa2be17a67fcf2451060d75cf550ec5ea221bb3a73d15f993879734341dddcdcbdd5b2e6aea21f5fcca0404424545468403b8c12f66abac15aa810f20d6156d4090298425221889b31404f73769095832b8054241424501017091f68d1092b7c77fa36279923259138aea569c88796cd08e2a32ce4a41ee6d12c5d3ffbe78c181e8da67a4dbf88fb5e69617ffe22c8fd3120c530c79c0908949f501be43b48b17014808b2aa67b332e79e825545c2d6850c23f55116785b89e5085909e8e9c12382f3cf608354a1c1835118b849c56957d46150ec8bdf2a55536ada04cef08ef1d9b2f8b27dc56613e2182ccdecf760ebfede4c72d0034459c4580c0642414245b50101020000004d298d10000000009e3bd67b41b15e23af7477b0c62e7eaa7d62be401e57bbaa1b99feb10cf24c79a2d16799170fc613860bff19501e12ad1b5c171aaf97335aa540a3b262f93305a9484bc54a5ba99e223c9ad9f60f4592276e1ee9725a6f7c4ad011f1425cd90d04424545468403f2f83f6480d5bfd604b81c8cdda5d6b9ea00535d9643c10e574e7ec74f4a2c3f05424142450101049ba73622ca10b29c7cebce4da99b5b25d11b9b5a510fd8f781263654c2eb20d5697c940bfaa2f489ae88a060ef4bba31817c3c51473080c3c23e692678318788e2dae4951adbe1d940fd6b36a6e20edbf6a4915070f3f0e388557fbd7495d5e90832c5a43ad6f6c2fa215346c3b2800273f226928f4336a7ea1e2dd9bdac7189b29ba6dbb60a9db0855a2419c6796645c2287169fc5b961b53b06cfc79bd2108c40c0642414245b50103030000004e298d1000000000641861613ae6d1c71edf9fb496d6abc9b456aec1db485ad8ea9816a6ba71784fb0757d442e0fdca0e5caab4cf2144549a92211aa3968c4fe6484a2b9a972a80c84b28e47dc172e988f5eeeb2f4625c2d5a59f09472be8c2c6147b09849d9a701044245454684030b8430634476c39cd3035d8cdeceee7d69186619063ae229beb2aaf830e4c56905424142450101f2261e8b3f9daf0076054e2656723cde0f182ae0ebb8d5964fa30dca27f280762f5486a25d586b8b38087399ce7a9e5e0b26f802107e82ad94b1b92887faaf85d1ad08dbd6c7eb2060ab387482d8329e1266d059dd5a36178e6661c94774aa95ed08c42ac0934cbc572c92713f1ce42f619abcb367fafa9d8851dba67c70fc7670500396a159b94f22c91ae4066fc5e9005f45f38d35420561cb6daff9444cb4a9cb0c0642414245b50103020000004f298d1000000000348740fca48eecadea414bbf3fbb6c039ef8a9f2dcd7579b207311ee7190a44e29594c103f9e8a8b3749831f5f8867c531795892ce538867f6d8e011054dbc0eb4f88c78e3afc8c892fb8e036719c79d50193c6a2d581b4fb9165f08c4a2420504424545468403e50914182ac653a3711ca91163b2f75460662302ea38705bb05b99e776994d4d0542414245010158b049553a8a48399c55fa49d0b412b899baa9c25dc0203e1fa38af714e7d0012083494417cbf241b47b00f7783b2355352a184cc5e8ee82cd2da9bf4b22c38dffa16180dc3100aa75efb8821dac38da3daa75119cd36b56317b62f074852117f10880b2019dcac3c1f8e2bf0aae0527a3567565c2df49c45d7cda62fa67a6a5d5c9d89904a28c03c8ba83aaead46911e3cf7fb79f6a049463a575a8276283f025cc0c0642414245b501030100000050298d10000000001e1915745901eac9b593c5bcf353d6e2742aa7226551e1607b13aeca4ab64c0e547336ee86fa06a0acf72567f9ceea1c3666825e1fe06af53625e112dbc9890ab34cc6a282ae49b5f02e167bc429a2ca882add9a969268f5fb1ae97dbe146d0f04424545468403dcf5da7fda7022fe9de06234d5080949ff62002e958ce1e724cd08133252fa2d054241424501019aba7409a6277ea3d55c31a1ff667016f6bb9054b89e42da01a0885a16fb9217ac7a6b135893a94324ef21d453436694a10639da9bfeab8a7a3c213cae2a66888d792cc9e80bd0841fd9b58dc7db7e8c13bcf9baff2d0c3613b7ea63620f9276f508d7e2363e9db377128ff601524dd6450cd86b2fa51d122b4ca4b3972425bd953ea728d9ff78f0c5fca156f9ad53cf4d8f907ae4753f8dc9eb504200975f35c54d0c0642414245b501010400000051298d100000000046d50e8d5696d8a54b5b14e4bd9e89ace3d07842e209ac716b62f91a7a8b5d6359a0f2e2a8c030321893b9add7ed41c6f8c1dedaeba3b7dba92ec3034b05440ba3a6cb54f7344bde5a711bf9c8e4617c3d5ddc93c7ed3af8adf7c21c543d4201044245454684035ccf2536a31ee1ee3d178a72c2ab9b152b3ccc2199ffd1528cd2ae4a2860b8c605424142450101fcdfa5375fb823cd702ea26289a4fe2f987794440e440e0b1c5610eae79aaf02900b3962b4926408c31da6a9adf12ed18007dcce651f0a0ffd834d62389a07819d9adb2ecba8ff573d2791196dc0883f5fc25471c75a6f98f0a098d051932112f908cf2c796c540c905c709430180243f36eff1421748fe002ec33f956f53aee32bd94db1377ec2038644a70881aa24b34b7df667af944078997f75e9aef8d339e110c0642414245b501030500000052298d1000000000501efd41a2386d40ca69b5e80b72a50171940b348a592d5ddc36fa5761175b23c07950376ce636fdc4fcb8202a33bbc6dd15af390a82b67b27ba85d53b62830bfffcbfaacfb939a8fa10d866e8f1294326936d4a839dc8416c7ab50c5f81790904424545468403b9166804915da789e4c665655f09e7d49045ec2a8bfb5b91307afd3d4125d79e054241424501015e753eaa2a8a9f5d0a96407498fd1a9b9c4e4c034335f31c5e16232c70a3e0443fbde121507a593649698e728903d450049117e02ffc9075a1a58f0ef2a2a38fbccc75cee0add229f35c33ae2a3195f3f6b75519f95fff37a630b96ed945d500fd0889c015226962e7cdb19f7fd7a3a0b7e5c03224bef00a768f5d13006586a7b7edb280722dd140d682bc8af49e9fa8d4183c9a33458d02d075c8a257477eb149140c0642414245b501030500000053298d100000000072578fb7758767fcf2a69e158fbea14322efaa3528fd29e9db1cc70cf6d2f009b6a403a6bfdef9ce1487c37ab59116218ce890c6db5c892df1165d7b5b564d01c2fa50c17f0bcda517abfbca8ab27b4eab5f62a9dbb77fc52722653696d98f0504424545468403548c36e8373aeb7999102460b01fae879f78d8a717896c2248f75f76e46429a805424142450101b0813a82e0f757773ac3137045e57b67e1463d9ba33f8130508575c0b494484b92fd2e75c8156485a7019c63a74641f17e5139916c44a41f0736363f97cd9f88c5449343d4ec5820740f8d7a953c578bf552c91d62ed19b2ce85561e33cf769d0109e53bba0d5ca4ca3b6c2f2936f35ae93b72c87fd4bec73bcd2df7f19d64fdda436358483ddde64b9cf3746641da5a294351ccc73ff0f84e6f876bd41c017117ab0c0642414245b501030200000054298d10000000009cbdbcb0de40909caedade587347a2ed41cdd083c8385321195322a461456616415dfe07a5aba8516927c0819ab3e0ff8b300dafe21666aa1fbb4d91204fc80598e85c2920f4a6371f2affdae693cc1db7e2db833f3af0f23aa73c71188b4b08044245454684036fad55e35b9ca16a3f5f715376aad575a91eca3a872ba86e59972060d1bc12bc054241424501013888cfede63fb41525e4a615b1fe2ec4fa527e263ad5060aca058562a9b8d34a5b13c5d6d9210f81578bd603a9e60bc1a2e21c1894e2c721df42d81a56cda1883b4dfbf8d948b19b3c83dcd7aab1a4365dcff20efc92d656b0861bac64982e250509588c49e26246b650fcbbfa7ef3b69664dc16e1d2901fb660bf53ca6853b532d23197888f22e46aee2d4dbb4c75f4eb3de91ff19d6a290cde98777a89a345ec540c0642414245b501030000000055298d1000000000a40a6fbff0463441859fc71cb6c26b5195cc39abf58ef47a0ca0dde598e92d48583c6ceea28e7b2a00319849a19f995b724486355b8e0a20a794351c9680f60f89350f886fedd163b149ade889cf80d8086a772ed73f9be1e15e7ee93da0ab0c044245454684037bb3decf36f12cbe849bf3634923f11cc7b6c995caa7f558445f8bebe36d5deb054241424501016e1c7d68abc09f392720df60f6ce087335aaeb74e71c14e3e2999651b079917bcca1e7eb58d8c78b02061762520e67bb3a24802fd9f1510ec0fb4156f7215f8d88d4a300f78309bb980324b753ff65c377075f6e5b4e33db93b1e26dad5526370909590014c070b1a2bf651f590910bee426353639753aa6354d0918d5a747036076d06ff11786bf34ead40b7386992ff18fc61e41dea899d1440a0f27a2fa5e26490c0642414245b501030500000056298d1000000000e84cf9eb44432f1aa1c290feb0349d1dd51e92c5ca1097dd77d94c988630b27f95b0003841d24d51280d89006f5cb81bf4030d84f988e0437c70e630d15c4a0a41c7292879f95f18a41442ffd7224d3f8df7da881668721d7c3a9c6228b8de0c044245454684035fa4ddcad9471d091a0aa33d33c571baf72f84578d0270d0db657f2d684aeea80542414245010140fb9ad4447d3af87bef12057318db1c1e9e13f079b2c3fa6e671a430ea1ef5de08c91bf845b10f557e7292b1c4780f3d8b1a89dec4caa6c21203ddb3e4a7d80be89059fec9cdbb963ad0f27d6ee3ad44c6a28084853481bce2f3bcea71361a80d098faeb2e4c69b003ba5571768918fb833dce467237fbcfb24b4138768989b37eb556fdbc9527db2cbad9b76d42a3ab8a214faa6abab961e0efb0cf85ceca05ab90c0642414245b501030200000057298d10000000008a1b2c8de7e83e73c4afca95e6e06e8207073f647d8e410fed5a35be07963438100ad29f5c1aed721c3aecf21ae4afd3304b2ec1a255f7c716519014a0550e055668587009066c17ca2ed6a7c2e9f3eeaf8df346d5c5565e0371eec71ec4f40204424545468403e764e47e4ece37bef31566e7453afddaa14a4f80bd02ca4e1eb313334839d02b054241424501016ae1fc1bb4fce0a73184cfb0461481406a78586fad5bb8b8bfb86db13bb4fb7158669266a539ce75d3224a9b32ccea08a91032b3c4698be156f1ce36599c858f8141418f4a116187376e5e0ffe39ca5fceea7fe02f5fc23a6e67cd13c00444841109fe37832b666e7be1ed0e10f546b36800bafd49c4084391a8a7c7d79e5c3edc744a6f98bf5737eb7a25e48558c972f8c334ae4a63c13a5cebc22de5e454d87e920c0642414245b501030300000058298d1000000000ee9bec5996b8126fb7115dd628b73f58a3d10fca72a769dd8824e34f1c9c797ab8d03352240cba90f0ab6689f27d7bb74ca59b8d81cdf4d0a85065fe314140015f73a02d0b8624268b505f1ca6f5f8f14701e9cf63275d5cee9c369ffe99850204424545468403b8366f24e974561d9e095acf32eba4cfda1579e27b78d6765402da5e2bf68fda054241424501018001c8c2ff47a5d3d5d31b3a3ad153e5d201d9c037be34864bcca430cb3aaf7c2aefd89faf3a7e4e8ed89c36911ddc7a3425db212821b683c3109b9658fc4b8ff3db5bc2190a11cc3bd755a39dbee298fd4a1e99befc18829969382e1e4397d315093654bbb927594cbe72daabf783ed60a546955c7ed36af18cfc62eab991584a101998406b311697eb267ad6390a13afc13cfcce6a284d3b6e401b50a49afccfa70c0642414245b501030400000059298d1000000000901f63ec9dbd7c01e71233909b432e02126a6c1a80551ba10b19a92bfa20d473fc56e6d71d9c4289f87d5094aa3699ab730e9a83b431776b9c08974751f4a501ce8d58520372a167ee6883643499b6708aade80d8f617a1f80f21d6b02dab00a0442454546840317740b3bf879bedd42e1abd9517ca3310b255954c62970de2055cfd61b67c31205424142450101d4e381c276753a363f48230fb2370e5d08b185c3352049939a50f7af68ac4f79f6da80c03eaf3b2a538f48d3c23f81ca8f79e09503adf74a3f8133fa88d89d8126100e2955683f7b0354df2135809b9e7c4d4baddf761ef921e6af061aa5246019095294be054e0c3819009888c17d5f315c85e1eee92674cbbe250dd4d7f4f0c3e248ca40354b7d2d6503cac5db0d423216b1aedfd745b8b8a80ef876e3a1b1d7a20c0642414245b50101030000005a298d1000000000a8a436978699b8bf2e874efb467a33e195bc1b81325b33e9cf2fcbd0488f546741ba7c960b447983ffcc2f0be93bc31792624f43c5b6da2b72d689dc65edac02470617ce8c5d91837bdad8db80bd57fe6ba062fc807c3ff592cd8df29489b80304424545468403626c701386657a5cf91c38f97eeafbf52c47f7b39d29c7393e4084d496bd72950542414245010116151a576f08a8fb943ecdc06f62909a5a73ac0e4dc5dff12b60c3e348b9f25544f0fc26952021522e4d433afb2d123cf38663da1bfc266b8e8890983ae8498861be61dcf92bc833e2dcae987945df74b80274e348c8af54496d85bebde4d0a11d0911fb72593a26fb0dfa48e1fe0e1a3cbc18c8622c45c6a2f05a52b1864dc6cb2bb16a42f0ce4d21f206ac85ac9377daf5b5655232be039af8a730795a0d9c42f20c0642414245b50103030000005b298d1000000000346446d69f04efe5a2a9d48107d8d2d938737f888ca9e15c27d551dae4b5a41d660dc9ea8d71de650dfd74fe554744cfbb491305ec55b27b82134833fb6d2407874baf273100ce8dbdb4d7c3c97dc1cfa14f255975a4eed334de6b6ef418400f044245454684038e0f870465108ac3dd90fc3c6e02054494837a581c53693d0e368dd0c4aa28fe05424142450101a8ea6296c1af41bbe51ae9e318963e6e82eb596e894a8b873e6ff4355346d61cfea00f5b9ecb6b86bf9ead378e4f44bdd3e340c7c55a5ad3e8d8d735ef4a0287e967e7170a477d19b20dc6ef345a5497845b51cd288cdc04b53bfe0fcddc7ce22109edbf254fdd7eef87682cd2134a59136d74be35e355bdec8227f434f59c8903353798cfcd6b1654dbe48de1dc6955f74c8ece56363a90c5783490e49a2315e53e0c0642414245b50103020000005c298d1000000000f094cce62e7b0bec60773ce8299acf138e20756708c599cbc066069578fefc09393d6441379b92f2ca6b0eb91c3b80bff2c2e4fb1ba2e3765f12f0c367f14d0f869fb0f218d7aec7aa1494c1b4e3c81e58481d97eb9d3f5dc85f05239652e008044245454684033a15a58375ae807862f3a3b8b8a1f629b11a5186282610e6bb23bafa39eea31f054241424501019a436c5843c642ff61b569b22f8457508cec7c2c82ec05b3496ec3c964efa71d33f807ef85f498fdb8821dfd6ed23ffcc659d87427ea66543233f370f287958c7aafdd96fce5be08532ecefa7a39d68fcc7c7b072a84de1dfd5041596ddca7b0250943d7bd8dcc6257322a9f7c31006e8f57d906b4d1f781e3f60353183c8bed9b8045b61c5d7bc53e9ea77e4ae80e473c740e6f4d57e907887548d3d191b336d4d90c0642414245b50103030000005d298d100000000074dee134f4c04d69cfebbae3fb52527433e19f243ed1e3cf1b572cd0880cd045c2a8af91165ca30747f00cda51fba6805f5e66f06c82ab903788f008a247000255aa3c7a2495b064512a46c9eff306452bc510ca1d7c7dcb1099547a8fa997060442454546840371a3b8e86fac8b5bb4aacff3799f1270e0091e0431d32b9d5fc68fff40dcb28c0542414245010160f22ba20d2e735278838368423727c14e806cf870f21e7d15b0e3a206202d4a8c41a7538179e23458fdf660a133bb23adb430fb340982e08fafc50e8f62ae84156d6c53cb0d79a36c11e886e20cd24542975104df116bdebe86064a11670b46290985cd0bcb6b6506addf270159662a895f430e1c0478f726d345869c8a6fd7abe7bdaa6104ce40f2453002fcceae889132ee352c2d3dfbfd9ecc4861b7309aa6300c0642414245b50103020000005e298d1000000000246facba4beb0698a84f3d3a95398c47d4a1717982b0524e2476e7b51b413917a1b9c6472f30fa49b3e5af574b1d646f079138c4e32ee9bbdb3754902d14b20cb3a391ccba9e254369787d848705939f11e92b6bf4cfe711473e0e3b7c838c0904424545468403196f487c97d6c46dfa70a76a491c9183b291c3e10516a5e15cec88bfe29d8e7f0542414245010142d8d76c78c1a74cb9323c14e7c2c0b6dfca55a0c6a723be37258ea4dd74ad388672f2c00cae3f830b20caf053444e77f62802e7f1bc57ff39c39c4661150283c7827202c44888762029d5c4f9511b02b10d0e831ddd080b16cc41611e4b3fa62d097471c8e6bf9f26866bac3672edac970ee238d05b592b8f79b07213a5cf795a41480e28024543ca7f234e2779bafd027664810ca5682e0ac6c6fd3cec51fa10b10c0642414245b50103000000005f298d1000000000bc82fb1d84d33bf3aa15400df927f503f8d32cd28405c273c861bb0b6416ce11914e0fdf4031f4567315ed54223ed0c4ddad954e656c6253f17ccf928c009a0e0a13875bae4e40258b9b5a601b423c942c19e0a511bb2733685eabd81691fd00044245454684036945765b28d7bf50322b3322a533d8e8ce67618bbc8b1ff6dfc8670a462e92270542414245010168bb00d3ea1cc96324ef2cd31590c838c80991f9ee76578e072335f4bdc6ff6265ca93afb82548ba1b5904055077e5e6bf6dd8947d68055f604726ae47642782"
				// ).unwrap();

				let trusted_finality_proof =
					FinalityProof::<RelayChainHeader>::decode(&mut &encoded[..])?;

				// dbg!(&header);
				// dbg!(&trusted_finality_proof);
				if header.finality_proof.block != trusted_finality_proof.block {
					log::info!("block mismatch");
					let trusted_justification = GrandpaJustification::decode(
						&mut trusted_finality_proof.justification.as_slice(),
					)?;
					if justification.round != trusted_justification.round {
						log::error!(
							"round mismatch {} != {}",
							justification.round,
							trusted_justification.round
						);
					}
					// assert_eq!(
					// 	justification.commit.target_number,
					// 	trusted_justification.commit.target_number
					// );

					let api = self.relay_client.storage();
					let current_set_id_addr = polkadot::api::storage().grandpa().current_set_id();
					let current_set_id = api
						.fetch(
							&current_set_id_addr,
							Some(T::Hash::from(trusted_finality_proof.block.clone()).into()),
						)
						.await?
						.expect("Failed to fetch current set id");

					// let current_set_id = 13;

					log::info!("current_set_id: {}", current_set_id);

					let mut fraud_precommits = Vec::new();
					for first_precommit in &justification.commit.precommits {
						for second_precommit in &trusted_justification.commit.precommits {
							if first_precommit.id == second_precommit.id &&
								first_precommit.precommit != second_precommit.precommit
							{
								log::info!("found misbehaviour");
								fraud_precommits
									.push((first_precommit.clone(), second_precommit.clone()));
							}
						}
					}

					let mut equivocations = Vec::new();
					let mut equivocation_calls = Vec::new();

					// let datas = [
					// 	hex::decode("01a50c0e0000001c990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c1d028086028002a298c143f0a47a345c04f9e5fbd68251b611b2b4cb2ec5b235c6369fbcfc468073ee0d947847eb2c9be43138aec951f2744a5a6a7786d446630af21269116e6c802574392569bd8c6799c1d35698fb5906219d97952bdc79c976aa13d5cda219dd80d89f5db1ed27408f4966e4594dc909d0149591504ead826d79c7d32073dfe3bdb1028872616e80322180658754c8f9b7647e3458e22366732ea1dce4737c4df7e078369943158967eee080dc12493f168c3baaaf3232c24ef0b2cbca3af22b996fb432f0c80b55e686b3b380848cdb9dbfb49abfdc56108a994460528e6e07fafacf8673990c15f51e71013980a7e333f29c7da055a58aeed91c31baf5a06de68279f8536291a1baa95965bc5c805ff6a05c78cd63fe454e0ccafd1bbbf7ac26cd5a942ad339fa2603123f4c1c129c7f00017c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae691001000000990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c2503803f008054bb05c436856a7c6126397063110b88e9498067d7aa645c920fc3fb83818f9a80233d5e1136003f0a7d979fe65242e7835ccae1b0bb9a8d59b7fa23714097e8c580e898a6581d4a82374fd76e5a7878af6cd3655649349abea4cf6c260b7d1a29cf806d8de9fdde7c4da15440e7d57fdb556fbdb73da0f354943792bc7bc2fd7fe12a80e35a2486e527e502fe60d005a024b26645c7048b8886095ce53c8f3e2e76657680bdae6007e0271d9ec586e4760c6b34ba2bfcc7f3174711c61b4e9d6c6ba34061944600000080fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e06000000").unwrap(),
					// 	hex::decode("01a50c0e0000001c990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c1d028086028002a298c143f0a47a345c04f9e5fbd68251b611b2b4cb2ec5b235c6369fbcfc468073ee0d947847eb2c9be43138aec951f2744a5a6a7786d446630af21269116e6c802574392569bd8c6799c1d35698fb5906219d97952bdc79c976aa13d5cda219dd80d89f5db1ed27408f4966e4594dc909d0149591504ead826d79c7d32073dfe3bdb1028872616e80322180658754c8f9b7647e3458e22366732ea1dce4737c4df7e078369943158967eee080dc12493f168c3baaaf3232c24ef0b2cbca3af22b996fb432f0c80b55e686b3b380848cdb9dbfb49abfdc56108a994460528e6e07fafacf8673990c15f51e71013980a7e333f29c7da055a58aeed91c31baf5a06de68279f8536291a1baa95965bc5c805ff6a05c78cd63fe454e0ccafd1bbbf7ac26cd5a942ad339fa2603123f4c1c129c7f00039660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f1002000000990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c2503803f008054bb05c436856a7c6126397063110b88e9498067d7aa645c920fc3fb83818f9a80233d5e1136003f0a7d979fe65242e7835ccae1b0bb9a8d59b7fa23714097e8c580e898a6581d4a82374fd76e5a7878af6cd3655649349abea4cf6c260b7d1a29cf806d8de9fdde7c4da15440e7d57fdb556fbdb73da0f354943792bc7bc2fd7fe12a80e35a2486e527e502fe60d005a024b26645c7048b8886095ce53c8f3e2e76657680bdae6007e0271d9ec586e4760c6b34ba2bfcc7f3174711c61b4e9d6c6ba340619446000000801e07379407fecc4b89eb7dbd287c2c781cfb1907a96947a3eb18e4f8e719862506000000").unwrap(),
					// 	hex::decode("01b90d0e00000020990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c1d028086028002a298c143f0a47a345c04f9e5fbd68251b611b2b4cb2ec5b235c6369fbcfc468073ee0d947847eb2c9be43138aec951f2744a5a6a7786d446630af21269116e6c802574392569bd8c6799c1d35698fb5906219d97952bdc79c976aa13d5cda219dd80d89f5db1ed27408f4966e4594dc909d0149591504ead826d79c7d32073dfe3bdb1028872616e80322180658754c8f9b7647e3458e22366732ea1dce4737c4df7e078369943158967eee080dc12493f168c3baaaf3232c24ef0b2cbca3af22b996fb432f0c80b55e686b3b380848cdb9dbfb49abfdc56108a994460528e6e07fafacf8673990c15f51e71013980a7e333f29c7da055a58aeed91c31baf5a06de68279f8536291a1baa95965bc5c805ff6a05c78cd63fe454e0ccafd1bbbf7ac26cd5a942ad339fa2603123f4c1c121501804040809530da6c0da3116a40f964cfb8d986aad4a6c580dce9f926bc098f40206bdf39806890fe5573d5f7852fa06f701defe223f7a356bddf338cd58796e9f0c4a37ab7947e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d91003000000990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c2503803f008054bb05c436856a7c6126397063110b88e9498067d7aa645c920fc3fb83818f9a80233d5e1136003f0a7d979fe65242e7835ccae1b0bb9a8d59b7fa23714097e8c580e898a6581d4a82374fd76e5a7878af6cd3655649349abea4cf6c260b7d1a29cf806d8de9fdde7c4da15440e7d57fdb556fbdb73da0f354943792bc7bc2fd7fe12a80e35a2486e527e502fe60d005a024b26645c7048b8886095ce53c8f3e2e76657680bdae6007e0271d9ec586e4760c6b34ba2bfcc7f3174711c61b4e9d6c6ba34061944600000080e860f1b1c7227f7c22602f53f15af80747814dffd839719731ee3bba6edc126c06000000").unwrap(),
					// 	hex::decode("01a50c0e0000001c990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c1d028086028002a298c143f0a47a345c04f9e5fbd68251b611b2b4cb2ec5b235c6369fbcfc468073ee0d947847eb2c9be43138aec951f2744a5a6a7786d446630af21269116e6c802574392569bd8c6799c1d35698fb5906219d97952bdc79c976aa13d5cda219dd80d89f5db1ed27408f4966e4594dc909d0149591504ead826d79c7d32073dfe3bdb1028872616e80322180658754c8f9b7647e3458e22366732ea1dce4737c4df7e078369943158967eee080dc12493f168c3baaaf3232c24ef0b2cbca3af22b996fb432f0c80b55e686b3b380848cdb9dbfb49abfdc56108a994460528e6e07fafacf8673990c15f51e71013980a7e333f29c7da055a58aeed91c31baf5a06de68279f8536291a1baa95965bc5c805ff6a05c78cd63fe454e0ccafd1bbbf7ac26cd5a942ad339fa2603123f4c1c129c7f000dfe3e22cc0d45c70779c1095f7489a8ef3cf52d62fbd8c2fa38c9f1723502b51004000000990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c2503803f008054bb05c436856a7c6126397063110b88e9498067d7aa645c920fc3fb83818f9a80233d5e1136003f0a7d979fe65242e7835ccae1b0bb9a8d59b7fa23714097e8c580e898a6581d4a82374fd76e5a7878af6cd3655649349abea4cf6c260b7d1a29cf806d8de9fdde7c4da15440e7d57fdb556fbdb73da0f354943792bc7bc2fd7fe12a80e35a2486e527e502fe60d005a024b26645c7048b8886095ce53c8f3e2e76657680bdae6007e0271d9ec586e4760c6b34ba2bfcc7f3174711c61b4e9d6c6ba340619446000000808ac59e11963af19174d0b94d5d78041c233f55d2e19324665bafdfb62925af2d06000000").unwrap(),
					// 	hex::decode("01b90d0e00000020990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c1d028086028002a298c143f0a47a345c04f9e5fbd68251b611b2b4cb2ec5b235c6369fbcfc468073ee0d947847eb2c9be43138aec951f2744a5a6a7786d446630af21269116e6c802574392569bd8c6799c1d35698fb5906219d97952bdc79c976aa13d5cda219dd80d89f5db1ed27408f4966e4594dc909d0149591504ead826d79c7d32073dfe3bdb1028872616e80322180658754c8f9b7647e3458e22366732ea1dce4737c4df7e078369943158967eee080dc12493f168c3baaaf3232c24ef0b2cbca3af22b996fb432f0c80b55e686b3b380848cdb9dbfb49abfdc56108a994460528e6e07fafacf8673990c15f51e71013980a7e333f29c7da055a58aeed91c31baf5a06de68279f8536291a1baa95965bc5c805ff6a05c78cd63fe454e0ccafd1bbbf7ac26cd5a942ad339fa2603123f4c1c121501804040809530da6c0da3116a40f964cfb8d986aad4a6c580dce9f926bc098f40206bdf39806890fe5573d5f7852fa06f701defe223f7a356bddf338cd58796e9f0c4a37ab7947e8cb4a574c6d178feb39c27dfc8b3f789e5f5423e19c71633c748b9acf086b51005000000990180c100809b4bfd233682ec27089e020ebd393091d7935d1a76f610da71ad95c61e7de46c80a3fcd058b4ddb0cc64a6d25cfee80290f70be7f75b5fa68d0287f4d87e6929ff807c3fef8f0986577532d33077b3f472d5d794ba3f6158c7468784bee001aa2d2c2503803f008054bb05c436856a7c6126397063110b88e9498067d7aa645c920fc3fb83818f9a80233d5e1136003f0a7d979fe65242e7835ccae1b0bb9a8d59b7fa23714097e8c580e898a6581d4a82374fd76e5a7878af6cd3655649349abea4cf6c260b7d1a29cf806d8de9fdde7c4da15440e7d57fdb556fbdb73da0f354943792bc7bc2fd7fe12a80e35a2486e527e502fe60d005a024b26645c7048b8886095ce53c8f3e2e76657680bdae6007e0271d9ec586e4760c6b34ba2bfcc7f3174711c61b4e9d6c6ba34061944600000080101191192fc877c24d725b337120fa3edc63d227bbc92705db1e2cb65f56981a06000000").unwrap(),
					// ];
					let mut i = 0;
					for (first, second) in fraud_precommits {
						let key_ownership_proof: OpaqueKeyOwnershipProof = {
							let bytes = self
								.relay_client
								.rpc()
								.request::<String>(
									"state_call",
									rpc_params!(
										"GrandpaApi_generate_key_ownership_proof",
										format!(
											"0x{}",
											hex::encode((&current_set_id, &first.id).encode())
										)
									),
								)
								.await
								.map(|res| hex::decode(&res[2..]))
								.expect("Failed to fetch key ownership proof")?;
							log::info!("data: {}", hex::encode(&bytes));

							// let bytes = datas[i].clone();
							i += 1;
							Option::decode(&mut &bytes[..])
								.expect("Failed to scale decode key ownership proof")
								.expect("Failed to fetch key ownership proof")
						};

						let equivocation = Equivocation::Precommit(grandpa::Equivocation {
							round_number: trusted_justification.round,
							identity: first.id.clone(),
							first: (first.precommit.clone(), first.signature.clone()),
							second: (second.precommit.clone(), second.signature.clone()),
						});

						let polkadot_equivocation =
							construct_polkadot_grandpa_equivocation(&equivocation);
						let equivocation_proof =
							polkadot::api::runtime_types::sp_finality_grandpa::EquivocationProof {
								set_id: current_set_id,
								equivocation: polkadot_equivocation,
							};

						if !sp_finality_grandpa::check_equivocation_proof(
							sp_finality_grandpa::EquivocationProof::new(
								current_set_id,
								equivocation.clone(),
							),
						) {
							log::error!("Equivocation proof is invalid: {:?}", equivocation);
						} else {
							log::info!("Equivocation proof is valid {:?}", equivocation);
						}
						// for sid in 0..100 {
						// 	match &equivocation {
						// 		Equivocation::Precommit(eq) => {
						// 			if check_message_signature(
						// 				&grandpa::Message::Precommit(eq.second.0.clone()),
						// 				&eq.identity,
						// 				&eq.second.1,
						// 				eq.round_number,
						// 				current_set_id,
						// 			) {
						// 				log::info!("sid: {}", sid);
						// 				break
						// 			} else {
						// 			}
						// 		},
						// 		_ => panic!(),
						// 	};
						// }
						log::info!("equiv = {}", hex::encode(equivocation_proof.encode()));
						let call = rococo_runtime::Call::Grandpa(
							pallet_grandpa::pallet::Call::report_equivocation {
								equivocation_proof: Box::new(equivocation_proof),
								key_owner_proof: key_ownership_proof.decode().unwrap(),
							},
						);
						// log::info!("AAA");
						equivocation_calls.push(call);
						equivocations.push(equivocation);
					}

					let misbehaviour = ClientMessage::Misbehaviour(Misbehaviour {
						set_id: current_set_id,
						equivocations,
						first_finality_proof: todo!(),
						second_finality_proof: todo!(),
					});

					let batch_call = polkadot::api::tx().utility().batch(equivocation_calls);
					let equivocation_report_future = self
						.submit_call_relaychain(batch_call)
						.map_err(|e| log::error!("Failed to submit equivocation report: {:?}", e))
						.map(|res| {
							log::info!("equivocation report submitted: {:?}", res,);
						});
					let misbehaviour_report_future = counterparty
						.submit(vec![MsgUpdateAnyClient::<LocalClientTypes>::new(
							self.client_id(),
							AnyClientMessage::Grandpa(misbehaviour.clone()),
							counterparty.account_id(),
						)
						.to_any()])
						.map_err(|e| log::error!("Failed to submit misbehaviour report: {:?}", e))
						.map(|res| {
							log::info!("misbehaviour report submitted: {:?}", res,);
						});
					future::join(
						Box::pin(equivocation_report_future)
							as Pin<Box<dyn Future<Output = ()> + Send>>,
						Box::pin(misbehaviour_report_future)
							as Pin<Box<dyn Future<Output = ()> + Send>>,
					)
					.await;
					log::info!("submitted misbehaviour");
				}
			},
			_ => {},
		}
		Ok(())
	}
}

fn construct_polkadot_grandpa_equivocation<H: Copy, N: Copy>(
	equivocation: &Equivocation<H, N>,
) -> polkadot_grandpa::Equivocation<H, N> {
	use polkadot::api::runtime_types::{
		finality_grandpa as polkadot_finality_grandpa,
		finality_grandpa::Precommit,
		sp_core::ed25519::{Public, Signature},
	};

	match equivocation {
		Equivocation::Precommit(equiv) =>
			polkadot_grandpa::Equivocation::Precommit(polkadot_finality_grandpa::Equivocation {
				round_number: equiv.round_number,
				identity: polkadot_grandpa::app::Public(Public(
					equiv.identity.to_raw_vec().try_into().unwrap(),
				)),
				first: (
					Precommit {
						target_number: equiv.first.0.target_number,
						target_hash: equiv.first.0.target_hash,
					},
					polkadot_grandpa::app::Signature(Signature(
						(&*equiv.first.1).try_into().unwrap(),
					)),
				),
				second: (
					Precommit {
						target_number: equiv.second.0.target_number,
						target_hash: equiv.second.0.target_hash,
					},
					polkadot_grandpa::app::Signature(Signature(
						(&*equiv.second.1).try_into().unwrap(),
					)),
				),
			}),
		_ => {
			unimplemented!()
		},
	}
}

#[async_trait::async_trait]
impl<T: Config + Send + Sync> Chain for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: BlockNumberOps + From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256> + From<[u8; 32]>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	sp_core::H256: From<T::Hash>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
{
	fn name(&self) -> &str {
		&*self.name
	}

	fn block_max_weight(&self) -> u64 {
		self.max_extrinsic_weight
	}

	async fn estimate_weight(&self, messages: Vec<Any>) -> Result<u64, Self::Error> {
		let extrinsic = {
			// todo: put this in utils
			let signer = ExtrinsicSigner::<T, Self>::new(
				self.key_store.clone(),
				self.key_type_id.clone(),
				self.public_key.clone(),
			);

			let messages = messages
				.into_iter()
				.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
				.collect::<Vec<_>>();

			let tx_params = SubstrateExtrinsicParamsBuilder::new()
				.tip(AssetTip::new(100_000))
				.era(Era::Immortal, self.para_client.genesis_hash());
			let call = api::tx().ibc().deliver(messages);
			self.para_client.tx().create_signed(&call, &signer, tx_params.into()).await?
		};
		let dispatch_info =
			TransactionPaymentApiClient::<sp_core::H256, RuntimeDispatchInfo<u128>>::query_info(
				&*self.para_ws_client,
				extrinsic.encoded().to_vec().into(),
				None,
			)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(dispatch_info.weight)
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = <Self as IbcProvider>::FinalityEvent> + Send + Sync>> {
		match self.finality_protocol {
			FinalityProtocol::Grandpa => {
				let subscription =
					GrandpaApiClient::<JustificationNotification, sp_core::H256, u32>::subscribe_justifications(
						&*self.relay_ws_client,
					)
						.await
						.expect("Failed to subscribe to grandpa justifications")
						.chunks(6)
						.map(|mut notifs| notifs.remove(notifs.len() - 1)); // skip every 4 finality notifications

				let stream = subscription.filter_map(|justification_notif| {
					let encoded_justification = match justification_notif {
						Ok(JustificationNotification(sp_core::Bytes(justification))) =>
							justification,
						Err(err) => {
							log::error!("Failed to fetch Justification: {}", err);
							return futures::future::ready(None)
						},
					};

					let justification =
						match GrandpaJustification::decode(&mut &*encoded_justification) {
							Ok(j) => j,
							Err(err) => {
								log::error!("Grandpa Justification scale decode error: {}", err);
								return futures::future::ready(None)
							},
						};
					futures::future::ready(Some(Self::FinalityEvent::Grandpa(justification)))
				});

				Box::pin(Box::new(stream))
			},
			FinalityProtocol::Beefy => {
				let subscription =
					BeefyApiClient::<JustificationNotification, sp_core::H256>::subscribe_justifications(
						&*self.relay_ws_client,
					)
						.await
						.expect("Failed to subscribe to beefy justifications");

				let stream = subscription.filter_map(|commitment_notification| {
					let encoded_commitment = match commitment_notification {
						Ok(JustificationNotification(sp_core::Bytes(commitment))) => commitment,
						Err(err) => {
							log::error!("Failed to fetch Commitment: {}", err);
							return futures::future::ready(None)
						},
					};

					let signed_commitment =
						match BeefyJustification::decode(&mut &*encoded_commitment) {
							Ok(c) => c,
							Err(err) => {
								log::error!("SignedCommitment scale decode error: {}", err);
								return futures::future::ready(None)
							},
						};
					futures::future::ready(Some(Self::FinalityEvent::Beefy(signed_commitment)))
				});

				Box::pin(Box::new(stream))
			},
		}
	}

	async fn submit(&self, messages: Vec<Any>) -> Result<(), Error> {
		let messages = messages
			.into_iter()
			.map(|msg| RawAny { type_url: msg.type_url.as_bytes().to_vec(), value: msg.value })
			.collect::<Vec<_>>();

		let call = api::tx().ibc().deliver(messages);
		log::info!("submitted call {:?}", self.submit_call(call).await?);

		Ok(())
	}

	async fn query_client_message(
		&self,
		host_block_hash: [u8; 32],
		transaction_id: u32,
		event_index: usize,
	) -> Result<AnyClientMessage, primitives::error::Error> {
		use api::runtime_types::{
			dali_runtime::Call as RuntimeCall, pallet_ibc::pallet::Call as IbcCall,
		};

		let extrinsic_data_addr =
			parachain::api::storage().system().extrinsic_data(&transaction_id);
		let h256 = H256(host_block_hash);
		log::info!("Querying extrinsic data at {:?} {}", h256, transaction_id);
		// let block = timeout(Duration::from_secs(20), || async {
		// 	let maybe_block = self.para_client.rpc().block(Some(h256.into())).await?;
		// 	match maybe_block {
		// 		Some(block) =>
		// 			Box::new(ready(block)) as Box<dyn Future<Output = _> + Send + 'static>,
		// 		None => Box::new(pending()) as Box<dyn Future<Output = _> + Send + 'static>,
		// 	}
		// })
		// .await?;

		let now = Instant::now();
		let block = loop {
			let maybe_block = self.para_client.rpc().block(Some(h256.into())).await?;
			match maybe_block {
				Some(block) => {
					log::info!("block query took {}", now.elapsed().as_millis());
					break block
				},
				None => {
					if now.elapsed() > Duration::from_secs(20) {
						return Err(primitives::error::Error::from(
							"Timeout while waiting for block".to_owned(),
						))
					}
					sleep(Duration::from_millis(100)).await;
				},
			}
		};
		let extrinsic_opaque = block
			.block
			.extrinsics
			.get(transaction_id as usize)
			.expect("Extrinsic not found");

		// let extrinsic_opaque = self
		// 	.para_client
		// 	.storage()
		// 	.fetch(&extrinsic_data_addr, Some(h256.into()))
		// 	.await?
		// 	.expect("Extrinsic should exist");
		let unchecked_extrinsic = UncheckedExtrinsic::<T>::decode(&mut &*extrinsic_opaque.encode())
			.map_err(|e| {
				primitives::error::Error::from(format!("Extrinsic decode error: {}", e))
			})?;

		match unchecked_extrinsic.function {
			RuntimeCall::Ibc(IbcCall::deliver { messages }) => {
				let message = messages.get(event_index).ok_or_else(|| {
					primitives::error::Error::from(format!(
						"Message index {} out of bounds",
						event_index
					))
				})?;
				let envelope = Ics26Envelope::<LocalClientTypes>::try_from(Any {
					type_url: String::from_utf8(message.type_url.clone())?,
					value: message.value.clone(),
				});
				match envelope {
					Ok(Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(update_msg))) =>
						return Ok(update_msg.client_message),
					_ => (),
				}
			},
			_ => (),
		}
		Err(primitives::error::Error::Custom("No ICS02 update message found".into()))
	}
}
