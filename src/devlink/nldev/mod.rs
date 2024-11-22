// SPDX-License-Identifier: MIT

use crate::constants::*;
use anyhow::Context;
use byteorder::{ByteOrder, NativeEndian};
use netlink_packet_utils::{
    nla::{Nla, NlaBuffer, NlasIterator},
    parsers::*,
    traits::*,
    DecodeError,
};
use std::mem::size_of_val;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenlDevlinkAttrs {
    BusName(String),
    Location(String),
    PortIndex(u32),
    PortType(u16),
    DesiredType(u16),
    NetdevIndex(u32),
    NetdevName(String),
    PortIbdevName(String),
    PortSplitCount(u32),
    PortSplitGroup(u32),
    SbIndex(u32),
    SbSize(u32),
    SbIngressPoolCount(u16),
    SbEgressPoolCount(u16),
    SbIngressTcCount(u16),
    SbEgressTcCount(u16),
    SbPoolIndex(u16),
    SbPoolType(u8),
    SbPoolSize(u32),
    SbPoolThresholdType(u8),
    SbPoolThreshold(u32),
    SbTcIndex(u16),
    SbOccCur(u32),
    SbOccMax(u32),
    EswitchMode(u16),
    EswitchInlineMode(u8),
    DpipeTables(Vec<GenlDevlinkAttrs>),
    DpipeTable(Vec<GenlDevlinkAttrs>),
    DpipeTableName(String),
    DpipeTableSize(u64),
    DpipeTableMatches(Vec<GenlDevlinkAttrs>),
    DpipeTableActions(Vec<GenlDevlinkAttrs>),
    DpipeTableCountersEnabled(u8),
    DpipeEntries(Vec<GenlDevlinkAttrs>),
    DpipeEntry(Vec<GenlDevlinkAttrs>),
    DpipeEntryIndex(u64),
    DpipeEntryMatchValues(Vec<GenlDevlinkAttrs>),
    DpipeEntryActionValues(Vec<GenlDevlinkAttrs>),
    DpipeEntryCounter(u64),
    DpipeMatch(Vec<GenlDevlinkAttrs>),
    DpipeMatchValue(Vec<GenlDevlinkAttrs>),
    DpipeMatchType(u32),
    DpipeAction(Vec<GenlDevlinkAttrs>),
    DpipeActionValue(Vec<GenlDevlinkAttrs>),
    DpipeActionType(u32),
    DpipeValue(u32),
    DpipeValueMask(u32),
    DpipeValueMapping(u32),
    DpipeHeaders(Vec<GenlDevlinkAttrs>),
    DpipeHader(Vec<GenlDevlinkAttrs>),
    DpipeHeaderName(String),
    DpipeHeaderId(u32),
    DpipeHeaderFields(Vec<GenlDevlinkAttrs>),
    DpipeHeaderGlobal(u8),
    DpipeHeaderIndex(u32),
    DpipeField(Vec<GenlDevlinkAttrs>),
    DpipeFieldName(String),
    DpipeFieldId(u32),
    DpipeFieldBitwidth(u32),
    DpipeFieldMappingType(u32),
    EswitchEncapMode(u8),
    ResourceList(Vec<GenlDevlinkAttrs>),
    Resource(Vec<GenlDevlinkAttrs>),
    ResoureceName(String),
    ResourceId(u64),
    ResourceSize(u64),
    ResourceSizeNew(u64),
    ResourceSizeValid(u8),
    ResourceSizeMin(u64),
    ResourceSizeMax(u64),
    ResourceSizeGran(u64),
    ResourceUnit(u8),
    ResourceOcc(u64),
    DpipeTableResourceId(u64),
    DpipeTableResourceUnit(u64),
    PortFlavour(u16),
    PortNumber(u32),
    Param(Vec<GenlDevlinkAttrs>),
    ParamName(String),
    ParamGeneric(bool),
    ParamType(u8),
    ParamValueList(Vec<GenlDevlinkAttrs>),
    ParamValue(u64),
    ParamValueData(Vec<GenlDevlinkAttrs>),
    ParamValueCmode(u8),
    RegionName(String),
    RegionSize(u64),
    RegionSnapshots(Vec<GenlDevlinkAttrs>),
    RegionSnapshot(Vec<GenlDevlinkAttrs>),
    RegionSnapshotId(u32),
    RegionChunks(Vec<GenlDevlinkAttrs>),
    RegionChunk(Vec<GenlDevlinkAttrs>),
    RegionChunkData(Vec<u8>),
    RegionChunkOffset(u64),
    RegionChunkSize(u64),
    InfoDriverName(String),
    InfoSerialNo(String),
    InfoVersionFixed(Vec<GenlDevlinkAttrs>),
    InfoVersionRunning(Vec<GenlDevlinkAttrs>),
    InfoVersionStored(Vec<GenlDevlinkAttrs>),
    InfoVersionName(String),
    InfoVersionValue(String),
    SbPoolCellSize(u32),
    Fmsg(Vec<GenlDevlinkAttrs>),
    FmsgObjNestStart(bool),
    FmsgPairNestStart(bool),
    FmsgArrNestStart(bool),
    FmsgNestEnd(bool),
    FmsgObjName(String),
    FmsgObjValueType(u8),
    FmsgObjValueData(Vec<GenlDevlinkAttrs>),
    HealthReporter(Vec<GenlDevlinkAttrs>),
    HealthReporterName(String),
    HealthReporterState(u8),
    HealthReporterErrCount(u64),
    HealthReporterRecoverCount(u64),
    HealthReporterDumpTs(u64),
    HealthReporterGracefulPeriod(u64),
    HealthReporterAucoRecover(u8),
    FlashUpdateFileName(String),
    FlashUpdateComponent(String),
    FlashUpdateStatusMsg(String),
    FlashUpdateStatusDone(u64),
    FlashUpdateStatusTotal(u64),
    PortPciPfNumber(u16),
    PortPciVfNumber(u16),
    Stats(Vec<GenlDevlinkAttrs>),
    TrapName(String),
    TrapAction(u8),
    TrapType(u8),
    TrapGeneric(bool),
    TrapMetadata(Vec<GenlDevlinkAttrs>),
    TrapGroupName(String),
    ReloadStatus(u8),
    HealthReporterDumpTsNs(u64),
    NetnsFd(u32),
    NetnsPid(u32),
    NetnsId(u32),
    HealthReporterAutoDump(u8),
    TrapPolicerId(u32),
    TrapPolicerRate(u64),
    TrapPolicerBurst(u64),
    PortFunction(Vec<GenlDevlinkAttrs>),
    InfoBoardSerialNumber(String),
    PortLanes(u32),
    PortSplittable(u8),
    PortExternal(u8),
    PortControllerNo(u32),
    FlashUpdateStatusTimeout(u32),
    FlashUpdateOverWriteMask(u32),
    ReloadAction(u8),
    ReloadActionPerformed(u32),
    ReloadLimits(u32),
    DevStats(Vec<GenlDevlinkAttrs>),
    ReloadStats(Vec<GenlDevlinkAttrs>),
    ReloadStatsEntry(Vec<GenlDevlinkAttrs>),
    ReloadStatsLimit(u8),
    ReloadStatsValue(u32),
    RemoteReloadStats(Vec<GenlDevlinkAttrs>),
    ReloadActionInfo(Vec<GenlDevlinkAttrs>),
    ReloadActionStats(Vec<GenlDevlinkAttrs>),
    PortPciSfNo(u32),
    RateType(u16),
    RateTxShare(u64),
    RateTxMax(u64),
    RateNodeName(String),
    RateParentNodeName(String),
    RegionMaxSnapshots(u32),
}

trait TypeInfo {
//    fn type_of(&self) -> &'static str;
    fn buf_len(&self) -> usize;
}

// impl<T> TypeInfo for T {
//     fn type_of(&self) -> &'static str {
//         std::any::type_name::<T>()
//     }
//     fn buffer_len(&self) -> usize {
//         size_of_val(self)
//     }
// }

impl TypeInfo for Vec<GenlDevlinkAttrs> {
    // fn type_of(&self) -> &'static str {
    //     "Vec"
    // }
    fn buf_len(&self) -> usize {
        self.iter().map(|nla| nla.buffer_len()).sum()
    }
}
impl TypeInfo for Vec<u8> {
    // fn type_of(&self) -> &'static str {
    //     "Vec"
    // }
    fn buf_len(&self) -> usize {
        self.len()
    }
}
impl TypeInfo for u32 {
    // fn type_of(&self) -> &'static str {
    //     "u32"
    // }
    fn buf_len(&self) -> usize {
        size_of_val(self)
    }
}

impl TypeInfo for u64 {
    // fn type_of(&self) -> &'static str {
    //     "u64"
    // }
    fn buf_len(&self) -> usize {
        size_of_val(self)
    }
}

impl TypeInfo for u16 {
    // fn type_of(&self) -> &'static str {
    //     "u16"
    // }
    fn buf_len(&self) -> usize {
        size_of_val(self)
    }
}

impl TypeInfo for u8 {
    // fn type_of(&self) -> &'static str {
    //     "u8"
    // }
    fn buf_len(&self) -> usize {
        size_of_val(self)
    }
}

impl TypeInfo for bool {
    // fn type_of(&self) -> &'static str {
    //     "bool"
    // }
    fn buf_len(&self) -> usize {
        size_of_val(self)
    }
}

impl TypeInfo for String {
    // fn type_of(&self) -> &'static str {
    //     "String"
    // }
    fn buf_len(&self) -> usize {
        self.len() + 1
    }
}

// sprawdzic czy da sie zastosowac iteratior po wariantach (zdaje sie i tak bedzie trzeba rozdzielic obliczanie na typy zmiennych,
// tak zeby na etapie kompilacji typ zmiennej byl zedfiniowany). Innym rozwiazaniem jest zastosowanie makra

// sprawdzic czy mozna rozwiazac to po typie a nie listowac calosci
// lub posortowac calosc
impl Nla for GenlDevlinkAttrs {
    fn value_len(&self) -> usize {
        use GenlDevlinkAttrs::*;
        match self {
            ParamType(v)
            | ReloadStatus(v)
            | HealthReporterAutoDump(v)
            | ReloadAction(v)
            | ReloadStatsLimit(v)
            | SbPoolType(v)
            | EswitchInlineMode(v)
            | DpipeHeaderGlobal(v)
            | SbPoolThresholdType(v)
            | DpipeTableCountersEnabled(v)
            | EswitchEncapMode(v)
            | ResourceSizeValid(v)
            | ResourceUnit(v)
            | PortSplittable(v)
            | PortExternal(v)
            | FmsgObjValueType(v)
            | HealthReporterState(v)
            | HealthReporterAucoRecover(v)
            | TrapAction(v)
            | ParamValueCmode(v)
            | TrapType(v) => v.buf_len(),

            PortType(v)
            | PortFlavour(v)
            | DesiredType(v)
            | SbIngressPoolCount(v)
            | SbEgressPoolCount(v)
            | SbIngressTcCount(v)
            | SbEgressTcCount(v)
            | SbPoolIndex(v)
            | SbTcIndex(v)
            | EswitchMode(v)
            | RateType(v)
            | PortPciPfNumber(v)
            | PortPciVfNumber(v) => v.buf_len(),

            PortIndex(v)
            | PortNumber(v)
            | NetdevIndex(v)
            | ReloadStatsValue(v)
            | RegionMaxSnapshots(v)
            | PortSplitCount(v)
            | PortSplitGroup(v)
            | SbIndex(v)
            | SbSize(v)
            | RegionSnapshotId(v)
            | SbPoolSize(v)
            | SbPoolThreshold(v)
            | SbOccCur(v)
            | SbOccMax(v)
            | DpipeMatchType(v)
            | DpipeActionType(v)
            | DpipeValue(v)
            | DpipeValueMask(v)
            | DpipeValueMapping(v)
            | DpipeHeaderId(v)
            | DpipeHeaderIndex(v)
            | DpipeFieldId(v)
            | DpipeFieldBitwidth(v)
            | DpipeFieldMappingType(v)
            | PortLanes(v)
            | PortControllerNo(v)
            | FlashUpdateStatusTimeout(v)
            | FlashUpdateOverWriteMask(v)
            | ReloadActionPerformed(v)
            | ReloadLimits(v)
            | PortPciSfNo(v)
            | SbPoolCellSize(v)
            | NetnsFd(v)
            | NetnsPid(v)
            | NetnsId(v)
            | TrapPolicerId(v) => v.buf_len(),

            RegionSize(v)
            | ParamValue(v)
            | RegionChunkOffset(v)
            | RegionChunkSize(v)
            | DpipeTableSize(v)
            | DpipeEntryIndex(v)
            | DpipeEntryCounter(v)
            | ResourceId(v)
            | ResourceSize(v)
            | ResourceSizeNew(v)
            | ResourceSizeMin(v)
            | ResourceSizeMax(v)
            | ResourceSizeGran(v)
            | ResourceOcc(v)
            | DpipeTableResourceId(v)
            | DpipeTableResourceUnit(v)
            | RateTxShare(v)
            | RateTxMax(v)
            | HealthReporterErrCount(v)
            | HealthReporterRecoverCount(v)
            | HealthReporterDumpTs(v)
            | HealthReporterGracefulPeriod(v)
            | FlashUpdateStatusDone(v)
            | FlashUpdateStatusTotal(v)
            | HealthReporterDumpTsNs(v)
            | TrapPolicerRate(v)
            | TrapPolicerBurst(v) => v.buf_len(),

            PortIbdevName(s)
            | RegionName(s)
            | InfoDriverName(s)
            | InfoSerialNo(s)
            | InfoVersionName(s)
            | InfoVersionValue(s)
            | FlashUpdateFileName(s)
            | InfoBoardSerialNumber(s)
            | ResoureceName(s)
            | DpipeTableName(s)
            | DpipeFieldName(s)
            | BusName(s)
            | Location(s)
            | NetdevName(s)
            | TrapName(s)
            | TrapGroupName(s)
            | HealthReporterName(s)
            | RateNodeName(s)
            | DpipeHeaderName(s)
            | RateParentNodeName(s)
            | FmsgObjName(s)
            | FlashUpdateComponent(s)
            | FlashUpdateStatusMsg(s)
            | ParamName(s) => s.buf_len(),

            ParamValueList(v)
            | ParamValueData(v)
            | RegionSnapshots(v)
            | RegionSnapshot(v)
            | RegionChunks(v)
            | RegionChunk(v)
            | RemoteReloadStats(v)
            | ReloadActionInfo(v)
            | ReloadActionStats(v)
            | DevStats(v)
            | ReloadStats(v)
            | ReloadStatsEntry(v)
            | InfoVersionFixed(v)
            | InfoVersionRunning(v)
            | InfoVersionStored(v)
            | DpipeTables(v)
            | DpipeTable(v)
            | DpipeTableMatches(v)
            | DpipeTableActions(v)
            | DpipeEntries(v)
            | DpipeEntry(v)
            | DpipeEntryMatchValues(v)
            | DpipeEntryActionValues(v)
            | DpipeMatch(v)
            | DpipeMatchValue(v)
            | DpipeAction(v)
            | DpipeActionValue(v)
            | DpipeHeaders(v)
            | DpipeHader(v)
            | DpipeHeaderFields(v)
            | DpipeField(v)
            | ResourceList(v)
            | Resource(v)
            | PortFunction(v)
            | Fmsg(v)
            | FmsgObjValueData(v)
            | HealthReporter(v)
            | Stats(v)
            | TrapMetadata(v)
            | Param(v) => v.buf_len(),

            RegionChunkData(nla) => nla.len(),

            ParamGeneric(v)
            | FmsgObjNestStart(v)
            | FmsgPairNestStart(v)
            | FmsgArrNestStart(v)
            | FmsgNestEnd(v)
            | TrapGeneric(v) => v.buf_len(),
        }
    }

    fn is_nested(&self) -> bool {
        use GenlDevlinkAttrs::*;
        match self {
            Param(_)
            | ParamValueList(_)
            | ParamValue(_)
            | RegionSnapshots(_)
            | RegionSnapshot(_)
            | RegionChunks(_)
            | RegionChunk(_)
            | InfoVersionFixed(_)
            | InfoVersionRunning(_)
            | InfoVersionStored(_)
            | DevStats(_)
            | ReloadStats(_)
            | ReloadStatsEntry(_)
            | RemoteReloadStats(_)
            | ReloadActionInfo(_)
            | ReloadActionStats(_)
            | Stats(_)
            | TrapMetadata(_)
            | HealthReporter(_)
            | Fmsg(_)
            | PortFunction(_)
            | DpipeTables(_)
            | DpipeTable(_)
            | DpipeTableMatches(_)
            | DpipeTableActions(_)
            | DpipeEntries(_)
            | DpipeEntry(_)
            | DpipeEntryMatchValues(_)
            | DpipeEntryActionValues(_)
            | DpipeMatch(_)
            | DpipeMatchValue(_)
            | DpipeAction(_)
            | DpipeActionValue(_)
            | DpipeHeaders(_)
            | DpipeHader(_)
            | DpipeHeaderFields(_)
            | DpipeField(_)
            | ResourceList(_)
            | Resource(_) => true,
            _ => false,
        }
    }

    fn kind(&self) -> u16 {
        use GenlDevlinkAttrs::*;
        match self {
            BusName(_) => DEVLINK_ATTR_BUS_NAME,
            Location(_) => DEVLINK_ATTR_LOCATION,
            PortIndex(_) => DEVLINK_ATTR_PORT_INDEX,
            PortType(_) => DEVLINK_ATTR_PORT_TYPE,
            DesiredType(_) => DEVLINK_ATTR_DESIRED_TYPE,
            NetdevIndex(_) => DEVLINK_ATTR_NETDEV_IF_INDEX,
            NetdevName(_) => DEVLINK_ATTR_NETDEV_NAME,
            PortFlavour(_) => DEVLINK_ATTR_PORT_FLAVOUR,
            PortNumber(_) => DEVLINK_ATTR_PORT_NUMBER,
            Param(_) => DEVLINK_ATTR_PARAM,
            ParamName(_) => DEVLINK_ATTR_PARAM_NAME,
            ParamGeneric(_) => DEVLINK_ATTR_PARAM_GENERIC,
            ParamType(_) => DEVLINK_ATTR_PARAM_TYPE,
            ParamValueList(_) => DEVLINK_ATTR_PARAM_VALUES_LIST,
            ParamValue(_) => DEVLINK_ATTR_PARAM_VALUE,
            ParamValueData(_) => DEVLINK_ATTR_PARAM_VALUE_DATA,
            ParamValueCmode(_) => DEVLINK_ATTR_PARAM_VALUE_CMODE,
            RegionName(_) => DEVLINK_ATTR_REGION_NAME,
            RegionSize(_) => DEVLINK_ATTR_REGION_SIZE,
            RegionSnapshots(_) => DEVLINK_ATTR_REGION_SNAPSHOTS,
            RegionSnapshot(_) => DEVLINK_ATTR_REGION_SNAPSHOT,
            RegionSnapshotId(_) => DEVLINK_ATTR_REGION_SNAPSHOT_ID,
            RegionChunks(_) => DEVLINK_ATTR_REGION_CHUNKS,
            RegionChunk(_) => DEVLINK_ATTR_REGION_CHUNK,
            RegionChunkData(_) => DEVLINK_ATTR_REGION_CHUNK_DATA,
            RegionChunkOffset(_) => DEVLINK_ATTR_REGION_CHUNK_ADDR,
            RegionChunkSize(_) => DEVLINK_ATTR_REGION_SIZE,
            InfoDriverName(_) => DEVLINK_ATTR_INFO_DRIVER_NAME,
            InfoSerialNo(_) => DEVLINK_ATTR_INFO_SERIAL_NUMBER,
            InfoVersionFixed(_) => DEVLINK_ATTR_INFO_VERSION_FIXED,
            InfoVersionRunning(_) => DEVLINK_ATTR_INFO_VERSION_RUNNING,
            InfoVersionStored(_) => DEVLINK_ATTR_INFO_VERSION_STORED,
            InfoVersionName(_) => DEVLINK_ATTR_INFO_VERSION_NAME,
            InfoVersionValue(_) => DEVLINK_ATTR_INFO_VERSION_VALUE,
            FlashUpdateFileName(_) => DEVLINK_ATTR_FLASH_UPDATE_FILE_NAME,
            ReloadStatus(_) => DEVLINK_ATTR_RELOAD_FAILED,
            ReloadAction(_) => DEVLINK_ATTR_RELOAD_ACTION,
            DevStats(_) => DEVLINK_ATTR_DEV_STATS,
            ReloadStats(_) => DEVLINK_ATTR_RELOAD_STATS,
            ReloadStatsEntry(_) => DEVLINK_ATTR_RELOAD_STATS_ENTRY,
            ReloadStatsLimit(_) => DEVLINK_ATTR_RELOAD_STATS_LIMIT,
            ReloadStatsValue(_) => DEVLINK_ATTR_RELOAD_STATS_VALUE,
            RemoteReloadStats(_) => DEVLINK_ATTR_REMOTE_RELOAD_SATS,
            ReloadActionInfo(_) => DEVLINK_ATTR_RELOAD_ACTION_INFO,
            ReloadActionStats(_) => DEVLINK_ATTR_RELAOD_ACTION_STATS,
            RegionMaxSnapshots(_) => DEVLINK_ATTR_REGION_MAX_SNAPSHOTS,
            PortPciPfNumber(_) => DEVLINK_ATTR_PORT_PCI_PF_NUMBER,
            PortPciVfNumber(_) => DEVLINK_ATTR_PORT_PCI_VF_NUMBER,
            Stats(_) => DEVLINK_ATTR_STATS,
            TrapName(_) => DEVLINK_ATTR_TRAP_NAME,
            TrapAction(_) => DEVLINK_ATTR_TRAP_ACTION,
            TrapType(_) => DEVLINK_ATTR_TRAP_TYPE,
            TrapGeneric(_) => DEVLINK_ATTR_TRAP_GENERIC,
            TrapMetadata(_) => DEVLINK_ATTR_TRAP_METADATA,
            TrapGroupName(_) => DEVLINK_ATTR_TRAP_GROUP_NAME,
            HealthReporter(_) => DEVLINK_ATTR_HEALTH_REPORTER,
            HealthReporterName(_) => DEVLINK_ATTR_HEALTH_REPORTER_NAME,
            HealthReporterState(_) => DEVLINK_ATTR_HEALTH_REPORTER_STATE,
            HealthReporterErrCount(_) => DEVLINK_ATTR_HEALTH_REPORTER_ERR_COUNT,
            HealthReporterRecoverCount(_) => {
                DEVLINK_ATTR_HEALTH_REPORTER_RECOVER_COUNT
            }
            HealthReporterDumpTs(_) => DEVLINK_ATTR_HEALTH_REPORTER_DUMP_TS,
            HealthReporterGracefulPeriod(_) => {
                DEVLINK_ATTR_HEALTH_REPORTER_GRACEFUL_PERIOD
            }
            HealthReporterAucoRecover(_) => {
                DEVLINK_ATTR_HEALTH_REPORTER_AUTO_RECOVER
            }
            FlashUpdateComponent(_) => DEVLINK_ATTR_FLASH_UPDATE_COMPONENT,
            FlashUpdateStatusMsg(_) => DEVLINK_ATTR_FLASH_UPDATE_STATUS_MSG,
            FlashUpdateStatusDone(_) => DEVLINK_ATTR_FLASH_UPDATE_STATUS_DONE,
            FlashUpdateStatusTotal(_) => DEVLINK_ATTR_FLASH_UPDATE_STATUS_TOTAL,
            SbPoolCellSize(_) => DEVLINK_ATTR_SB_POOL_CELL_SIZE,
            Fmsg(_) => DEVLINK_ATTR_FMSG,
            FmsgObjNestStart(_) => DEVLINK_ATTR_FMSG_OBJ_NEST_START,
            FmsgPairNestStart(_) => DEVLINK_ATTR_FMSG_PAIR_NEST_START,
            FmsgArrNestStart(_) => DEVLINK_ATTR_FMSG_ARR_NEST_START,
            FmsgNestEnd(_) => DEVLINK_ATTR_FMSG_NEST_END,
            FmsgObjName(_) => DEVLINK_ATTR_FMSG_OBJ_NAME,
            FmsgObjValueType(_) => DEVLINK_ATTR_FMSG_OBJ_VALUE_TYPE,
            FmsgObjValueData(_) => DEVLINK_ATTR_FMSG_OBJ_VALUE_DATA,
            HealthReporterDumpTsNs(_) => {
                DEVLINK_ATTR_HEALTH_REPORTER_DUMP_TS_NS
            }
            NetnsFd(_) => DEVLINK_ATTR_NETNS_FD,
            NetnsPid(_) => DEVLINK_ATTR_NETNS_PID,
            NetnsId(_) => DEVLINK_ATTR_NETNS_ID,
            HealthReporterAutoDump(_) => DEVLINK_ATTR_HEALTH_REPORTER_AUTO_DUMP,
            TrapPolicerId(_) => DEVLINK_ATTR_TRAP_POLICER_ID,
            TrapPolicerRate(_) => DEVLINK_ATTR_TRAP_POLICER_RATE,
            TrapPolicerBurst(_) => DEVLINK_ATTR_TRAP_POLICER_BURST,
            PortFunction(_) => DEVLINK_ATTR_PORT_FUNCTION,
            InfoBoardSerialNumber(_) => DEVLINK_ATTR_INFO_BOARD_SERIAL_NUMBER,
            PortLanes(_) => DEVLINK_ATTR_PORT_LANES,
            PortSplittable(_) => DEVLINK_ATTR_PORT_SPLITTABLE,
            PortExternal(_) => DEVLINK_ATTR_PORT_EXTERNAL,
            PortControllerNo(_) => DEVLINK_ATTR_PORT_CONTROLLER_NUMBER,
            FlashUpdateStatusTimeout(_) => {
                DEVLINK_ATTR_FLASH_UPDATE_STATUS_TIMEOUT
            }
            FlashUpdateOverWriteMask(_) => {
                DEVLINK_ATTR_FLASH_UPDATE_OVERWRITE_MASK
            }
            ReloadActionPerformed(_) => DEVLINK_ATTR_RELOAD_ACTIONS_PERFORMED,
            ReloadLimits(_) => DEVLINK_ATTR_RELOAD_LIMITS,
            PortPciSfNo(_) => DEVLINK_ATTR_PORT_PCI_SF_NUMBER,
            RateType(_) => DEVLINK_ATTR_RATE_TYPE,
            RateTxShare(_) => DEVLINK_ATTR_RATE_TX_SHARE,
            RateTxMax(_) => DEVLINK_ATTR_RATE_TX_MAX,
            RateNodeName(_) => DEVLINK_ATTR_RATE_NODE_NAME,
            RateParentNodeName(_) => DEVLINK_ATTR_RATE_PARENT_NODE_NAME,
            PortSplitCount(_) => DEVLINK_ATTR_PORT_SPLIT_COUNT,
            PortSplitGroup(_) => DEVLINK_ATTR_PORT_SPLIT_GROUP,
            SbIndex(_) => DEVLINK_ATTR_SB_INDEX,
            SbSize(_) => DEVLINK_ATTR_SB_SIZE,
            SbIngressPoolCount(_) => DEVLINK_ATTR_SB_INGRESS_POOL_COUNT,
            SbEgressPoolCount(_) => DEVLINK_ATTR_SB_EGRESS_POOL_COUNT,
            SbIngressTcCount(_) => DEVLINK_ATTR_SB_INGRESS_TC_COUNT,
            SbEgressTcCount(_) => DEVLINK_ATTR_SB_EGRESS_TC_COUNT,
            SbPoolIndex(_) => DEVLINK_ATTR_SB_POOL_INDEX,
            SbPoolType(_) => DEVLINK_ATTR_SB_POOL_TYPE,
            SbPoolSize(_) => DEVLINK_ATTR_SB_POOL_SIZE,
            SbPoolThresholdType(_) => DEVLINK_ATTR_SB_POOL_THRESHOLD_TYPE,
            SbPoolThreshold(_) => DEVLINK_ATTR_SB_THRESHOLD,
            SbTcIndex(_) => DEVLINK_ATTR_SB_TC_INDEX,
            SbOccCur(_) => DEVLINK_ATTR_SB_OCC_CUR,
            SbOccMax(_) => DEVLINK_ATTR_SB_OCC_MAX,
            EswitchMode(_) => DEVLINK_ATTR_ESWITCH_MODE,
            EswitchInlineMode(_) => DEVLINK_ATTR_ESWITCH_INLINE_MODE,
            DpipeTables(_) => DEVLINK_ATTR_DPIPE_TABLES,
            DpipeTable(_) => DEVLINK_ATTR_DPIPE_TABLE,
            DpipeTableName(_) => DEVLINK_ATTR_DPIPE_TABLE_NAME,
            DpipeTableSize(_) => DEVLINK_ATTR_DPIPE_TABLE_SIZE,
            DpipeTableMatches(_) => DEVLINK_ATTR_DPIPE_TABLE_MATCHES,
            DpipeTableActions(_) => DEVLINK_ATTR_DPIPE_TABLE_ACTIONS,
            DpipeTableCountersEnabled(_) => {
                DEVLINK_ATTR_DPIPE_TABLE_COUNTERS_ENABLED
            }
            DpipeEntries(_) => DEVLINK_ATTR_DPIPE_ENTRIES,
            DpipeEntry(_) => DEVLINK_ATTR_DPIPE_ENTRY,
            DpipeEntryIndex(_) => DEVLINK_ATTR_DPIPE_ENTRY_INDEX,
            DpipeEntryMatchValues(_) => DEVLINK_ATTR_DPIPE_ENTRY_MATCH_VALUES,
            DpipeEntryActionValues(_) => DEVLINK_ATTR_DPIPE_ENTRY_ACTION_VALUES,
            DpipeEntryCounter(_) => DEVLINK_ATTR_DPIPE_ENTRY_COUNTER,
            DpipeMatch(_) => DEVLINK_ATTR_DPIPE_MATCH,
            DpipeMatchValue(_) => DEVLINK_ATTR_DPIPE_MATCH_VALUE,
            DpipeMatchType(_) => DEVLINK_ATTR_DPIPE_MATCH_TYPE,
            DpipeAction(_) => DEVLINK_ATTR_DPIPE_ACTION,
            DpipeActionValue(_) => DEVLINK_ATTR_DPIPE_ACTION_VALUE,
            DpipeActionType(_) => DEVLINK_ATTR_DPIPE_ACTION_TYPE,
            DpipeValue(_) => DEVLINK_ATTR_DPIPE_VALUE,
            DpipeValueMask(_) => DEVLINK_ATTR_DPIPE_VALUE_MASK,
            DpipeValueMapping(_) => DEVLINK_ATTR_DPIPE_VALUE_MAPPING,
            DpipeHeaders(_) => DEVLINK_ATTR_DPIPE_HEADERS,
            DpipeHader(_) => DEVLINK_ATTR_DPIPE_HEADER,
            DpipeHeaderName(_) => DEVLINK_ATTR_DPIPE_HEADER_NAME,
            DpipeHeaderId(_) => DEVLINK_ATTR_DPIPE_HEADER_ID,
            DpipeHeaderFields(_) => DEVLINK_ATTR_DPIPE_HEADER_FIELDS,
            DpipeHeaderGlobal(_) => DEVLINK_ATTR_DPIPE_HEADER_GLOBAL,
            DpipeHeaderIndex(_) => DEVLINK_ATTR_DPIPE_HEADER_INDEX,
            DpipeField(_) => DEVLINK_ATTR_DPIPE_FIELD,
            DpipeFieldName(_) => DEVLINK_ATTR_DPIPE_FIELD_NAME,
            DpipeFieldId(_) => DEVLINK_ATTR_DPIPE_FIELD_ID,
            DpipeFieldBitwidth(_) => DEVLINK_ATTR_DPIPE_FIELD_BITWIDTH,
            DpipeFieldMappingType(_) => DEVLINK_ATTR_DPIPE_FIELD_MAPPING_TYPE,
            EswitchEncapMode(_) => DEVLINK_ATTR_ESWITCH_ENCAP_MODE,
            ResourceList(_) => DEVLINK_ATTR_RESOURCE_LIST,
            Resource(_) => DEVLINK_ATTR_RESOURCE,
            ResoureceName(_) => DEVLINK_ATTR_RESOURCE_NAME,
            ResourceId(_) => DEVLINK_ATTR_RESOURCE_ID,
            ResourceSize(_) => DEVLINK_ATTR_RESOURCE_SIZE,
            ResourceSizeNew(_) => DEVLINK_ATTR_RESOURCE_SIZE_NEW,
            ResourceSizeValid(_) => DEVLINK_ATTR_RESOURCE_SIZE_VALID,
            ResourceSizeMin(_) => DEVLINK_ATTR_RESOURCE_SIZE_MIN,
            ResourceSizeMax(_) => DEVLINK_ATTR_RESOURCE_SIZE_MAX,
            ResourceSizeGran(_) => DEVLINK_ATTR_RESOURCE_SIZE_GRAN,
            ResourceUnit(_) => DEVLINK_ATTR_RESOURCE_UNIT,
            ResourceOcc(_) => DEVLINK_ATTR_RESOURCE_OCC,
            DpipeTableResourceId(_) => DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_ID,
            DpipeTableResourceUnit(_) => {
                DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_UNITS
            }
            PortIbdevName(_) => DEVLINK_ATTR_PORT_IBDEV_NAME,
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use GenlDevlinkAttrs::*;
        match self {
            BusName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            Location(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            PortIndex(v) => NativeEndian::write_u32(buffer, *v),
            PortType(v) => NativeEndian::write_u16(buffer, *v),
            DesiredType(v) => NativeEndian::write_u16(buffer, *v),
            NetdevIndex(v) => NativeEndian::write_u32(buffer, *v),
            NetdevName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            PortFlavour(v) => NativeEndian::write_u16(buffer, *v),
            PortNumber(v) => NativeEndian::write_u32(buffer, *v),
            Param(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            ParamName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            ParamGeneric(v) => buffer[0] = *v as u8,
            ParamType(v) => buffer[0] = *v,
            ParamValueList(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            ParamValue(v) => NativeEndian::write_u64(buffer, *v),
            ParamValueData(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            ParamValueCmode(v) => buffer[0] = *v,
            RegionName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            RegionSize(v) => NativeEndian::write_u64(buffer, *v),
            RegionSnapshots(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            RegionSnapshot(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            RegionSnapshotId(v) => NativeEndian::write_u32(buffer, *v),
            RegionChunks(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            RegionChunk(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            RegionChunkData(s) => {
                buffer[..s.len()].copy_from_slice(s);
            }
            RegionChunkOffset(v) => NativeEndian::write_u64(buffer, *v),
            RegionChunkSize(v) => NativeEndian::write_u64(buffer, *v),
            InfoDriverName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            InfoSerialNo(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            InfoVersionFixed(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            InfoVersionRunning(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            InfoVersionStored(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            InfoVersionName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            InfoVersionValue(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            FlashUpdateFileName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            ReloadStatus(v) => buffer[0] = *v,
            ReloadAction(v) => buffer[0] = *v,
            DevStats(v) => {
                v.iter().for_each(|val| val.emit_value(buffer));
            }
            ReloadStats(v) => {
                v.iter().for_each(|val| val.emit_value(buffer));
            }
            ReloadStatsEntry(v) => {
                v.iter().for_each(|val| val.emit_value(buffer));
            }
            ReloadStatsLimit(v) => buffer[0] = *v,
            ReloadStatsValue(v) => NativeEndian::write_u32(buffer, *v),
            RemoteReloadStats(v) => {
                v.iter().for_each(|val| val.emit_value(buffer));
            }
            ReloadActionInfo(v) => {
                v.iter().for_each(|val| val.emit_value(buffer));
            }
            ReloadActionStats(v) => {
                v.iter().for_each(|val| val.emit_value(buffer));
            }
            RegionMaxSnapshots(v) => NativeEndian::write_u32(buffer, *v),
            PortPciPfNumber(v) => NativeEndian::write_u16(buffer, *v),
            PortPciVfNumber(v) => NativeEndian::write_u16(buffer, *v),
            Stats(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            TrapName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            TrapAction(v) => buffer[0] = *v,
            TrapType(v) => buffer[0] = *v,
            TrapGeneric(v) => buffer[0] = *v as u8,
            TrapMetadata(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            TrapGroupName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            HealthReporter(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            HealthReporterName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            HealthReporterState(v) => buffer[0] = *v,
            HealthReporterErrCount(v) => NativeEndian::write_u64(buffer, *v),
            HealthReporterRecoverCount(v) => {
                NativeEndian::write_u64(buffer, *v)
            }
            HealthReporterDumpTs(v) => NativeEndian::write_u64(buffer, *v),
            HealthReporterGracefulPeriod(v) => {
                NativeEndian::write_u64(buffer, *v)
            }
            HealthReporterAucoRecover(v) => buffer[0] = *v,
            FlashUpdateComponent(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            FlashUpdateStatusMsg(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            FlashUpdateStatusDone(v) => NativeEndian::write_u64(buffer, *v),
            FlashUpdateStatusTotal(v) => NativeEndian::write_u64(buffer, *v),
            SbPoolCellSize(v) => NativeEndian::write_u32(buffer, *v),
            Fmsg(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            FmsgObjNestStart(v) => buffer[0] = *v as u8,
            FmsgPairNestStart(v) => buffer[0] = *v as u8,
            FmsgArrNestStart(v) => buffer[0] = *v as u8,
            FmsgNestEnd(v) => buffer[0] = *v as u8,
            FmsgObjName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            FmsgObjValueType(v) => buffer[0] = *v,
            FmsgObjValueData(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            HealthReporterDumpTsNs(v) => NativeEndian::write_u64(buffer, *v),
            NetnsFd(v) => NativeEndian::write_u32(buffer, *v),
            NetnsPid(v) => NativeEndian::write_u32(buffer, *v),
            NetnsId(v) => NativeEndian::write_u32(buffer, *v),
            HealthReporterAutoDump(v) => buffer[0] = *v,
            TrapPolicerId(v) => NativeEndian::write_u32(buffer, *v),
            TrapPolicerRate(v) => NativeEndian::write_u64(buffer, *v),
            TrapPolicerBurst(v) => NativeEndian::write_u64(buffer, *v),
            PortFunction(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            InfoBoardSerialNumber(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            PortLanes(v) => NativeEndian::write_u32(buffer, *v),
            PortSplittable(v) => buffer[0] = *v,
            PortExternal(v) => buffer[0] = *v,
            PortControllerNo(v) => NativeEndian::write_u32(buffer, *v),
            FlashUpdateStatusTimeout(v) => NativeEndian::write_u32(buffer, *v),
            FlashUpdateOverWriteMask(v) => NativeEndian::write_u32(buffer, *v),
            ReloadActionPerformed(v) => NativeEndian::write_u32(buffer, *v),
            ReloadLimits(v) => NativeEndian::write_u32(buffer, *v),
            PortPciSfNo(v) => NativeEndian::write_u32(buffer, *v),
            RateType(v) => NativeEndian::write_u16(buffer, *v),
            RateTxShare(v) => NativeEndian::write_u64(buffer, *v),
            RateTxMax(v) => NativeEndian::write_u64(buffer, *v),
            RateNodeName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            RateParentNodeName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            PortIbdevName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            PortSplitCount(v) => NativeEndian::write_u32(buffer, *v),
            PortSplitGroup(v) => NativeEndian::write_u32(buffer, *v),
            SbIndex(v) => NativeEndian::write_u32(buffer, *v),
            SbSize(v) => NativeEndian::write_u32(buffer, *v),
            SbIngressPoolCount(v) => NativeEndian::write_u16(buffer, *v),
            SbEgressPoolCount(v) => NativeEndian::write_u16(buffer, *v),
            SbIngressTcCount(v) => NativeEndian::write_u16(buffer, *v),
            SbEgressTcCount(v) => NativeEndian::write_u16(buffer, *v),
            SbPoolIndex(v) => NativeEndian::write_u16(buffer, *v),
            SbPoolType(v) => buffer[0] = *v,
            SbPoolSize(v) => NativeEndian::write_u32(buffer, *v),
            SbPoolThresholdType(v) => buffer[0] = *v,
            SbPoolThreshold(v) => NativeEndian::write_u32(buffer, *v),
            SbTcIndex(v) => NativeEndian::write_u16(buffer, *v),
            SbOccCur(v) => NativeEndian::write_u32(buffer, *v),
            SbOccMax(v) => NativeEndian::write_u32(buffer, *v),
            EswitchMode(v) => NativeEndian::write_u16(buffer, *v),
            EswitchInlineMode(v) => buffer[0] = *v,
            DpipeTables(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeTable(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeTableName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            DpipeTableSize(v) => NativeEndian::write_u64(buffer, *v),
            DpipeTableMatches(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeTableActions(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeTableCountersEnabled(v) => buffer[0] = *v,
            DpipeEntries(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeEntry(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeEntryIndex(v) => NativeEndian::write_u64(buffer, *v),
            DpipeEntryMatchValues(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeEntryActionValues(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeEntryCounter(v) => NativeEndian::write_u64(buffer, *v),
            DpipeMatch(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeMatchValue(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeMatchType(v) => NativeEndian::write_u32(buffer, *v),
            DpipeAction(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeActionValue(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeActionType(v) => NativeEndian::write_u32(buffer, *v),
            DpipeValue(v) => NativeEndian::write_u32(buffer, *v),
            DpipeValueMask(v) => NativeEndian::write_u32(buffer, *v),
            DpipeValueMapping(v) => NativeEndian::write_u32(buffer, *v),
            DpipeHeaders(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeHader(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeHeaderName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            DpipeHeaderId(v) => NativeEndian::write_u32(buffer, *v),
            DpipeHeaderFields(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeHeaderGlobal(v) => buffer[0] = *v,
            DpipeHeaderIndex(v) => NativeEndian::write_u32(buffer, *v),
            DpipeField(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            DpipeFieldName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            DpipeFieldId(v) => NativeEndian::write_u32(buffer, *v),
            DpipeFieldBitwidth(v) => NativeEndian::write_u32(buffer, *v),
            DpipeFieldMappingType(v) => NativeEndian::write_u32(buffer, *v),
            EswitchEncapMode(v) => buffer[0] = *v,
            ResourceList(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            Resource(nla) => {
                nla.iter().for_each(|val| val.emit_value(buffer));
            }
            ResoureceName(s) => {
                buffer[..s.len()].copy_from_slice(s.as_bytes());
                buffer[s.len()] = 0;
            }
            ResourceId(v) => NativeEndian::write_u64(buffer, *v),
            ResourceSize(v) => NativeEndian::write_u64(buffer, *v),
            ResourceSizeNew(v) => NativeEndian::write_u64(buffer, *v),
            ResourceSizeValid(v) => buffer[0] = *v,
            ResourceSizeMin(v) => NativeEndian::write_u64(buffer, *v),
            ResourceSizeMax(v) => NativeEndian::write_u64(buffer, *v),
            ResourceSizeGran(v) => NativeEndian::write_u64(buffer, *v),
            ResourceUnit(v) => buffer[0] = *v,
            ResourceOcc(v) => NativeEndian::write_u64(buffer, *v),
            DpipeTableResourceId(v) => NativeEndian::write_u64(buffer, *v),
            DpipeTableResourceUnit(v) => NativeEndian::write_u64(buffer, *v),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>>
    for GenlDevlinkAttrs
{
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            DEVLINK_ATTR_BUS_NAME => Self::BusName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_BUS_NAME value")?,
            ),
            DEVLINK_ATTR_LOCATION => Self::Location(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_LOCATION value")?,
            ),
            DEVLINK_ATTR_PORT_INDEX => Self::PortIndex(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_INDEX value")?,
            ),
            DEVLINK_ATTR_PORT_TYPE => Self::PortType(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_PORT_TYPE value")?,
            ),
            DEVLINK_ATTR_DESIRED_TYPE => Self::DesiredType(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_DESIRED_TYPE value")?,
            ),
            DEVLINK_ATTR_NETDEV_IF_INDEX => Self::NetdevIndex(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_NETDEV_IF_INDEX value")?,
            ),
            DEVLINK_ATTR_NETDEV_NAME => Self::NetdevName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_NETDEV_NAME value")?,
            ),
            DEVLINK_ATTR_PORT_FLAVOUR => Self::PortFlavour(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_PORT_FLAVOUR value")?,
            ),
            DEVLINK_ATTR_PORT_NUMBER => Self::PortNumber(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_NUMBER value")?,
            ),
            DEVLINK_ATTR_PARAM => Self::Param({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_PARAM")?;
                fixed
            }),
            DEVLINK_ATTR_PARAM_NAME => Self::ParamName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_PARAM_NAME value")?,
            ),
            DEVLINK_ATTR_PARAM_GENERIC => Self::ParamGeneric({
                let val = parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_PARAM_GENERIC value")?;
                if val == 0 {
                    false
                } else {
                    true
                }
            }),
            DEVLINK_ATTR_PARAM_TYPE => Self::ParamType(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_PARAM_TYPE value")?,
            ),
            DEVLINK_ATTR_PARAM_VALUES_LIST => Self::ParamValueList({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_PARAM_VALUE_LIST")?;
                fixed
            }),
            DEVLINK_ATTR_PARAM_VALUE => Self::ParamValue(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_PARAM_VALUE value")?,
            ),
            DEVLINK_ATTR_PARAM_VALUE_DATA => Self::ParamValueData({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_REGION_CHUNK")?;
                fixed
            }),
            DEVLINK_ATTR_PARAM_VALUE_CMODE => Self::ParamValueCmode(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_PARAM_VALUE_CMODE value")?,
            ),
            DEVLINK_ATTR_REGION_NAME => Self::RegionName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_REGION_NAME value")?,
            ),
            DEVLINK_ATTR_REGION_SIZE => Self::RegionSize(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_REGION_SIZE value")?,
            ),
            DEVLINK_ATTR_REGION_SNAPSHOTS => Self::RegionSnapshots({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_REGION_SNAPSHOTS")?;
                fixed
            }),
            DEVLINK_ATTR_REGION_SNAPSHOT => Self::RegionSnapshot({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_REGION_CHUNK")?;
                fixed
            }),
            DEVLINK_ATTR_REGION_SNAPSHOT_ID => Self::RegionSnapshotId(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_REGION_SNAPSHOT_ID value")?,
            ),
            DEVLINK_ATTR_REGION_CHUNKS => Self::RegionChunks({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_REGION_CHUNKS")?;
                fixed
            }),
            DEVLINK_ATTR_REGION_CHUNK => Self::RegionChunk({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_REGION_CHUNK")?;
                fixed
            }),
            // DEVLINK_ATTR_REGION_CHUNK_DATA => Self::RegionChunkData(_),
            DEVLINK_ATTR_REGION_CHUNK_ADDR => Self::RegionChunkOffset(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_REGION_CHUNK_ADDR value")?,
            ),
            DEVLINK_ATTR_INFO_DRIVER_NAME => Self::InfoDriverName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_INFO_DRIVER_NAME value")?,
            ),
            DEVLINK_ATTR_INFO_SERIAL_NUMBER => Self::InfoSerialNo(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_INFO_SERIAL_NUMBER value")?,
            ),
            DEVLINK_ATTR_INFO_VERSION_FIXED => Self::InfoVersionFixed({
                let fixed = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_INFO_VERSION_FIXED",
                    )?;
                fixed
            }),
            DEVLINK_ATTR_INFO_VERSION_RUNNING => Self::InfoVersionRunning({
                let running = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_INFO_VERSION_RUNNING",
                    )?;
                running
            }),
            DEVLINK_ATTR_INFO_VERSION_STORED => Self::InfoVersionStored({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_INFO_VERSION_STORED",
                    )?;
                stored
            }),
            DEVLINK_ATTR_INFO_VERSION_NAME => Self::InfoVersionName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_INFO_VERSION_NAME value")?,
            ),
            DEVLINK_ATTR_INFO_VERSION_VALUE => Self::InfoVersionValue(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_INFO_VERSION_VALUE value")?,
            ),
            DEVLINK_ATTR_FLASH_UPDATE_FILE_NAME => {
                Self::FlashUpdateFileName(parse_string(payload).context(
                    "invalid DEVLINK_ATTR_FLASH_UPDATE_FILE_NAME value",
                )?)
            }
            DEVLINK_ATTR_RELOAD_FAILED => Self::ReloadStatus(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_RELOAD_FAILED value")?,
            ),
            DEVLINK_ATTR_RELOAD_ACTION => Self::ReloadAction(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_RELOAD_ACTION value")?,
            ),
            DEVLINK_ATTR_DEV_STATS => Self::DevStats({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_DEV_STATS",
                    )?;
                stored
            }),
            DEVLINK_ATTR_RELOAD_STATS => Self::ReloadStats({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_RELOAD_STATS",
                    )?;
                stored
            }),
            DEVLINK_ATTR_RELOAD_STATS_ENTRY => Self::ReloadStatsEntry({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_RELOAD_STATS_ENTRY",
                    )?;
                stored
            }),
            DEVLINK_ATTR_RELOAD_STATS_LIMIT => Self::ReloadStatsLimit(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_RELOAD_STATS_LIMIT value")?,
            ),
            DEVLINK_ATTR_RELOAD_STATS_VALUE => Self::ReloadStatsValue(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_RELOAD_STATS_VALUE value")?,
            ),
            DEVLINK_ATTR_REMOTE_RELOAD_SATS => Self::RemoteReloadStats({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_REMOTE_RELOAD_SATS",
                    )?;
                stored
            }),
            DEVLINK_ATTR_RELOAD_ACTION_INFO => Self::ReloadActionInfo({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_RELOAD_ACTION_INFO",
                    )?;
                stored
            }),
            DEVLINK_ATTR_RELAOD_ACTION_STATS => Self::ReloadActionStats({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_RELAOD_ACTION_STATS",
                    )?;
                stored
            }),
            DEVLINK_ATTR_REGION_MAX_SNAPSHOTS => Self::RegionMaxSnapshots(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_REGION_MAX_SNAPSHOTS value")?,
            ),
            DEVLINK_ATTR_PORT_PCI_PF_NUMBER => Self::PortPciPfNumber(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_PORT_PCI_PF_NUMBER value")?,
            ),
            DEVLINK_ATTR_PORT_PCI_VF_NUMBER => Self::PortPciVfNumber(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_PORT_PCI_VF_NUMBER value")?,
            ),
            DEVLINK_ATTR_STATS => Self::Stats({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_STATS")?;
                stored
            }),
            DEVLINK_ATTR_TRAP_NAME => Self::TrapName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_NAME value")?,
            ),
            DEVLINK_ATTR_TRAP_ACTION => Self::TrapAction(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_ACTION value")?,
            ),
            DEVLINK_ATTR_TRAP_TYPE => Self::TrapType(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_TYPE value")?,
            ),
            DEVLINK_ATTR_TRAP_GENERIC => Self::TrapGeneric({
                let val = parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_GENERIC value")?;
                if val == 0 {
                    false
                } else {
                    true
                }
            }),
            DEVLINK_ATTR_TRAP_METADATA => Self::TrapMetadata({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_TRAP_METADATA")?;
                stored
            }),
            DEVLINK_ATTR_TRAP_GROUP_NAME => Self::TrapGroupName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_GROUP_NAME value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER => Self::HealthReporter({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_HEALTH_REPORTER")?;
                stored
            }),
            DEVLINK_ATTR_HEALTH_REPORTER_NAME => Self::HealthReporterName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_NAME value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER_STATE => Self::HealthReporterState(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_STATE value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER_ERR_COUNT => Self::HealthReporterErrCount(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_ERR_COUNT value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER_RECOVER_COUNT => Self::HealthReporterRecoverCount(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_RECOVER_COUNT value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER_DUMP_TS => Self::HealthReporterDumpTs(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_DUMP_TS value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER_GRACEFUL_PERIOD => Self::HealthReporterGracefulPeriod(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_GRACEFUL_PERIOD value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER_AUTO_RECOVER => Self::HealthReporterAucoRecover(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_AUTO_RECOVER value")?,
            ),
            DEVLINK_ATTR_FLASH_UPDATE_COMPONENT => Self::FlashUpdateComponent(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_FLASH_UPDATE_COMPONENT value")?,
            ),
            DEVLINK_ATTR_FLASH_UPDATE_STATUS_MSG => Self::FlashUpdateStatusMsg(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_FLASH_UPDATE_STATUS_MSG value")?,
            ),
            DEVLINK_ATTR_FLASH_UPDATE_STATUS_DONE => Self::FlashUpdateStatusDone(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_FLASH_UPDATE_STATUS_DONE value")?,
            ),
            DEVLINK_ATTR_FLASH_UPDATE_STATUS_TOTAL => Self::FlashUpdateStatusTotal(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_FLASH_UPDATE_STATUS_TOTAL value")?,
            ),
            DEVLINK_ATTR_SB_POOL_CELL_SIZE => Self::SbPoolCellSize(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_SB_POOL_CELL_SIZE value")?,
            ),
            DEVLINK_ATTR_FMSG => Self::Fmsg({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_FMSG")?;
                stored
            }),
            DEVLINK_ATTR_FMSG_OBJ_NEST_START => Self::FmsgObjNestStart(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_FMSG_OBJ_NEST_START value")? != 0,
            ),
            DEVLINK_ATTR_FMSG_PAIR_NEST_START => Self::FmsgPairNestStart(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_FMSG_PAIR_NEST_START value")? != 0,
            ),
            DEVLINK_ATTR_FMSG_ARR_NEST_START => Self::FmsgArrNestStart(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_FMSG_ARR_NEST_START value")? != 0,
            ),
            DEVLINK_ATTR_FMSG_NEST_END => Self::FmsgNestEnd(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_FMSG_NEST_END value")? != 0,
            ),
            DEVLINK_ATTR_FMSG_OBJ_NAME => Self::FmsgObjName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_FMSG_OBJ_NAME value")?,
            ),
            DEVLINK_ATTR_FMSG_OBJ_VALUE_TYPE => Self::FmsgObjValueType(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_FMSG_OBJ_VALUE_TYPE value")?,
            ),
            DEVLINK_ATTR_FMSG_OBJ_VALUE_DATA => Self::FmsgObjValueData({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_FMSG_OBJ_VALUE_DATA")?;
                stored
            }),
            DEVLINK_ATTR_HEALTH_REPORTER_DUMP_TS_NS => Self::HealthReporterDumpTsNs(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_DUMP_TS_NS value")?,
            ),
            DEVLINK_ATTR_NETNS_FD => Self::NetnsFd(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_NETNS_FD value")?,
            ),
            DEVLINK_ATTR_NETNS_PID => Self::NetnsPid(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_NETNS_PID value")?,
            ),
            DEVLINK_ATTR_NETNS_ID => Self::NetnsId(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_NETNS_ID value")?,
            ),
            DEVLINK_ATTR_HEALTH_REPORTER_AUTO_DUMP => Self::HealthReporterAutoDump(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_HEALTH_REPORTER_AUTO_DUMP value")?,
            ),
            DEVLINK_ATTR_TRAP_POLICER_ID => Self::TrapPolicerId(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_POLICER_ID value")?,
            ),
            DEVLINK_ATTR_TRAP_POLICER_RATE => Self::TrapPolicerRate(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_POLICER_RATE value")?,
            ),
            DEVLINK_ATTR_TRAP_POLICER_BURST => Self::TrapPolicerBurst(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_TRAP_POLICER_BURST value")?,
            ),
            DEVLINK_ATTR_PORT_FUNCTION => Self::PortFunction({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_PORT_FUNCTION")?;
                stored
            }),
            DEVLINK_ATTR_INFO_BOARD_SERIAL_NUMBER => Self::InfoBoardSerialNumber(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_INFO_BOARD_SERIAL_NUMBER value")?,
            ),
            DEVLINK_ATTR_PORT_LANES => Self::PortLanes(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_LANES value")?,
            ),
            DEVLINK_ATTR_PORT_SPLITTABLE => Self::PortSplittable(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_PORT_SPLITTABLE value")?,
            ),
            DEVLINK_ATTR_PORT_EXTERNAL => Self::PortExternal(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_PORT_EXTERNAL value")?,
            ),
            DEVLINK_ATTR_PORT_CONTROLLER_NUMBER => Self::PortControllerNo(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_CONTROLLER_NUMBER value")?,
            ),
            DEVLINK_ATTR_FLASH_UPDATE_STATUS_TIMEOUT => Self::FlashUpdateStatusTimeout(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_FLASH_UPDATE_STATUS_TIMEOUT value")?,
            ),
            DEVLINK_ATTR_FLASH_UPDATE_OVERWRITE_MASK => Self::FlashUpdateOverWriteMask(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_FLASH_UPDATE_OVERWRITE_MASK value")?,
            ),
            DEVLINK_ATTR_RELOAD_ACTIONS_PERFORMED => Self::ReloadActionPerformed(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_RELOAD_ACTIONS_PERFORMED value")?,
            ),
            DEVLINK_ATTR_RELOAD_LIMITS => Self::ReloadLimits(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_RELOAD_LIMITS value")?,
            ),
            DEVLINK_ATTR_PORT_PCI_SF_NUMBER => Self::PortPciSfNo(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_PCI_SF_NUMBER value")?,
            ),
            DEVLINK_ATTR_RATE_TYPE => Self::RateType(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_RATE_TYPE value")?,
            ),
            DEVLINK_ATTR_RATE_TX_SHARE => Self::RateTxShare(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RATE_TX_SHARE value")?,
            ),
            DEVLINK_ATTR_RATE_TX_MAX => Self::RateTxMax(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RATE_TX_MAX value")?,
            ),
            DEVLINK_ATTR_RATE_NODE_NAME => Self::RateNodeName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_RATE_NODE_NAME value")?,
            ),
            DEVLINK_ATTR_RATE_PARENT_NODE_NAME => Self::RateParentNodeName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_RATE_PARENT_NODE_NAME value")?,
            ),
            DEVLINK_ATTR_PORT_SPLIT_COUNT => Self::PortSplitCount(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_SPLIT_COUNT value")?,
            ),
            DEVLINK_ATTR_PORT_SPLIT_GROUP => Self::PortSplitGroup(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_SPLIT_GROUP value")?,
            ),
            DEVLINK_ATTR_SB_INDEX => Self::SbIndex(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_SB_INDEX value")?,
            ),
            DEVLINK_ATTR_SB_SIZE => Self::SbSize(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_SB_SIZE value")?,
            ),
            DEVLINK_ATTR_SB_INGRESS_POOL_COUNT => Self::SbIngressPoolCount(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_SB_INGRESS_POOL_COUNT value")?,
            ),
            DEVLINK_ATTR_SB_EGRESS_POOL_COUNT => Self::SbEgressPoolCount(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_SB_EGRESS_POOL_COUNT value")?,
            ),
            DEVLINK_ATTR_SB_INGRESS_TC_COUNT => Self::SbIngressTcCount(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_SB_INGRESS_TC_COUNT value")?,
            ),
            DEVLINK_ATTR_SB_EGRESS_TC_COUNT => Self::SbEgressTcCount(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_SB_EGRESS_TC_COUNT value")?,
            ),
            DEVLINK_ATTR_SB_POOL_INDEX => Self::SbPoolIndex(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_SB_POOL_INDEX value")?,
            ),
            DEVLINK_ATTR_SB_POOL_TYPE => Self::SbPoolType(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_SB_POOL_TYPE value")?,
            ),
            DEVLINK_ATTR_SB_POOL_SIZE => Self::SbPoolSize(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_SB_POOL_SIZE value")?,
            ),
            DEVLINK_ATTR_SB_POOL_THRESHOLD_TYPE => Self::SbPoolThresholdType(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_SB_POOL_THRESHOLD_TYPE value")?,
            ),
            DEVLINK_ATTR_SB_THRESHOLD => Self::SbPoolThreshold(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_SB_POOL_THRESHOLD value")?,
            ),
            DEVLINK_ATTR_SB_TC_INDEX => Self::SbTcIndex(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_SB_TC_INDEX value")?,
            ),
            DEVLINK_ATTR_SB_OCC_CUR => Self::SbOccCur(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_SB_OCC_CUR value")?,
            ),
            DEVLINK_ATTR_SB_OCC_MAX => Self::SbOccMax(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_SB_OCC_MAX value")?,
            ),
            DEVLINK_ATTR_ESWITCH_MODE => Self::EswitchMode(
                parse_u16(payload)
                    .context("invalid DEVLINK_ATTR_ESWITCH_MODE value")?,
            ),
            DEVLINK_ATTR_ESWITCH_INLINE_MODE => Self::EswitchInlineMode(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_ESWITCH_INLINE_MODE value")?,
            ),
            DEVLINK_ATTR_DPIPE_TABLES => Self::DpipeTables({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_TABLES")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_TABLE => Self::DpipeTable({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_TABLE")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_TABLE_NAME => Self::DpipeTableName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_TABLE_NAME value")?,
            ),
            DEVLINK_ATTR_DPIPE_TABLE_SIZE => Self::DpipeTableSize(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_TABLE_SIZE value")?,
            ),
            DEVLINK_ATTR_DPIPE_TABLE_MATCHES => Self::DpipeTableMatches({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_TABLE_MATCHES")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_TABLE_ACTIONS => Self::DpipeTableActions({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_TABLE_ACTIONS")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_TABLE_COUNTERS_ENABLED => Self::DpipeTableCountersEnabled(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_TABLE_COUNTERS_ENABLED value")?,
            ),
            DEVLINK_ATTR_DPIPE_ENTRIES => Self::DpipeEntries({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_ENTRIES")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_ENTRY => Self::DpipeEntry({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_ENTRY")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_ENTRY_INDEX => Self::DpipeEntryIndex(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_ENTRY_INDEX value")?,
            ),
            DEVLINK_ATTR_DPIPE_ENTRY_MATCH_VALUES => Self::DpipeEntryMatchValues({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_ENTRY_MATCH_VALUES")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_ENTRY_ACTION_VALUES => Self::DpipeEntryActionValues({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_ENTRY_ACTION_VALUES")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_ENTRY_COUNTER => Self::DpipeEntryCounter(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_ENTRY_COUNTER value")?,
            ),
            DEVLINK_ATTR_DPIPE_MATCH_TYPE => Self::DpipeMatchType(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_MATCH_TYPE value")?,
            ),
            DEVLINK_ATTR_DPIPE_MATCH_VALUE => Self::DpipeMatchValue({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_MATCH_VALUE")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_ACTION => Self::DpipeAction({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_ACTION")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_ACTION_TYPE => Self::DpipeActionType(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_ACTION_TYPE value")?,
            ),
            DEVLINK_ATTR_DPIPE_ACTION_VALUE => Self::DpipeActionValue({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context(
                        "failed to parse DEVLINK_ATTR_DPIPE_TABLE_COUNTERS",
                    )?;
                stored
            }),
            DEVLINK_ATTR_PORT_IBDEV_NAME => Self::PortIbdevName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_PORT_IBDEV_NAME value")?,
            ),
            DEVLINK_ATTR_DPIPE_MATCH => Self::DpipeMatch({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_MATCH")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_VALUE => Self::DpipeValue(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_VALUE value")?,
            ),
            DEVLINK_ATTR_DPIPE_VALUE_MASK => Self::DpipeValueMask(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_VALUE_MASK value")?,
            ),
            DEVLINK_ATTR_DPIPE_VALUE_MAPPING => Self::DpipeValueMapping(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_VALUE_MAPPING value")?,
            ),
            DEVLINK_ATTR_DPIPE_HEADERS => Self::DpipeHeaders({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_HEADERS")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_HEADER => Self::DpipeHader({
                let stored = NlasIterator::new(payload)
                    .map(|nla| {
                        nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_HEADER")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_HEADER_NAME => Self::DpipeHeaderName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_HEADER_NAME value")?,
            ),
            DEVLINK_ATTR_DPIPE_HEADER_ID => Self::DpipeHeaderId(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_HEADER_ID value")?,
            ),
            DEVLINK_ATTR_DPIPE_HEADER_FIELDS => Self::DpipeHeaderFields({
                let stored = NlasIterator::new(payload)
                    .map(|nla| nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla)))
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_HEADER_FIELDS")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_HEADER_GLOBAL => Self::DpipeHeaderGlobal(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_HEADER_GLOBAL value")?,
            ),
            DEVLINK_ATTR_DPIPE_HEADER_INDEX => Self::DpipeHeaderIndex(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_HEADER_INDEX value")?,
            ),
            DEVLINK_ATTR_DPIPE_FIELD => Self::DpipeField({
                let stored = NlasIterator::new(payload)
                    .map(|nla| nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla)))
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_DPIPE_FIELD")?;
                stored
            }),
            DEVLINK_ATTR_DPIPE_FIELD_NAME => Self::DpipeFieldName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_FIELD_NAME value")?,
            ),
            DEVLINK_ATTR_DPIPE_FIELD_ID => Self::DpipeFieldId(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_FIELD_ID value")?,
            ),
            DEVLINK_ATTR_DPIPE_FIELD_BITWIDTH => Self::DpipeFieldBitwidth(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_FIELD_BITWIDTH value")?,
            ),
            DEVLINK_ATTR_DPIPE_FIELD_MAPPING_TYPE => Self::DpipeFieldMappingType(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_FIELD_MAPPING_TYPE value")?,
            ),
            DEVLINK_ATTR_ESWITCH_ENCAP_MODE => Self::EswitchEncapMode(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_ESWITCH_ENCAP_MODE value")?,
            ),
            DEVLINK_ATTR_RESOURCE_LIST => Self::ResourceList({
                let stored = NlasIterator::new(payload)
                    .map(|nla| nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla)))
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_RESOURCE_LIST")?;
                stored
            }),
            DEVLINK_ATTR_RESOURCE => Self::Resource({
                let stored = NlasIterator::new(payload)
                    .map(|nla| nla.and_then(|nla| GenlDevlinkAttrs::parse(&nla)))
                    .collect::<Result<Vec<_>, _>>()
                    .context("failed to parse DEVLINK_ATTR_RESOURCE")?;
                stored
            }),
            DEVLINK_ATTR_RESOURCE_NAME => Self::ResoureceName(
                parse_string(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_NAME value")?,
            ),
            DEVLINK_ATTR_RESOURCE_ID => Self::ResourceId(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_ID value")?,
            ),
            DEVLINK_ATTR_RESOURCE_SIZE => Self::ResourceSize(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_SIZE value")?,
            ),
            DEVLINK_ATTR_RESOURCE_SIZE_NEW => Self::ResourceSizeNew(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_SIZE_NEW value")?,
            ),
            DEVLINK_ATTR_RESOURCE_SIZE_VALID => Self::ResourceSizeValid(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_SIZE_VALID value")?,
            ),
            DEVLINK_ATTR_RESOURCE_SIZE_MIN => Self::ResourceSizeMin(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_SIZE_MIN value")?,
            ),
            DEVLINK_ATTR_RESOURCE_SIZE_MAX => Self::ResourceSizeMax(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_SIZE_MAX value")?,
            ),
            DEVLINK_ATTR_RESOURCE_SIZE_GRAN => Self::ResourceSizeGran(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_SIZE_GRAN value")?,
            ),
            DEVLINK_ATTR_RESOURCE_UNIT => Self::ResourceUnit(
                parse_u8(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_UNIT value")?,
            ),
            DEVLINK_ATTR_RESOURCE_OCC => Self::ResourceOcc(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_RESOURCE_OCC value")?,
            ),
            DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_ID => Self::DpipeTableResourceId(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_ID value")?,
            ),
            DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_UNITS => Self::DpipeTableResourceUnit(
                parse_u64(payload)
                    .context("invalid DEVLINK_ATTR_DPIPE_TABLE_RESOURCE_UNITS value")?,
            ),
            DEVLINK_ATTR_PORT_SPLIT_SUBPORT_NUMBER => Self::PortSplitCount(
                parse_u32(payload)
                    .context("invalid DEVLINK_ATTR_PORT_SPLIT_SUBPORT_NUMBER value")?,
            ),
            _ => {
                return Err(DecodeError::from(format!(
                    "Unknown NLA type: {}",
                    buf.kind()
                )))
            }
        })
    }
}
