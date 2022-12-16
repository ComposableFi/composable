#![feature(prelude_import)]
#![warn(
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::todo,
    clippy::unwrap_used,
    clippy::panic
)]
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod currency {
    //! CurrencyId implementation
    use codec::{CompactAs, Decode, Encode, MaxEncodedLen};
    use composable_support::validation::Validate;
    use composable_traits::{
        assets::Asset, currency::Exponent, xcm::assets::XcmAssetLocation,
    };
    use core::{fmt::Display, ops::Div, str::FromStr};
    use frame_support::WeakBoundedVec;
    use scale_info::TypeInfo;
    use sp_runtime::{
        sp_std::{ops::Deref, vec::Vec},
        RuntimeDebug,
    };
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    use crate::topology;
    use xcm::latest::prelude::*;
    /// Trait used to write generalized code over well know currencies
    /// We use const to allow for match on these
    /// Allows to have reuse of code amids runtime and cross relay transfers in future.
    pub trait WellKnownCurrency {
        const NATIVE: Self;
        /// usually we expect running with relay,
        /// but if  not, than degenerative case would be this equal to `NATIVE`
        const RELAY_NATIVE: Self;
    }
    #[repr(transparent)]
    pub struct CurrencyId(pub u128);
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CurrencyId {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "CurrencyId",
                    &self.0,
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CurrencyId {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CurrencyId>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CurrencyId;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct CurrencyId",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: u128 = match <u128 as _serde::Deserialize>::deserialize(
                            __e,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(CurrencyId(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            u128,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct CurrencyId with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CurrencyId(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "CurrencyId",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CurrencyId>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Encode for CurrencyId {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                ::codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::codec::Encode::using_encoded(&&self.0, f)
            }
        }
        #[automatically_derived]
        impl ::codec::EncodeLike for CurrencyId {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::Decode for CurrencyId {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(
                    CurrencyId({
                        let __codec_res_edqy = <u128 as ::codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `CurrencyId.0`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    }),
                )
            }
        }
    };
    const _: () = {
        impl ::codec::MaxEncodedLen for CurrencyId {
            fn max_encoded_len() -> ::core::primitive::usize {
                0_usize.saturating_add(<u128>::max_encoded_len())
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralEq for CurrencyId {}
    #[automatically_derived]
    impl ::core::cmp::Eq for CurrencyId {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CurrencyId {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CurrencyId {
        #[inline]
        fn eq(&self, other: &CurrencyId) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CurrencyId {}
    #[automatically_derived]
    impl ::core::clone::Clone for CurrencyId {
        #[inline]
        fn clone(&self) -> CurrencyId {
            let _: ::core::clone::AssertParamIsClone<u128>;
            *self
        }
    }
    impl core::fmt::Debug for CurrencyId {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("CurrencyId").field(&self.0).finish()
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for CurrencyId {
        #[inline]
        fn partial_cmp(
            &self,
            other: &CurrencyId,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for CurrencyId {
        #[inline]
        fn cmp(&self, other: &CurrencyId) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for CurrencyId {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("CurrencyId", "primitives::currency"))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::unnamed()
                            .field(|f| f.ty::<u128>().type_name("u128")),
                    )
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::codec::CompactAs for CurrencyId {
            type As = u128;
            fn encode_as(&self) -> &u128 {
                &self.0
            }
            fn decode_from(
                x: u128,
            ) -> ::core::result::Result<CurrencyId, ::codec::Error> {
                ::core::result::Result::Ok(CurrencyId(x))
            }
        }
        #[automatically_derived]
        impl From<::codec::Compact<CurrencyId>> for CurrencyId {
            fn from(x: ::codec::Compact<CurrencyId>) -> CurrencyId {
                x.0
            }
        }
    };
    #[automatically_derived]
    impl ::core::hash::Hash for CurrencyId {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl FromStr for CurrencyId {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            u128::from_str(s).map(CurrencyId).map_err(|_| ())
        }
    }
    impl Display for CurrencyId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let CurrencyId(id) = self;
            f.write_fmt(
                ::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_display(&id)],
                ),
            )
        }
    }
    impl WellKnownCurrency for CurrencyId {
        const NATIVE: CurrencyId = CurrencyId::PICA;
        const RELAY_NATIVE: CurrencyId = CurrencyId::KSM;
    }
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    impl CurrencyId {
        pub const INVALID: CurrencyId = CurrencyId(0);
        /// Runtime native token Kusama
        pub const PICA: CurrencyId = CurrencyId(1);
        /// Runtime native token Polkadot
        pub const LAYR: CurrencyId = CurrencyId(2);
        /// Kusama native token
        pub const KSM: CurrencyId = CurrencyId(4);
        pub const PBLO: CurrencyId = CurrencyId(5);
        pub const ibcDOT: CurrencyId = CurrencyId(6);
        /// Karura KAR
        pub const KAR: CurrencyId = CurrencyId(101);
        /// BIFROST BNC
        pub const BNC: CurrencyId = CurrencyId(102);
        /// BIFROST vKSM
        pub const vKSM: CurrencyId = CurrencyId(103);
        /// Moonriver MOVR
        pub const MOVR: CurrencyId = CurrencyId(104);
        pub const KSM_USDT_LPT: CurrencyId = CurrencyId(105);
        pub const PICA_USDT_LPT: CurrencyId = CurrencyId(106);
        /// Karura stable coin(Acala Dollar), not native.
        pub const kUSD: CurrencyId = CurrencyId(129);
        /// Statemine USDT
        pub const USDT: CurrencyId = CurrencyId(130);
        pub const USDC: CurrencyId = CurrencyId(131);
        /// Wrapped BTC
        pub const wBTC: CurrencyId = CurrencyId(132);
        /// Wrapped ETH
        pub const wETH: CurrencyId = CurrencyId(133);
        /// Staked asset xPICA Token
        pub const xPICA: CurrencyId = CurrencyId(1001);
        /// Staked asset xLAYR Token
        pub const xLAYR: CurrencyId = CurrencyId(1002);
        /// Staked asset xKSM Token
        pub const xKSM: CurrencyId = CurrencyId(1004);
        /// Staked asset xPBLO Token
        pub const xPBLO: CurrencyId = CurrencyId(1005);
        /// PICA Stake fNFT Collection
        pub const PICA_STAKE_FNFT_COLLECTION: CurrencyId = CurrencyId(2001);
        /// PBLO Stake fNFT Collection
        pub const PBLO_STAKE_FNFT_COLLECTION: CurrencyId = CurrencyId(2005);
        pub fn native_asset_name(id: u128) -> Result<&'static str, &'static str> {
            match id {
                1 => Ok("PICA"),
                2 => Ok("LAYR"),
                4 => Ok("KSM"),
                5 => Ok("PBLO"),
                6 => Ok("ibcDOT"),
                101 => Ok("KAR"),
                102 => Ok("BNC"),
                103 => Ok("vKSM"),
                104 => Ok("MOVR"),
                105 => Ok("KSM_USDT_LPT"),
                106 => Ok("PICA_USDT_LPT"),
                129 => Ok("kUSD"),
                130 => Ok("USDT"),
                131 => Ok("USDC"),
                132 => Ok("wBTC"),
                133 => Ok("wETH"),
                1001 => Ok("xPICA"),
                1002 => Ok("xLAYR"),
                1004 => Ok("xKSM"),
                1005 => Ok("xPBLO"),
                2001 => Ok("PICA_STAKE_FNFT_COLLECTION"),
                2005 => Ok("PBLO_STAKE_FNFT_COLLECTION"),
                _ => Err("Invalid native asset"),
            }
        }
        pub fn to_native_id(name: &str) -> Result<CurrencyId, &'static str> {
            match name {
                "PICA" => Ok(CurrencyId::PICA),
                "LAYR" => Ok(CurrencyId::LAYR),
                "KSM" => Ok(CurrencyId::KSM),
                "PBLO" => Ok(CurrencyId::PBLO),
                "ibcDOT" => Ok(CurrencyId::ibcDOT),
                "KAR" => Ok(CurrencyId::KAR),
                "BNC" => Ok(CurrencyId::BNC),
                "vKSM" => Ok(CurrencyId::vKSM),
                "MOVR" => Ok(CurrencyId::MOVR),
                "KSM_USDT_LPT" => Ok(CurrencyId::KSM_USDT_LPT),
                "PICA_USDT_LPT" => Ok(CurrencyId::PICA_USDT_LPT),
                "kUSD" => Ok(CurrencyId::kUSD),
                "USDT" => Ok(CurrencyId::USDT),
                "USDC" => Ok(CurrencyId::USDC),
                "wBTC" => Ok(CurrencyId::wBTC),
                "wETH" => Ok(CurrencyId::wETH),
                "xPICA" => Ok(CurrencyId::xPICA),
                "xLAYR" => Ok(CurrencyId::xLAYR),
                "xKSM" => Ok(CurrencyId::xKSM),
                "xPBLO" => Ok(CurrencyId::xPBLO),
                "PICA_STAKE_FNFT_COLLECTION" => {
                    Ok(CurrencyId::PICA_STAKE_FNFT_COLLECTION)
                }
                "PBLO_STAKE_FNFT_COLLECTION" => {
                    Ok(CurrencyId::PBLO_STAKE_FNFT_COLLECTION)
                }
                _ => Err("Invalid native asset"),
            }
        }
        pub fn local_to_xcm_reserve(
            id: CurrencyId,
        ) -> Option<xcm::latest::MultiLocation> {
            match id {
                CurrencyId::PICA => Some(topology::this::LOCAL),
                CurrencyId::KSM => Some(MultiLocation::parent()),
                CurrencyId::PBLO => {
                    Some(MultiLocation {
                        parents: 0,
                        interior: X1(GeneralIndex(5)),
                    })
                }
                CurrencyId::ibcDOT => None,
                CurrencyId::KAR => {
                    Some(MultiLocation {
                        parents: 1,
                        interior: X2(
                            Parachain(topology::karura::ID),
                            GeneralKey(
                                WeakBoundedVec::force_from(
                                    topology::karura::KAR_KEY.to_vec(),
                                    None,
                                ),
                            ),
                        ),
                    })
                }
                CurrencyId::BNC => None,
                CurrencyId::vKSM => None,
                CurrencyId::MOVR => None,
                CurrencyId::KSM_USDT_LPT => None,
                CurrencyId::PICA_USDT_LPT => None,
                CurrencyId::kUSD => {
                    Some(MultiLocation {
                        parents: 1,
                        interior: X2(
                            Parachain(topology::karura::ID),
                            GeneralKey(
                                WeakBoundedVec::force_from(
                                    topology::karura::AUSD_KEY.to_vec(),
                                    None,
                                ),
                            ),
                        ),
                    })
                }
                CurrencyId::USDT => {
                    Some(MultiLocation {
                        parents: 1,
                        interior: X3(
                            Parachain(topology::common_good_assets::ID),
                            PalletInstance(topology::common_good_assets::ASSETS),
                            GeneralIndex(topology::common_good_assets::USDT),
                        ),
                    })
                }
                CurrencyId::USDC => None,
                CurrencyId::wBTC => None,
                CurrencyId::wETH => None,
                CurrencyId::xPICA => None,
                CurrencyId::xLAYR => None,
                CurrencyId::xKSM => None,
                CurrencyId::xPBLO => None,
                CurrencyId::PICA_STAKE_FNFT_COLLECTION => None,
                CurrencyId::PBLO_STAKE_FNFT_COLLECTION => None,
                _ => None,
            }
        }
        pub fn xcm_reserve_to_local(
            remote_id: xcm::latest::MultiLocation,
        ) -> Option<CurrencyId> {
            use lazy_static::lazy_static;
            use sp_std::collections::btree_map::BTreeMap;
            #[allow(missing_copy_implementations)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            struct XCM_ASSETS {
                __private_field: (),
            }
            #[doc(hidden)]
            static XCM_ASSETS: XCM_ASSETS = XCM_ASSETS { __private_field: () };
            impl ::lazy_static::__Deref for XCM_ASSETS {
                type Target = BTreeMap<Vec<u8>, CurrencyId>;
                fn deref(&self) -> &BTreeMap<Vec<u8>, CurrencyId> {
                    #[inline(always)]
                    fn __static_ref_initialize() -> BTreeMap<Vec<u8>, CurrencyId> {
                        {
                            let mut map = BTreeMap::new();
                            let xcm_id: Option<xcm::latest::MultiLocation> = Some(
                                topology::this::LOCAL,
                            );
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::PICA);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = Some(
                                MultiLocation::parent(),
                            );
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::KSM);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = Some(MultiLocation {
                                parents: 0,
                                interior: X1(GeneralIndex(5)),
                            });
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::PBLO);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::ibcDOT);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = Some(MultiLocation {
                                parents: 1,
                                interior: X2(
                                    Parachain(topology::karura::ID),
                                    GeneralKey(
                                        WeakBoundedVec::force_from(
                                            topology::karura::KAR_KEY.to_vec(),
                                            None,
                                        ),
                                    ),
                                ),
                            });
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::KAR);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::BNC);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::vKSM);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::MOVR);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::KSM_USDT_LPT);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::PICA_USDT_LPT);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = Some(MultiLocation {
                                parents: 1,
                                interior: X2(
                                    Parachain(topology::karura::ID),
                                    GeneralKey(
                                        WeakBoundedVec::force_from(
                                            topology::karura::AUSD_KEY.to_vec(),
                                            None,
                                        ),
                                    ),
                                ),
                            });
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::kUSD);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = Some(MultiLocation {
                                parents: 1,
                                interior: X3(
                                    Parachain(topology::common_good_assets::ID),
                                    PalletInstance(topology::common_good_assets::ASSETS),
                                    GeneralIndex(topology::common_good_assets::USDT),
                                ),
                            });
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::USDT);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::USDC);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::wBTC);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::wETH);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::xPICA);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::xLAYR);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::xKSM);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(xcm_id.encode(), CurrencyId::xPBLO);
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(
                                    xcm_id.encode(),
                                    CurrencyId::PICA_STAKE_FNFT_COLLECTION,
                                );
                            }
                            let xcm_id: Option<xcm::latest::MultiLocation> = None;
                            if let Some(xcm_id) = xcm_id {
                                map.insert(
                                    xcm_id.encode(),
                                    CurrencyId::PBLO_STAKE_FNFT_COLLECTION,
                                );
                            }
                            map
                        }
                    }
                    #[inline(always)]
                    fn __stability() -> &'static BTreeMap<Vec<u8>, CurrencyId> {
                        static LAZY: ::lazy_static::lazy::Lazy<
                            BTreeMap<Vec<u8>, CurrencyId>,
                        > = ::lazy_static::lazy::Lazy::INIT;
                        LAZY.get(__static_ref_initialize)
                    }
                    __stability()
                }
            }
            impl ::lazy_static::LazyStatic for XCM_ASSETS {
                fn initialize(lazy: &Self) {
                    let _ = &**lazy;
                }
            }
            XCM_ASSETS.get(&remote_id.encode()).map(|x| *x)
        }
        pub fn list_assets() -> Vec<Asset<u128, XcmAssetLocation>> {
            [
                Asset {
                    id: CurrencyId::PICA.0 as u128,
                    name: Some("PICA".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::PICA)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::LAYR.0 as u128,
                    name: Some("LAYR".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::LAYR)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::KSM.0 as u128,
                    name: Some("KSM".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::KSM)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::PBLO.0 as u128,
                    name: Some("PBLO".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::PBLO)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::ibcDOT.0 as u128,
                    name: Some("ibcDOT".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::ibcDOT)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::KAR.0 as u128,
                    name: Some("KAR".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::KAR)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::BNC.0 as u128,
                    name: Some("BNC".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::BNC)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::vKSM.0 as u128,
                    name: Some("vKSM".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::vKSM)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::MOVR.0 as u128,
                    name: Some("MOVR".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::MOVR)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::KSM_USDT_LPT.0 as u128,
                    name: Some("KSM_USDT_LPT".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::KSM_USDT_LPT)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::PICA_USDT_LPT.0 as u128,
                    name: Some("PICA_USDT_LPT".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::PICA_USDT_LPT)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::kUSD.0 as u128,
                    name: Some("kUSD".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::kUSD)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::USDT.0 as u128,
                    name: Some("USDT".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::USDT)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::USDC.0 as u128,
                    name: Some("USDC".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::USDC)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::wBTC.0 as u128,
                    name: Some("wBTC".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::wBTC)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::wETH.0 as u128,
                    name: Some("wETH".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::wETH)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::xPICA.0 as u128,
                    name: Some("xPICA".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::xPICA)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::xLAYR.0 as u128,
                    name: Some("xLAYR".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::xLAYR)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::xKSM.0 as u128,
                    name: Some("xKSM".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::xKSM)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::xPBLO.0 as u128,
                    name: Some("xPBLO".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(CurrencyId::xPBLO)
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::PICA_STAKE_FNFT_COLLECTION.0 as u128,
                    name: Some("PICA_STAKE_FNFT_COLLECTION".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(
                            CurrencyId::PICA_STAKE_FNFT_COLLECTION,
                        )
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
                Asset {
                    id: CurrencyId::PBLO_STAKE_FNFT_COLLECTION.0 as u128,
                    name: Some("PBLO_STAKE_FNFT_COLLECTION".as_bytes().to_vec()),
                    ratio: None,
                    decimals: Self::decimals(),
                    foreign_id: Self::local_to_xcm_reserve(
                            CurrencyId::PBLO_STAKE_FNFT_COLLECTION,
                        )
                        .map(XcmAssetLocation::new),
                    existential_deposit: 0_u128,
                },
            ]
                .to_vec()
        }
        #[inline(always)]
        pub const fn decimals() -> Exponent {
            12
        }
        pub fn unit<T: From<u64>>() -> T {
            T::from(10_u64.pow(Self::decimals().into()))
        }
        pub fn milli<T: From<u64> + Div<Output = T>>() -> T {
            Self::unit::<T>() / T::from(1000_u64)
        }
    }
    pub struct ValidateCurrencyId;
    #[automatically_derived]
    impl ::core::clone::Clone for ValidateCurrencyId {
        #[inline]
        fn clone(&self) -> ValidateCurrencyId {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ValidateCurrencyId {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ValidateCurrencyId {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "ValidateCurrencyId")
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ValidateCurrencyId {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ValidateCurrencyId {
        #[inline]
        fn eq(&self, other: &ValidateCurrencyId) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ValidateCurrencyId {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ValidateCurrencyId {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for ValidateCurrencyId {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "ValidateCurrencyId",
                            "primitives::currency",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .composite(::scale_info::build::Fields::unit())
            }
        }
    };
    impl Validate<CurrencyId, ValidateCurrencyId> for ValidateCurrencyId {
        fn validate(input: CurrencyId) -> Result<CurrencyId, &'static str> {
            if input != CurrencyId::INVALID {
                Ok(input)
            } else {
                Err("Invalid Currency")
            }
        }
    }
    impl Validate<u64, ValidateCurrencyId> for ValidateCurrencyId {
        fn validate(input: u64) -> Result<u64, &'static str> {
            if input != 0_u64 { Ok(input) } else { Err("Invalid Currency") }
        }
    }
    impl Validate<u128, ValidateCurrencyId> for ValidateCurrencyId {
        fn validate(input: u128) -> Result<u128, &'static str> {
            if input != 0_u128 { Ok(input) } else { Err("Invalid Currency") }
        }
    }
    impl Default for CurrencyId {
        #[inline]
        fn default() -> Self {
            CurrencyId::INVALID
        }
    }
    impl Deref for CurrencyId {
        type Target = u128;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl From<CurrencyId> for u128 {
        #[inline]
        fn from(id: CurrencyId) -> Self {
            id.0
        }
    }
    impl From<u128> for CurrencyId {
        #[inline]
        fn from(raw: u128) -> Self {
            CurrencyId(raw)
        }
    }
    impl From<CurrencyId> for xcm::latest::Junction {
        fn from(this: CurrencyId) -> Self {
            xcm::latest::Junction::GeneralIndex(this.0)
        }
    }
    mod ops {
        use super::CurrencyId;
        use core::ops::{Add, Mul};
        use sp_runtime::traits::{Bounded, CheckedAdd, CheckedMul, One, Saturating, Zero};
        impl Add for CurrencyId {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                CurrencyId(self.0.add(rhs.0))
            }
        }
        impl Mul for CurrencyId {
            type Output = CurrencyId;
            fn mul(self, rhs: Self) -> Self::Output {
                CurrencyId(self.0.mul(rhs.0))
            }
        }
        impl CheckedAdd for CurrencyId {
            fn checked_add(&self, v: &Self) -> Option<Self> {
                Some(CurrencyId(self.0.checked_add(v.0)?))
            }
        }
        impl CheckedMul for CurrencyId {
            fn checked_mul(&self, v: &Self) -> Option<Self> {
                Some(CurrencyId(self.0.checked_mul(v.0)?))
            }
        }
        impl Zero for CurrencyId {
            fn zero() -> Self {
                CurrencyId(0)
            }
            fn is_zero(&self) -> bool {
                self.0.is_zero()
            }
        }
        impl One for CurrencyId {
            fn one() -> Self {
                CurrencyId(u128::one())
            }
        }
        impl Bounded for CurrencyId {
            fn min_value() -> Self {
                CurrencyId(u128::min_value())
            }
            fn max_value() -> Self {
                CurrencyId(u128::max_value())
            }
        }
        impl Saturating for CurrencyId {
            fn saturating_add(self, rhs: Self) -> Self {
                self.0.saturating_add(rhs.0).into()
            }
            fn saturating_sub(self, rhs: Self) -> Self {
                <u128 as Saturating>::saturating_sub(self.0, rhs.0).into()
            }
            fn saturating_mul(self, rhs: Self) -> Self {
                <u128 as Saturating>::saturating_mul(self.0, rhs.0).into()
            }
            fn saturating_pow(self, exp: usize) -> Self {
                <u128 as Saturating>::saturating_pow(self.0, exp).into()
            }
        }
    }
}
pub mod topology {
    pub mod karura {
        pub const ID: u32 = 2000;
        pub const AUSD_KEY: [u8; 2] = [0, 129];
        pub const KAR_KEY: [u8; 2] = [0, 128];
    }
    pub mod statemine {
        use super::common_good_assets;
        pub const ID: u32 = common_good_assets::ID;
        pub const ASSETS: u8 = common_good_assets::ASSETS;
        pub const USDT: u128 = common_good_assets::USDT;
    }
    pub mod rockmine {
        use super::common_good_assets;
        pub const ID: u32 = common_good_assets::ID;
        pub const ASSETS: u8 = common_good_assets::ASSETS;
        pub const USDT: u128 = common_good_assets::USDT;
    }
    pub mod common_good_assets {
        pub const ID: u32 = 1000;
        pub const ASSETS: u8 = 50;
        pub const USDT: u128 = 1984;
    }
    pub mod relay {
        use xcm::latest::prelude::*;
        pub const LOCATION: MultiLocation = MultiLocation {
            parents: 1,
            interior: Here,
        };
    }
    pub mod this {
        use xcm::latest::prelude::*;
        pub const LOCAL: MultiLocation = MultiLocation {
            parents: 0,
            interior: Here,
        };
        pub fn sibling(para_id: u32) -> MultiLocation {
            MultiLocation::new(1, X1(Parachain(para_id)))
        }
    }
}
