// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use crate::shared::basetsd::UINT32;
use crate::shared::guiddef::GUID;
use crate::shared::minwindef::{BYTE, DWORD, FILETIME, INT, UINT, ULONG};
use crate::shared::wtypes::{BSTR, VARIANT_BOOL};
use super::oaidl::{IDispatch, IDispatchVtbl, IEnumVARIANT};
use super::unknwnbase::{IUnknown, IUnknownVtbl};
use super::winnt::{HRESULT, WCHAR};
pub const NA_DomainAuthenticationFailed: &str = "NA_DomainAuthenticationFailed";
pub const NA_NetworkClass: &str = "NA_NetworkClass";
pub const NA_NameSetByPolicy: &str = "NA_NameSetByPolicy";
pub const NA_IconSetByPolicy: &str = "NA_IconSetByPolicy";
pub const NA_DescriptionSetByPolicy: &str = "NA_DescriptionSetByPolicy";
pub const NA_CategorySetByPolicy: &str = "NA_CategorySetByPolicy";
pub const NA_NameReadOnly: &str = "NA_NameReadOnly";
pub const NA_IconReadOnly: &str = "NA_IconReadOnly";
pub const NA_DescriptionReadOnly: &str = "NA_DescriptionReadOnly";
pub const NA_CategoryReadOnly: &str = "NA_CategoryReadOnly";
pub const NA_AllowMerge: &str = "NA_AllowMerge";
pub const NA_InternetConnectivityV4: &str = "NA_InternetConnectivityV4";
pub const NA_InternetConnectivityV6: &str = "NA_InternetConnectivityV6";
pub const NLM_MAX_ADDRESS_LIST_SIZE: INT = 10;
pub const NLM_UNKNOWN_DATAPLAN_STATUS: UINT = 0xffffffff;
ENUM!{enum NLM_CONNECTION_COST {
    NLM_CONNECTION_COST_UNKNOWN = 0,
    NLM_CONNECTION_COST_UNRESTRICTED = 0x1,
    NLM_CONNECTION_COST_FIXED = 0x2,
    NLM_CONNECTION_COST_VARIABLE = 0x4,
    NLM_CONNECTION_COST_OVERDATALIMIT = 0x10000,
    NLM_CONNECTION_COST_CONGESTED = 0x20000,
    NLM_CONNECTION_COST_ROAMING = 0x40000,
    NLM_CONNECTION_COST_APPROACHINGDATALIMIT = 0x80000,
}}
STRUCT!{struct NLM_USAGE_DATA {
    UsageInMegabytes: DWORD,
    LastSyncTime: FILETIME,
}}
STRUCT!{struct NLM_DATAPLAN_STATUS {
    InterfaceGuid: GUID,
    UsageData: NLM_USAGE_DATA,
    DataLimitInMegabytes: DWORD,
    InboundBandwidthInKbps: DWORD,
    OutboundBandwidthInKbps: DWORD,
    NextBillingCycle: FILETIME,
    MaxTransferSizeInMegabytes: DWORD,
    Reserved: DWORD,
}}
STRUCT!{struct NLM_SOCKADDR {
    data: [BYTE; 128],
}}
ENUM!{enum NLM_NETWORK_CLASS {
    NLM_NETWORK_IDENTIFYING = 0x1,
    NLM_NETWORK_IDENTIFIED = 0x2,
    NLM_NETWORK_UNIDENTIFIED = 0x3,
}}
STRUCT!{struct NLM_SIMULATED_PROFILE_INFO {
    ProfileName: [WCHAR; 256],
    cost: NLM_CONNECTION_COST,
    UsageInMegabytes: DWORD,
    DataLimitInMegabytes: DWORD,
}}
ENUM!{enum NLM_INTERNET_CONNECTIVITY {
    NLM_INTERNET_CONNECTIVITY_WEBHIJACK = 0x1,
    NLM_INTERNET_CONNECTIVITY_PROXIED = 0x2,
    NLM_INTERNET_CONNECTIVITY_CORPORATE = 0x4,
}}
ENUM!{enum NLM_CONNECTIVITY {
    NLM_CONNECTIVITY_DISCONNECTED = 0,
    NLM_CONNECTIVITY_IPV4_NOTRAFFIC = 0x1,
    NLM_CONNECTIVITY_IPV6_NOTRAFFIC = 0x2,
    NLM_CONNECTIVITY_IPV4_SUBNET = 0x10,
    NLM_CONNECTIVITY_IPV4_LOCALNETWORK = 0x20,
    NLM_CONNECTIVITY_IPV4_INTERNET = 0x40,
    NLM_CONNECTIVITY_IPV6_SUBNET = 0x100,
    NLM_CONNECTIVITY_IPV6_LOCALNETWORK = 0x200,
    NLM_CONNECTIVITY_IPV6_INTERNET = 0x400,
}}
ENUM!{enum NLM_DOMAIN_TYPE {
    NLM_DOMAIN_TYPE_NON_DOMAIN_NETWORK = 0,
    NLM_DOMAIN_TYPE_DOMAIN_NETWORK = 0x1,
    NLM_DOMAIN_TYPE_DOMAIN_AUTHENTICATED = 0x2,
}}
ENUM!{enum NLM_ENUM_NETWORK {
    NLM_ENUM_NETWORK_CONNECTED = 0x1,
    NLM_ENUM_NETWORK_DISCONNECTED = 0x2,
    NLM_ENUM_NETWORK_ALL = 0x3,
}}
RIDL!{#[uuid(0xdcb00000, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkListManager(INetworkListManagerVtbl): IDispatch(IDispatchVtbl) {
    fn GetNetworks(
        Flags: NLM_ENUM_NETWORK,
        ppEnumNetwork: *mut *mut IEnumNetworks,
    ) -> HRESULT,
    fn GetNetwork(
        gdNetworkId: GUID,
        ppNetwork: *mut *mut INetwork,
    ) -> HRESULT,
    fn GetNetworkConnections(
        ppEnum: *mut *mut IEnumNetworkConnections,
    ) -> HRESULT,
    fn GetNetworkConnection(
        gdNetworkConnectionId: GUID,
        ppNetworkConnection: *mut *mut INetworkConnection,
    ) -> HRESULT,
    fn get_IsConnectedToInternet(
        pbIsConnected: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConnected(
        pbIsConnected: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetConnectivity(
        pConnectivity: *mut NLM_CONNECTIVITY,
    ) -> HRESULT,
    fn SetSimulatedProfileInfo(
        pSimulatedInfo: *mut NLM_SIMULATED_PROFILE_INFO,
    ) -> HRESULT,
    fn ClearSimulatedProfileInfo() -> HRESULT,
}}
RIDL!{#[uuid(0xdcb00001, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkListManagerEvents(INetworkListManagerEventsVtbl): IUnknown(IUnknownVtbl) {
    fn ConnectivityChanged(
        newConnectivity: NLM_CONNECTIVITY,
    ) -> HRESULT,
}}
ENUM!{enum NLM_NETWORK_CATEGORY {
    NLM_NETWORK_CATEGORY_PUBLIC = 0,
    NLM_NETWORK_CATEGORY_PRIVATE = 0x1,
    NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED = 0x2,
}}
RIDL!{#[uuid(0xdcb00002, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetwork(INetworkVtbl): IDispatch(IDispatchVtbl) {
    fn GetName(
        pszNetworkName: *mut BSTR,
    ) -> HRESULT,
    fn SetName(
        szNetworkNewName: BSTR,
    ) -> HRESULT,
    fn GetDescription(
        pszDescription: *mut BSTR,
    ) -> HRESULT,
    fn SetDescription(
        szDescription: BSTR,
    ) -> HRESULT,
    fn GetNetworkId(
        pgdGuidNetworkId: *mut GUID,
    ) -> HRESULT,
    fn GetDomainType(
        pNetworkType: *mut NLM_DOMAIN_TYPE,
    ) -> HRESULT,
    fn GetNetworkConnections(
        ppEnumNetworkConnection: *mut *mut IEnumNetworkConnections,
    ) -> HRESULT,
    fn GetTimeCreatedAndConnected(
        pdwLowDateTimeCreated: *mut DWORD,
        pdwHighDateTimeCreated: *mut DWORD,
        pdwLowDateTimeConnected: *mut DWORD,
        pdwHighDateTimeConnected: *mut DWORD,
    ) -> HRESULT,
    fn get_IsConnectedToInternet(
        pbIsConnected: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConnected(
        pbIsConnected: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetConnectivity(
        pConnectivity: *mut NLM_CONNECTIVITY,
    ) -> HRESULT,
    fn GetCategory(
        pCategory: *mut NLM_NETWORK_CATEGORY,
    ) -> HRESULT,
    fn SetCategory(
        NewCategory: NLM_NETWORK_CATEGORY,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb00003, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface IEnumNetworks(IEnumNetworksVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        ppEnumVar: *mut *mut IEnumVARIANT,
    ) -> HRESULT,
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut INetwork,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnumNetwork: *mut *mut IEnumNetworks,
    ) -> HRESULT,
}}
ENUM!{enum NLM_NETWORK_PROPERTY_CHANGE {
    NLM_NETWORK_PROPERTY_CHANGE_CONNECTION = 0x1,
    NLM_NETWORK_PROPERTY_CHANGE_DESCRIPTION = 0x2,
    NLM_NETWORK_PROPERTY_CHANGE_NAME = 0x4,
    NLM_NETWORK_PROPERTY_CHANGE_ICON = 0x8,
    NLM_NETWORK_PROPERTY_CHANGE_CATEGORY_VALUE = 0x10,
}}
RIDL!{#[uuid(0xdcb00004, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkEvents(INetworkEventsVtbl): IUnknown(IUnknownVtbl) {
    fn NetworkAdded(
        networkId: GUID,
    ) -> HRESULT,
    fn NetworkDeleted(
        networkId: GUID,
    ) -> HRESULT,
    fn NetworkConnectivityChanged(
        networkId: GUID,
        newConnectivity: NLM_CONNECTIVITY,
    ) -> HRESULT,
    fn NetworkPropertyChanged(
        networkId: GUID,
        flags: NLM_NETWORK_PROPERTY_CHANGE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb00005, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkConnection(INetworkConnectionVtbl): IDispatch(IDispatchVtbl) {
    fn GetNetwork(
        ppNetwork: *mut *mut INetwork,
    ) -> HRESULT,
    fn get_IsConnectedToInternet(
        pbIsConnected: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsConnected(
        pbIsConnected: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetConnectivity(
        pConnectivity: *mut NLM_CONNECTIVITY,
    ) -> HRESULT,
    fn GetConnectionId(
        pgdConnectionId: *mut GUID,
    ) -> HRESULT,
    fn GetAdapterId(
        pgdAdapterId: *mut GUID,
    ) -> HRESULT,
    fn GetDomainType(
        pDomainType: *mut NLM_DOMAIN_TYPE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb00006, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface IEnumNetworkConnections(IEnumNetworkConnectionsVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        ppEnumVar: *mut *mut IEnumVARIANT,
    ) -> HRESULT,
    fn Next(
        celt: ULONG,
        rgelt: *mut *mut INetworkConnection,
        pceltFetched: *mut ULONG,
    ) -> HRESULT,
    fn Skip(
        celt: ULONG,
    ) -> HRESULT,
    fn Reset() -> HRESULT,
    fn Clone(
        ppEnumNetwork: *mut *mut IEnumNetworkConnections,
    ) -> HRESULT,
}}
ENUM!{enum NLM_CONNECTION_PROPERTY_CHANGE {
    NLM_CONNECTION_PROPERTY_CHANGE_AUTHENTICATION = 0x1,
}}
RIDL!{#[uuid(0xdcb00007, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkConnectionEvents(INetworkConnectionEventsVtbl): IUnknown(IUnknownVtbl) {
    fn NetworkConnectionConnectivityChanged(
        connectionId: GUID,
        newConnectivity: NLM_CONNECTIVITY,
    ) -> HRESULT,
    fn NetworkConnectionPropertyChanged(
        connectionId: GUID,
        flags: NLM_CONNECTION_PROPERTY_CHANGE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb00008, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkCostManager(INetworkCostManagerVtbl): IUnknown(IUnknownVtbl) {
    fn GetCost(
        pCost: *mut DWORD,
        pDestIPAddr: *mut NLM_SOCKADDR,
    ) -> HRESULT,
    fn GetDataPlanStatus(
        pDataPlanStatus: *mut NLM_DATAPLAN_STATUS,
        pDestIPAddr: *mut NLM_SOCKADDR,
    ) -> HRESULT,
    fn SetDestinationAddresses(
        length: UINT32,
        pDestIPAddrList: *mut NLM_SOCKADDR,
        bAppend: VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb00009, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkCostManagerEvents(INetworkCostManagerEventsVtbl): IUnknown(IUnknownVtbl) {
    fn CostChanged(
        newCost: DWORD,
        pDestAddr: *mut NLM_SOCKADDR,
    ) -> HRESULT,
    fn DataPlanStatusChanged(
        pDestIPAddr: *mut NLM_SOCKADDR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb0000a, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkConnectionCost(INetworkConnectionCostVtbl): IUnknown(IUnknownVtbl) {
    fn GetCost(
        pCost: *mut DWORD,
    ) -> HRESULT,
    fn GetDataPlanStatus(
        pDataPlanStatus: *mut NLM_DATAPLAN_STATUS,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb0000b, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9f, 0xdb, 0xa5, 0x72, 0x3b)]
interface INetworkConnectionCostEvents(INetworkConnectionCostEventsVtbl): IUnknown(IUnknownVtbl) {
    fn ConnectionCostChanged(
        connectionId: GUID,
        newCost: DWORD,
    ) -> HRESULT,
    fn ConnectionDataPlanStatusChanged(
        connectionId: GUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdcb00c01, 0x570f, 0x4a9b, 0x8d, 0x69, 0x19, 0x9F, 0xDB, 0xA5, 0x72, 0x3B)]
class NetworkListManager;}
