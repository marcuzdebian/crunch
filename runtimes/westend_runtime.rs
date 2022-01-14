#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 3)]
        Indices(indices::Event),
        #[codec(index = 4)]
        Balances(balances::Event),
        #[codec(index = 6)]
        Staking(staking::Event),
        #[codec(index = 7)]
        Offences(offences::Event),
        #[codec(index = 8)]
        Session(session::Event),
        #[codec(index = 10)]
        Grandpa(grandpa::Event),
        #[codec(index = 11)]
        ImOnline(im_online::Event),
        #[codec(index = 16)]
        Utility(utility::Event),
        #[codec(index = 17)]
        Identity(identity::Event),
        #[codec(index = 18)]
        Recovery(recovery::Event),
        #[codec(index = 19)]
        Vesting(vesting::Event),
        #[codec(index = 20)]
        Scheduler(scheduler::Event),
        #[codec(index = 21)]
        Sudo(sudo::Event),
        #[codec(index = 22)]
        Proxy(proxy::Event),
        #[codec(index = 23)]
        Multisig(multisig::Event),
        #[codec(index = 24)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 25)]
        BagsList(bags_list::Event),
        #[codec(index = 44)]
        ParaInclusion(para_inclusion::Event),
        #[codec(index = 47)]
        Paras(paras::Event),
        #[codec(index = 50)]
        Ump(ump::Event),
        #[codec(index = 51)]
        Hrmp(hrmp::Event),
        #[codec(index = 60)]
        Registrar(registrar::Event),
        #[codec(index = 61)]
        Slots(slots::Event),
        #[codec(index = 63)]
        Auctions(auctions::Event),
        #[codec(index = 64)]
        Crowdloan(crowdloan::Event),
        #[codec(index = 99)]
        XcmPallet(xcm_pallet::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<T, FillBlock> {
                    let call = FillBlock { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, Remark> {
                    let call = Remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHeapPages> {
                    let call = SetHeapPages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCode> {
                    let call = SetCode { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCodeWithoutChecks> {
                    let call = SetCodeWithoutChecks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetStorage> {
                    let call = SetStorage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<T, KillStorage> {
                    let call = KillStorage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, KillPrefix> {
                    let call = KillPrefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, RemarkWithEvent> {
                    let call = RemarkWithEvent { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CodeUpdated {}
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::westend_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ExtrinsicData>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::Error,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::westend_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::Error,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::Error,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    runtime_types::sp_consensus_babe::app::Public,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    runtime_types::sp_consensus_babe::app::Public,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PlanConfigChange {
                pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            impl ::subxt::Call for PlanConfigChange {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocation> {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocationUnsigned> {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<T, PlanConfigChange> {
                    let call = PlanConfigChange { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    [::core::primitive::u8; 32usize],
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::sp_consensus_slots::Slot, ::subxt::Error>
                {
                    let entry = GenesisSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::sp_consensus_slots::Slot, ::subxt::Error>
                {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::Error>
                {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_epoch_config_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::Error>
                {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::Error,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::Error,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::Error,
                > {
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Set> {
                    let call = Set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod indices {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Claim {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Claim {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "claim";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Free {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Free {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "free";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
                pub freeze: ::core::primitive::bool,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Freeze {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Freeze {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "freeze";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn claim(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Claim> {
                    let call = Claim { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { new, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn free(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Free> {
                    let call = Free { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                    freeze: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer { new, index, freeze };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Freeze> {
                    let call = Freeze { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_indices::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndexAssigned {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Event for IndexAssigned {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexAssigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndexFreed {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Event for IndexFreed {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFreed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndexFrozen {
                pub index: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for IndexFrozen {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFrozen";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Accounts(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Accounts {
                const PALLET: &'static str = "Indices";
                const STORAGE: &'static str = "Accounts";
                type Value = (
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::bool,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn accounts(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::bool,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Accounts(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn accounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Accounts>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetBalance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferKeepAlive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
                    let call = Transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetBalance> {
                    let call = SetBalance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
                    let call = ForceTransfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
                    let call = TransferKeepAlive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<T, TransferAll> {
                    let call = TransferAll { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceUnreserve> {
                    let call = ForceUnreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveRepatriated {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slashed {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_balances::Releases, ::subxt::Error>
                {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::Error,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::Error,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for SetUncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetUncles> {
                    let call = SetUncles { new_uncles };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Uncles;
            impl ::subxt::StorageEntry for Uncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Uncles";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Author";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetUncles;
            impl ::subxt::StorageEntry for DidSetUncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "DidSetUncles";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bond {
                pub controller:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for Bond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BondExtra {
                #[codec(compact)]
                pub max_additional: ::core::primitive::u128,
            }
            impl ::subxt::Call for BondExtra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond_extra";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unbond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Unbond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "unbond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct WithdrawUnbonded {
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for WithdrawUnbonded {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "withdraw_unbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Validate {
                pub prefs: runtime_types::pallet_staking::ValidatorPrefs,
            }
            impl ::subxt::Call for Validate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "validate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Nominate {
                pub targets: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for Nominate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "nominate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Chill {}
            impl ::subxt::Call for Chill {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetPayee {
                pub payee: runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for SetPayee {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_payee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetController {
                pub controller:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetController {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_controller";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetValidatorCount {
                #[codec(compact)]
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IncreaseValidatorCount {
                #[codec(compact)]
                pub additional: ::core::primitive::u32,
            }
            impl ::subxt::Call for IncreaseValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "increase_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScaleValidatorCount {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for ScaleValidatorCount {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "scale_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNoEras {}
            impl ::subxt::Call for ForceNoEras {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_no_eras";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNewEra {}
            impl ::subxt::Call for ForceNewEra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetInvulnerables {
                pub invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for SetInvulnerables {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnstake {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceUnstake {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_unstake";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNewEraAlways {}
            impl ::subxt::Call for ForceNewEraAlways {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era_always";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelDeferredSlash {
                pub era: ::core::primitive::u32,
                pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for CancelDeferredSlash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "cancel_deferred_slash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PayoutStakers {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::Call for PayoutStakers {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "payout_stakers";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rebond {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Rebond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "rebond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHistoryDepth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHistoryDepth {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReapStash {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReapStash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "reap_stash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Kick {
                pub who: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for Kick {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "kick";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetStakingLimits {
                pub min_nominator_bond: ::core::primitive::u128,
                pub min_validator_bond: ::core::primitive::u128,
                pub max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                pub max_validator_count: ::core::option::Option<::core::primitive::u32>,
                pub threshold:
                    ::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>,
            }
            impl ::subxt::Call for SetStakingLimits {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_staking_limits";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChillOther {
                pub controller: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ChillOther {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill_other";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn bond(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Bond> {
                    let call = Bond {
                        controller,
                        value,
                        payee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bond_extra(
                    &self,
                    max_additional: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, BondExtra> {
                    let call = BondExtra { max_additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Unbond> {
                    let call = Unbond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, WithdrawUnbonded> {
                    let call = WithdrawUnbonded { num_slashing_spans };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn validate(
                    &self,
                    prefs: runtime_types::pallet_staking::ValidatorPrefs,
                ) -> ::subxt::SubmittableExtrinsic<T, Validate> {
                    let call = Validate { prefs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominate(
                    &self,
                    targets: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Nominate> {
                    let call = Nominate { targets };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill(&self) -> ::subxt::SubmittableExtrinsic<T, Chill> {
                    let call = Chill {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_payee(
                    &self,
                    payee: runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetPayee> {
                    let call = SetPayee { payee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_controller(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetController> {
                    let call = SetController { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidatorCount> {
                    let call = SetValidatorCount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, IncreaseValidatorCount> {
                    let call = IncreaseValidatorCount { additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<T, ScaleValidatorCount> {
                    let call = ScaleValidatorCount { factor };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_no_eras(&self) -> ::subxt::SubmittableExtrinsic<T, ForceNoEras> {
                    let call = ForceNoEras {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era(&self) -> ::subxt::SubmittableExtrinsic<T, ForceNewEra> {
                    let call = ForceNewEra {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_invulnerables(
                    &self,
                    invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetInvulnerables> {
                    let call = SetInvulnerables { invulnerables };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceUnstake> {
                    let call = ForceUnstake {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era_always(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceNewEraAlways> {
                    let call = ForceNewEraAlways {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_deferred_slash(
                    &self,
                    era: ::core::primitive::u32,
                    slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelDeferredSlash> {
                    let call = CancelDeferredSlash { era, slash_indices };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::sp_core::crypto::AccountId32,
                    era: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, PayoutStakers> {
                    let call = PayoutStakers {
                        validator_stash,
                        era,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rebond(
                    &self,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Rebond> {
                    let call = Rebond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHistoryDepth> {
                    let call = SetHistoryDepth {
                        new_history_depth,
                        era_items_deleted,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_stash(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ReapStash> {
                    let call = ReapStash {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kick(
                    &self,
                    who: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Kick> {
                    let call = Kick { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_staking_limits(
                    &self,
                    min_nominator_bond: ::core::primitive::u128,
                    min_validator_bond: ::core::primitive::u128,
                    max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                    max_validator_count: ::core::option::Option<::core::primitive::u32>,
                    threshold: ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetStakingLimits> {
                    let call = SetStakingLimits {
                        min_nominator_bond,
                        min_validator_bond,
                        max_nominator_count,
                        max_validator_count,
                        threshold,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill_other(
                    &self,
                    controller: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, ChillOther> {
                    let call = ChillOther { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_staking::pallet::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraPaid(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
            impl ::subxt::Event for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakersElected {}
            impl ::subxt::Event for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Bonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Bonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unbonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Unbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdrawn(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrawn {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Withdrawn";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Kicked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakingElectionFailed {}
            impl ::subxt::Event for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Chilled(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PayoutStarted(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PayoutStarted {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "PayoutStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorCount;
            impl ::subxt::StorageEntry for ValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinimumValidatorCount;
            impl ::subxt::StorageEntry for MinimumValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinimumValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Invulnerables";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bonded(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Bonded {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Bonded";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct MinNominatorBond;
            impl ::subxt::StorageEntry for MinNominatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinNominatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinValidatorBond;
            impl ::subxt::StorageEntry for MinValidatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinValidatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Ledger(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Ledger {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Ledger";
                type Value = runtime_types::pallet_staking::StakingLedger<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Payee(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Payee {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Payee";
                type Value = runtime_types::pallet_staking::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Validators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Validators";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CounterForValidators;
            impl ::subxt::StorageEntry for CounterForValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForValidators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxValidatorsCount;
            impl ::subxt::StorageEntry for MaxValidatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxValidatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nominators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Nominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Nominators";
                type Value = runtime_types::pallet_staking::Nominations<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CounterForNominators;
            impl ::subxt::StorageEntry for CounterForNominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForNominators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxNominatorsCount;
            impl ::subxt::StorageEntry for MaxNominatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxNominatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::pallet_staking::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasStakers(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakers";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasStakersClipped(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakersClipped {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakersClipped";
                type Value = runtime_types::pallet_staking::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasValidatorPrefs(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasValidatorPrefs {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorPrefs";
                type Value = runtime_types::pallet_staking::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasValidatorReward(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasRewardPoints(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::pallet_staking::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasTotalStake(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ForceEra;
            impl ::subxt::StorageEntry for ForceEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ForceEra";
                type Value = runtime_types::pallet_staking::Forcing;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SlashRewardFraction;
            impl ::subxt::StorageEntry for SlashRewardFraction {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashRewardFraction";
                type Value = runtime_types::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CanceledSlashPayout;
            impl ::subxt::StorageEntry for CanceledSlashPayout {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CanceledSlashPayout";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnappliedSlashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnappliedSlashes {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "UnappliedSlashes";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_staking::UnappliedSlash<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "BondedEras";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSlashInEra(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ValidatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorSlashInEra";
                type Value = (
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct NominatorSlashInEra(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for NominatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "NominatorSlashInEra";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct SlashingSpans(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SlashingSpans {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashingSpans";
                type Value = runtime_types::pallet_staking::slashing::SlashingSpans;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct SpanSlash(
                ::subxt::sp_core::crypto::AccountId32,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for SpanSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SpanSlash";
                type Value =
                    runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct EarliestUnappliedSlash;
            impl ::subxt::StorageEntry for EarliestUnappliedSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "EarliestUnappliedSlash";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OffendingValidators;
            impl ::subxt::StorageEntry for OffendingValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "OffendingValidators";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_staking::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChillThreshold;
            impl ::subxt::StorageEntry for ChillThreshold {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ChillThreshold";
                type Value = runtime_types::sp_arithmetic::per_things::Percent;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = ValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn minimum_validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = MinimumValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn invulnerables(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Invulnerables;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bonded(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Bonded(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bonded_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Bonded>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn min_nominator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = MinNominatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn min_validator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = MinValidatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ledger(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::StakingLedger<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Ledger(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn ledger_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Ledger>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn payee(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Payee(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn payee_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Payee>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn validators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::Error,
                > {
                    let entry = Validators(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Validators>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CounterForValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_validators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = MaxValidatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_staking::Nominations<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Nominators(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nominators>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_nominators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CounterForNominators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_nominators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = MaxNominatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_staking::ActiveEraInfo>,
                    ::subxt::Error,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasStakers>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers_clipped(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasStakersClipped(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_clipped_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakersClipped>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_prefs(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::ValidatorPrefs,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorPrefs(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_validator_prefs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorPrefs>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasRewardPoints>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasTotalStake>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn force_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_staking::Forcing, ::subxt::Error>
                {
                    let entry = ForceEra;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn slash_reward_fraction(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::Error,
                > {
                    let entry = SlashRewardFraction;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn canceled_slash_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error>
                {
                    let entry = CanceledSlashPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_staking::UnappliedSlash<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UnappliedSlashes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, UnappliedSlashes>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = ValidatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn validator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ValidatorSlashInEra>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nominator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = NominatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NominatorSlashInEra>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn slashing_spans(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_staking::slashing::SlashingSpans>,
                    ::subxt::Error,
                > {
                    let entry = SlashingSpans(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn slashing_spans_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SlashingSpans>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn span_slash(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = SpanSlash(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn span_slash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SpanSlash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn earliest_unapplied_slash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = EarliestUnappliedSlash;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn offending_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
                    ::subxt::Error,
                > {
                    let entry = OffendingValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_staking::Releases, ::subxt::Error>
                {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn chill_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>,
                    ::subxt::Error,
                > {
                    let entry = ChillThreshold;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod offences {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_offences::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Offence {
                pub kind: [::core::primitive::u8; 16usize],
                pub timeslot: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for Offence {
                const PALLET: &'static str = "Offences";
                const EVENT: &'static str = "Offence";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Reports(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Reports {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "Reports";
                type Value = runtime_types::sp_staking::offence::OffenceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    (
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_staking::Exposure<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    ),
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ConcurrentReportsIndex(
                [::core::primitive::u8; 16usize],
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for ConcurrentReportsIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ConcurrentReportsIndex";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ReportsByKindIndex(pub [::core::primitive::u8; 16usize]);
            impl ::subxt::StorageEntry for ReportsByKindIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ReportsByKindIndex";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn reports(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_staking::offence::OffenceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            (
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::pallet_staking::Exposure<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    ::core::primitive::u128,
                                >,
                            ),
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Reports(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reports_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reports>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn concurrent_reports_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::subxt::sp_core::H256>, ::subxt::Error>
                {
                    let entry = ConcurrentReportsIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn concurrent_reports_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConcurrentReportsIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn reports_by_kind_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
                {
                    let entry = ReportsByKindIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reports_by_kind_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReportsByKindIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetKeys {
                pub keys: runtime_types::westend_runtime::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PurgeKeys {}
            impl ::subxt::Call for PurgeKeys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_keys(
                    &self,
                    keys: runtime_types::westend_runtime::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetKeys> {
                    let call = SetKeys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(&self) -> ::subxt::SubmittableExtrinsic<T, PurgeKeys> {
                    let call = PurgeKeys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::westend_runtime::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value = runtime_types::westend_runtime::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner(
                runtime_types::sp_core::crypto::KeyTypeId,
                ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for KeyOwner {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error>
                {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::westend_runtime::SessionKeys,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u32>, ::subxt::Error>
                {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::westend_runtime::SessionKeys>,
                    ::subxt::Error,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextKeys>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, KeyOwner>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocation {
                pub equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u32,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u32,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocation> {
                    let call = ReportEquivocation {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<T, ReportEquivocationUnsigned> {
                    let call = ReportEquivocationUnsigned {
                        equivocation_proof,
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, NoteStalled> {
                    let call = NoteStalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Paused {}
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Resumed {}
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SetIdSession>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            impl ::subxt::Call for Heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ) -> ::subxt::SubmittableExtrinsic<T, Heartbeat> {
                    let call = Heartbeat {
                        heartbeat,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HeartbeatReceived {
                pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            }
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AllGood {}
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SomeOffline {
                pub offline: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_staking::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                )>,
            }
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats(::core::primitive::u32, ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReceivedHeartbeats {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
                    runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct AuthoredBlocks(
                ::core::primitive::u32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AuthoredBlocks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod authority_discovery {
        use super::runtime_types;
    }
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Batch {
                pub calls: ::std::vec::Vec<runtime_types::westend_runtime::Call>,
            }
            impl ::subxt::Call for Batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsDerivative {
                pub index: ::core::primitive::u16,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for AsDerivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchAll {
                pub calls: ::std::vec::Vec<runtime_types::westend_runtime::Call>,
            }
            impl ::subxt::Call for BatchAll {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DispatchAs {
                pub as_origin: runtime_types::westend_runtime::OriginCaller,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for DispatchAs {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "dispatch_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::westend_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, Batch> {
                    let call = Batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsDerivative> {
                    let call = AsDerivative { index, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::westend_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<T, BatchAll> {
                    let call = BatchAll { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dispatch_as(
                    &self,
                    as_origin: runtime_types::westend_runtime::OriginCaller,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, DispatchAs> {
                    let call = DispatchAs { as_origin, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchInterrupted {
                pub index: ::core::primitive::u32,
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BatchCompleted {}
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ItemCompleted {}
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct DispatchedAs(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
    }
    pub mod identity {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddRegistrar {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for AddRegistrar {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_registrar";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetIdentity {
                pub info: runtime_types::pallet_identity::types::IdentityInfo,
            }
            impl ::subxt::Call for SetIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetSubs {
                pub subs: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                )>,
            }
            impl ::subxt::Call for SetSubs {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_subs";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearIdentity {}
            impl ::subxt::Call for ClearIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "clear_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RequestJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                #[codec(compact)]
                pub max_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for RequestJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "request_judgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelRequest {
                pub reg_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for CancelRequest {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "cancel_request";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetFee {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetFee {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetAccountId {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetAccountId {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_account_id";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetFields {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub fields: runtime_types::pallet_identity::types::BitFlags<
                    runtime_types::pallet_identity::types::IdentityField,
                >,
            }
            impl ::subxt::Call for SetFields {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "set_fields";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProvideJudgement {
                #[codec(compact)]
                pub reg_index: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub judgement:
                    runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
            }
            impl ::subxt::Call for ProvideJudgement {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "provide_judgement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillIdentity {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for KillIdentity {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "kill_identity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddSub {
                pub sub:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for AddSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "add_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RenameSub {
                pub sub:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub data: runtime_types::pallet_identity::types::Data,
            }
            impl ::subxt::Call for RenameSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "rename_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveSub {
                pub sub:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for RemoveSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "remove_sub";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct QuitSub {}
            impl ::subxt::Call for QuitSub {
                const PALLET: &'static str = "Identity";
                const FUNCTION: &'static str = "quit_sub";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn add_registrar(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, AddRegistrar> {
                    let call = AddRegistrar { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_identity(
                    &self,
                    info: runtime_types::pallet_identity::types::IdentityInfo,
                ) -> ::subxt::SubmittableExtrinsic<T, SetIdentity> {
                    let call = SetIdentity { info };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_subs(
                    &self,
                    subs: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetSubs> {
                    let call = SetSubs { subs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_identity(&self) -> ::subxt::SubmittableExtrinsic<T, ClearIdentity> {
                    let call = ClearIdentity {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn request_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    max_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, RequestJudgement> {
                    let call = RequestJudgement { reg_index, max_fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_request(
                    &self,
                    reg_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelRequest> {
                    let call = CancelRequest { reg_index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fee(
                    &self,
                    index: ::core::primitive::u32,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFee> {
                    let call = SetFee { index, fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_account_id(
                    &self,
                    index: ::core::primitive::u32,
                    new: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetAccountId> {
                    let call = SetAccountId { index, new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_fields(
                    &self,
                    index: ::core::primitive::u32,
                    fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetFields> {
                    let call = SetFields { index, fields };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn provide_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    judgement: runtime_types::pallet_identity::types::Judgement<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ProvideJudgement> {
                    let call = ProvideJudgement {
                        reg_index,
                        target,
                        judgement,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_identity(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, KillIdentity> {
                    let call = KillIdentity { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::SubmittableExtrinsic<T, AddSub> {
                    let call = AddSub { sub, data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rename_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::SubmittableExtrinsic<T, RenameSub> {
                    let call = RenameSub { sub, data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_sub(
                    &self,
                    sub: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveSub> {
                    let call = RemoveSub { sub };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn quit_sub(&self) -> ::subxt::SubmittableExtrinsic<T, QuitSub> {
                    let call = QuitSub {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_identity::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentitySet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for IdentitySet {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentitySet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentityCleared {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::Event for IdentityCleared {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityCleared";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IdentityKilled {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::Event for IdentityKilled {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityKilled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementRequested {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for JudgementRequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementRequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementUnrequested {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for JudgementUnrequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementUnrequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct JudgementGiven {
                pub target: ::subxt::sp_core::crypto::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for JudgementGiven {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementGiven";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RegistrarAdded {
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for RegistrarAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "RegistrarAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityAdded {
                pub sub: ::subxt::sp_core::crypto::AccountId32,
                pub main: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::Event for SubIdentityAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityRemoved {
                pub sub: ::subxt::sp_core::crypto::AccountId32,
                pub main: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::Event for SubIdentityRemoved {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRemoved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubIdentityRevoked {
                pub sub: ::subxt::sp_core::crypto::AccountId32,
                pub main: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::Event for SubIdentityRevoked {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRevoked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct IdentityOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for IdentityOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "IdentityOf";
                type Value =
                    runtime_types::pallet_identity::types::Registration<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct SuperOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SuperOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "SuperOf";
                type Value = (
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_identity::types::Data,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct SubsOf(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SubsOf {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "SubsOf";
                type Value = (
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Registrars;
            impl ::subxt::StorageEntry for Registrars {
                const PALLET: &'static str = "Identity";
                const STORAGE: &'static str = "Registrars";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::option::Option<
                        runtime_types::pallet_identity::types::RegistrarInfo<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn identity_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_identity::types::Registration<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = IdentityOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn identity_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, IdentityOf>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn super_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = SuperOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn super_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SuperOf>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn subs_of(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    ),
                    ::subxt::Error,
                > {
                    let entry = SubsOf(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn subs_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SubsOf>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn registrars(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::option::Option<
                            runtime_types::pallet_identity::types::RegistrarInfo<
                                ::core::primitive::u128,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Registrars;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod recovery {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsRecovered {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for AsRecovered {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "as_recovered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetRecovered {
                pub lost: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for SetRecovered {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "set_recovered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CreateRecovery {
                pub friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub threshold: ::core::primitive::u16,
                pub delay_period: ::core::primitive::u32,
            }
            impl ::subxt::Call for CreateRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "create_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InitiateRecovery {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for InitiateRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "initiate_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VouchRecovery {
                pub lost: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for VouchRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "vouch_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClaimRecovery {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for ClaimRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "claim_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CloseRecovery {
                pub rescuer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for CloseRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "close_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveRecovery {}
            impl ::subxt::Call for RemoveRecovery {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "remove_recovery";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelRecovered {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for CancelRecovered {
                const PALLET: &'static str = "Recovery";
                const FUNCTION: &'static str = "cancel_recovered";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn as_recovered(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsRecovered> {
                    let call = AsRecovered { account, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_recovered(
                    &self,
                    lost: ::subxt::sp_core::crypto::AccountId32,
                    rescuer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetRecovered> {
                    let call = SetRecovered { lost, rescuer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create_recovery(
                    &self,
                    friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    threshold: ::core::primitive::u16,
                    delay_period: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, CreateRecovery> {
                    let call = CreateRecovery {
                        friends,
                        threshold,
                        delay_period,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn initiate_recovery(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, InitiateRecovery> {
                    let call = InitiateRecovery { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vouch_recovery(
                    &self,
                    lost: ::subxt::sp_core::crypto::AccountId32,
                    rescuer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, VouchRecovery> {
                    let call = VouchRecovery { lost, rescuer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn claim_recovery(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, ClaimRecovery> {
                    let call = ClaimRecovery { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close_recovery(
                    &self,
                    rescuer: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, CloseRecovery> {
                    let call = CloseRecovery { rescuer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_recovery(&self) -> ::subxt::SubmittableExtrinsic<T, RemoveRecovery> {
                    let call = RemoveRecovery {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_recovered(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelRecovered> {
                    let call = CancelRecovered { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_recovery::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryCreated {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for RecoveryCreated {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryInitiated {
                pub lost_account: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for RecoveryInitiated {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryInitiated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryVouched {
                pub lost_account: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
                pub sender: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for RecoveryVouched {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryVouched";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryClosed {
                pub lost_account: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for RecoveryClosed {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryClosed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountRecovered {
                pub lost_account: ::subxt::sp_core::crypto::AccountId32,
                pub rescuer_account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for AccountRecovered {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "AccountRecovered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryRemoved {
                pub lost_account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for RecoveryRemoved {
                const PALLET: &'static str = "Recovery";
                const EVENT: &'static str = "RecoveryRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Recoverable(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Recoverable {
                const PALLET: &'static str = "Recovery";
                const STORAGE: &'static str = "Recoverable";
                type Value = runtime_types::pallet_recovery::RecoveryConfig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ActiveRecoveries(
                ::subxt::sp_core::crypto::AccountId32,
                ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ActiveRecoveries {
                const PALLET: &'static str = "Recovery";
                const STORAGE: &'static str = "ActiveRecoveries";
                type Value = runtime_types::pallet_recovery::ActiveRecovery<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct Proxy(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proxy {
                const PALLET: &'static str = "Recovery";
                const STORAGE: &'static str = "Proxy";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn recoverable(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_recovery::RecoveryConfig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Recoverable(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn recoverable_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Recoverable>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn active_recoveries(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_recovery::ActiveRecovery<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ActiveRecoveries(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_recoveries_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ActiveRecoveries>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn proxy(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::Error,
                > {
                    let entry = Proxy(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proxy_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proxy>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod vesting {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Vest {}
            impl ::subxt::Call for Vest {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestOther {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for VestOther {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vest_other";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestedTransfer {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for VestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceVestedTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >,
            }
            impl ::subxt::Call for ForceVestedTransfer {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "force_vested_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MergeSchedules {
                pub schedule1_index: ::core::primitive::u32,
                pub schedule2_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for MergeSchedules {
                const PALLET: &'static str = "Vesting";
                const FUNCTION: &'static str = "merge_schedules";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn vest(&self) -> ::subxt::SubmittableExtrinsic<T, Vest> {
                    let call = Vest {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vest_other(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestOther> {
                    let call = VestOther { target };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vested_transfer(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, VestedTransfer> {
                    let call = VestedTransfer { target, schedule };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_vested_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceVestedTransfer> {
                    let call = ForceVestedTransfer {
                        source,
                        target,
                        schedule,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn merge_schedules(
                    &self,
                    schedule1_index: ::core::primitive::u32,
                    schedule2_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, MergeSchedules> {
                    let call = MergeSchedules {
                        schedule1_index,
                        schedule2_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_vesting::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingUpdated {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub unvested: ::core::primitive::u128,
            }
            impl ::subxt::Event for VestingUpdated {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VestingCompleted {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for VestingCompleted {
                const PALLET: &'static str = "Vesting";
                const EVENT: &'static str = "VestingCompleted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Vesting(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Vesting {
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "Vesting";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Vesting";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_vesting::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn vesting(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Vesting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn vesting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Vesting>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_vesting::Releases, ::subxt::Error>
                {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod scheduler {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for Schedule {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for Cancel {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduleNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for ScheduleNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelNamed {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for CancelNamed {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduleAfter {
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for ScheduleAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_after";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduleNamedAfter {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for ScheduleNamedAfter {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named_after";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Schedule> {
                    let call = Schedule {
                        when,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Cancel> {
                    let call = Cancel { when, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ScheduleNamed> {
                    let call = ScheduleNamed {
                        id,
                        when,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, CancelNamed> {
                    let call = CancelNamed { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ScheduleAfter> {
                    let call = ScheduleAfter {
                        after,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named_after(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ScheduleNamedAfter> {
                    let call = ScheduleNamedAfter {
                        id,
                        after,
                        maybe_periodic,
                        priority,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Scheduled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Canceled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Dispatched(
                pub (::core::primitive::u32, ::core::primitive::u32),
                pub ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Agenda(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Agenda {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Agenda";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<
                        runtime_types::pallet_scheduler::ScheduledV2<
                            runtime_types::westend_runtime::Call,
                            ::core::primitive::u32,
                            runtime_types::westend_runtime::OriginCaller,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Lookup(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for Lookup {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Lookup";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_scheduler::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn agenda(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<
                            runtime_types::pallet_scheduler::ScheduledV2<
                                runtime_types::westend_runtime::Call,
                                ::core::primitive::u32,
                                runtime_types::westend_runtime::OriginCaller,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Agenda(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn agenda_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Agenda>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn lookup(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = Lookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Lookup>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_scheduler::Releases, ::subxt::Error>
                {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudo {
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoUncheckedWeight {
                pub call: runtime_types::westend_runtime::Call,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAs {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Sudo> {
                    let call = Sudo { call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::westend_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoUncheckedWeight> {
                    let call = SudoUncheckedWeight { call, weight };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, SetKey> {
                    let call = SetKey { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoAs> {
                    let call = SudoAs { who, call };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KeyChanged {
                pub new_sudoer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::crypto::AccountId32, ::subxt::Error>
                {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod proxy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Proxy {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::westend_runtime::ProxyType>,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for Proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::westend_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for AddProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "add_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveProxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::westend_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for RemoveProxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveProxies {}
            impl ::subxt::Call for RemoveProxies {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxies";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Anonymous {
                pub proxy_type: runtime_types::westend_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
                pub index: ::core::primitive::u16,
            }
            impl ::subxt::Call for Anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct KillAnonymous {
                pub spawner: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::westend_runtime::ProxyType,
                pub index: ::core::primitive::u16,
                #[codec(compact)]
                pub height: ::core::primitive::u32,
                #[codec(compact)]
                pub ext_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillAnonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "kill_anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Announce {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for Announce {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "announce";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RemoveAnnouncement {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RemoveAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RejectAnnouncement {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for RejectAnnouncement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "reject_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyAnnounced {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::westend_runtime::ProxyType>,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for ProxyAnnounced {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy_announced";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn proxy(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::westend_runtime::ProxyType,
                    >,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, Proxy> {
                    let call = Proxy {
                        real,
                        force_proxy_type,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::westend_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, AddProxy> {
                    let call = AddProxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::westend_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveProxy> {
                    let call = RemoveProxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxies(&self) -> ::subxt::SubmittableExtrinsic<T, RemoveProxies> {
                    let call = RemoveProxies {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn anonymous(
                    &self,
                    proxy_type: runtime_types::westend_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<T, Anonymous> {
                    let call = Anonymous {
                        proxy_type,
                        delay,
                        index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_anonymous(
                    &self,
                    spawner: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::westend_runtime::ProxyType,
                    index: ::core::primitive::u16,
                    height: ::core::primitive::u32,
                    ext_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, KillAnonymous> {
                    let call = KillAnonymous {
                        spawner,
                        proxy_type,
                        index,
                        height,
                        ext_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn announce(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, Announce> {
                    let call = Announce { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_announcement(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, RemoveAnnouncement> {
                    let call = RemoveAnnouncement { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_announcement(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<T, RejectAnnouncement> {
                    let call = RejectAnnouncement {
                        delegate,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn proxy_announced(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::westend_runtime::ProxyType,
                    >,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, ProxyAnnounced> {
                    let call = ProxyAnnounced {
                        delegate,
                        real,
                        force_proxy_type,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_proxy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyExecuted {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for ProxyExecuted {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AnonymousCreated {
                pub anonymous: ::subxt::sp_core::crypto::AccountId32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::westend_runtime::ProxyType,
                pub disambiguation_index: ::core::primitive::u16,
            }
            impl ::subxt::Event for AnonymousCreated {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "AnonymousCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Announced {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub proxy: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Announced {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "Announced";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyAdded {
                pub delegator: ::subxt::sp_core::crypto::AccountId32,
                pub delegatee: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::westend_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProxyAdded {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyAdded";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proxies(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proxies {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Proxies";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::ProxyDefinition<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::westend_runtime::ProxyType,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Announcements(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Announcements {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Announcements";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::Announcement<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proxies(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::ProxyDefinition<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::westend_runtime::ProxyType,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    ),
                    ::subxt::Error,
                > {
                    let entry = Proxies(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proxies_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proxies>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn announcements(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::Announcement<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    ),
                    ::subxt::Error,
                > {
                    let entry = Announcements(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announcements_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Announcements>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMultiThreshold1 {
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: runtime_types::westend_runtime::Call,
            }
            impl ::subxt::Call for AsMultiThreshold1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold1";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call: ::subxt::WrapperKeepOpaque<runtime_types::westend_runtime::Call>,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for AsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ApproveAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for ApproveAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelAsMulti {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for CancelAsMulti {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "cancel_as_multi";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn as_multi_threshold1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    call: runtime_types::westend_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<T, AsMultiThreshold1> {
                    let call = AsMultiThreshold1 {
                        other_signatories,
                        call,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::subxt::WrapperKeepOpaque<runtime_types::westend_runtime::Call>,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, AsMulti> {
                    let call = AsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call,
                        store_call,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ApproveAsMulti> {
                    let call = ApproveAsMulti {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<T, CancelAsMulti> {
                    let call = CancelAsMulti {
                        threshold,
                        other_signatories,
                        timepoint,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewMultisig {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigApproval {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigExecuted {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MultisigCancelled {
                pub cancelling: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                ::subxt::sp_core::crypto::AccountId32,
                [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::subxt::WrapperKeepOpaque<runtime_types::westend_runtime::Call>,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn multisigs(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Multisigs>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::WrapperKeepOpaque<runtime_types::westend_runtime::Call>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod election_provider_multi_phase {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SubmitUnsigned {
                pub raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
                    runtime_types::westend_runtime::NposCompactSolution16,
                >,
                pub witness:
                    runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
            }
            impl ::subxt::Call for SubmitUnsigned {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMinimumUntrustedScore {
                pub maybe_next_score: ::core::option::Option<[::core::primitive::u128; 3usize]>,
            }
            impl ::subxt::Call for SetMinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_minimum_untrusted_score";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetEmergencyElectionResult {
                pub supports: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_npos_elections::Support<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            impl ::subxt::Call for SetEmergencyElectionResult {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_emergency_election_result";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Submit {
                pub raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
                    runtime_types::westend_runtime::NposCompactSolution16,
                >,
                pub num_signed_submissions: ::core::primitive::u32,
            }
            impl ::subxt::Call for Submit {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn submit_unsigned(
                    &self,
                    raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
                        runtime_types::westend_runtime::NposCompactSolution16,
                    >,
                    witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
                ) -> ::subxt::SubmittableExtrinsic<T, SubmitUnsigned> {
                    let call = SubmitUnsigned {
                        raw_solution,
                        witness,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_minimum_untrusted_score(
                    &self,
                    maybe_next_score: ::core::option::Option<[::core::primitive::u128; 3usize]>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMinimumUntrustedScore> {
                    let call = SetMinimumUntrustedScore { maybe_next_score };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_emergency_election_result(
                    &self,
                    supports: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_npos_elections::Support<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetEmergencyElectionResult> {
                    let call = SetEmergencyElectionResult { supports };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit(
                    &self,
                    raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
                        runtime_types::westend_runtime::NposCompactSolution16,
                    >,
                    num_signed_submissions: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, Submit> {
                    let call = Submit {
                        raw_solution,
                        num_signed_submissions,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_election_provider_multi_phase::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SolutionStored {
                pub election_compute:
                    runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                pub prev_ejected: ::core::primitive::bool,
            }
            impl ::subxt::Event for SolutionStored {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SolutionStored";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ElectionFinalized {
                pub election_compute: ::core::option::Option<
                    runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                >,
            }
            impl ::subxt::Event for ElectionFinalized {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFinalized";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rewarded {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Slashed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SignedPhaseStarted {
                pub round: ::core::primitive::u32,
            }
            impl ::subxt::Event for SignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SignedPhaseStarted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnsignedPhaseStarted {
                pub round: ::core::primitive::u32,
            }
            impl ::subxt::Event for UnsignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "UnsignedPhaseStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Round;
            impl ::subxt::StorageEntry for Round {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Round";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPhase;
            impl ::subxt::StorageEntry for CurrentPhase {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "CurrentPhase";
                type Value = runtime_types::pallet_election_provider_multi_phase::Phase<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedSolution;
            impl ::subxt::StorageEntry for QueuedSolution {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "QueuedSolution";
                type Value = runtime_types::pallet_election_provider_multi_phase::ReadySolution<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Snapshot;
            impl ::subxt::StorageEntry for Snapshot {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Snapshot";
                type Value = runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DesiredTargets;
            impl ::subxt::StorageEntry for DesiredTargets {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "DesiredTargets";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SnapshotMetadata;
            impl ::subxt::StorageEntry for SnapshotMetadata {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SnapshotMetadata";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionNextIndex;
            impl ::subxt::StorageEntry for SignedSubmissionNextIndex {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionNextIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionIndices;
            impl ::subxt::StorageEntry for SignedSubmissionIndices {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionIndices";
                type Value =
                    runtime_types::frame_support::storage::bounded_btree_map::BoundedBTreeMap<
                        [::core::primitive::u128; 3usize],
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionsMap(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for SignedSubmissionsMap {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionsMap";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::signed::SignedSubmission<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        runtime_types::westend_runtime::NposCompactSolution16,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct MinimumUntrustedScore;
            impl ::subxt::StorageEntry for MinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "MinimumUntrustedScore";
                type Value = [::core::primitive::u128; 3usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn round(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = Round;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentPhase;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_solution(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_election_provider_multi_phase::ReadySolution<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = QueuedSolution;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn snapshot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Snapshot;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn desired_targets(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = DesiredTargets;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn snapshot_metadata(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
                    >,
                    ::subxt::Error,
                > {
                    let entry = SnapshotMetadata;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submission_next_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = SignedSubmissionNextIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signed_submission_indices(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_btree_map::BoundedBTreeMap<
                        [::core::primitive::u128; 3usize],
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = SignedSubmissionIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signed_submissions_map(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_election_provider_multi_phase::signed::SignedSubmission<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        runtime_types::westend_runtime::NposCompactSolution16,
                    >,
                    ::subxt::Error,
                > {
                    let entry = SignedSubmissionsMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signed_submissions_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedSubmissionsMap>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn minimum_untrusted_score(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
                    ::subxt::Error,
                > {
                    let entry = MinimumUntrustedScore;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod bags_list {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rebag {
                pub dislocated: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for Rebag {
                const PALLET: &'static str = "BagsList";
                const FUNCTION: &'static str = "rebag";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn rebag(
                    &self,
                    dislocated: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<T, Rebag> {
                    let call = Rebag { dislocated };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bags_list::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Rebagged {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub from: ::core::primitive::u64,
                pub to: ::core::primitive::u64,
            }
            impl ::subxt::Event for Rebagged {
                const PALLET: &'static str = "BagsList";
                const EVENT: &'static str = "Rebagged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ListNodes(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for ListNodes {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "ListNodes";
                type Value = runtime_types::pallet_bags_list::list::Node;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CounterForListNodes;
            impl ::subxt::StorageEntry for CounterForListNodes {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "CounterForListNodes";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ListBags(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for ListBags {
                const PALLET: &'static str = "BagsList";
                const STORAGE: &'static str = "ListBags";
                type Value = runtime_types::pallet_bags_list::list::Bag;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn list_nodes(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_bags_list::list::Node>,
                    ::subxt::Error,
                > {
                    let entry = ListNodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn list_nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ListNodes>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_list_nodes(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CounterForListNodes;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn list_bags(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_bags_list::list::Bag>,
                    ::subxt::Error,
                > {
                    let entry = ListBags(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn list_bags_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ListBags>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod parachains_origin {
        use super::runtime_types;
    }
    pub mod configuration {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetValidationUpgradeFrequency {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetValidationUpgradeFrequency {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_validation_upgrade_frequency";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetValidationUpgradeDelay {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetValidationUpgradeDelay {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_validation_upgrade_delay";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetCodeRetentionPeriod {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetCodeRetentionPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_code_retention_period";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxCodeSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxCodeSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_code_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxPovSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxPovSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_pov_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxHeadDataSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxHeadDataSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_head_data_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetParathreadCores {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetParathreadCores {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_parathread_cores";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetParathreadRetries {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetParathreadRetries {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_parathread_retries";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetGroupRotationFrequency {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetGroupRotationFrequency {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_group_rotation_frequency";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetChainAvailabilityPeriod {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetChainAvailabilityPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_chain_availability_period";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetThreadAvailabilityPeriod {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetThreadAvailabilityPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_thread_availability_period";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetSchedulingLookahead {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetSchedulingLookahead {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_scheduling_lookahead";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxValidatorsPerCore {
                pub new: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for SetMaxValidatorsPerCore {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_validators_per_core";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxValidators {
                pub new: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for SetMaxValidators {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_validators";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetDisputePeriod {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetDisputePeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_period";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetDisputePostConclusionAcceptancePeriod {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetDisputePostConclusionAcceptancePeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_post_conclusion_acceptance_period";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetDisputeMaxSpamSlots {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetDisputeMaxSpamSlots {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_max_spam_slots";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetDisputeConclusionByTimeOutPeriod {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetDisputeConclusionByTimeOutPeriod {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_conclusion_by_time_out_period";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetNoShowSlots {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetNoShowSlots {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_no_show_slots";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetNDelayTranches {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetNDelayTranches {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_n_delay_tranches";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetZerothDelayTrancheWidth {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetZerothDelayTrancheWidth {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_zeroth_delay_tranche_width";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetNeededApprovals {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetNeededApprovals {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_needed_approvals";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetRelayVrfModuloSamples {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetRelayVrfModuloSamples {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_relay_vrf_modulo_samples";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxUpwardQueueCount {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxUpwardQueueCount {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_queue_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxUpwardQueueSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxUpwardQueueSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_queue_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxDownwardMessageSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxDownwardMessageSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_downward_message_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetUmpServiceTotalWeight {
                pub new: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetUmpServiceTotalWeight {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_ump_service_total_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxUpwardMessageSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxUpwardMessageSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_message_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetMaxUpwardMessageNumPerCandidate {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetMaxUpwardMessageNumPerCandidate {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_message_num_per_candidate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpOpenRequestTtl {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpOpenRequestTtl {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_open_request_ttl";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpSenderDeposit {
                pub new: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetHrmpSenderDeposit {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_sender_deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpRecipientDeposit {
                pub new: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetHrmpRecipientDeposit {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_recipient_deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpChannelMaxCapacity {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpChannelMaxCapacity {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_capacity";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpChannelMaxTotalSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpChannelMaxTotalSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_total_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpMaxParachainInboundChannels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpMaxParachainInboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parachain_inbound_channels";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpMaxParathreadInboundChannels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpMaxParathreadInboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parathread_inbound_channels";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpChannelMaxMessageSize {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpChannelMaxMessageSize {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_message_size";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpMaxParachainOutboundChannels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpMaxParachainOutboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parachain_outbound_channels";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpMaxParathreadOutboundChannels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpMaxParathreadOutboundChannels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parathread_outbound_channels";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetHrmpMaxMessageNumPerCandidate {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for SetHrmpMaxMessageNumPerCandidate {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_message_num_per_candidate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SetUmpMaxIndividualWeight {
                pub new: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetUmpMaxIndividualWeight {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_ump_max_individual_weight";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn set_validation_upgrade_frequency(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidationUpgradeFrequency>
                {
                    let call = SetValidationUpgradeFrequency { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validation_upgrade_delay(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetValidationUpgradeDelay> {
                    let call = SetValidationUpgradeDelay { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_retention_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetCodeRetentionPeriod> {
                    let call = SetCodeRetentionPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_code_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxCodeSize> {
                    let call = SetMaxCodeSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_pov_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxPovSize> {
                    let call = SetMaxPovSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_head_data_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxHeadDataSize> {
                    let call = SetMaxHeadDataSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parathread_cores(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetParathreadCores> {
                    let call = SetParathreadCores { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parathread_retries(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetParathreadRetries> {
                    let call = SetParathreadRetries { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_group_rotation_frequency(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetGroupRotationFrequency> {
                    let call = SetGroupRotationFrequency { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_chain_availability_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetChainAvailabilityPeriod> {
                    let call = SetChainAvailabilityPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_thread_availability_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetThreadAvailabilityPeriod> {
                    let call = SetThreadAvailabilityPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_scheduling_lookahead(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetSchedulingLookahead> {
                    let call = SetSchedulingLookahead { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_validators_per_core(
                    &self,
                    new: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxValidatorsPerCore> {
                    let call = SetMaxValidatorsPerCore { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_validators(
                    &self,
                    new: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxValidators> {
                    let call = SetMaxValidators { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDisputePeriod> {
                    let call = SetDisputePeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_post_conclusion_acceptance_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDisputePostConclusionAcceptancePeriod>
                {
                    let call = SetDisputePostConclusionAcceptancePeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_max_spam_slots(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDisputeMaxSpamSlots> {
                    let call = SetDisputeMaxSpamSlots { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_conclusion_by_time_out_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetDisputeConclusionByTimeOutPeriod>
                {
                    let call = SetDisputeConclusionByTimeOutPeriod { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_no_show_slots(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetNoShowSlots> {
                    let call = SetNoShowSlots { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_n_delay_tranches(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetNDelayTranches> {
                    let call = SetNDelayTranches { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_zeroth_delay_tranche_width(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetZerothDelayTrancheWidth> {
                    let call = SetZerothDelayTrancheWidth { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_needed_approvals(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetNeededApprovals> {
                    let call = SetNeededApprovals { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_relay_vrf_modulo_samples(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetRelayVrfModuloSamples> {
                    let call = SetRelayVrfModuloSamples { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_queue_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardQueueCount> {
                    let call = SetMaxUpwardQueueCount { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_queue_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardQueueSize> {
                    let call = SetMaxUpwardQueueSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_downward_message_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxDownwardMessageSize> {
                    let call = SetMaxDownwardMessageSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_ump_service_total_weight(
                    &self,
                    new: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetUmpServiceTotalWeight> {
                    let call = SetUmpServiceTotalWeight { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_message_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardMessageSize> {
                    let call = SetMaxUpwardMessageSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_message_num_per_candidate(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetMaxUpwardMessageNumPerCandidate>
                {
                    let call = SetMaxUpwardMessageNumPerCandidate { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_open_request_ttl(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpOpenRequestTtl> {
                    let call = SetHrmpOpenRequestTtl { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_sender_deposit(
                    &self,
                    new: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpSenderDeposit> {
                    let call = SetHrmpSenderDeposit { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_recipient_deposit(
                    &self,
                    new: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpRecipientDeposit> {
                    let call = SetHrmpRecipientDeposit { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_capacity(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpChannelMaxCapacity> {
                    let call = SetHrmpChannelMaxCapacity { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_total_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpChannelMaxTotalSize> {
                    let call = SetHrmpChannelMaxTotalSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parachain_inbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParachainInboundChannels>
                {
                    let call = SetHrmpMaxParachainInboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parathread_inbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParathreadInboundChannels>
                {
                    let call = SetHrmpMaxParathreadInboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_message_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpChannelMaxMessageSize>
                {
                    let call = SetHrmpChannelMaxMessageSize { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parachain_outbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParachainOutboundChannels>
                {
                    let call = SetHrmpMaxParachainOutboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parathread_outbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxParathreadOutboundChannels>
                {
                    let call = SetHrmpMaxParathreadOutboundChannels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_message_num_per_candidate(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SetHrmpMaxMessageNumPerCandidate>
                {
                    let call = SetHrmpMaxMessageNumPerCandidate { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_ump_max_individual_weight(
                    &self,
                    new: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, SetUmpMaxIndividualWeight> {
                    let call = SetUmpMaxIndividualWeight { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ActiveConfig;
            impl ::subxt::StorageEntry for ActiveConfig {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "ActiveConfig";
                type Value =
                    runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingConfig(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for PendingConfig {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "PendingConfig";
                type Value =
                    runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn active_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ActiveConfig;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn pending_config (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < :: core :: primitive :: u32 > > , :: subxt :: Error >{
                    let entry = PendingConfig(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_config_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PendingConfig>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod paras_shared {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct CurrentSessionIndex;
            impl ::subxt::StorageEntry for CurrentSessionIndex {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "CurrentSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveValidatorIndices;
            impl ::subxt::StorageEntry for ActiveValidatorIndices {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "ActiveValidatorIndices";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveValidatorKeys;
            impl ::subxt::StorageEntry for ActiveValidatorKeys {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "ActiveValidatorKeys";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::validator_app::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn current_session_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CurrentSessionIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active_validator_indices(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    ::subxt::Error,
                > {
                    let entry = ActiveValidatorIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active_validator_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::validator_app::Public>,
                    ::subxt::Error,
                > {
                    let entry = ActiveValidatorKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod para_inclusion {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateBacked(
                pub runtime_types::polkadot_primitives::v1::CandidateReceipt<::subxt::sp_core::H256>,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
                pub runtime_types::polkadot_primitives::v1::GroupIndex,
            );
            impl ::subxt::Event for CandidateBacked {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateBacked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateIncluded(
                pub runtime_types::polkadot_primitives::v1::CandidateReceipt<::subxt::sp_core::H256>,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
                pub runtime_types::polkadot_primitives::v1::GroupIndex,
            );
            impl ::subxt::Event for CandidateIncluded {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateIncluded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateTimedOut(
                pub runtime_types::polkadot_primitives::v1::CandidateReceipt<::subxt::sp_core::H256>,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
            );
            impl ::subxt::Event for CandidateTimedOut {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateTimedOut";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AvailabilityBitfields(
                pub runtime_types::polkadot_primitives::v0::ValidatorIndex,
            );
            impl ::subxt::StorageEntry for AvailabilityBitfields {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "AvailabilityBitfields";
                type Value = runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PendingAvailability(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PendingAvailability {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "PendingAvailability";
                type Value = runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PendingAvailabilityCommitments(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for PendingAvailabilityCommitments {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "PendingAvailabilityCommitments";
                type Value = runtime_types::polkadot_primitives::v1::CandidateCommitments<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn availability_bitfields (& self , _0 : runtime_types :: polkadot_primitives :: v0 :: ValidatorIndex , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < :: core :: primitive :: u32 > > , :: subxt :: Error >{
                    let entry = AvailabilityBitfields(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn availability_bitfields_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AvailabilityBitfields>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn pending_availability (& self , _0 : runtime_types :: polkadot_parachain :: primitives :: Id , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , :: subxt :: Error >{
                    let entry = PendingAvailability(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_availability_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PendingAvailability>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn pending_availability_commitments(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::CandidateCommitments<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PendingAvailabilityCommitments(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_availability_commitments_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PendingAvailabilityCommitments>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod para_inherent {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Enter {
                pub data: runtime_types::polkadot_primitives::v1::InherentData<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for Enter {
                const PALLET: &'static str = "ParaInherent";
                const FUNCTION: &'static str = "enter";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn enter(
                    &self,
                    data: runtime_types::polkadot_primitives::v1::InherentData<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<T, Enter> {
                    let call = Enter { data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Included;
            impl ::subxt::StorageEntry for Included {
                const PALLET: &'static str = "ParaInherent";
                const STORAGE: &'static str = "Included";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OnChainVotes;
            impl ::subxt::StorageEntry for OnChainVotes {
                const PALLET: &'static str = "ParaInherent";
                const STORAGE: &'static str = "OnChainVotes";
                type Value = runtime_types::polkadot_primitives::v1::ScrapedOnChainVotes<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn included(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error>
                {
                    let entry = Included;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn on_chain_votes(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::ScrapedOnChainVotes<
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = OnChainVotes;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod para_scheduler {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct ValidatorGroups;
            impl ::subxt::StorageEntry for ValidatorGroups {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ValidatorGroups";
                type Value = ::std::vec::Vec<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParathreadQueue;
            impl ::subxt::StorageEntry for ParathreadQueue {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ParathreadQueue";
                type Value =
                    runtime_types::polkadot_runtime_parachains::scheduler::ParathreadClaimQueue;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AvailabilityCores;
            impl ::subxt::StorageEntry for AvailabilityCores {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "AvailabilityCores";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<runtime_types::polkadot_primitives::v1::CoreOccupied>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParathreadClaimIndex;
            impl ::subxt::StorageEntry for ParathreadClaimIndex {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ParathreadClaimIndex";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SessionStartBlock;
            impl ::subxt::StorageEntry for SessionStartBlock {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "SessionStartBlock";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Scheduled;
            impl ::subxt::StorageEntry for Scheduled {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "Scheduled";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_runtime_parachains::scheduler::CoreAssignment,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validator_groups(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ValidatorGroups;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parathread_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_runtime_parachains::scheduler::ParathreadClaimQueue,
                    ::subxt::Error,
                > {
                    let entry = ParathreadQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn availability_cores(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<
                            runtime_types::polkadot_primitives::v1::CoreOccupied,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = AvailabilityCores;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parathread_claim_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = ParathreadClaimIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn session_start_block(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = SessionStartBlock;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn scheduled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_runtime_parachains::scheduler::CoreAssignment,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Scheduled;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod paras {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceSetCurrentCode {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for ForceSetCurrentCode {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_set_current_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceSetCurrentHead {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
            }
            impl ::subxt::Call for ForceSetCurrentHead {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_set_current_head";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceScheduleCodeUpgrade {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                pub relay_parent_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceScheduleCodeUpgrade {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_schedule_code_upgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceNoteNewHead {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
            }
            impl ::subxt::Call for ForceNoteNewHead {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_note_new_head";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceQueueAction {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ForceQueueAction {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_queue_action";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn force_set_current_code(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSetCurrentCode> {
                    let call = ForceSetCurrentCode { para, new_code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_current_head(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSetCurrentHead> {
                    let call = ForceSetCurrentHead { para, new_head };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_schedule_code_upgrade(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                    relay_parent_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceScheduleCodeUpgrade> {
                    let call = ForceScheduleCodeUpgrade {
                        para,
                        new_code,
                        relay_parent_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_note_new_head(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceNoteNewHead> {
                    let call = ForceNoteNewHead { para, new_head };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_queue_action(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceQueueAction> {
                    let call = ForceQueueAction { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::paras::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CurrentCodeUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for CurrentCodeUpdated {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CurrentCodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CurrentHeadUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for CurrentHeadUpdated {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CurrentHeadUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CodeUpgradeScheduled(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for CodeUpgradeScheduled {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CodeUpgradeScheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewHeadNoted(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for NewHeadNoted {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "NewHeadNoted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ActionQueued(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for ActionQueued {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "ActionQueued";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Parachains;
            impl ::subxt::StorageEntry for Parachains {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "Parachains";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParaLifecycles(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for ParaLifecycles {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "ParaLifecycles";
                type Value = runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Heads(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Heads {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "Heads";
                type Value = runtime_types::polkadot_parachain::primitives::HeadData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CurrentCodeHash(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for CurrentCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CurrentCodeHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodeHash(
                runtime_types::polkadot_parachain::primitives::Id,
                ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for PastCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodeHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodeMeta(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PastCodeMeta {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodeMeta";
                type Value = runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodePruning;
            impl ::subxt::StorageEntry for PastCodePruning {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodePruning";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct FutureCodeUpgrades(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for FutureCodeUpgrades {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "FutureCodeUpgrades";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct FutureCodeHash(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for FutureCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "FutureCodeHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeGoAheadSignal(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for UpgradeGoAheadSignal {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeGoAheadSignal";
                type Value = runtime_types::polkadot_primitives::v1::UpgradeGoAhead;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeRestrictionSignal(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for UpgradeRestrictionSignal {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeRestrictionSignal";
                type Value = runtime_types::polkadot_primitives::v1::UpgradeRestriction;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeCooldowns;
            impl ::subxt::StorageEntry for UpgradeCooldowns {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeCooldowns";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpcomingUpgrades;
            impl ::subxt::StorageEntry for UpcomingUpgrades {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpcomingUpgrades";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActionsQueue(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ActionsQueue {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "ActionsQueue";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpcomingParasGenesis(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for UpcomingParasGenesis {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpcomingParasGenesis";
                type Value = runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CodeByHashRefs(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            );
            impl ::subxt::StorageEntry for CodeByHashRefs {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CodeByHashRefs";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CodeByHash(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            );
            impl ::subxt::StorageEntry for CodeByHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CodeByHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCode;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn parachains(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = Parachains;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn para_lifecycles(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
                    >,
                    ::subxt::Error,
                > {
                    let entry = ParaLifecycles(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn para_lifecycles_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ParaLifecycles>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn heads(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_parachain::primitives::HeadData>,
                    ::subxt::Error,
                > {
                    let entry = Heads(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn heads_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Heads>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentCodeHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_code_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, CurrentCodeHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn past_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PastCodeHash(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn past_code_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PastCodeHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn past_code_meta(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
                        ::core::primitive::u32,
                    >,
                    ::subxt::Error,
                > {
                    let entry = PastCodeMeta(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn past_code_meta_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PastCodeMeta>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn past_code_pruning(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = PastCodePruning;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn future_code_upgrades(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = FutureCodeUpgrades(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn future_code_upgrades_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, FutureCodeUpgrades>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn future_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::Error,
                > {
                    let entry = FutureCodeHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn future_code_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, FutureCodeHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn upgrade_go_ahead_signal(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_primitives::v1::UpgradeGoAhead>,
                    ::subxt::Error,
                > {
                    let entry = UpgradeGoAheadSignal(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgrade_go_ahead_signal_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpgradeGoAheadSignal>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn upgrade_restriction_signal(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::UpgradeRestriction,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UpgradeRestrictionSignal(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgrade_restriction_signal_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpgradeRestrictionSignal>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn upgrade_cooldowns(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = UpgradeCooldowns;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upcoming_upgrades(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = UpcomingUpgrades;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn actions_queue(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = ActionsQueue(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn actions_queue_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ActionsQueue>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn upcoming_paras_genesis(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
                    >,
                    ::subxt::Error,
                > {
                    let entry = UpcomingParasGenesis(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upcoming_paras_genesis_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpcomingParasGenesis>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn code_by_hash_refs(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = CodeByHashRefs(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn code_by_hash_refs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, CodeByHashRefs>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn code_by_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CodeByHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn code_by_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, CodeByHash>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod initializer {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceApprove {
                pub up_to: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceApprove {
                const PALLET: &'static str = "Initializer";
                const FUNCTION: &'static str = "force_approve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn force_approve(
                    &self,
                    up_to: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceApprove> {
                    let call = ForceApprove { up_to };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HasInitialized;
            impl ::subxt::StorageEntry for HasInitialized {
                const PALLET: &'static str = "Initializer";
                const STORAGE: &'static str = "HasInitialized";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BufferedSessionChanges;
            impl ::subxt::StorageEntry for BufferedSessionChanges {
                const PALLET: &'static str = "Initializer";
                const STORAGE: &'static str = "BufferedSessionChanges";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_runtime_parachains::initializer::BufferedSessionChange,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn has_initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error>
                {
                    let entry = HasInitialized;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn buffered_session_changes (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: std :: vec :: Vec < runtime_types :: polkadot_runtime_parachains :: initializer :: BufferedSessionChange > , :: subxt :: Error >{
                    let entry = BufferedSessionChanges;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod dmp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct DownwardMessageQueues(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for DownwardMessageQueues {
                const PALLET: &'static str = "Dmp";
                const STORAGE: &'static str = "DownwardMessageQueues";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DownwardMessageQueueHeads(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for DownwardMessageQueueHeads {
                const PALLET: &'static str = "Dmp";
                const STORAGE: &'static str = "DownwardMessageQueueHeads";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn downward_message_queues(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = DownwardMessageQueues(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn downward_message_queues_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DownwardMessageQueues>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn downward_message_queue_heads(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error>
                {
                    let entry = DownwardMessageQueueHeads(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn downward_message_queue_heads_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DownwardMessageQueueHeads>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod ump {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ServiceOverweight {
                pub index: ::core::primitive::u64,
                pub weight_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for ServiceOverweight {
                const PALLET: &'static str = "Ump";
                const FUNCTION: &'static str = "service_overweight";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, ServiceOverweight> {
                    let call = ServiceOverweight {
                        index,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::ump::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for InvalidFormat {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for UnsupportedVersion {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ExecutedUpward(
                pub [::core::primitive::u8; 32usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::Event for ExecutedUpward {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "ExecutedUpward";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct WeightExhausted(
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for WeightExhausted {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "WeightExhausted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UpwardMessagesReceived(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for UpwardMessagesReceived {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "UpwardMessagesReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OverweightEnqueued(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for OverweightEnqueued {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OverweightServiced(pub ::core::primitive::u64, pub ::core::primitive::u64);
            impl ::subxt::Event for OverweightServiced {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RelayDispatchQueues(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for RelayDispatchQueues {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "RelayDispatchQueues";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct RelayDispatchQueueSize(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for RelayDispatchQueueSize {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "RelayDispatchQueueSize";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NeedsDispatch;
            impl ::subxt::StorageEntry for NeedsDispatch {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "NeedsDispatch";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextDispatchRoundStartWith;
            impl ::subxt::StorageEntry for NextDispatchRoundStartWith {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "NextDispatchRoundStartWith";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Overweight(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Overweight {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "Overweight";
                type Value = (
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::std::vec::Vec<::core::primitive::u8>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct OverweightCount;
            impl ::subxt::StorageEntry for OverweightCount {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "OverweightCount";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn relay_dispatch_queues(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::Error,
                > {
                    let entry = RelayDispatchQueues(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relay_dispatch_queues_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, RelayDispatchQueues>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn relay_dispatch_queue_size(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::Error,
                > {
                    let entry = RelayDispatchQueueSize(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relay_dispatch_queue_size_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, RelayDispatchQueueSize>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn needs_dispatch(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = NeedsDispatch;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_dispatch_round_start_with(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = NextDispatchRoundStartWith;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = Overweight(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Overweight>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn overweight_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = OverweightCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod hrmp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HrmpInitOpenChannel {
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub proposed_max_capacity: ::core::primitive::u32,
                pub proposed_max_message_size: ::core::primitive::u32,
            }
            impl ::subxt::Call for HrmpInitOpenChannel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_init_open_channel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HrmpAcceptOpenChannel {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for HrmpAcceptOpenChannel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_accept_open_channel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HrmpCloseChannel {
                pub channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            }
            impl ::subxt::Call for HrmpCloseChannel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_close_channel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceCleanHrmp {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ForceCleanHrmp {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_clean_hrmp";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceProcessHrmpOpen {}
            impl ::subxt::Call for ForceProcessHrmpOpen {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_process_hrmp_open";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceProcessHrmpClose {}
            impl ::subxt::Call for ForceProcessHrmpClose {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_process_hrmp_close";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HrmpCancelOpenRequest {
                pub channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            }
            impl ::subxt::Call for HrmpCancelOpenRequest {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_cancel_open_request";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn hrmp_init_open_channel(
                    &self,
                    recipient: runtime_types::polkadot_parachain::primitives::Id,
                    proposed_max_capacity: ::core::primitive::u32,
                    proposed_max_message_size: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpInitOpenChannel> {
                    let call = HrmpInitOpenChannel {
                        recipient,
                        proposed_max_capacity,
                        proposed_max_message_size,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_accept_open_channel(
                    &self,
                    sender: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpAcceptOpenChannel> {
                    let call = HrmpAcceptOpenChannel { sender };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_close_channel(
                    &self,
                    channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpCloseChannel> {
                    let call = HrmpCloseChannel { channel_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clean_hrmp(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceCleanHrmp> {
                    let call = ForceCleanHrmp { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_process_hrmp_open(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceProcessHrmpOpen> {
                    let call = ForceProcessHrmpOpen {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_process_hrmp_close(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceProcessHrmpClose> {
                    let call = ForceProcessHrmpClose {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_cancel_open_request(
                    &self,
                    channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                ) -> ::subxt::SubmittableExtrinsic<T, HrmpCancelOpenRequest> {
                    let call = HrmpCancelOpenRequest { channel_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenChannelRequested(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for OpenChannelRequested {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelRequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenChannelCanceled(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::Event for OpenChannelCanceled {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelCanceled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpenChannelAccepted(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for OpenChannelAccepted {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChannelClosed(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::Event for ChannelClosed {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "ChannelClosed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HrmpOpenChannelRequests(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpOpenChannelRequests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequests";
                type Value =
                    runtime_types::polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpOpenChannelRequestsList;
            impl ::subxt::StorageEntry for HrmpOpenChannelRequestsList {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequestsList";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpOpenChannelRequestCount(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpOpenChannelRequestCount {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequestCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpAcceptedChannelRequestCount(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpAcceptedChannelRequestCount {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpAcceptedChannelRequestCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpCloseChannelRequests(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpCloseChannelRequests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpCloseChannelRequests";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpCloseChannelRequestsList;
            impl ::subxt::StorageEntry for HrmpCloseChannelRequestsList {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpCloseChannelRequestsList";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpWatermarks(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for HrmpWatermarks {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpWatermarks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannels(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpChannels {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannels";
                type Value = runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpIngressChannelsIndex(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpIngressChannelsIndex {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpIngressChannelsIndex";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpEgressChannelsIndex(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpEgressChannelsIndex {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpEgressChannelsIndex";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannelContents(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpChannelContents {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannelContents";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannelDigests(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for HrmpChannelDigests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannelDigests";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn hrmp_open_channel_requests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HrmpOpenChannelRequests(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_open_channel_requests_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpOpenChannelRequests>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_open_channel_requests_list(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
                    ::subxt::Error,
                > {
                    let entry = HrmpOpenChannelRequestsList;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_open_channel_request_count(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HrmpOpenChannelRequestCount(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_open_channel_request_count_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpOpenChannelRequestCount>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_accepted_channel_request_count(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = HrmpAcceptedChannelRequestCount(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_accepted_channel_request_count_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpAcceptedChannelRequestCount>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_close_channel_requests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error>
                {
                    let entry = HrmpCloseChannelRequests(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_close_channel_requests_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpCloseChannelRequests>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_close_channel_requests_list(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
                    ::subxt::Error,
                > {
                    let entry = HrmpCloseChannelRequestsList;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_watermarks(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = HrmpWatermarks(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_watermarks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, HrmpWatermarks>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_channels(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HrmpChannels(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_channels_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, HrmpChannels>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_ingress_channels_index(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = HrmpIngressChannelsIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_ingress_channels_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpIngressChannelsIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_egress_channels_index(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = HrmpEgressChannelsIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_egress_channels_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpEgressChannelsIndex>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_channel_contents(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = HrmpChannelContents(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_channel_contents_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpChannelContents>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_channel_digests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = HrmpChannelDigests(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_channel_digests_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpChannelDigests>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod para_session_info {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct AssignmentKeysUnsafe;
            impl ::subxt::StorageEntry for AssignmentKeysUnsafe {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "AssignmentKeysUnsafe";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v1::assignment_app::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EarliestStoredSession;
            impl ::subxt::StorageEntry for EarliestStoredSession {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "EarliestStoredSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Sessions(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Sessions {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "Sessions";
                type Value = runtime_types::polkadot_primitives::v1::SessionInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn assignment_keys_unsafe(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v1::assignment_app::Public>,
                    ::subxt::Error,
                > {
                    let entry = AssignmentKeysUnsafe;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn earliest_stored_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = EarliestStoredSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn sessions(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_primitives::v1::SessionInfo>,
                    ::subxt::Error,
                > {
                    let entry = Sessions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn sessions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Sessions>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod registrar {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Register {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for Register {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "register";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceRegister {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for ForceRegister {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "force_register";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deregister {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Deregister {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "deregister";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Swap {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub other: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Swap {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "swap";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceRemoveLock {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ForceRemoveLock {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "force_remove_lock";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reserve {}
            impl ::subxt::Call for Reserve {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "reserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn register(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<T, Register> {
                    let call = Register {
                        id,
                        genesis_head,
                        validation_code,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_register(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceRegister> {
                    let call = ForceRegister {
                        who,
                        deposit,
                        id,
                        genesis_head,
                        validation_code,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deregister(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Deregister> {
                    let call = Deregister { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn swap(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    other: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Swap> {
                    let call = Swap { id, other };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_remove_lock(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceRemoveLock> {
                    let call = ForceRemoveLock { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reserve(&self) -> ::subxt::SubmittableExtrinsic<T, Reserve> {
                    let call = Reserve {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Registered(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Registered {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Registered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Deregistered(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Deregistered {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Deregistered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reserved(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Reserved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PendingSwap(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PendingSwap {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "PendingSwap";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Paras(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Paras {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "Paras";
                type Value = runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NextFreeParaId;
            impl ::subxt::StorageEntry for NextFreeParaId {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "NextFreeParaId";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn pending_swap(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = PendingSwap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_swap_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PendingSwap>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn paras(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Paras(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn paras_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Paras>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_free_para_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::subxt::Error,
                > {
                    let entry = NextFreeParaId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod slots {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceLease {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub leaser: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub period_begin: ::core::primitive::u32,
                pub period_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceLease {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "force_lease";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ClearAllLeases {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for ClearAllLeases {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "clear_all_leases";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TriggerOnboard {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for TriggerOnboard {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "trigger_onboard";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn force_lease(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    leaser: ::subxt::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                    period_begin: ::core::primitive::u32,
                    period_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceLease> {
                    let call = ForceLease {
                        para,
                        leaser,
                        amount,
                        period_begin,
                        period_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_all_leases(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, ClearAllLeases> {
                    let call = ClearAllLeases { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn trigger_onboard(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, TriggerOnboard> {
                    let call = TriggerOnboard { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::slots::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewLeasePeriod(pub ::core::primitive::u32);
            impl ::subxt::Event for NewLeasePeriod {
                const PALLET: &'static str = "Slots";
                const EVENT: &'static str = "NewLeasePeriod";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Leased(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Leased {
                const PALLET: &'static str = "Slots";
                const EVENT: &'static str = "Leased";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Leases(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Leases {
                const PALLET: &'static str = "Slots";
                const STORAGE: &'static str = "Leases";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn leases(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Leases(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn leases_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Leases>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod paras_sudo_wrapper {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoScheduleParaInitialize {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis: runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
            }
            impl ::subxt::Call for SudoScheduleParaInitialize {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_para_initialize";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoScheduleParaCleanup {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for SudoScheduleParaCleanup {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_para_cleanup";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoScheduleParathreadUpgrade {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for SudoScheduleParathreadUpgrade {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_parathread_upgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoScheduleParachainDowngrade {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for SudoScheduleParachainDowngrade {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_parachain_downgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoQueueDownwardXcm {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub xcm: runtime_types::xcm::VersionedXcm,
            }
            impl ::subxt::Call for SudoQueueDownwardXcm {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_queue_downward_xcm";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SudoEstablishHrmpChannel {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub max_capacity: ::core::primitive::u32,
                pub max_message_size: ::core::primitive::u32,
            }
            impl ::subxt::Call for SudoEstablishHrmpChannel {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_establish_hrmp_channel";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn sudo_schedule_para_initialize(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis: runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoScheduleParaInitialize> {
                    let call = SudoScheduleParaInitialize { id, genesis };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_schedule_para_cleanup(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoScheduleParaCleanup> {
                    let call = SudoScheduleParaCleanup { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_schedule_parathread_upgrade(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoScheduleParathreadUpgrade>
                {
                    let call = SudoScheduleParathreadUpgrade { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_schedule_parachain_downgrade(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoScheduleParachainDowngrade>
                {
                    let call = SudoScheduleParachainDowngrade { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_queue_downward_xcm(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    xcm: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoQueueDownwardXcm> {
                    let call = SudoQueueDownwardXcm { id, xcm };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_establish_hrmp_channel(
                    &self,
                    sender: runtime_types::polkadot_parachain::primitives::Id,
                    recipient: runtime_types::polkadot_parachain::primitives::Id,
                    max_capacity: ::core::primitive::u32,
                    max_message_size: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, SudoEstablishHrmpChannel> {
                    let call = SudoEstablishHrmpChannel {
                        sender,
                        recipient,
                        max_capacity,
                        max_message_size,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
    }
    pub mod auctions {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NewAuction {
                #[codec(compact)]
                pub duration: ::core::primitive::u32,
                #[codec(compact)]
                pub lease_period_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for NewAuction {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "new_auction";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Bid {
                #[codec(compact)]
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub auction_index: ::core::primitive::u32,
                #[codec(compact)]
                pub first_slot: ::core::primitive::u32,
                #[codec(compact)]
                pub last_slot: ::core::primitive::u32,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for Bid {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "bid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CancelAuction {}
            impl ::subxt::Call for CancelAuction {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "cancel_auction";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn new_auction(
                    &self,
                    duration: ::core::primitive::u32,
                    lease_period_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, NewAuction> {
                    let call = NewAuction {
                        duration,
                        lease_period_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bid(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    auction_index: ::core::primitive::u32,
                    first_slot: ::core::primitive::u32,
                    last_slot: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<T, Bid> {
                    let call = Bid {
                        para,
                        auction_index,
                        first_slot,
                        last_slot,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_auction(&self) -> ::subxt::SubmittableExtrinsic<T, CancelAuction> {
                    let call = CancelAuction {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::auctions::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AuctionStarted(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for AuctionStarted {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "AuctionStarted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AuctionClosed(pub ::core::primitive::u32);
            impl ::subxt::Event for AuctionClosed {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "AuctionClosed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveConfiscated(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for ReserveConfiscated {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "ReserveConfiscated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BidAccepted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u128,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for BidAccepted {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "BidAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct WinningOffset(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for WinningOffset {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "WinningOffset";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AuctionCounter;
            impl ::subxt::StorageEntry for AuctionCounter {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "AuctionCounter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuctionInfo;
            impl ::subxt::StorageEntry for AuctionInfo {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "AuctionInfo";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedAmounts(
                ::subxt::sp_core::crypto::AccountId32,
                runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for ReservedAmounts {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "ReservedAmounts";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Winning(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Winning {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "Winning";
                type Value = [::core::option::Option<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u128,
                )>; 36usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn auction_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AuctionCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn auction_info(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::Error,
                > {
                    let entry = AuctionInfo;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_amounts(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::Error,
                > {
                    let entry = ReservedAmounts(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_amounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ReservedAmounts>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn winning(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        [::core::option::Option<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                        )>; 36usize],
                    >,
                    ::subxt::Error,
                > {
                    let entry = Winning(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn winning_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Winning>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod crowdloan {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Create {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub cap: ::core::primitive::u128,
                #[codec(compact)]
                pub first_period: ::core::primitive::u32,
                #[codec(compact)]
                pub last_period: ::core::primitive::u32,
                #[codec(compact)]
                pub end: ::core::primitive::u32,
                pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
            }
            impl ::subxt::Call for Create {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Contribute {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
            }
            impl ::subxt::Call for Contribute {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "contribute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Withdraw {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Refund {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Refund {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "refund";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Dissolve {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Dissolve {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "dissolve";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Edit {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub cap: ::core::primitive::u128,
                #[codec(compact)]
                pub first_period: ::core::primitive::u32,
                #[codec(compact)]
                pub last_period: ::core::primitive::u32,
                #[codec(compact)]
                pub end: ::core::primitive::u32,
                pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
            }
            impl ::subxt::Call for Edit {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "edit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddMemo {
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                pub memo: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for AddMemo {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "add_memo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Poke {
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for Poke {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "poke";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn create(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    cap: ::core::primitive::u128,
                    first_period: ::core::primitive::u32,
                    last_period: ::core::primitive::u32,
                    end: ::core::primitive::u32,
                    verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                ) -> ::subxt::SubmittableExtrinsic<T, Create> {
                    let call = Create {
                        index,
                        cap,
                        first_period,
                        last_period,
                        end,
                        verifier,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn contribute(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    value: ::core::primitive::u128,
                    signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                ) -> ::subxt::SubmittableExtrinsic<T, Contribute> {
                    let call = Contribute {
                        index,
                        value,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Withdraw> {
                    let call = Withdraw { who, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn refund(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Refund> {
                    let call = Refund { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dissolve(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Dissolve> {
                    let call = Dissolve { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn edit(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    cap: ::core::primitive::u128,
                    first_period: ::core::primitive::u32,
                    last_period: ::core::primitive::u32,
                    end: ::core::primitive::u32,
                    verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                ) -> ::subxt::SubmittableExtrinsic<T, Edit> {
                    let call = Edit {
                        index,
                        cap,
                        first_period,
                        last_period,
                        end,
                        verifier,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_memo(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    memo: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<T, AddMemo> {
                    let call = AddMemo { index, memo };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn poke(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<T, Poke> {
                    let call = Poke { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::crowdloan::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Created(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Contributed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Contributed {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Contributed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Withdrew(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrew {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Withdrew";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PartiallyRefunded(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for PartiallyRefunded {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "PartiallyRefunded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AllRefunded(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for AllRefunded {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "AllRefunded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Dissolved(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Dissolved {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Dissolved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct HandleBidResult(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for HandleBidResult {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "HandleBidResult";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Edited(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Edited {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Edited";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MemoUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for MemoUpdated {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "MemoUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AddedToNewRaise(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for AddedToNewRaise {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "AddedToNewRaise";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Funds(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Funds {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "Funds";
                type Value = runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NewRaise;
            impl ::subxt::StorageEntry for NewRaise {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "NewRaise";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EndingsCount;
            impl ::subxt::StorageEntry for EndingsCount {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "EndingsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextTrieIndex;
            impl ::subxt::StorageEntry for NextTrieIndex {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "NextTrieIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn funds(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Funds(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn funds_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Funds>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn new_raise(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::Error,
                > {
                    let entry = NewRaise;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn endings_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = EndingsCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_trie_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = NextTrieIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod xcm_pallet {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Send {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub message: runtime_types::xcm::VersionedXcm,
            }
            impl ::subxt::Call for Send {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "send";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct TeleportAssets {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
                pub assets: runtime_types::xcm::VersionedMultiAssets,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for TeleportAssets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "teleport_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveTransferAssets {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
                pub assets: runtime_types::xcm::VersionedMultiAssets,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for ReserveTransferAssets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "reserve_transfer_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Execute {
                pub message: runtime_types::xcm::VersionedXcm,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for Execute {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceXcmVersion {
                pub location: runtime_types::xcm::v1::multilocation::MultiLocation,
                pub xcm_version: ::core::primitive::u32,
            }
            impl ::subxt::Call for ForceXcmVersion {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceDefaultXcmVersion {
                pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for ForceDefaultXcmVersion {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_default_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceSubscribeVersionNotify {
                pub location: runtime_types::xcm::VersionedMultiLocation,
            }
            impl ::subxt::Call for ForceSubscribeVersionNotify {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_subscribe_version_notify";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ForceUnsubscribeVersionNotify {
                pub location: runtime_types::xcm::VersionedMultiLocation,
            }
            impl ::subxt::Call for ForceUnsubscribeVersionNotify {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_unsubscribe_version_notify";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LimitedReserveTransferAssets {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
                pub assets: runtime_types::xcm::VersionedMultiAssets,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            impl ::subxt::Call for LimitedReserveTransferAssets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "limited_reserve_transfer_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LimitedTeleportAssets {
                pub dest: runtime_types::xcm::VersionedMultiLocation,
                pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
                pub assets: runtime_types::xcm::VersionedMultiAssets,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            impl ::subxt::Call for LimitedTeleportAssets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "limited_teleport_assets";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
            where
                T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub fn send(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    message: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::SubmittableExtrinsic<T, Send> {
                    let call = Send { dest, message };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, TeleportAssets> {
                    let call = TeleportAssets {
                        dest,
                        beneficiary,
                        assets,
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ReserveTransferAssets> {
                    let call = ReserveTransferAssets {
                        dest,
                        beneficiary,
                        assets,
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    message: runtime_types::xcm::VersionedXcm,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<T, Execute> {
                    let call = Execute {
                        message,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_xcm_version(
                    &self,
                    location: runtime_types::xcm::v1::multilocation::MultiLocation,
                    xcm_version: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceXcmVersion> {
                    let call = ForceXcmVersion {
                        location,
                        xcm_version,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_default_xcm_version(
                    &self,
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceDefaultXcmVersion> {
                    let call = ForceDefaultXcmVersion { maybe_xcm_version };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_subscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceSubscribeVersionNotify> {
                    let call = ForceSubscribeVersionNotify { location };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unsubscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<T, ForceUnsubscribeVersionNotify>
                {
                    let call = ForceUnsubscribeVersionNotify { location };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn limited_reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                ) -> ::subxt::SubmittableExtrinsic<T, LimitedReserveTransferAssets>
                {
                    let call = LimitedReserveTransferAssets {
                        dest,
                        beneficiary,
                        assets,
                        fee_asset_item,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn limited_teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                ) -> ::subxt::SubmittableExtrinsic<T, LimitedTeleportAssets> {
                    let call = LimitedTeleportAssets {
                        dest,
                        beneficiary,
                        assets,
                        fee_asset_item,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
            impl ::subxt::Event for Attempted {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "Attempted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Sent(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v2::Xcm,
            );
            impl ::subxt::Event for Sent {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "Sent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnexpectedResponse(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for UnexpectedResponse {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "UnexpectedResponse";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ResponseReady(
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::Response,
            );
            impl ::subxt::Event for ResponseReady {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "ResponseReady";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Notified(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for Notified {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "Notified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyOverweight(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyOverweight {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyOverweight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyDispatchError(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDispatchError {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyDispatchError";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyDecodeFailed(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDecodeFailed {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyDecodeFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InvalidResponder(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
            );
            impl ::subxt::Event for InvalidResponder {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "InvalidResponder";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InvalidResponderVersion(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for InvalidResponderVersion {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "InvalidResponderVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ResponseTaken(pub ::core::primitive::u64);
            impl ::subxt::Event for ResponseTaken {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "ResponseTaken";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AssetsTrapped(
                pub ::subxt::sp_core::H256,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::VersionedMultiAssets,
            );
            impl ::subxt::Event for AssetsTrapped {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "AssetsTrapped";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct VersionChangeNotified(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for VersionChangeNotified {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "VersionChangeNotified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SupportedVersionChanged(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for SupportedVersionChanged {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "SupportedVersionChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyTargetSendFail(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::traits::Error,
            );
            impl ::subxt::Event for NotifyTargetSendFail {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyTargetSendFail";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NotifyTargetMigrationFail(
                pub runtime_types::xcm::VersionedMultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyTargetMigrationFail {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyTargetMigrationFail";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct QueryCounter;
            impl ::subxt::StorageEntry for QueryCounter {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "QueryCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Queries(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Queries {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "Queries";
                type Value = runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct AssetTraps(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for AssetTraps {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "AssetTraps";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct SafeXcmVersion;
            impl ::subxt::StorageEntry for SafeXcmVersion {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "SafeXcmVersion";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SupportedVersion(
                ::core::primitive::u32,
                runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for SupportedVersion {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "SupportedVersion";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionNotifiers(
                ::core::primitive::u32,
                runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for VersionNotifiers {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "VersionNotifiers";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionNotifyTargets(
                ::core::primitive::u32,
                runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for VersionNotifyTargets {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "VersionNotifyTargets";
                type Value = (
                    ::core::primitive::u64,
                    ::core::primitive::u64,
                    ::core::primitive::u32,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionDiscoveryQueue;
            impl ::subxt::StorageEntry for VersionDiscoveryQueue {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "VersionDiscoveryQueue";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                    runtime_types::xcm::VersionedMultiLocation,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentMigration;
            impl ::subxt::StorageEntry for CurrentMigration {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "CurrentMigration";
                type Value = runtime_types::pallet_xcm::pallet::VersionMigrationStage;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn query_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error>
                {
                    let entry = QueryCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queries(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
                    >,
                    ::subxt::Error,
                > {
                    let entry = Queries(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn queries_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Queries>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset_traps(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error>
                {
                    let entry = AssetTraps(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_traps_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AssetTraps>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn safe_xcm_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = SafeXcmVersion;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn supported_version(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::Error,
                > {
                    let entry = SupportedVersion(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn supported_version_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SupportedVersion>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_notifiers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::Error,
                > {
                    let entry = VersionNotifiers(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn version_notifiers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, VersionNotifiers>, ::subxt::Error>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_notify_targets(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = VersionNotifyTargets(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn version_notify_targets_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VersionNotifyTargets>,
                    ::subxt::Error,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_discovery_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::Error,
                > {
                    let entry = VersionDiscoveryQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_migration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_xcm::pallet::VersionMigrationStage,
                    >,
                    ::subxt::Error,
                > {
                    let entry = CurrentMigration;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bitvec {
            use super::runtime_types;
            pub mod order {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Lsb0 {}
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_btree_map {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct BoundedBTreeMap<_0, _1>(pub ::std::collections::BTreeMap<_0, _1>);
                }
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct WrapperKeepOpaque<_0>(::core::primitive::u32, pub _0);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct WrapperOpaque<_0>(::core::primitive::u32, pub _0);
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckGenesis {}
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckNonce(pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckSpecVersion {}
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckTxVersion {}
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct CheckWeight {}
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(_0),
                #[codec(index = 2)]
                None,
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    TooManyUncles,
                    #[codec(index = 3)]
                    GenesisUncle,
                    #[codec(index = 4)]
                    TooHighUncle,
                    #[codec(index = 5)]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    OldUncle,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_bags_list {
            use super::runtime_types;
            pub mod list {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Bag {
                    pub head: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    pub tail: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Node {
                    pub id: ::subxt::sp_core::crypto::AccountId32,
                    pub prev: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    pub next: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    pub bag_upper: ::core::primitive::u64,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    rebag {
                        dislocated: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Rebagged {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        from: ::core::primitive::u64,
                        to: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    # [codec (index = 0)] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: westend_runtime :: NposCompactSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < [:: core :: primitive :: u128 ; 3usize] > , } , # [codec (index = 2)] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: westend_runtime :: NposCompactSolution16 > > , num_signed_submissions : :: core :: primitive :: u32 , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    PreDispatchEarlySubmission,
                    #[codec(index = 1)]
                    PreDispatchWrongWinnerCount,
                    #[codec(index = 2)]
                    PreDispatchWeakSubmission,
                    #[codec(index = 3)]
                    SignedQueueFull,
                    #[codec(index = 4)]
                    SignedCannotPayDeposit,
                    #[codec(index = 5)]
                    SignedInvalidWitness,
                    #[codec(index = 6)]
                    SignedTooMuchWeight,
                    #[codec(index = 7)]
                    OcwCallWrongEra,
                    #[codec(index = 8)]
                    MissingSnapshotMetadata,
                    #[codec(index = 9)]
                    InvalidSubmissionIndex,
                    #[codec(index = 10)]
                    CallNotAllowed,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    SolutionStored {
                        election_compute:
                            runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                        prev_ejected: ::core::primitive::bool,
                    },
                    #[codec(index = 1)]
                    ElectionFinalized {
                        election_compute: ::core::option::Option<
                            runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                        >,
                    },
                    #[codec(index = 2)]
                    Rewarded {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    Slashed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    SignedPhaseStarted { round: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    UnsignedPhaseStarted { round: ::core::primitive::u32 },
                }
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SignedSubmission<_0, _1, _2> {
                    pub who: _0,
                    pub deposit: _1,
                    pub raw_solution:
                        runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
                    pub reward: _1,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ElectionCompute {
                #[codec(index = 0)]
                OnChain,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned,
                #[codec(index = 3)]
                Fallback,
                #[codec(index = 4)]
                Emergency,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Phase<_0> {
                #[codec(index = 0)]
                Off,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned((::core::primitive::bool, _0)),
                #[codec(index = 3)]
                Emergency,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: [::core::primitive::u128; 3usize],
                pub round: ::core::primitive::u32,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ReadySolution<_0> {
                pub supports: ::std::vec::Vec<(_0, runtime_types::sp_npos_elections::Support<_0>)>,
                pub score: [::core::primitive::u128; 3usize],
                pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RoundSnapshot<_0> {
                pub voters: ::std::vec::Vec<(_0, ::core::primitive::u64, ::std::vec::Vec<_0>)>,
                pub targets: ::std::vec::Vec<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SolutionOrSnapshotSize {
                #[codec(compact)]
                pub voters: ::core::primitive::u32,
                #[codec(compact)]
                pub targets: ::core::primitive::u32,
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    add_registrar {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    set_identity {
                        info:
                            ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
                    },
                    #[codec(index = 2)]
                    set_subs {
                        subs: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    #[codec(index = 3)]
                    clear_identity,
                    #[codec(index = 4)]
                    request_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        #[codec(compact)]
                        max_fee: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    cancel_request { reg_index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    set_fee {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    set_account_id {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    set_fields {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        fields: runtime_types::pallet_identity::types::BitFlags<
                            runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    #[codec(index = 9)]
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        judgement: runtime_types::pallet_identity::types::Judgement<
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 10)]
                    kill_identity {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 11)]
                    add_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 12)]
                    rename_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 13)]
                    remove_sub {
                        sub: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    quit_sub,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManySubAccounts,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    NotNamed,
                    #[codec(index = 3)]
                    EmptyIndex,
                    #[codec(index = 4)]
                    FeeChanged,
                    #[codec(index = 5)]
                    NoIdentity,
                    #[codec(index = 6)]
                    StickyJudgement,
                    #[codec(index = 7)]
                    JudgementGiven,
                    #[codec(index = 8)]
                    InvalidJudgement,
                    #[codec(index = 9)]
                    InvalidIndex,
                    #[codec(index = 10)]
                    InvalidTarget,
                    #[codec(index = 11)]
                    TooManyFields,
                    #[codec(index = 12)]
                    TooManyRegistrars,
                    #[codec(index = 13)]
                    AlreadyClaimed,
                    #[codec(index = 14)]
                    NotSub,
                    #[codec(index = 15)]
                    NotOwned,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    IdentitySet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    IdentityCleared {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    IdentityKilled {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    JudgementRequested {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    JudgementUnrequested {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    JudgementGiven {
                        target: ::subxt::sp_core::crypto::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    RegistrarAdded {
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    SubIdentityAdded {
                        sub: ::subxt::sp_core::crypto::AccountId32,
                        main: ::subxt::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    SubIdentityRemoved {
                        sub: ::subxt::sp_core::crypto::AccountId32,
                        main: ::subxt::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    SubIdentityRevoked {
                        sub: ::subxt::sp_core::crypto::AccountId32,
                        main: ::subxt::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Data {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Raw0([::core::primitive::u8; 0usize]),
                    #[codec(index = 2)]
                    Raw1([::core::primitive::u8; 1usize]),
                    #[codec(index = 3)]
                    Raw2([::core::primitive::u8; 2usize]),
                    #[codec(index = 4)]
                    Raw3([::core::primitive::u8; 3usize]),
                    #[codec(index = 5)]
                    Raw4([::core::primitive::u8; 4usize]),
                    #[codec(index = 6)]
                    Raw5([::core::primitive::u8; 5usize]),
                    #[codec(index = 7)]
                    Raw6([::core::primitive::u8; 6usize]),
                    #[codec(index = 8)]
                    Raw7([::core::primitive::u8; 7usize]),
                    #[codec(index = 9)]
                    Raw8([::core::primitive::u8; 8usize]),
                    #[codec(index = 10)]
                    Raw9([::core::primitive::u8; 9usize]),
                    #[codec(index = 11)]
                    Raw10([::core::primitive::u8; 10usize]),
                    #[codec(index = 12)]
                    Raw11([::core::primitive::u8; 11usize]),
                    #[codec(index = 13)]
                    Raw12([::core::primitive::u8; 12usize]),
                    #[codec(index = 14)]
                    Raw13([::core::primitive::u8; 13usize]),
                    #[codec(index = 15)]
                    Raw14([::core::primitive::u8; 14usize]),
                    #[codec(index = 16)]
                    Raw15([::core::primitive::u8; 15usize]),
                    #[codec(index = 17)]
                    Raw16([::core::primitive::u8; 16usize]),
                    #[codec(index = 18)]
                    Raw17([::core::primitive::u8; 17usize]),
                    #[codec(index = 19)]
                    Raw18([::core::primitive::u8; 18usize]),
                    #[codec(index = 20)]
                    Raw19([::core::primitive::u8; 19usize]),
                    #[codec(index = 21)]
                    Raw20([::core::primitive::u8; 20usize]),
                    #[codec(index = 22)]
                    Raw21([::core::primitive::u8; 21usize]),
                    #[codec(index = 23)]
                    Raw22([::core::primitive::u8; 22usize]),
                    #[codec(index = 24)]
                    Raw23([::core::primitive::u8; 23usize]),
                    #[codec(index = 25)]
                    Raw24([::core::primitive::u8; 24usize]),
                    #[codec(index = 26)]
                    Raw25([::core::primitive::u8; 25usize]),
                    #[codec(index = 27)]
                    Raw26([::core::primitive::u8; 26usize]),
                    #[codec(index = 28)]
                    Raw27([::core::primitive::u8; 27usize]),
                    #[codec(index = 29)]
                    Raw28([::core::primitive::u8; 28usize]),
                    #[codec(index = 30)]
                    Raw29([::core::primitive::u8; 29usize]),
                    #[codec(index = 31)]
                    Raw30([::core::primitive::u8; 30usize]),
                    #[codec(index = 32)]
                    Raw31([::core::primitive::u8; 31usize]),
                    #[codec(index = 33)]
                    Raw32([::core::primitive::u8; 32usize]),
                    #[codec(index = 34)]
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    #[codec(index = 35)]
                    Sha256([::core::primitive::u8; 32usize]),
                    #[codec(index = 36)]
                    Keccak256([::core::primitive::u8; 32usize]),
                    #[codec(index = 37)]
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum IdentityField {
                    #[codec(index = 1)]
                    Display,
                    #[codec(index = 2)]
                    Legal,
                    #[codec(index = 4)]
                    Web,
                    #[codec(index = 8)]
                    Riot,
                    #[codec(index = 16)]
                    Email,
                    #[codec(index = 32)]
                    PgpFingerprint,
                    #[codec(index = 64)]
                    Image,
                    #[codec(index = 128)]
                    Twitter,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct IdentityInfo {
                    pub additional:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            runtime_types::pallet_identity::types::Data,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    pub display: runtime_types::pallet_identity::types::Data,
                    pub legal: runtime_types::pallet_identity::types::Data,
                    pub web: runtime_types::pallet_identity::types::Data,
                    pub riot: runtime_types::pallet_identity::types::Data,
                    pub email: runtime_types::pallet_identity::types::Data,
                    pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
                    pub image: runtime_types::pallet_identity::types::Data,
                    pub twitter: runtime_types::pallet_identity::types::Data,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Judgement<_0> {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    FeePaid(_0),
                    #[codec(index = 2)]
                    Reasonable,
                    #[codec(index = 3)]
                    KnownGood,
                    #[codec(index = 4)]
                    OutOfDate,
                    #[codec(index = 5)]
                    LowQuality,
                    #[codec(index = 6)]
                    Erroneous,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Registration<_0> {
                    pub judgements:
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            ::core::primitive::u32,
                            runtime_types::pallet_identity::types::Judgement<_0>,
                        )>,
                    pub deposit: _0,
                    pub info: runtime_types::pallet_identity::types::IdentityInfo,
                }
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    HeartbeatReceived {
                        authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    },
                    #[codec(index = 1)]
                    AllGood,
                    #[codec(index = 2)]
                    SomeOffline {
                        offline: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pallet_staking::Exposure<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::core::primitive::u128,
                            >,
                        )>,
                    },
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BoundedOpaqueNetworkState {
                pub peer_id:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                pub external_addresses:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_indices {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    claim { index: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    free { index: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    force_transfer {
                        new: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                        freeze: ::core::primitive::bool,
                    },
                    #[codec(index = 4)]
                    freeze { index: ::core::primitive::u32 },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NotAssigned,
                    #[codec(index = 1)]
                    NotOwner,
                    #[codec(index = 2)]
                    InUse,
                    #[codec(index = 3)]
                    NotTransfer,
                    #[codec(index = 4)]
                    Permanent,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    IndexAssigned {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    IndexFreed { index: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    IndexFrozen {
                        index: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::subxt::WrapperKeepOpaque<runtime_types::westend_runtime::Call>,
                        store_call: ::core::primitive::bool,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    TooManySignatories,
                    #[codec(index = 5)]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    NotFound,
                    #[codec(index = 8)]
                    NotOwner,
                    #[codec(index = 9)]
                    NoTimepoint,
                    #[codec(index = 10)]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    AlreadyStored,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewMultisig {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    MultisigApproval {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    MultisigExecuted {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    MultisigCancelled {
                        cancelling: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_offences {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Offence {
                        kind: [::core::primitive::u8; 16usize],
                        timeslot: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    proxy {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type:
                            ::core::option::Option<runtime_types::westend_runtime::ProxyType>,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    add_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::westend_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    remove_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::westend_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    remove_proxies,
                    #[codec(index = 4)]
                    anonymous {
                        proxy_type: runtime_types::westend_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                        index: ::core::primitive::u16,
                    },
                    #[codec(index = 5)]
                    kill_anonymous {
                        spawner: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::westend_runtime::ProxyType,
                        index: ::core::primitive::u16,
                        #[codec(compact)]
                        height: ::core::primitive::u32,
                        #[codec(compact)]
                        ext_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    announce {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 7)]
                    remove_announcement {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 8)]
                    reject_announcement {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 9)]
                    proxy_announced {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type:
                            ::core::option::Option<runtime_types::westend_runtime::ProxyType>,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooMany,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    NotProxy,
                    #[codec(index = 3)]
                    Unproxyable,
                    #[codec(index = 4)]
                    Duplicate,
                    #[codec(index = 5)]
                    NoPermission,
                    #[codec(index = 6)]
                    Unannounced,
                    #[codec(index = 7)]
                    NoSelfProxy,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    ProxyExecuted {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    AnonymousCreated {
                        anonymous: ::subxt::sp_core::crypto::AccountId32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::westend_runtime::ProxyType,
                        disambiguation_index: ::core::primitive::u16,
                    },
                    #[codec(index = 2)]
                    Announced {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        proxy: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 3)]
                    ProxyAdded {
                        delegator: ::subxt::sp_core::crypto::AccountId32,
                        delegatee: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::westend_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
            }
        }
        pub mod pallet_recovery {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    as_recovered {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    set_recovered {
                        lost: ::subxt::sp_core::crypto::AccountId32,
                        rescuer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    create_recovery {
                        friends: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        threshold: ::core::primitive::u16,
                        delay_period: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    initiate_recovery {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    vouch_recovery {
                        lost: ::subxt::sp_core::crypto::AccountId32,
                        rescuer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    claim_recovery {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    close_recovery {
                        rescuer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    remove_recovery,
                    #[codec(index = 8)]
                    cancel_recovered {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NotAllowed,
                    #[codec(index = 1)]
                    ZeroThreshold,
                    #[codec(index = 2)]
                    NotEnoughFriends,
                    #[codec(index = 3)]
                    MaxFriends,
                    #[codec(index = 4)]
                    NotSorted,
                    #[codec(index = 5)]
                    NotRecoverable,
                    #[codec(index = 6)]
                    AlreadyRecoverable,
                    #[codec(index = 7)]
                    AlreadyStarted,
                    #[codec(index = 8)]
                    NotStarted,
                    #[codec(index = 9)]
                    NotFriend,
                    #[codec(index = 10)]
                    DelayPeriod,
                    #[codec(index = 11)]
                    AlreadyVouched,
                    #[codec(index = 12)]
                    Threshold,
                    #[codec(index = 13)]
                    StillActive,
                    #[codec(index = 14)]
                    AlreadyProxy,
                    #[codec(index = 15)]
                    BadState,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    RecoveryCreated {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    RecoveryInitiated {
                        lost_account: ::subxt::sp_core::crypto::AccountId32,
                        rescuer_account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    RecoveryVouched {
                        lost_account: ::subxt::sp_core::crypto::AccountId32,
                        rescuer_account: ::subxt::sp_core::crypto::AccountId32,
                        sender: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 3)]
                    RecoveryClosed {
                        lost_account: ::subxt::sp_core::crypto::AccountId32,
                        rescuer_account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    AccountRecovered {
                        lost_account: ::subxt::sp_core::crypto::AccountId32,
                        rescuer_account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    RecoveryRemoved {
                        lost_account: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ActiveRecovery<_0, _1, _2> {
                pub created: _0,
                pub deposit: _1,
                pub friends: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RecoveryConfig<_0, _1, _2> {
                pub delay_period: _0,
                pub deposit: _1,
                pub friends: ::std::vec::Vec<_2>,
                pub threshold: ::core::primitive::u16,
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    schedule_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 3)]
                    cancel_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 5)]
                    schedule_named_after {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    FailedToSchedule,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    TargetBlockNumberInPast,
                    #[codec(index = 3)]
                    RescheduleNoChange,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Scheduled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 1)]
                    Canceled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 2)]
                    Dispatched(
                        (::core::primitive::u32, ::core::primitive::u32),
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1,
                #[codec(index = 1)]
                V2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ScheduledV2<_0, _1, _2, _3> {
                pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub priority: ::core::primitive::u8,
                pub call: _0,
                pub maybe_periodic: ::core::option::Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_keys {
                        keys: runtime_types::westend_runtime::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    purge_keys,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        bond {
                            controller: ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        },
                        #[codec(index = 1)]
                        bond_extra {
                            #[codec(compact)]
                            max_additional: ::core::primitive::u128,
                        },
                        #[codec(index = 2)]
                        unbond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 3)]
                        withdraw_unbonded {
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        validate {
                            prefs: runtime_types::pallet_staking::ValidatorPrefs,
                        },
                        #[codec(index = 5)]
                        nominate {
                            targets: ::std::vec::Vec<
                                ::subxt::sp_runtime::MultiAddress<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    (),
                                >,
                            >,
                        },
                        #[codec(index = 6)]
                        chill,
                        #[codec(index = 7)]
                        set_payee {
                            payee: runtime_types::pallet_staking::RewardDestination<
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        },
                        #[codec(index = 8)]
                        set_controller {
                            controller: ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        },
                        #[codec(index = 9)]
                        set_validator_count {
                            #[codec(compact)]
                            new: ::core::primitive::u32,
                        },
                        #[codec(index = 10)]
                        increase_validator_count {
                            #[codec(compact)]
                            additional: ::core::primitive::u32,
                        },
                        #[codec(index = 11)]
                        scale_validator_count {
                            factor: runtime_types::sp_arithmetic::per_things::Percent,
                        },
                        #[codec(index = 12)]
                        force_no_eras,
                        #[codec(index = 13)]
                        force_new_era,
                        #[codec(index = 14)]
                        set_invulnerables {
                            invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        },
                        #[codec(index = 15)]
                        force_unstake {
                            stash: ::subxt::sp_core::crypto::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 16)]
                        force_new_era_always,
                        #[codec(index = 17)]
                        cancel_deferred_slash {
                            era: ::core::primitive::u32,
                            slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                        },
                        #[codec(index = 18)]
                        payout_stakers {
                            validator_stash: ::subxt::sp_core::crypto::AccountId32,
                            era: ::core::primitive::u32,
                        },
                        #[codec(index = 19)]
                        rebond {
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                        },
                        #[codec(index = 20)]
                        set_history_depth {
                            #[codec(compact)]
                            new_history_depth: ::core::primitive::u32,
                            #[codec(compact)]
                            era_items_deleted: ::core::primitive::u32,
                        },
                        #[codec(index = 21)]
                        reap_stash {
                            stash: ::subxt::sp_core::crypto::AccountId32,
                            num_slashing_spans: ::core::primitive::u32,
                        },
                        #[codec(index = 22)]
                        kick {
                            who: ::std::vec::Vec<
                                ::subxt::sp_runtime::MultiAddress<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    (),
                                >,
                            >,
                        },
                        #[codec(index = 23)]
                        set_staking_limits {
                            min_nominator_bond: ::core::primitive::u128,
                            min_validator_bond: ::core::primitive::u128,
                            max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                            max_validator_count: ::core::option::Option<::core::primitive::u32>,
                            threshold: ::core::option::Option<
                                runtime_types::sp_arithmetic::per_things::Percent,
                            >,
                        },
                        #[codec(index = 24)]
                        chill_other {
                            controller: ::subxt::sp_core::crypto::AccountId32,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        NotController,
                        #[codec(index = 1)]
                        NotStash,
                        #[codec(index = 2)]
                        AlreadyBonded,
                        #[codec(index = 3)]
                        AlreadyPaired,
                        #[codec(index = 4)]
                        EmptyTargets,
                        #[codec(index = 5)]
                        DuplicateIndex,
                        #[codec(index = 6)]
                        InvalidSlashIndex,
                        #[codec(index = 7)]
                        InsufficientBond,
                        #[codec(index = 8)]
                        NoMoreChunks,
                        #[codec(index = 9)]
                        NoUnlockChunk,
                        #[codec(index = 10)]
                        FundedTarget,
                        #[codec(index = 11)]
                        InvalidEraToReward,
                        #[codec(index = 12)]
                        InvalidNumberOfNominations,
                        #[codec(index = 13)]
                        NotSortedAndUnique,
                        #[codec(index = 14)]
                        AlreadyClaimed,
                        #[codec(index = 15)]
                        IncorrectHistoryDepth,
                        #[codec(index = 16)]
                        IncorrectSlashingSpans,
                        #[codec(index = 17)]
                        BadState,
                        #[codec(index = 18)]
                        TooManyTargets,
                        #[codec(index = 19)]
                        BadTarget,
                        #[codec(index = 20)]
                        CannotChillOther,
                        #[codec(index = 21)]
                        TooManyNominators,
                        #[codec(index = 22)]
                        TooManyValidators,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        EraPaid(
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 1)]
                        Rewarded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 2)]
                        Slashed(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 3)]
                        OldSlashingReportDiscarded(::core::primitive::u32),
                        #[codec(index = 4)]
                        StakersElected,
                        #[codec(index = 5)]
                        Bonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 6)]
                        Unbonded(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 7)]
                        Withdrawn(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 8)]
                        Kicked(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        #[codec(index = 9)]
                        StakingElectionFailed,
                        #[codec(index = 10)]
                        Chilled(::subxt::sp_core::crypto::AccountId32),
                        #[codec(index = 11)]
                        PayoutStarted(
                            ::core::primitive::u32,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                    }
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SlashingSpans {
                    pub span_index: ::core::primitive::u32,
                    pub last_start: ::core::primitive::u32,
                    pub last_nonzero_slash: ::core::primitive::u32,
                    pub prior: ::std::vec::Vec<::core::primitive::u32>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SpanRecord<_0> {
                    pub slashed: _0,
                    pub paid_out: _0,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Exposure<_0, _1> {
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub own: _1,
                pub others:
                    ::std::vec::Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Forcing {
                #[codec(index = 0)]
                NotForcing,
                #[codec(index = 1)]
                ForceNew,
                #[codec(index = 2)]
                ForceNone,
                #[codec(index = 3)]
                ForceAlways,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct IndividualExposure<_0, _1> {
                pub who: _0,
                #[codec(compact)]
                pub value: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Nominations<_0> {
                pub targets: ::std::vec::Vec<_0>,
                pub submitted_in: ::core::primitive::u32,
                pub suppressed: ::core::primitive::bool,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0Ancient,
                #[codec(index = 1)]
                V2_0_0,
                #[codec(index = 2)]
                V3_0_0,
                #[codec(index = 3)]
                V4_0_0,
                #[codec(index = 4)]
                V5_0_0,
                #[codec(index = 5)]
                V6_0_0,
                #[codec(index = 6)]
                V7_0_0,
                #[codec(index = 7)]
                V8_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum RewardDestination<_0> {
                #[codec(index = 0)]
                Staked,
                #[codec(index = 1)]
                Stash,
                #[codec(index = 2)]
                Controller,
                #[codec(index = 3)]
                Account(_0),
                #[codec(index = 4)]
                None,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct StakingLedger<_0, _1> {
                pub stash: _0,
                #[codec(compact)]
                pub total: _1,
                #[codec(compact)]
                pub active: _1,
                pub unlocking: ::std::vec::Vec<runtime_types::pallet_staking::UnlockChunk<_1>>,
                pub claimed_rewards: ::std::vec::Vec<::core::primitive::u32>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnappliedSlash<_0, _1> {
                pub validator: _0,
                pub own: _1,
                pub others: ::std::vec::Vec<(_0, _1)>,
                pub reporters: ::std::vec::Vec<_0>,
                pub payout: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct UnlockChunk<_0> {
                #[codec(compact)]
                pub value: _0,
                #[codec(compact)]
                pub era: ::core::primitive::u32,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ValidatorPrefs {
                #[codec(compact)]
                pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
                pub blocked: ::core::primitive::bool,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    KeyChanged {
                        new_sudoer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct ChargeTransactionPayment(pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    batch {
                        calls: ::std::vec::Vec<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 2)]
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::westend_runtime::Call>,
                    },
                    #[codec(index = 3)]
                    dispatch_as {
                        as_origin: ::std::boxed::Box<runtime_types::westend_runtime::OriginCaller>,
                        call: ::std::boxed::Box<runtime_types::westend_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCalls,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    BatchCompleted,
                    #[codec(index = 2)]
                    ItemCompleted,
                    #[codec(index = 3)]
                    DispatchedAs(
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
        }
        pub mod pallet_vesting {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    vest,
                    #[codec(index = 1)]
                    vest_other {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 2)]
                    vested_transfer {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 3)]
                    force_vested_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    },
                    #[codec(index = 4)]
                    merge_schedules {
                        schedule1_index: ::core::primitive::u32,
                        schedule2_index: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    NotVesting,
                    #[codec(index = 1)]
                    AtMaxVestingSchedules,
                    #[codec(index = 2)]
                    AmountLow,
                    #[codec(index = 3)]
                    ScheduleIndexOutOfBounds,
                    #[codec(index = 4)]
                    InvalidScheduleParams,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    VestingUpdated {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        unvested: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    VestingCompleted {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
            }
            pub mod vesting_info {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct VestingInfo<_0, _1> {
                    pub locked: _0,
                    pub per_block: _0,
                    pub starting_block: _1,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Releases {
                #[codec(index = 0)]
                V0,
                #[codec(index = 1)]
                V1,
            }
        }
        pub mod pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Call {
                    #[codec(index = 0)]
                    send {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                    },
                    #[codec(index = 1)]
                    teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    execute {
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    force_xcm_version {
                        location:
                            ::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
                        xcm_version: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    force_default_xcm_version {
                        maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 6)]
                    force_subscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 7)]
                    force_unsubscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 8)]
                    limited_reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 9)]
                    limited_teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Error {
                    #[codec(index = 0)]
                    Unreachable,
                    #[codec(index = 1)]
                    SendFailure,
                    #[codec(index = 2)]
                    Filtered,
                    #[codec(index = 3)]
                    UnweighableMessage,
                    #[codec(index = 4)]
                    DestinationNotInvertible,
                    #[codec(index = 5)]
                    Empty,
                    #[codec(index = 6)]
                    CannotReanchor,
                    #[codec(index = 7)]
                    TooManyAssets,
                    #[codec(index = 8)]
                    InvalidOrigin,
                    #[codec(index = 9)]
                    BadVersion,
                    #[codec(index = 10)]
                    BadLocation,
                    #[codec(index = 11)]
                    NoSubscription,
                    #[codec(index = 12)]
                    AlreadySubscribed,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Event {
                    #[codec(index = 0)]
                    Attempted(runtime_types::xcm::v2::traits::Outcome),
                    #[codec(index = 1)]
                    Sent(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v2::Xcm,
                    ),
                    #[codec(index = 2)]
                    UnexpectedResponse(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
                    #[codec(index = 4)]
                    Notified(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 5)]
                    NotifyOverweight(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    NotifyDispatchError(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 7)]
                    NotifyDecodeFailed(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 8)]
                    InvalidResponder(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        ::core::option::Option<
                            runtime_types::xcm::v1::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 9)]
                    InvalidResponderVersion(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 10)]
                    ResponseTaken(::core::primitive::u64),
                    #[codec(index = 11)]
                    AssetsTrapped(
                        ::subxt::sp_core::H256,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::VersionedMultiAssets,
                    ),
                    #[codec(index = 12)]
                    VersionChangeNotified(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 13)]
                    SupportedVersionChanged(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 14)]
                    NotifyTargetSendFail(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        runtime_types::xcm::v2::traits::Error,
                    ),
                    #[codec(index = 15)]
                    NotifyTargetMigrationFail(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u64,
                    ),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Origin {
                    #[codec(index = 0)]
                    Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
                    #[codec(index = 1)]
                    Response(runtime_types::xcm::v1::multilocation::MultiLocation),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum QueryStatus<_0> {
                    #[codec(index = 0)]
                    Pending {
                        responder: runtime_types::xcm::VersionedMultiLocation,
                        maybe_notify:
                            ::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
                        timeout: _0,
                    },
                    #[codec(index = 1)]
                    VersionNotifier {
                        origin: runtime_types::xcm::VersionedMultiLocation,
                        is_active: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    Ready {
                        response: runtime_types::xcm::VersionedResponse,
                        at: _0,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum VersionMigrationStage {
                    #[codec(index = 0)]
                    MigrateSupportedVersion,
                    #[codec(index = 1)]
                    MigrateVersionNotifiers,
                    #[codec(index = 2)]
                    NotifyCurrentTargets(
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ),
                    #[codec(index = 3)]
                    MigrateAndNotifyOldTargets,
                }
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct CandidateHash(pub ::subxt::sp_core::H256);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HrmpChannelId {
                    pub sender: runtime_types::polkadot_parachain::primitives::Id,
                    pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                }
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Id(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ValidationCode(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ValidationCodeHash(pub ::subxt::sp_core::H256);
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v0 {
                use super::runtime_types;
                pub mod collator_app {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                pub mod validator_app {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct ValidatorIndex(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ValidityAttestation {
                    #[codec(index = 1)]
                    Implicit(runtime_types::polkadot_primitives::v0::validator_app::Signature),
                    #[codec(index = 2)]
                    Explicit(runtime_types::polkadot_primitives::v0::validator_app::Signature),
                }
            }
            pub mod v1 {
                use super::runtime_types;
                pub mod assignment_app {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
                pub mod signed {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct UncheckedSigned<_0, _1> {
                        pub payload: _0,
                        pub validator_index: runtime_types::polkadot_primitives::v0::ValidatorIndex,
                        pub signature:
                            runtime_types::polkadot_primitives::v0::validator_app::Signature,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AvailabilityBitfield(
                    pub  ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                );
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BackedCandidate<_0> {
                    pub candidate:
                        runtime_types::polkadot_primitives::v1::CommittedCandidateReceipt<_0>,
                    pub validity_votes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::ValidityAttestation,
                    >,
                    pub validator_indices: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CandidateCommitments<_0> {
                    pub upward_messages: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub horizontal_messages: ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                    pub new_validation_code: ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                    pub head_data: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub processed_downward_messages: _0,
                    pub hrmp_watermark: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CandidateDescriptor<_0> {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub relay_parent: _0,
                    pub collator: runtime_types::polkadot_primitives::v0::collator_app::Public,
                    pub persisted_validation_data_hash: _0,
                    pub pov_hash: _0,
                    pub erasure_root: _0,
                    pub signature: runtime_types::polkadot_primitives::v0::collator_app::Signature,
                    pub para_head: _0,
                    pub validation_code_hash:
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CandidateReceipt<_0> {
                    pub descriptor: runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub commitments_hash: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CommittedCandidateReceipt<_0> {
                    pub descriptor: runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub commitments: runtime_types::polkadot_primitives::v1::CandidateCommitments<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct CoreIndex(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum CoreOccupied {
                    #[codec(index = 0)]
                    Parathread(runtime_types::polkadot_primitives::v1::ParathreadEntry),
                    #[codec(index = 1)]
                    Parachain,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum DisputeStatement {
                    #[codec(index = 0)]
                    Valid(runtime_types::polkadot_primitives::v1::ValidDisputeStatementKind),
                    #[codec(index = 1)]
                    Invalid(runtime_types::polkadot_primitives::v1::InvalidDisputeStatementKind),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DisputeStatementSet {
                    pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    pub session: ::core::primitive::u32,
                    pub statements: ::std::vec::Vec<(
                        runtime_types::polkadot_primitives::v1::DisputeStatement,
                        runtime_types::polkadot_primitives::v0::ValidatorIndex,
                        runtime_types::polkadot_primitives::v0::validator_app::Signature,
                    )>,
                }
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct GroupIndex(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct InherentData<_0> {
                    pub bitfields: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::signed::UncheckedSigned<
                            runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                            runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                        >,
                    >,
                    pub backed_candidates: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::BackedCandidate<
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    pub disputes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::DisputeStatementSet,
                    >,
                    pub parent_header: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum InvalidDisputeStatementKind {
                    #[codec(index = 0)]
                    Explicit,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ParathreadClaim(
                    pub runtime_types::polkadot_parachain::primitives::Id,
                    pub runtime_types::polkadot_primitives::v0::collator_app::Public,
                );
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ParathreadEntry {
                    pub claim: runtime_types::polkadot_primitives::v1::ParathreadClaim,
                    pub retries: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ScrapedOnChainVotes<_0> {
                    pub session: ::core::primitive::u32,
                    pub backing_validators_per_candidate: ::std::vec::Vec<(
                        runtime_types::polkadot_primitives::v1::CandidateReceipt<_0>,
                        ::std::vec::Vec<(
                            runtime_types::polkadot_primitives::v0::ValidatorIndex,
                            runtime_types::polkadot_primitives::v0::ValidityAttestation,
                        )>,
                    )>,
                    pub disputes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::DisputeStatementSet,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct SessionInfo {
                    pub validators: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub discovery_keys:
                        ::std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
                    pub assignment_keys: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::assignment_app::Public,
                    >,
                    pub validator_groups: ::std::vec::Vec<
                        ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    >,
                    pub n_cores: ::core::primitive::u32,
                    pub zeroth_delay_tranche_width: ::core::primitive::u32,
                    pub relay_vrf_modulo_samples: ::core::primitive::u32,
                    pub n_delay_tranches: ::core::primitive::u32,
                    pub no_show_slots: ::core::primitive::u32,
                    pub needed_approvals: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum UpgradeGoAhead {
                    #[codec(index = 0)]
                    Abort,
                    #[codec(index = 1)]
                    GoAhead,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum UpgradeRestriction {
                    #[codec(index = 0)]
                    Present,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ValidDisputeStatementKind {
                    #[codec(index = 0)]
                    Explicit,
                    #[codec(index = 1)]
                    BackingSeconded(::subxt::sp_core::H256),
                    #[codec(index = 2)]
                    BackingValid(::subxt::sp_core::H256),
                    #[codec(index = 3)]
                    ApprovalChecking,
                }
            }
        }
        pub mod polkadot_runtime_common {
            use super::runtime_types;
            pub mod auctions {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        new_auction {
                            #[codec(compact)]
                            duration: ::core::primitive::u32,
                            #[codec(compact)]
                            lease_period_index: ::core::primitive::u32,
                        },
                        #[codec(index = 1)]
                        bid {
                            #[codec(compact)]
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            auction_index: ::core::primitive::u32,
                            #[codec(compact)]
                            first_slot: ::core::primitive::u32,
                            #[codec(compact)]
                            last_slot: ::core::primitive::u32,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 2)]
                        cancel_auction,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        AuctionInProgress,
                        #[codec(index = 1)]
                        LeasePeriodInPast,
                        #[codec(index = 2)]
                        ParaNotRegistered,
                        #[codec(index = 3)]
                        NotCurrentAuction,
                        #[codec(index = 4)]
                        NotAuction,
                        #[codec(index = 5)]
                        AuctionEnded,
                        #[codec(index = 6)]
                        AlreadyLeasedOut,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        AuctionStarted(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 1)]
                        AuctionClosed(::core::primitive::u32),
                        #[codec(index = 2)]
                        Reserved(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 3)]
                        Unreserved(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 4)]
                        ReserveConfiscated(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 5)]
                        BidAccepted(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 6)]
                        WinningOffset(::core::primitive::u32, ::core::primitive::u32),
                    }
                }
            }
            pub mod crowdloan {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        create {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            cap: ::core::primitive::u128,
                            #[codec(compact)]
                            first_period: ::core::primitive::u32,
                            #[codec(compact)]
                            last_period: ::core::primitive::u32,
                            #[codec(compact)]
                            end: ::core::primitive::u32,
                            verifier:
                                ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                        },
                        #[codec(index = 1)]
                        contribute {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                            signature:
                                ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                        },
                        #[codec(index = 2)]
                        withdraw {
                            who: ::subxt::sp_core::crypto::AccountId32,
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 3)]
                        refund {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        dissolve {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 5)]
                        edit {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            cap: ::core::primitive::u128,
                            #[codec(compact)]
                            first_period: ::core::primitive::u32,
                            #[codec(compact)]
                            last_period: ::core::primitive::u32,
                            #[codec(compact)]
                            end: ::core::primitive::u32,
                            verifier:
                                ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                        },
                        #[codec(index = 6)]
                        add_memo {
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            memo: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 7)]
                        poke {
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        FirstPeriodInPast,
                        #[codec(index = 1)]
                        FirstPeriodTooFarInFuture,
                        #[codec(index = 2)]
                        LastPeriodBeforeFirstPeriod,
                        #[codec(index = 3)]
                        LastPeriodTooFarInFuture,
                        #[codec(index = 4)]
                        CannotEndInPast,
                        #[codec(index = 5)]
                        EndTooFarInFuture,
                        #[codec(index = 6)]
                        Overflow,
                        #[codec(index = 7)]
                        ContributionTooSmall,
                        #[codec(index = 8)]
                        InvalidParaId,
                        #[codec(index = 9)]
                        CapExceeded,
                        #[codec(index = 10)]
                        ContributionPeriodOver,
                        #[codec(index = 11)]
                        InvalidOrigin,
                        #[codec(index = 12)]
                        NotParachain,
                        #[codec(index = 13)]
                        LeaseActive,
                        #[codec(index = 14)]
                        BidOrLeaseActive,
                        #[codec(index = 15)]
                        FundNotEnded,
                        #[codec(index = 16)]
                        NoContributions,
                        #[codec(index = 17)]
                        NotReadyToDissolve,
                        #[codec(index = 18)]
                        InvalidSignature,
                        #[codec(index = 19)]
                        MemoTooLarge,
                        #[codec(index = 20)]
                        AlreadyInNewRaise,
                        #[codec(index = 21)]
                        VrfDelayInProgress,
                        #[codec(index = 22)]
                        NoLeasePeriod,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        Created(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 1)]
                        Contributed(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 2)]
                        Withdrew(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 3)]
                        PartiallyRefunded(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 4)]
                        AllRefunded(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 5)]
                        Dissolved(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 6)]
                        HandleBidResult(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                        ),
                        #[codec(index = 7)]
                        Edited(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 8)]
                        MemoUpdated(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 9)]
                        AddedToNewRaise(runtime_types::polkadot_parachain::primitives::Id),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct FundInfo<_0, _1, _2, _3> {
                    pub depositor: _0,
                    pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                    pub deposit: _1,
                    pub raised: _1,
                    pub end: _2,
                    pub cap: _1,
                    pub last_contribution:
                        runtime_types::polkadot_runtime_common::crowdloan::LastContribution<_2>,
                    pub first_period: _2,
                    pub last_period: _2,
                    pub trie_index: _2,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum LastContribution<_0> {
                    #[codec(index = 0)]
                    Never,
                    #[codec(index = 1)]
                    PreEnding(_0),
                    #[codec(index = 2)]
                    Ending(_0),
                }
            }
            pub mod paras_registrar {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        register {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                            validation_code:
                                runtime_types::polkadot_parachain::primitives::ValidationCode,
                        },
                        #[codec(index = 1)]
                        force_register {
                            who: ::subxt::sp_core::crypto::AccountId32,
                            deposit: ::core::primitive::u128,
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                            validation_code:
                                runtime_types::polkadot_parachain::primitives::ValidationCode,
                        },
                        #[codec(index = 2)]
                        deregister {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 3)]
                        swap {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            other: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        force_remove_lock {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 5)]
                        reserve,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        NotRegistered,
                        #[codec(index = 1)]
                        AlreadyRegistered,
                        #[codec(index = 2)]
                        NotOwner,
                        #[codec(index = 3)]
                        CodeTooLarge,
                        #[codec(index = 4)]
                        HeadDataTooLarge,
                        #[codec(index = 5)]
                        NotParachain,
                        #[codec(index = 6)]
                        NotParathread,
                        #[codec(index = 7)]
                        CannotDeregister,
                        #[codec(index = 8)]
                        CannotDowngrade,
                        #[codec(index = 9)]
                        CannotUpgrade,
                        #[codec(index = 10)]
                        ParaLocked,
                        #[codec(index = 11)]
                        NotReserved,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        Registered(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        #[codec(index = 1)]
                        Deregistered(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 2)]
                        Reserved(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ParaInfo<_0, _1> {
                    pub manager: _0,
                    pub deposit: _1,
                    pub locked: ::core::primitive::bool,
                }
            }
            pub mod paras_sudo_wrapper {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        sudo_schedule_para_initialize {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            genesis:
                                runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
                        },
                        #[codec(index = 1)]
                        sudo_schedule_para_cleanup {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 2)]
                        sudo_schedule_parathread_upgrade {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 3)]
                        sudo_schedule_parachain_downgrade {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        sudo_queue_downward_xcm {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            xcm: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                        },
                        #[codec(index = 5)]
                        sudo_establish_hrmp_channel {
                            sender: runtime_types::polkadot_parachain::primitives::Id,
                            recipient: runtime_types::polkadot_parachain::primitives::Id,
                            max_capacity: ::core::primitive::u32,
                            max_message_size: ::core::primitive::u32,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        ParaDoesntExist,
                        #[codec(index = 1)]
                        ParaAlreadyExists,
                        #[codec(index = 2)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 3)]
                        CouldntCleanup,
                        #[codec(index = 4)]
                        NotParathread,
                        #[codec(index = 5)]
                        NotParachain,
                        #[codec(index = 6)]
                        CannotUpgrade,
                        #[codec(index = 7)]
                        CannotDowngrade,
                    }
                }
            }
            pub mod slots {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        force_lease {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            leaser: ::subxt::sp_core::crypto::AccountId32,
                            amount: ::core::primitive::u128,
                            period_begin: ::core::primitive::u32,
                            period_count: ::core::primitive::u32,
                        },
                        #[codec(index = 1)]
                        clear_all_leases {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 2)]
                        trigger_onboard {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        ParaNotOnboarding,
                        #[codec(index = 1)]
                        LeaseError,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        NewLeasePeriod(::core::primitive::u32),
                        #[codec(index = 1)]
                        Leased(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                    }
                }
            }
        }
        pub mod polkadot_runtime_parachains {
            use super::runtime_types;
            pub mod configuration {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        set_validation_upgrade_frequency { new: ::core::primitive::u32 },
                        #[codec(index = 1)]
                        set_validation_upgrade_delay { new: ::core::primitive::u32 },
                        #[codec(index = 2)]
                        set_code_retention_period { new: ::core::primitive::u32 },
                        #[codec(index = 3)]
                        set_max_code_size { new: ::core::primitive::u32 },
                        #[codec(index = 4)]
                        set_max_pov_size { new: ::core::primitive::u32 },
                        #[codec(index = 5)]
                        set_max_head_data_size { new: ::core::primitive::u32 },
                        #[codec(index = 6)]
                        set_parathread_cores { new: ::core::primitive::u32 },
                        #[codec(index = 7)]
                        set_parathread_retries { new: ::core::primitive::u32 },
                        #[codec(index = 8)]
                        set_group_rotation_frequency { new: ::core::primitive::u32 },
                        #[codec(index = 9)]
                        set_chain_availability_period { new: ::core::primitive::u32 },
                        #[codec(index = 10)]
                        set_thread_availability_period { new: ::core::primitive::u32 },
                        #[codec(index = 11)]
                        set_scheduling_lookahead { new: ::core::primitive::u32 },
                        #[codec(index = 12)]
                        set_max_validators_per_core {
                            new: ::core::option::Option<::core::primitive::u32>,
                        },
                        #[codec(index = 13)]
                        set_max_validators {
                            new: ::core::option::Option<::core::primitive::u32>,
                        },
                        #[codec(index = 14)]
                        set_dispute_period { new: ::core::primitive::u32 },
                        #[codec(index = 15)]
                        set_dispute_post_conclusion_acceptance_period {
                            new: ::core::primitive::u32,
                        },
                        #[codec(index = 16)]
                        set_dispute_max_spam_slots { new: ::core::primitive::u32 },
                        #[codec(index = 17)]
                        set_dispute_conclusion_by_time_out_period { new: ::core::primitive::u32 },
                        #[codec(index = 18)]
                        set_no_show_slots { new: ::core::primitive::u32 },
                        #[codec(index = 19)]
                        set_n_delay_tranches { new: ::core::primitive::u32 },
                        #[codec(index = 20)]
                        set_zeroth_delay_tranche_width { new: ::core::primitive::u32 },
                        #[codec(index = 21)]
                        set_needed_approvals { new: ::core::primitive::u32 },
                        #[codec(index = 22)]
                        set_relay_vrf_modulo_samples { new: ::core::primitive::u32 },
                        #[codec(index = 23)]
                        set_max_upward_queue_count { new: ::core::primitive::u32 },
                        #[codec(index = 24)]
                        set_max_upward_queue_size { new: ::core::primitive::u32 },
                        #[codec(index = 25)]
                        set_max_downward_message_size { new: ::core::primitive::u32 },
                        #[codec(index = 26)]
                        set_ump_service_total_weight { new: ::core::primitive::u64 },
                        #[codec(index = 27)]
                        set_max_upward_message_size { new: ::core::primitive::u32 },
                        #[codec(index = 28)]
                        set_max_upward_message_num_per_candidate { new: ::core::primitive::u32 },
                        #[codec(index = 29)]
                        set_hrmp_open_request_ttl { new: ::core::primitive::u32 },
                        #[codec(index = 30)]
                        set_hrmp_sender_deposit { new: ::core::primitive::u128 },
                        #[codec(index = 31)]
                        set_hrmp_recipient_deposit { new: ::core::primitive::u128 },
                        #[codec(index = 32)]
                        set_hrmp_channel_max_capacity { new: ::core::primitive::u32 },
                        #[codec(index = 33)]
                        set_hrmp_channel_max_total_size { new: ::core::primitive::u32 },
                        #[codec(index = 34)]
                        set_hrmp_max_parachain_inbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 35)]
                        set_hrmp_max_parathread_inbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 36)]
                        set_hrmp_channel_max_message_size { new: ::core::primitive::u32 },
                        #[codec(index = 37)]
                        set_hrmp_max_parachain_outbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 38)]
                        set_hrmp_max_parathread_outbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 39)]
                        set_hrmp_max_message_num_per_candidate { new: ::core::primitive::u32 },
                        #[codec(index = 40)]
                        set_ump_max_individual_weight { new: ::core::primitive::u64 },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        InvalidNewValue,
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HostConfiguration<_0> {
                    pub max_code_size: _0,
                    pub max_head_data_size: _0,
                    pub max_upward_queue_count: _0,
                    pub max_upward_queue_size: _0,
                    pub max_upward_message_size: _0,
                    pub max_upward_message_num_per_candidate: _0,
                    pub hrmp_max_message_num_per_candidate: _0,
                    pub validation_upgrade_frequency: _0,
                    pub validation_upgrade_delay: _0,
                    pub max_pov_size: _0,
                    pub max_downward_message_size: _0,
                    pub ump_service_total_weight: ::core::primitive::u64,
                    pub hrmp_max_parachain_outbound_channels: _0,
                    pub hrmp_max_parathread_outbound_channels: _0,
                    pub hrmp_sender_deposit: ::core::primitive::u128,
                    pub hrmp_recipient_deposit: ::core::primitive::u128,
                    pub hrmp_channel_max_capacity: _0,
                    pub hrmp_channel_max_total_size: _0,
                    pub hrmp_max_parachain_inbound_channels: _0,
                    pub hrmp_max_parathread_inbound_channels: _0,
                    pub hrmp_channel_max_message_size: _0,
                    pub code_retention_period: _0,
                    pub parathread_cores: _0,
                    pub parathread_retries: _0,
                    pub group_rotation_frequency: _0,
                    pub chain_availability_period: _0,
                    pub thread_availability_period: _0,
                    pub scheduling_lookahead: _0,
                    pub max_validators_per_core: ::core::option::Option<_0>,
                    pub max_validators: ::core::option::Option<_0>,
                    pub dispute_period: _0,
                    pub dispute_post_conclusion_acceptance_period: _0,
                    pub dispute_max_spam_slots: _0,
                    pub dispute_conclusion_by_time_out_period: _0,
                    pub no_show_slots: _0,
                    pub n_delay_tranches: _0,
                    pub zeroth_delay_tranche_width: _0,
                    pub needed_approvals: _0,
                    pub relay_vrf_modulo_samples: _0,
                    pub ump_max_individual_weight: ::core::primitive::u64,
                }
            }
            pub mod dmp {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {}
                }
            }
            pub mod hrmp {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        hrmp_init_open_channel {
                            recipient: runtime_types::polkadot_parachain::primitives::Id,
                            proposed_max_capacity: ::core::primitive::u32,
                            proposed_max_message_size: ::core::primitive::u32,
                        },
                        #[codec(index = 1)]
                        hrmp_accept_open_channel {
                            sender: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 2)]
                        hrmp_close_channel {
                            channel_id:
                                runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        },
                        #[codec(index = 3)]
                        force_clean_hrmp {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        force_process_hrmp_open,
                        #[codec(index = 5)]
                        force_process_hrmp_close,
                        #[codec(index = 6)]
                        hrmp_cancel_open_request {
                            channel_id:
                                runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        OpenHrmpChannelToSelf,
                        #[codec(index = 1)]
                        OpenHrmpChannelInvalidRecipient,
                        #[codec(index = 2)]
                        OpenHrmpChannelZeroCapacity,
                        #[codec(index = 3)]
                        OpenHrmpChannelCapacityExceedsLimit,
                        #[codec(index = 4)]
                        OpenHrmpChannelZeroMessageSize,
                        #[codec(index = 5)]
                        OpenHrmpChannelMessageSizeExceedsLimit,
                        #[codec(index = 6)]
                        OpenHrmpChannelAlreadyExists,
                        #[codec(index = 7)]
                        OpenHrmpChannelAlreadyRequested,
                        #[codec(index = 8)]
                        OpenHrmpChannelLimitExceeded,
                        #[codec(index = 9)]
                        AcceptHrmpChannelDoesntExist,
                        #[codec(index = 10)]
                        AcceptHrmpChannelAlreadyConfirmed,
                        #[codec(index = 11)]
                        AcceptHrmpChannelLimitExceeded,
                        #[codec(index = 12)]
                        CloseHrmpChannelUnauthorized,
                        #[codec(index = 13)]
                        CloseHrmpChannelDoesntExist,
                        #[codec(index = 14)]
                        CloseHrmpChannelAlreadyUnderway,
                        #[codec(index = 15)]
                        CancelHrmpOpenChannelUnauthorized,
                        #[codec(index = 16)]
                        OpenHrmpChannelDoesntExist,
                        #[codec(index = 17)]
                        OpenHrmpChannelAlreadyConfirmed,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        OpenChannelRequested(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 1)]
                        OpenChannelCanceled(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        ),
                        #[codec(index = 2)]
                        OpenChannelAccepted(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        #[codec(index = 3)]
                        ChannelClosed(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HrmpChannel {
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                    pub max_message_size: ::core::primitive::u32,
                    pub msg_count: ::core::primitive::u32,
                    pub total_size: ::core::primitive::u32,
                    pub mqc_head: ::core::option::Option<::subxt::sp_core::H256>,
                    pub sender_deposit: ::core::primitive::u128,
                    pub recipient_deposit: ::core::primitive::u128,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct HrmpOpenChannelRequest {
                    pub confirmed: ::core::primitive::bool,
                    pub _age: ::core::primitive::u32,
                    pub sender_deposit: ::core::primitive::u128,
                    pub max_message_size: ::core::primitive::u32,
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                }
            }
            pub mod inclusion {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {}
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        WrongBitfieldSize,
                        #[codec(index = 1)]
                        BitfieldDuplicateOrUnordered,
                        #[codec(index = 2)]
                        ValidatorIndexOutOfBounds,
                        #[codec(index = 3)]
                        InvalidBitfieldSignature,
                        #[codec(index = 4)]
                        UnscheduledCandidate,
                        #[codec(index = 5)]
                        CandidateScheduledBeforeParaFree,
                        #[codec(index = 6)]
                        WrongCollator,
                        #[codec(index = 7)]
                        ScheduledOutOfOrder,
                        #[codec(index = 8)]
                        HeadDataTooLarge,
                        #[codec(index = 9)]
                        PrematureCodeUpgrade,
                        #[codec(index = 10)]
                        NewCodeTooLarge,
                        #[codec(index = 11)]
                        CandidateNotInParentContext,
                        #[codec(index = 12)]
                        InvalidGroupIndex,
                        #[codec(index = 13)]
                        InsufficientBacking,
                        #[codec(index = 14)]
                        InvalidBacking,
                        #[codec(index = 15)]
                        NotCollatorSigned,
                        #[codec(index = 16)]
                        ValidationDataHashMismatch,
                        #[codec(index = 17)]
                        IncorrectDownwardMessageHandling,
                        #[codec(index = 18)]
                        InvalidUpwardMessages,
                        #[codec(index = 19)]
                        HrmpWatermarkMishandling,
                        #[codec(index = 20)]
                        InvalidOutboundHrmp,
                        #[codec(index = 21)]
                        InvalidValidationCodeHash,
                        #[codec(index = 22)]
                        ParaHeadMismatch,
                        #[codec(index = 23)]
                        BitfieldReferencesFreedCore,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        CandidateBacked(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                            runtime_types::polkadot_primitives::v1::GroupIndex,
                        ),
                        #[codec(index = 1)]
                        CandidateIncluded(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                            runtime_types::polkadot_primitives::v1::GroupIndex,
                        ),
                        #[codec(index = 2)]
                        CandidateTimedOut(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AvailabilityBitfieldRecord<_0> {
                    pub bitfield: runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                    pub submitted_at: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CandidatePendingAvailability<_0, _1> {
                    pub core: runtime_types::polkadot_primitives::v1::CoreIndex,
                    pub hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    pub descriptor: runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub availability_votes: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub backers: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub relay_parent_number: _1,
                    pub backed_in_number: _1,
                    pub backing_group: runtime_types::polkadot_primitives::v1::GroupIndex,
                }
            }
            pub mod initializer {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        force_approve { up_to: ::core::primitive::u32 },
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BufferedSessionChange {
                    pub validators: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub queued: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub session_index: ::core::primitive::u32,
                }
            }
            pub mod origin {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Origin {
                        #[codec(index = 0)]
                        Parachain(runtime_types::polkadot_parachain::primitives::Id),
                    }
                }
            }
            pub mod paras {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        force_set_current_code {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                        },
                        #[codec(index = 1)]
                        force_set_current_head {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                        },
                        #[codec(index = 2)]
                        force_schedule_code_upgrade {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                            relay_parent_number: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        force_note_new_head {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                        },
                        #[codec(index = 4)]
                        force_queue_action {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        NotRegistered,
                        #[codec(index = 1)]
                        CannotOnboard,
                        #[codec(index = 2)]
                        CannotOffboard,
                        #[codec(index = 3)]
                        CannotUpgrade,
                        #[codec(index = 4)]
                        CannotDowngrade,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        CurrentCodeUpdated(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 1)]
                        CurrentHeadUpdated(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 2)]
                        CodeUpgradeScheduled(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 3)]
                        NewHeadNoted(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 4)]
                        ActionQueued(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u32,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ParaGenesisArgs {
                    pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub validation_code:
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    pub parachain: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum ParaLifecycle {
                    #[codec(index = 0)]
                    Onboarding,
                    #[codec(index = 1)]
                    Parathread,
                    #[codec(index = 2)]
                    Parachain,
                    #[codec(index = 3)]
                    UpgradingParathread,
                    #[codec(index = 4)]
                    DowngradingParachain,
                    #[codec(index = 5)]
                    OffboardingParathread,
                    #[codec(index = 6)]
                    OffboardingParachain,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ParaPastCodeMeta<_0> {
                    pub upgrade_times: ::std::vec::Vec<
                        runtime_types::polkadot_runtime_parachains::paras::ReplacementTimes<_0>,
                    >,
                    pub last_pruned: ::core::option::Option<_0>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ReplacementTimes<_0> {
                    pub expected_at: _0,
                    pub activated_at: _0,
                }
            }
            pub mod paras_inherent {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        enter {
                            data: runtime_types::polkadot_primitives::v1::InherentData<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                            >,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        TooManyInclusionInherents,
                        #[codec(index = 1)]
                        InvalidParentHeader,
                        #[codec(index = 2)]
                        CandidateConcludedInvalid,
                        #[codec(index = 3)]
                        InherentOverweight,
                    }
                }
            }
            pub mod scheduler {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum AssignmentKind {
                    #[codec(index = 0)]
                    Parachain,
                    #[codec(index = 1)]
                    Parathread(
                        runtime_types::polkadot_primitives::v0::collator_app::Public,
                        ::core::primitive::u32,
                    ),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct CoreAssignment {
                    pub core: runtime_types::polkadot_primitives::v1::CoreIndex,
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub kind: runtime_types::polkadot_runtime_parachains::scheduler::AssignmentKind,
                    pub group_idx: runtime_types::polkadot_primitives::v1::GroupIndex,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct ParathreadClaimQueue {
                    pub queue: ::std::vec::Vec<
                        runtime_types::polkadot_runtime_parachains::scheduler::QueuedParathread,
                    >,
                    pub next_core_offset: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct QueuedParathread {
                    pub claim: runtime_types::polkadot_primitives::v1::ParathreadEntry,
                    pub core_offset: ::core::primitive::u32,
                }
            }
            pub mod shared {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {}
                }
            }
            pub mod ump {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Call {
                        #[codec(index = 0)]
                        service_overweight {
                            index: ::core::primitive::u64,
                            weight_limit: ::core::primitive::u64,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        UnknownMessageIndex,
                        #[codec(index = 1)]
                        WeightOverLimit,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Event {
                        #[codec(index = 0)]
                        InvalidFormat([::core::primitive::u8; 32usize]),
                        #[codec(index = 1)]
                        UnsupportedVersion([::core::primitive::u8; 32usize]),
                        #[codec(index = 2)]
                        ExecutedUpward(
                            [::core::primitive::u8; 32usize],
                            runtime_types::xcm::v2::traits::Outcome,
                        ),
                        #[codec(index = 3)]
                        WeightExhausted(
                            [::core::primitive::u8; 32usize],
                            ::core::primitive::u64,
                            ::core::primitive::u64,
                        ),
                        #[codec(index = 4)]
                        UpwardMessagesReceived(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 5)]
                        OverweightEnqueued(
                            runtime_types::polkadot_parachain::primitives::Id,
                            [::core::primitive::u8; 32usize],
                            ::core::primitive::u64,
                            ::core::primitive::u64,
                        ),
                        #[codec(index = 6)]
                        OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
                    }
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct PerU16(pub ::core::primitive::u16);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                )]
                pub struct Percent(pub ::core::primitive::u8);
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_npos_elections {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Support<_0> {
                pub total: ::core::primitive::u128,
                pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(_1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct BlakeTwo256 {}
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_staking {
            use super::runtime_types;
            pub mod offence {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: ::std::vec::Vec<_0>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
        pub mod westend_runtime {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Indices(runtime_types::pallet_indices::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 5)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 6)]
                Staking(runtime_types::pallet_staking::pallet::pallet::Call),
                #[codec(index = 8)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 10)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 11)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 16)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 17)]
                Identity(runtime_types::pallet_identity::pallet::Call),
                #[codec(index = 18)]
                Recovery(runtime_types::pallet_recovery::pallet::Call),
                #[codec(index = 19)]
                Vesting(runtime_types::pallet_vesting::pallet::Call),
                #[codec(index = 20)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                #[codec(index = 21)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 22)]
                Proxy(runtime_types::pallet_proxy::pallet::Call),
                #[codec(index = 23)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 24)]
                ElectionProviderMultiPhase(
                    runtime_types::pallet_election_provider_multi_phase::pallet::Call,
                ),
                #[codec(index = 25)]
                BagsList(runtime_types::pallet_bags_list::pallet::Call),
                #[codec(index = 42)]
                Configuration(
                    runtime_types::polkadot_runtime_parachains::configuration::pallet::Call,
                ),
                #[codec(index = 43)]
                ParasShared(runtime_types::polkadot_runtime_parachains::shared::pallet::Call),
                #[codec(index = 44)]
                ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Call),
                #[codec(index = 45)]
                ParaInherent(
                    runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Call,
                ),
                #[codec(index = 47)]
                Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Call),
                #[codec(index = 48)]
                Initializer(runtime_types::polkadot_runtime_parachains::initializer::pallet::Call),
                #[codec(index = 49)]
                Dmp(runtime_types::polkadot_runtime_parachains::dmp::pallet::Call),
                #[codec(index = 50)]
                Ump(runtime_types::polkadot_runtime_parachains::ump::pallet::Call),
                #[codec(index = 51)]
                Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Call),
                #[codec(index = 60)]
                Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Call),
                #[codec(index = 61)]
                Slots(runtime_types::polkadot_runtime_common::slots::pallet::Call),
                #[codec(index = 62)]
                ParasSudoWrapper(
                    runtime_types::polkadot_runtime_common::paras_sudo_wrapper::pallet::Call,
                ),
                #[codec(index = 63)]
                Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Call),
                #[codec(index = 64)]
                Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Call),
                #[codec(index = 99)]
                XcmPallet(runtime_types::pallet_xcm::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 3)]
                Indices(runtime_types::pallet_indices::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 6)]
                Staking(runtime_types::pallet_staking::pallet::pallet::Event),
                #[codec(index = 7)]
                Offences(runtime_types::pallet_offences::pallet::Event),
                #[codec(index = 8)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 10)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 11)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 16)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 17)]
                Identity(runtime_types::pallet_identity::pallet::Event),
                #[codec(index = 18)]
                Recovery(runtime_types::pallet_recovery::pallet::Event),
                #[codec(index = 19)]
                Vesting(runtime_types::pallet_vesting::pallet::Event),
                #[codec(index = 20)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Event),
                #[codec(index = 21)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 22)]
                Proxy(runtime_types::pallet_proxy::pallet::Event),
                #[codec(index = 23)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 24)]
                ElectionProviderMultiPhase(
                    runtime_types::pallet_election_provider_multi_phase::pallet::Event,
                ),
                #[codec(index = 25)]
                BagsList(runtime_types::pallet_bags_list::pallet::Event),
                #[codec(index = 44)]
                ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event),
                #[codec(index = 47)]
                Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Event),
                #[codec(index = 50)]
                Ump(runtime_types::polkadot_runtime_parachains::ump::pallet::Event),
                #[codec(index = 51)]
                Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event),
                #[codec(index = 60)]
                Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event),
                #[codec(index = 61)]
                Slots(runtime_types::polkadot_runtime_common::slots::pallet::Event),
                #[codec(index = 63)]
                Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Event),
                #[codec(index = 64)]
                Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Event),
                #[codec(index = 99)]
                XcmPallet(runtime_types::pallet_xcm::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct NposCompactSolution16 {
                votes1: ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u16)>,
                votes2: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    (
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ),
                    ::core::primitive::u16,
                )>,
                votes3: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 2usize],
                    ::core::primitive::u16,
                )>,
                votes4: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 3usize],
                    ::core::primitive::u16,
                )>,
                votes5: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 4usize],
                    ::core::primitive::u16,
                )>,
                votes6: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 5usize],
                    ::core::primitive::u16,
                )>,
                votes7: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 6usize],
                    ::core::primitive::u16,
                )>,
                votes8: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 7usize],
                    ::core::primitive::u16,
                )>,
                votes9: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 8usize],
                    ::core::primitive::u16,
                )>,
                votes10: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 9usize],
                    ::core::primitive::u16,
                )>,
                votes11: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 10usize],
                    ::core::primitive::u16,
                )>,
                votes12: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 11usize],
                    ::core::primitive::u16,
                )>,
                votes13: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 12usize],
                    ::core::primitive::u16,
                )>,
                votes14: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 13usize],
                    ::core::primitive::u16,
                )>,
                votes15: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 14usize],
                    ::core::primitive::u16,
                )>,
                votes16: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    [(
                        ::core::primitive::u16,
                        runtime_types::sp_arithmetic::per_things::PerU16,
                    ); 15usize],
                    ::core::primitive::u16,
                )>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_system::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
                ),
                #[codec(index = 41)]
                ParachainsOrigin(
                    runtime_types::polkadot_runtime_parachains::origin::pallet::Origin,
                ),
                #[codec(index = 99)]
                XcmPallet(runtime_types::pallet_xcm::pallet::Origin),
                #[codec(index = 3)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum ProxyType {
                #[codec(index = 0)]
                Any,
                #[codec(index = 1)]
                NonTransfer,
                #[codec(index = 2)]
                Staking,
                #[codec(index = 3)]
                SudoBalances,
                #[codec(index = 4)]
                IdentityJudgement,
                #[codec(index = 5)]
                CancelProxy,
                #[codec(index = 6)]
                Auction,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct Runtime {}
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub struct SessionKeys {
                pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                pub babe: runtime_types::sp_consensus_babe::app::Public,
                pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                pub para_validator: runtime_types::polkadot_primitives::v0::validator_app::Public,
                pub para_assignment: runtime_types::polkadot_primitives::v1::assignment_app::Public,
                pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod double_encoded {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct DoubleEncoded {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod v0 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum BodyId {
                        #[codec(index = 0)]
                        Unit,
                        #[codec(index = 1)]
                        Named(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 2)]
                        Index(::core::primitive::u32),
                        #[codec(index = 3)]
                        Executive,
                        #[codec(index = 4)]
                        Technical,
                        #[codec(index = 5)]
                        Legislative,
                        #[codec(index = 6)]
                        Judicial,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum BodyPart {
                        #[codec(index = 0)]
                        Voice,
                        #[codec(index = 1)]
                        Members {
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                        #[codec(index = 2)]
                        Fraction {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        AtLeastProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        MoreThanProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parent,
                        #[codec(index = 1)]
                        Parachain(::core::primitive::u32),
                        #[codec(index = 2)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 3)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 4)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 5)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 6)]
                        GeneralIndex(::core::primitive::u128),
                        #[codec(index = 7)]
                        GeneralKey(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        OnlyChild,
                        #[codec(index = 9)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        Any,
                        #[codec(index = 1)]
                        Named(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                    }
                }
                pub mod multi_asset {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum MultiAsset {
                        #[codec(index = 0)]
                        None,
                        #[codec(index = 1)]
                        All,
                        #[codec(index = 2)]
                        AllFungible,
                        #[codec(index = 3)]
                        AllNonFungible,
                        #[codec(index = 4)]
                        AllAbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 5)]
                        AllAbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 6)]
                        AllConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 7)]
                        AllConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 8)]
                        AbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 9)]
                        AbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                        #[codec(index = 10)]
                        ConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 11)]
                        ConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                    }
                }
                pub mod multi_location {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum MultiLocation {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v0::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Order {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            receive:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            reserve: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v0::multi_asset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            xcm: ::std::vec::Vec<runtime_types::xcm::v0::Xcm>,
                        },
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum OriginKind {
                    #[codec(index = 0)]
                    Native,
                    #[codec(index = 1)]
                    SovereignAccount,
                    #[codec(index = 2)]
                    Superuser,
                    #[codec(index = 3)]
                    Xcm,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposit {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 2)]
                    TeleportAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v0::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v0::multi_location::MultiLocation,
                        message: ::std::boxed::Box<runtime_types::xcm::v0::Xcm>,
                    },
                }
            }
            pub mod v1 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                }
                pub mod multiasset {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                        #[codec(index = 6)]
                        Blob(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v1::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct MultiAssets(
                        pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
                    );
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum WildMultiAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::xcm::v1::multiasset::AssetId,
                            fun: runtime_types::xcm::v1::multiasset::WildFungibility,
                        },
                    }
                }
                pub mod multilocation {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v1::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v1::multilocation::Junctions,
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Order {
                        #[codec(index = 0)]
                        Noop,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            instructions: ::std::vec::Vec<runtime_types::xcm::v1::Xcm>,
                        },
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    Version(::core::primitive::u32),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposited {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v1::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v1::multilocation::Junctions,
                        message: ::std::boxed::Box<runtime_types::xcm::v1::Xcm>,
                    },
                    #[codec(index = 11)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 12)]
                    UnsubscribeVersion,
                }
            }
            pub mod v2 {
                use super::runtime_types;
                pub mod traits {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        MultiLocationFull,
                        #[codec(index = 5)]
                        MultiLocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        TooMuchWeightRequired,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        UnhandledXcmVersion,
                        #[codec(index = 23)]
                        WeightLimitReached(::core::primitive::u64),
                        #[codec(index = 24)]
                        Barrier,
                        #[codec(index = 25)]
                        WeightNotComputable,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                    pub enum Outcome {
                        #[codec(index = 0)]
                        Complete(::core::primitive::u64),
                        #[codec(index = 1)]
                        Incomplete(
                            ::core::primitive::u64,
                            runtime_types::xcm::v2::traits::Error,
                        ),
                        #[codec(index = 2)]
                        Error(runtime_types::xcm::v2::traits::Error),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v2::Response,
                        #[codec(compact)]
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        #[codec(compact)]
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v1::multilocation::Junctions),
                    #[codec(index = 12)]
                    ReportError {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 18)]
                    QueryHolding {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v2::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(::core::primitive::u64),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
                pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VersionedMultiAssets {
                #[codec(index = 0)]
                V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multiasset::MultiAssets),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VersionedMultiLocation {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::multi_location::MultiLocation),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multilocation::MultiLocation),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VersionedResponse {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Response),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Response),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Response),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
            pub enum VersionedXcm {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Xcm),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Xcm),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Xcm),
            }
        }
    }
    #[doc = r" Default configuration of common types for a target Substrate runtime."]
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DefaultConfig;
    impl ::subxt::Config for DefaultConfig {
        type Index = u32;
        type BlockNumber = u32;
        type Hash = ::subxt::sp_core::H256;
        type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
        type AccountId = ::subxt::sp_runtime::AccountId32;
        type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
        type Header = ::subxt::sp_runtime::generic::Header<
            Self::BlockNumber,
            ::subxt::sp_runtime::traits::BlakeTwo256,
        >;
        type Signature = ::subxt::sp_runtime::MultiSignature;
        type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
    }
    impl ::subxt::ExtrinsicExtraData<DefaultConfig> for DefaultConfig {
        type AccountData = AccountData;
        type Extra = ::subxt::DefaultExtra<DefaultConfig>;
    }
    pub type AccountData = self::system::storage::Account;
    impl ::subxt::AccountData<DefaultConfig> for AccountData {
        fn nonce(
            result: &<Self as ::subxt::StorageEntry>::Value,
        ) -> <DefaultConfig as ::subxt::Config>::Index {
            result.nonce
        }
        fn storage_entry(account_id: <DefaultConfig as ::subxt::Config>::AccountId) -> Self {
            Self(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        pub client: ::subxt::Client<T>,
    }
    impl<T> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self { client }
        }
    }
    impl<'a, T> RuntimeApi<T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T> {
            TransactionApi {
                client: &self.client,
            }
        }
    }
    pub struct StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn indices(&self) -> indices::storage::StorageApi<'a, T> {
            indices::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn staking(&self) -> staking::storage::StorageApi<'a, T> {
            staking::storage::StorageApi::new(self.client)
        }
        pub fn offences(&self) -> offences::storage::StorageApi<'a, T> {
            offences::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn identity(&self) -> identity::storage::StorageApi<'a, T> {
            identity::storage::StorageApi::new(self.client)
        }
        pub fn recovery(&self) -> recovery::storage::StorageApi<'a, T> {
            recovery::storage::StorageApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::storage::StorageApi<'a, T> {
            vesting::storage::StorageApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
            scheduler::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::storage::StorageApi<'a, T> {
            proxy::storage::StorageApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
            multisig::storage::StorageApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::storage::StorageApi<'a, T> {
            election_provider_multi_phase::storage::StorageApi::new(self.client)
        }
        pub fn bags_list(&self) -> bags_list::storage::StorageApi<'a, T> {
            bags_list::storage::StorageApi::new(self.client)
        }
        pub fn configuration(&self) -> configuration::storage::StorageApi<'a, T> {
            configuration::storage::StorageApi::new(self.client)
        }
        pub fn paras_shared(&self) -> paras_shared::storage::StorageApi<'a, T> {
            paras_shared::storage::StorageApi::new(self.client)
        }
        pub fn para_inclusion(&self) -> para_inclusion::storage::StorageApi<'a, T> {
            para_inclusion::storage::StorageApi::new(self.client)
        }
        pub fn para_inherent(&self) -> para_inherent::storage::StorageApi<'a, T> {
            para_inherent::storage::StorageApi::new(self.client)
        }
        pub fn para_scheduler(&self) -> para_scheduler::storage::StorageApi<'a, T> {
            para_scheduler::storage::StorageApi::new(self.client)
        }
        pub fn paras(&self) -> paras::storage::StorageApi<'a, T> {
            paras::storage::StorageApi::new(self.client)
        }
        pub fn initializer(&self) -> initializer::storage::StorageApi<'a, T> {
            initializer::storage::StorageApi::new(self.client)
        }
        pub fn dmp(&self) -> dmp::storage::StorageApi<'a, T> {
            dmp::storage::StorageApi::new(self.client)
        }
        pub fn ump(&self) -> ump::storage::StorageApi<'a, T> {
            ump::storage::StorageApi::new(self.client)
        }
        pub fn hrmp(&self) -> hrmp::storage::StorageApi<'a, T> {
            hrmp::storage::StorageApi::new(self.client)
        }
        pub fn para_session_info(&self) -> para_session_info::storage::StorageApi<'a, T> {
            para_session_info::storage::StorageApi::new(self.client)
        }
        pub fn registrar(&self) -> registrar::storage::StorageApi<'a, T> {
            registrar::storage::StorageApi::new(self.client)
        }
        pub fn slots(&self) -> slots::storage::StorageApi<'a, T> {
            slots::storage::StorageApi::new(self.client)
        }
        pub fn auctions(&self) -> auctions::storage::StorageApi<'a, T> {
            auctions::storage::StorageApi::new(self.client)
        }
        pub fn crowdloan(&self) -> crowdloan::storage::StorageApi<'a, T> {
            crowdloan::storage::StorageApi::new(self.client)
        }
        pub fn xcm_pallet(&self) -> xcm_pallet::storage::StorageApi<'a, T> {
            xcm_pallet::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> TransactionApi<'a, T>
    where
        T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn indices(&self) -> indices::calls::TransactionApi<'a, T> {
            indices::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn staking(&self) -> staking::calls::TransactionApi<'a, T> {
            staking::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn identity(&self) -> identity::calls::TransactionApi<'a, T> {
            identity::calls::TransactionApi::new(self.client)
        }
        pub fn recovery(&self) -> recovery::calls::TransactionApi<'a, T> {
            recovery::calls::TransactionApi::new(self.client)
        }
        pub fn vesting(&self) -> vesting::calls::TransactionApi<'a, T> {
            vesting::calls::TransactionApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T> {
            scheduler::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::calls::TransactionApi<'a, T> {
            proxy::calls::TransactionApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T> {
            multisig::calls::TransactionApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi<'a, T> {
            election_provider_multi_phase::calls::TransactionApi::new(self.client)
        }
        pub fn bags_list(&self) -> bags_list::calls::TransactionApi<'a, T> {
            bags_list::calls::TransactionApi::new(self.client)
        }
        pub fn configuration(&self) -> configuration::calls::TransactionApi<'a, T> {
            configuration::calls::TransactionApi::new(self.client)
        }
        pub fn paras_shared(&self) -> paras_shared::calls::TransactionApi<'a, T> {
            paras_shared::calls::TransactionApi::new(self.client)
        }
        pub fn para_inclusion(&self) -> para_inclusion::calls::TransactionApi<'a, T> {
            para_inclusion::calls::TransactionApi::new(self.client)
        }
        pub fn para_inherent(&self) -> para_inherent::calls::TransactionApi<'a, T> {
            para_inherent::calls::TransactionApi::new(self.client)
        }
        pub fn paras(&self) -> paras::calls::TransactionApi<'a, T> {
            paras::calls::TransactionApi::new(self.client)
        }
        pub fn initializer(&self) -> initializer::calls::TransactionApi<'a, T> {
            initializer::calls::TransactionApi::new(self.client)
        }
        pub fn dmp(&self) -> dmp::calls::TransactionApi<'a, T> {
            dmp::calls::TransactionApi::new(self.client)
        }
        pub fn ump(&self) -> ump::calls::TransactionApi<'a, T> {
            ump::calls::TransactionApi::new(self.client)
        }
        pub fn hrmp(&self) -> hrmp::calls::TransactionApi<'a, T> {
            hrmp::calls::TransactionApi::new(self.client)
        }
        pub fn registrar(&self) -> registrar::calls::TransactionApi<'a, T> {
            registrar::calls::TransactionApi::new(self.client)
        }
        pub fn slots(&self) -> slots::calls::TransactionApi<'a, T> {
            slots::calls::TransactionApi::new(self.client)
        }
        pub fn paras_sudo_wrapper(&self) -> paras_sudo_wrapper::calls::TransactionApi<'a, T> {
            paras_sudo_wrapper::calls::TransactionApi::new(self.client)
        }
        pub fn auctions(&self) -> auctions::calls::TransactionApi<'a, T> {
            auctions::calls::TransactionApi::new(self.client)
        }
        pub fn crowdloan(&self) -> crowdloan::calls::TransactionApi<'a, T> {
            crowdloan::calls::TransactionApi::new(self.client)
        }
        pub fn xcm_pallet(&self) -> xcm_pallet::calls::TransactionApi<'a, T> {
            xcm_pallet::calls::TransactionApi::new(self.client)
        }
    }
}
