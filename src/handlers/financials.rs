//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/financials` endpoint.

// Internal imports (std, crate)
use crate::common::*;
use crate::config::FdicApiConfig;

// External imports (alphabetized)
use rmcp::handler::server::tool::IntoCallToolResult;
use rmcp::model::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;
use tracing::info;
use utoipa::ToSchema;

/// Auto-generated parameters struct for `/financials` endpoint.
/// Spec: risview_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct FinancialsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[schemars(description = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#)]
    pub agg_by: Option<String>,
    #[schemars(description = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#)]
    pub agg_term_fields: Option<String>,
    #[schemars(description = r#"The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE."#)]
    pub agg_sum_fields: Option<String>,
    #[schemars(description = r#"The limit on how many aggregated results will be displayed"#)]
    pub agg_limit: Option<i32>,
}

// Implement FdicEndpoint for generic handler
impl FdicEndpoint for FinancialsParameters {
    fn name() -> &'static str {
        "financials"
    }
}

// Implement QueryParameters for generic handler
impl QueryParameters for FinancialsParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "ACTEVT",
        "ASSET",
        "BRANCH",
        "CALLFORM",
        "CB",
        "CBSADIV",
        "CBSANAME",
        "STMULT",
        "ADDRESS",
        "CBLRIND",
        "CD1T3",
        "CD1T3R",
        "CD3LES",
        "CD3LESR",
        "CD3LESS",
        "CD3LESSR",
        "CDOV3",
        "CDOV3R",
        "CDOV3S",
        "CDOV3SR",
        "CD3T12",
        "CD3T12R",
        "CD3T12S",
        "CD3T12SR",
        "CD1T3S",
        "CD1T3SR",
        "CERT",
        "CERTCONS",
        "CITYHCR",
        "CLCODE",
        "CLOSED",
        "CMSA",
        "CNTRYALP",
        "CNTRYNUM",
        "CNTYNUM",
        "CSA",
        "DENOVO",
        "DEP",
        "DEPR",
        "DEPDOM",
        "DEPDOMR",
        "DIVISION",
        "DOCKET",
        "EDGECODE",
        "ENTTYPE",
        "EQ",
        "EQ2",
        "EQR",
        "FAILED",
        "FDICAREA",
        "FDICTERR",
        "FLDOFDCA",
        "FORM31",
        "HCTMULT",
        "HCTNONE",
        "INSAGNT2",
        "INSBIF",
        "INSDIF",
        "INSTAG",
        "INSTCRCD",
        "INSSAIF",
        "MINORITY",
        "MUTUAL",
        "NAMEHCR",
        "NETINC",
        "NETINCR",
        "NETINCQ",
        "NETINCQA",
        "NETINCQR",
        "OFFDOM",
        "OFFFOR",
        "OFFOA",
        "PARCERT",
        "L_REPDTE",
        "REPDTE_RAW",
        "REPDTE",
        "REPYEAR",
        "RISDATE",
        "ROA",
        "ROAPTX",
        "ROAPTXQ",
        "ROAQ",
        "ROE",
        "ROEQ",
        "RSSDHCR",
        "SPECGRP",
        "SPECGRPDESC",
        "STALPHCR",
        "SUBCHAPS",
        "TRACT",
        "TRUST",
        "ACEPT",
        "ACTIVE",
        "BKCLASS",
        "BKPREM",
        "BKPREMR",
        "BRO",
        "BROR",
        "CALLYM",
        "CHBAL",
        "CHBALR",
        "CHBALI",
        "CHBALIR",
        "CHRTAGNT",
        "CONSERVE",
        "CRLNLS",
        "CRLNLSR",
        "CRLNLSQ",
        "CRLNLSQR",
        "CUSLI",
        "DDT",
        "DDTR",
        "DEPFOR",
        "DEPFORR",
        "DEPI",
        "DEPIFOR",
        "DEPIFORR",
        "DEPIPCCF",
        "DEPIPCCFR",
        "DEPIPCF",
        "DEPNI",
        "DEPNIFOR",
        "DEPNIFORR",
        "DRLNLS",
        "DRLNLSR",
        "DRLNLSQ",
        "DRLNLSQR",
        "EAMINTAN",
        "EAMINTANR",
        "EAMINTQ",
        "EAMINTQR",
        "EDEP",
        "EDEPDOM",
        "EDEPDOMR",
        "EDEPDOMQ",
        "EDEPDOMQR",
        "EDEPFOR",
        "EDEPFORR",
        "EDEPFORQ",
        "EDEPFORQR",
        "EFHLBADV",
        "EFREPP",
        "EFREPPR",
        "EFREPPQ",
        "EFREPPQR",
        "EINTEXP",
        "EINTEXPR",
        "EINTXQ",
        "EINTXQA",
        "EINTEXPA",
        "EINTXQR",
        "ELNATR",
        "ELNATRR",
        "ELNATQ",
        "ELNATQA",
        "ELNATQR",
        "ELNLOSQ",
        "NTTOTQ",
        "ELNLOS",
        "EMTGLS",
        "ADDNONINTEXP",
        "ADDNONINTEXPR",
        "ADDNONINTEXPQ",
        "ADDNONINTEXPQR",
        "EOTHNINT",
        "EOTHNINTR",
        "EOTHNINQ",
        "EOTHNINQR",
        "EPREMAGG",
        "EPREMAGGR",
        "EPREMAGQ",
        "EPREMAGQR",
        "EQCDIV",
        "EQCDIVR",
        "EQCDIVC",
        "EQCDIVCR",
        "EQCDIVP",
        "EQCDIVPR",
        "EQCDIVQ",
        "EQCDIVQR",
        "EQCFCTA",
        "EQCONSUB",
        "EQCS",
        "EQCSR",
        "EQNWCERT",
        "EQOTHCC",
        "EQPP",
        "EQPPR",
        "EQSUR",
        "EQSURR",
        "EQUP",
        "EQUPTOT",
        "EQUPTOTR",
        "ESAL",
        "ESALR",
        "ESALQ",
        "ESALQR",
        "ESUBND",
        "ETTLOTBO",
        "EXTRA",
        "EXTRAR",
        "EXTRAQ",
        "EXTRAQR",
        "FDICDBS",
        "FDICDBSDESC",
        "FDICSUPV",
        "FDICSUPVDESC",
        "FED",
        "FEDDESC",
        "FEDCHRTR",
        "FLDOFF",
        "FORCHRTR",
        "FORMCFR",
        "FREPO",
        "FREPOR",
        "FREPP",
        "FREPPR",
        "FRSMEM",
        "HCTONE",
        "IBA",
        "IBEFTAX",
        "ICHBAL",
        "ICHBALR",
        "ICHBALQ",
        "ICHBALQR",
        "IFREPO",
        "IFREPOR",
        "IFREPOQ",
        "IFREPOQR",
        "IGLSEC",
        "IGLSECR",
        "IGLSECQR",
        "ILNDOM",
        "ILNDOMR",
        "ILNDOMQ",
        "ILNDOMQR",
        "ILNFOR",
        "ILNFORR",
        "ILNFORQ",
        "ILNFORQR",
        "ILS",
        "ILSR",
        "ILSQ",
        "ILSQR",
        "INSALL",
        "INSCOML",
        "INSFDIC",
        "INSNONE",
        "INSSAVE",
        "INSTCOML",
        "INSTSAVE",
        "INSTTYPE",
        "INTAN",
        "INTANR",
        "INTEXPY",
        "INTEXPYQ",
        "INTINC",
        "INTINCR",
        "INTINQ",
        "INTINQR",
        "INTINQA",
        "INVSUB",
        "INVSUORE",
        "IOTHFEE",
        "IOTHII",
        "IOTHIIR",
        "IOTHIIQ",
        "IOTHIIQR",
        "IRAKEOGH",
        "IRAKEOGHR",
        "ISC",
        "ISCR",
        "ISCQ",
        "ISCQR",
        "ISERCHG",
        "ISERCHGR",
        "ITAX",
        "ITAXR",
        "ITAXQ",
        "ITAXQR",
        "ITRADE",
        "ITRADER",
        "ITRADEQ",
        "ITRADEQR",
        "LIAB",
        "LIABR",
        "LIABEQ",
        "LIABEQR",
        "LIPMTG",
        "LLPFDSTK",
        "LNACOTH",
        "LNAG",
        "LNAGR",
        "LNATRES",
        "LNATRESJ",
        "LNATRESRR",
        "LNAUTO",
        "LNAUTOR",
        "LNCI",
        "LNCIR",
        "LNCON",
        "LNCONR",
        "LNCONOT1",
        "LNCONOTH",
        "LNCONOTHR",
        "LNCRCD",
        "LNCRCDR",
        "LNCRCDRP",
        "LNDEP",
        "LNFG",
        "LNFGR",
        "LNLS",
        "LNLSGR",
        "LNLSGR2",
        "LNLSGRJ",
        "LNLSGRR",
        "LNLSNET",
        "LNLSNETR",
        "LNMUNI",
        "LNMUNIR",
        "LNOTCI",
        "LNOTCIR",
        "LNOTHER",
        "LNSOTHER",
        "LNSOTHERR",
        "LNRE",
        "LNRE2",
        "LNRECON2",
        "LNREMUL2",
        "LNREJ",
        "LNRE5",
        "LNRER",
        "LNREAG",
        "LNRECON5",
        "LNREAGR",
        "LNRECONS",
        "LNRECONSR",
        "LNREDOM",
        "LNREDOMR",
        "LNREFOR",
        "LNREFORR",
        "LNRELOC",
        "LNRELOCR",
        "LNRELOC2",
        "LNRELOC5",
        "LNREMULT",
        "LNREMUL5",
        "LNREMULTR",
        "LNRENRES",
        "LNRENRE5",
        "LNRENRE2",
        "LNRENRESR",
        "LNREPP",
        "LNRERES",
        "LNRERESR",
        "LNRERES2",
        "LNRERES5",
        "LNRESRE",
        "LS",
        "LSR",
        "METRO",
        "MI",
        "MICRO",
        "MNRTYCDE",
        "MNRTYDTE",
        "MTGLS",
        "N",
        "NALNLS",
        "NC",
        "NCLNLS",
        "NETIMIN",
        "NETIMINR",
        "NETIMINQ",
        "NETIMINQR",
        "NETINBM",
        "NETINBMR",
        "NETINBMQ",
        "NETINBXA",
        "NETIBXQA",
        "NETINBMQR",
        "NEWINST",
        "NFAA",
        "NIM",
        "NIMR",
        "NIMQ",
        "NIMQA",
        "NIMA",
        "NIMQR",
        "NM",
        "NONII",
        "NONIIR",
        "NONIX",
        "NONIXR",
        "NONIXQ",
        "NONIXQA",
        "NONIXQR",
        "NS",
        "NTLNLS",
        "NTLNLSCOR",
        "NTLNLSQ",
        "NTLNLSQA",
        "NTLNLSCOQR",
        "NTR",
        "NTRR",
        "NTRIPC",
        "NTRIPCR",
        "NTRMUNI",
        "NTRMUNIR",
        "NTRTIME",
        "NTRTMLG",
        "NTRTMLGJ",
        "NTRTMLGJR",
        "NTRTMMED",
        "NTRTMMEDR",
        "NTRUSGOV",
        "NTRUSGOVR",
        "NTIRTA",
        "NTTOT",
        "NUMEMP",
        "OA",
        "OAKAR",
        "OCCDIST",
        "OCCDISTDESC",
        "OFFDMULT",
        "OFFNDOM",
        "OFFOTH",
        "OFFSOD",
        "OFFSTATE",
        "OFFTOT",
        "OFFUSOA",
        "OI",
        "OTSDIST",
        "OTSREGNO",
        "OLMIN",
        "ORE",
        "ORER",
        "OTHBFHLB",
        "OTHBFHLBR",
        "OTHBOR",
        "OTHBRF",
        "OTHBRFR",
        "OTBFH1L",
        "OTBFH1LR",
        "OTBFH1T3",
        "OTBFH1T3R",
        "OTBFH3T5",
        "OTBFH3T5R",
        "OTBFHOV5",
        "OTBFHOV5R",
        "OTHBFH1L",
        "OTHBFH1LR",
        "OTBFHSTA",
        "OTBFHSTAR",
        "OTBOT1L",
        "OTBOT1LR",
        "OTBOT1T3",
        "OTBOT1T3R",
        "OTBOT3T5",
        "OTBOT3T5R",
        "OTBOTOV5",
        "OTBOTOV5R",
        "OTHBOT1L",
        "OTHBOT1LR",
        "ALLOTHL",
        "ALLOTHLR",
        "P3LNLS",
        "P9LNLS",
        "QBPRCOML",
        "QBPRCOMLDESC",
        "QBPRSAVB",
        "QBPRSAVS",
        "QTRNO",
        "REGAGNT",
        "RISKTERR",
        "S10T250B",
        "SASSER",
        "SB",
        "SC",
        "SCR",
        "SCAA",
        "SCHF",
        "SCAGE",
        "SCASPNHA",
        "SCASPNAF",
        "SCASPNSUM",
        "SCASPNSUMR",
        "SCDEQ",
        "SCDOMO",
        "SCDOMOR",
        "SCEQ",
        "SCFDEQ",
        "SCFORD",
        "SCFORDR",
        "SCMTGBK",
        "SCMTGBKR",
        "SCMUNI",
        "SCMUNIR",
        "SCMV",
        "SCODPC",
        "SCODPCR",
        "SCRES",
        "SCUS",
        "SCUSR",
        "SCUSA",
        "SCUST",
        "SCUSTR",
        "SIMS_LAT",
        "SIMS_LONG",
        "SL",
        "SM",
        "STALP",
        "STCHRTR",
        "STNAME",
        "STNUM",
        "SUBLLPF",
        "SUBND",
        "SZ25",
        "SZ100",
        "SZ100MP",
        "SZ100T3",
        "SZ100T5",
        "SZ100T1B",
        "SZ10BP",
        "SZ1BP",
        "SZ1BT10B",
        "SZ1BT3B",
        "SZ1BT5B",
        "SZ250BP",
        "SZ25T50",
        "SZ300T5",
        "SZ3BT10B",
        "SZ500T1B",
        "SZ50T100",
        "SZ5BP",
        "TFRA",
        "TRADE",
        "TRADEL",
        "TRADELR",
        "TRADER",
        "TRN",
        "TRNR",
        "TRNIPC",
        "TRNIPCOC",
        "TRNIPCOCR",
        "TRNMUNI",
        "TRNMUNIR",
        "TRNUSGOV",
        "TRNUSGOVR",
        "TRUSTPWR",
        "TS",
        "TSR",
        "TTL",
        "TTLOTBOR",
        "UNINC",
        "UNINUM",
        "USA",
        "UYAMTG",
        "ABCUBK",
        "ABCUBKR",
        "ABCUOTH",
        "ABCUOTHR",
        "ABCXBK",
        "ABCXBKR",
        "ABCXOTH",
        "ABCXOTHR",
        "ASCEOTH",
        "ASCEOTHR",
        "ASCERES",
        "ASCERESR",
        "ASDROTH",
        "ASDROTHR",
        "ASDRRES",
        "ASDRRESR",
        "ASSET2",
        "ASSET5",
        "ASSETFOR",
        "ASSTLT",
        "ASSTLTR",
        "ASTEMPM",
        "AVASSETJ",
        "AVASSETJR",
        "BROINS",
        "BROINSR",
        "CALLYMD",
        "CHBALFOR",
        "CHBALNI",
        "CHBALNIR",
        "CHCIC",
        "CHCICR",
        "CHCOIN",
        "CHCOINR",
        "CHFLA",
        "CHFLQ",
        "CHFRB",
        "CHFRBR",
        "CHITEM",
        "CHITEMR",
        "CHNUS",
        "CHNUSR",
        "CHNUSFBK",
        "CHUS",
        "CHUSR",
        "CHUSFBK",
        "CITY",
        "COREDEP",
        "COREDEPR",
        "CRAG",
        "CRAGR",
        "CRAGQ",
        "CRAGQR",
        "CRAGSM",
        "CRAGSMR",
        "CRAGSMQ",
        "CRAGSMQR",
        "CRAUTO",
        "CRAUTOR",
        "CRAUTOQ",
        "CRAUTOQR",
        "CRCI",
        "CRCIR",
        "CRCIQ",
        "CRCIQR",
        "CRCINUS",
        "CRCINUSR",
        "CRCINUSQ",
        "CRCINUSQR",
        "CRCON",
        "CRCONR",
        "CRCONQ",
        "CRCONQR",
        "CRCONOTH",
        "CRCONOTHR",
        "CRCONOTQ",
        "CRCONOTQR",
        "CRCRCD",
        "CRCRCDR",
        "CRCRCDQ",
        "CRCRCDQR",
        "CRDEP",
        "CRDEPR",
        "CRDEPQ",
        "CRDEPQR",
        "CRDEPNUS",
        "CRDEPNUSR",
        "CRDEPNUQ",
        "CRDEPNUQR",
        "CRFORGV",
        "CRFORGVR",
        "CRFORGVQ",
        "CRFORGVQR",
        "CRLS",
        "CRLSR",
        "CRLSQ",
        "CRLSQR",
        "CROTHER",
        "CROTHERR",
        "CROTHQ",
        "CROTHQR",
        "CRRE",
        "CRRER",
        "CRREQ",
        "CRREQR",
        "CRREAG",
        "CRREAGR",
        "CRREAGQ",
        "CRREAGQR",
        "CRRECNFM",
        "CRRECNOT",
        "CRRECONQ",
        "CRRECONQR",
        "CRRECONS",
        "CRRECONSR",
        "CRREFOR",
        "CRREFORR",
        "CRREFORQ",
        "CRREFORQR",
        "CRRELOC",
        "CRRELOCR",
        "CRRELOCQ",
        "CRRELOCQR",
        "CRREMULQ",
        "CRREMULQR",
        "CRREMULT",
        "CRREMULTR",
        "CRRENRES",
        "CRRENRESR",
        "CRRENROT",
        "CRRENROW",
        "CRRENRSQ",
        "CRRENRSQR",
        "CRRENUS",
        "CRRENUSR",
        "CRRENUSQ",
        "CRRENUSQR",
        "CRRERES",
        "CRRERESR",
        "CRRERESQ",
        "CRRERESQR",
        "CRRERSF2",
        "CRRERSF2R",
        "CRRERS2Q",
        "CRRERS2QR",
        "CRRERSFM",
        "CRRERSFMR",
        "CRRERSFQ",
        "CRRERSFQR",
        "CRREOFFDOM",
        "CRREOFFDOMR",
        "CRREOFFDOMQ",
        "CRREOFFDOMQR",
        "CTDERBEN",
        "CTDERGTY",
        "DEPBEFEX",
        "DEPCSBQ",
        "DEPCSBQR",
        "DEPDASTR",
        "DEPFBKF",
        "DEPFBKFR",
        "DEPFGOVF",
        "DEPFGOVFR",
        "DEPIDOM",
        "DEPIDOMR",
        "DEPINS",
        "DEPINSR",
        "DEPLGAMT",
        "DEPLGAMTR",
        "DEPLGB",
        "DEPLGRA",
        "DEPLGRAR",
        "DEPLGRN",
        "DEPLSNB",
        "DEPLSNBR",
        "DEPNIDOM",
        "DEPNIDOMR",
        "DEPSMAMT",
        "DEPSMAMTR",
        "DEPSMB",
        "DEPSMRA",
        "DEPSMRAR",
        "DEPSMRN",
        "DEPALLEX",
        "DEPUNA",
        "DEPUNAR",
        "DEPUNINS",
        "DEPUSBKF",
        "DEPUSBKFR",
        "DEPUSMF",
        "DEPUSMFR",
        "DRAG",
        "DRAGR",
        "DRAGQ",
        "DRAGQR",
        "DRAGSM",
        "DRAGSMR",
        "DRAGSMQ",
        "DRAGSMQR",
        "DRAUTO",
        "DRAUTOR",
        "DRAUTOQ",
        "DRAUTOQR",
        "DRCI",
        "DRCIR",
        "DRCIQ",
        "DRCIQR",
        "DRCINUS",
        "DRCINUSR",
        "DRCINUSQ",
        "DRCINUSQR",
        "DRCON",
        "DRCONR",
        "DRCONQ",
        "DRCONQR",
        "DRCONOTH",
        "DRCONOTHR",
        "DRCONOTQ",
        "DRCONOTQR",
        "DRCRCD",
        "DRCRCDR",
        "DRCRCDQ",
        "DRCRCDQR",
        "DRDEP",
        "DRDEPR",
        "DRDEPQ",
        "DRDEPQR",
        "DRDEPNUS",
        "DRDEPNUSR",
        "DRDEPNUQ",
        "DRDEPNUQR",
        "DRFORGV",
        "DRFORGVR",
        "DRFORGVQ",
        "DRFORGVQR",
        "DRLS",
        "DRLSR",
        "DRLSQ",
        "DRLSQR",
        "DROTHER",
        "DROTHERR",
        "DROTHQ",
        "DROTHQR",
        "DRRE",
        "DRRER",
        "DRREQ",
        "DRREQR",
        "DRREAG",
        "DRREAGR",
        "DRREAGQ",
        "DRREAGQR",
        "DRRECNFM",
        "DRRECNOT",
        "DRRECONQ",
        "DRRECONQR",
        "DRRECONS",
        "DRRECONSR",
        "DRREFOR",
        "DRREFORR",
        "DRREFORQ",
        "DRREFORQR",
        "DRRELOC",
        "DRRELOCR",
        "DRRELOCQ",
        "DRRELOCQR",
        "DRREMULQ",
        "DRREMULQR",
        "DRREMULT",
        "DRREMULTR",
        "DRRENRES",
        "DRRENRESR",
        "DRRENROT",
        "DRRENROW",
        "DRRENRSQ",
        "DRRENRSQR",
        "DRRENUS",
        "DRRENUSR",
        "DRRENUSQ",
        "DRRENUSQR",
        "DRRERES",
        "DRRERESR",
        "DRRERESQ",
        "DRRERESQR",
        "DRRERSF2",
        "DRRERSF2R",
        "DRRERS2Q",
        "DRRERS2QR",
        "DRRERSFM",
        "DRRERSFMR",
        "DRRERSFQ",
        "DRRERSFQR",
        "DRREOFFDOM",
        "DRREOFFDOMR",
        "DRREOFFDOMQ",
        "DRREOFFDOMQR",
        "EDCM",
        "EEFF",
        "EEFFQ",
        "EEFFR",
        "EEFFQR",
        "EFFDATE",
        "EINTGW",
        "EINTGWR",
        "EINTGWQ",
        "EINTGWQR",
        "EINTOTH",
        "EINTOTHR",
        "EINTOTHQ",
        "EINTOTHQR",
        "ELNANTR",
        "ELNATRA",
        "ELNATRY",
        "ELNATRYQ",
        "ENCEAUTO",
        "ENCEAUTOR",
        "ENCECI",
        "ENCECIR",
        "ENCECON",
        "ENCECONR",
        "ENCEOTH",
        "ENCEOTHR",
        "ENCERES",
        "ENCERESR",
        "EOTHINT",
        "EOTHINTR",
        "EOTHINTQ",
        "EOTHINTQR",
        "EQ5",
        "EQCBHCTR",
        "EQCBHCTRR",
        "EQCCOMPI",
        "EQCCOMPIR",
        "EQCDIVA",
        "EQCMRG",
        "EQCMRGR",
        "EQCPREV",
        "EQCPREVR",
        "EQCREST",
        "EQCRESTR",
        "EQCSTKRX",
        "EQCSTKRXR",
        "EQCSXQ",
        "EQCSXQR",
        "EQCTRSTX",
        "EQCTRSTXR",
        "EQTOT",
        "EQTOTR",
        "EQV",
        "ERNAST",
        "ERNAST2",
        "ERNAST5",
        "ERNASTR",
        "ESTYMD",
        "ENDEFYMD",
        "ORG_END_NUM_DTE",
        "ETTLOTMG",
        "FORMTFR",
        "FX",
        "FXFFC",
        "FXNVS",
        "FXPOC",
        "FXSPOT",
        "FXWOC",
        "IBEFTXQ",
        "IBEFXTR",
        "IBEFXTRR",
        "IBEFXTRQ",
        "IEFF",
        "IEFFQ",
        "IBEFXTRQR",
        "IFIDUC",
        "IFIDUCR",
        "IFIDUCQ",
        "IFIDUCQR",
        "IGLCMEX",
        "IGLCMEXR",
        "IGLCMEXQ",
        "IGLCMEXQR",
        "IGLCREX",
        "IGLCREXR",
        "IGLCREXQ",
        "IGLCREXQR",
        "IGLEDEX",
        "IGLEDEXR",
        "IGLEDEXQ",
        "IGLEDEXQR",
        "IGLFXEX",
        "IGLFXEXR",
        "IGLFXEXQ",
        "IGLFXEXQR",
        "IGLRTEX",
        "IGLRTEXR",
        "IGLRTEXQ",
        "IGLRTEXQR",
        "IGLSECQ",
        "IGLTRAD",
        "IGLTRADR",
        "IGLTRDQ",
        "IGLTRDQR",
        "IINSCOM",
        "IINSCOMR",
        "IINSCOMQ",
        "IINSCOMQR",
        "IINSOTH",
        "IINSOTHR",
        "IINSOTHQ",
        "IINSOTHQR",
        "IINSUND",
        "IINSUNDR",
        "IINSUNDQ",
        "IINSUNDQR",
        "IINVFEE",
        "IINVFEER",
        "IINVFEEQ",
        "IINVFEEQR",
        "INSAGNT1",
        "INTANGCC",
        "INTANGW",
        "INTANGWR",
        "INTANMSR",
        "INTANMSRR",
        "INTANOTH",
        "INTANOTHR",
        "INTINCYQ",
        "INTINCA",
        "IOTNII",
        "IOTNIIR",
        "IOTNIIQ",
        "IOTNIIQR",
        "ISECZ",
        "ISECZR",
        "ISECZQ",
        "ISECZQR",
        "ISERCHGQ",
        "ISERCHGQR",
        "ISERFEE",
        "ISERFEER",
        "ISERFEEQ",
        "ISERFEEQR",
        "IVENCAP",
        "IVENCAPR",
        "IVENCAPQ",
        "IVENCAPQR",
        "LAG",
        "LAGR",
        "LCI",
        "LCIR",
        "LCON",
        "LCONR",
        "LIABFOR",
        "LNAG1",
        "LNAG1R",
        "LNAG2",
        "LNAG2R",
        "LNAG3",
        "LNAG3R",
        "LNAG4",
        "LNAG4R",
        "LNAG5",
        "LNAG22",
        "LNAG1N",
        "LNAG1NR",
        "LNAG2N",
        "LNAG2NR",
        "LNAG3N",
        "LNAG3NR",
        "LNAG4N",
        "LNAG4NR",
        "LNAGFOR",
        "LNAGFORR",
        "LNATRESR",
        "LNAUTO2",
        "LNAUTO5",
        "LNCI1",
        "LNCI1R",
        "LNCI2",
        "LNCI2R",
        "LNCI3",
        "LNCI3R",
        "LNCI4",
        "LNCI4R",
        "LNCI5",
        "LNCI22",
        "LNCI1N",
        "LNCI1NR",
        "LNCI2N",
        "LNCI2NR",
        "LNCI3N",
        "LNCI3NR",
        "LNCI4N",
        "LNCI4NR",
        "LNCIFOR",
        "LNCIFORR",
        "LNCINUS",
        "LNCINUSF",
        "LNCINUSFR",
        "LNCOMRE",
        "LNCOMRER",
        "LNCOMRE2",
        "LNCOMRE5",
        "LNCON2",
        "LNCON5",
        "LNCONFOR",
        "LNCONFORR",
        "LNCONORP",
        "LNCONOT2",
        "LNCONOT5",
        "LNCONRP",
        "LNCONRPR",
        "LNCONTRA",
        "LNCONTRAR",
        "LNCRCD2",
        "LNCRCD5",
        "LNDEPAC",
        "LNDEPACD",
        "LNDEPAOBK",
        "LNDEPAOBKR",
        "LNDEPCB",
        "LNDEPCBF",
        "LNDEPCBFR",
        "LNDEPFC",
        "LNDEPFCF",
        "LNDEPFCFR",
        "LNDEPFUS",
        "LNDEPUS",
        "LNDEPUSB",
        "LNDEPUSF",
        "LNDEPUSFR",
        "LNEXAMT",
        "LNEXAMTR",
        "LNFGFOR",
        "LNFGFORR",
        "LNLSDEPR",
        "LNLSFOR",
        "LNLSFORR",
        "LNLSGR5",
        "LNLSGRF",
        "LNLSGRFR",
        "LNLSNTV",
        "LNLSNQR",
        "LNLSSALE",
        "LNLSSALER",
        "LNPLEDGE",
        "LNPLEDGER",
        "LNMUNIF",
        "LNMUNIFR",
        "LNOT1T3",
        "LNOT1T3R",
        "LNOT3LES",
        "LNOT3LESR",
        "LNOT3T5",
        "LNOT3T5R",
        "LNOT3T12",
        "LNOT3T12R",
        "LNOT5T15",
        "LNOT5T15R",
        "LNOTCI2",
        "LNOTCI5",
        "LNOTHERF",
        "LNOTHERFR",
        "LNOTOV15",
        "LNOTOV15R",
        "LNREAG1",
        "LNREAG1R",
        "LNREAG2",
        "LNREAG2R",
        "LNREAG3",
        "LNREAG3R",
        "LNREAG4",
        "LNREAG4R",
        "LNREAG1N",
        "LNREAG1NR",
        "LNREAG2N",
        "LNREAG2NR",
        "LNREAG3N",
        "LNREAG3NR",
        "LNREAG4N",
        "LNREAG4NR",
        "LNRECNFM",
        "LNRECNFMR",
        "LNRECNOT",
        "LNRECNOTR",
        "LNREOTH",
        "LNREOTH2",
        "LNREOTH5",
        "LNRENR1",
        "LNRENR1R",
        "LNRENR2",
        "LNRENR2R",
        "LNRENR3",
        "LNRENR3R",
        "LNRENR4",
        "LNRENR4R",
        "LNRENR1N",
        "LNRENR1NR",
        "LNRENR2N",
        "LNRENR2NR",
        "LNRENR3N",
        "LNRENR3NR",
        "LNRENR4N",
        "LNRENR4NR",
        "LNRENROT",
        "LNRENROTR",
        "LNRENROW",
        "LNRENROWR",
        "LNRENUS",
        "LNRENUSR",
        "LNRERSF1",
        "LNRERSF1R",
        "LNRERSF2",
        "LNRERSF2R",
        "LNRERSFM",
        "LNRERSFMR",
        "LNRESNCR",
        "LNRS1T3",
        "LNRS1T3R",
        "LNRS3LES",
        "LNRS3LESR",
        "LNRS3T5",
        "LNRS3T5R",
        "LNRS3T12",
        "LNRS3T12R",
        "LNRS5T15",
        "LNRS5T15R",
        "LNRSOV15",
        "LNRSOV15R",
        "LNSB",
        "LNSBR",
        "LNSERV",
        "LNSERVR",
        "LOCCOM",
        "LOCCOMR",
        "LOCFPSB",
        "LOCFPSBR",
        "LOCFPSBK",
        "LOCFPSBKR",
        "LOCFSB",
        "LOCFSBR",
        "LOCFSBK",
        "LOCFSBKR",
        "LOCPSB",
        "LOCPSBR",
        "LOCPSBK",
        "LOCPSBKR",
        "LOREGTY",
        "LOREGTYR",
        "LOTH",
        "LOTHR",
        "LREAG",
        "LREAGR",
        "LRECONS",
        "LRECONSR",
        "LREMULT",
        "LREMULTR",
        "LRENRES",
        "LRENRESR",
        "LRERES",
        "LRERESR",
        "LSALNLS",
        "LSALNLSR",
        "LSAOA",
        "LSAOAR",
        "LSAORE",
        "LSAORER",
        "LSASCDBT",
        "LSASCDBTR",
        "LSFOR",
        "LSFORR",
        "MSA",
        "MSRECE",
        "MSRECER",
        "MSRESFCL",
        "MSRESFCLR",
        "MSRNRECE",
        "MSRNRECER",
        "NAAG",
        "NAAGR",
        "NAAGSM",
        "NAAGSMR",
        "NAASSET",
        "NAASSETR",
        "NAAUTO",
        "NAAUTOR",
        "NACI",
        "NACIR",
        "NACINUS",
        "NACINUSR",
        "NACON",
        "NACONR",
        "NACONOTH",
        "NACONOTHR",
        "NACRCD",
        "NACRCDR",
        "NADEP",
        "NADEPR",
        "NADEPNUS",
        "NADEPNUSR",
        "NAFG",
        "NAFGR",
        "NAGTY",
        "NAGTYR",
        "NAGTYGNM",
        "NAGTYGNMR",
        "NAGTYPAR",
        "NAGTYPARR",
        "NALAG",
        "NALAGR",
        "NALCI",
        "NALCIR",
        "NALCON",
        "NALCONR",
        "NALGTY",
        "NALGTYR",
        "NALNSALE",
        "NALNSALER",
        "NALOTH",
        "NALOTHR",
        "NALREAG",
        "NALREAGR",
        "NALRECON",
        "NALRECONR",
        "NALREMUL",
        "NALREMULR",
        "NALRENRS",
        "NALRENRSR",
        "NALRERES",
        "NALRERESR",
        "NALS",
        "NALSR",
        "NALTOT",
        "NALTOTR",
        "NAME",
        "NAMEFULL",
        "NAOTHLN",
        "NAOTHLNR",
        "NARE",
        "NARER",
        "NAREAG",
        "NAREAGR",
        "NARECNFM",
        "NARECNFMR",
        "NARECNOT",
        "NARECNOTR",
        "NARECONS",
        "NARECONSR",
        "NAREFOR",
        "NAREFORR",
        "NARELOC",
        "NARELOCR",
        "NAREMULT",
        "NAREMULTR",
        "NARENRES",
        "NARENRESR",
        "NARENROT",
        "NARENROTR",
        "NARENROW",
        "NARENROWR",
        "NARENUS",
        "NARENUSR",
        "NARERES",
        "NARERESR",
        "NARERSF2",
        "NARERSF2R",
        "NARERSFM",
        "NARERSFMR",
        "NARSCI",
        "NARSCONS",
        "NARSLNFM",
        "NARSLNFMR",
        "NARSLNLS",
        "NARSLNLSR",
        "NARSLNLT",
        "NARSLNLTR",
        "NARSMULT",
        "NARSNRES",
        "NARSOTH",
        "NASCDEBT",
        "NASCDEBTR",
        "NCAG",
        "NCAUTO",
        "NCCI",
        "NCCOMRER",
        "NCCOMRE",
        "NCCON",
        "NCCONOTH",
        "NCCRCD",
        "NCDEP",
        "NCFG",
        "NCGTYPAR",
        "NCLNLSR",
        "NCLS",
        "NCOTHLN",
        "NCRE",
        "NCRECONR",
        "NCRECONS",
        "NCRELOC",
        "NCRELOCR",
        "NCREMULR",
        "NCREMULT",
        "NCRENRER",
        "NCRENRES",
        "NCRER",
        "NCRERESO",
        "NCREREOR",
        "NCRERES",
        "NCRERESR",
        "NETGNAST",
        "NETGNASTR",
        "NTGLFXAQ",
        "NTGLFXAQR",
        "NETGNSLN",
        "NETGNSLNR",
        "NTGLLNQ",
        "NTGLLNQR",
        "NETGNSRE",
        "NETGNSRER",
        "NTGLREQ",
        "NTGLREQR",
        "NETINCA",
        "NIMY",
        "NIMYQ",
        "NOIJ",
        "NOIJR",
        "NOIJY",
        "NOIJYQ",
        "NOIJA",
        "NOIJQ",
        "NOIJQA",
        "NOIJQR",
        "NONIIAY",
        "NONIIAYQ",
        "NONIIA",
        "NONIIQ",
        "NONIIQA",
        "NONIIQR",
        "NONIXAY",
        "NONIXAYQ",
        "NONIXA",
        "NPERF",
        "NPERFV",
        "NTAG",
        "NTAGR",
        "NTAGA",
        "NTAGQ",
        "NTAGQR",
        "NTAGSM",
        "NTAGSMR",
        "NTAGSMQ",
        "NTAGSMQR",
        "NTAUTO",
        "NTAUTOR",
        "NTAUTOA",
        "NTAUTOQ",
        "NTAUTOLNQR",
        "NTAUTOQR",
        "NTCI",
        "NTCIR",
        "NTCIA",
        "NTCINUS",
        "NTCINUSR",
        "NTCINUSQ",
        "NTCINUSQR",
        "NTCIQ",
        "NTCIQR",
        "NTCOMRER",
        "NTCOMREQ",
        "NTCOMREA",
        "NTCON",
        "NTCONR",
        "NTCONA",
        "NTCONOTA",
        "NTCONOTH",
        "NTCONOTHR",
        "NTCONOTQ",
        "NTCONOTQR",
        "NTCONQ",
        "NTCONQR",
        "NTCONTQR",
        "NTCRCD",
        "NTCRCDR",
        "NTCRCDA",
        "NTCRCDQ",
        "NTCRCDQR",
        "NTDEP",
        "NTDEPR",
        "NTDEPNUS",
        "NTDEPNUSR",
        "NTDEPNUQ",
        "NTDEPNUQR",
        "NTDEPQ",
        "NTDEPQR",
        "NTFORGV",
        "NTFORGVR",
        "NTFORGVQ",
        "NTFORGVQR",
        "NTINCHPP",
        "NTINCL",
        "NTINCLQ",
        "NTLNLSA",
        "NTINQHPP",
        "NTLNLSR",
        "NTLNLSQR",
        "NTLS",
        "NTLSR",
        "NTLSQ",
        "NTLSQR",
        "NTOTHER",
        "NTOTHERR",
        "NTOTHQ",
        "NTOTHQR",
        "NTRCDSM",
        "NTRCDSMR",
        "NTRCOMOT",
        "NTRCOMOTR",
        "NTRE",
        "NTREMUQA",
        "NTRECOQA",
        "NTRELNR",
        "NTREQ",
        "NTREQA",
        "NTRERQ",
        "NTREAG",
        "NTREAGR",
        "NTREAGQ",
        "NTREA",
        "NTREAGQR",
        "NTRECNFM",
        "NTRECNOT",
        "NTRECONQ",
        "NTRECONQR",
        "NTRECONS",
        "NTRECOSA",
        "NTRECONSR",
        "NTRECOSR",
        "NTRECOQR",
        "NTREFOR",
        "NTREFORR",
        "NTREFORQ",
        "NTREFORQR",
        "NTRELOC",
        "NTRELOCLNR",
        "NTRELOCQ",
        "NTRELOCA",
        "NTRELOCQR",
        "NTRELOCRQ",
        "NTRELOCR",
        "NTREMULQ",
        "NTREMULA",
        "NTREMULQR",
        "NTREMULR",
        "NTREMUQR",
        "NTREMULT",
        "NTREMULTR",
        "NTRENRES",
        "NTRENRESR",
        "NTRENROT",
        "NTRENROW",
        "NTRENRSA",
        "NTRENRSQ",
        "NTRENRSQR",
        "NTRENRSR",
        "NTRENRQR",
        "NTRENUS",
        "NTRENUSR",
        "NTRENUSQ",
        "NTREOTHA",
        "NTRENUSQR",
        "NTREOTHR",
        "NTREOTHRQR",
        "NTREOTQA",
        "NTRER",
        "NTREQR",
        "NTRERES",
        "NTRERESLNR",
        "NTRERESQ",
        "NTRERESA",
        "NTRERESQR",
        "NTRERESR",
        "NTRERESRQ",
        "NTRERSF2",
        "NTRERSF2R",
        "NTRERS2Q",
        "NTRERS2QR",
        "NTRERSFM",
        "NTRERSFMR",
        "NTRERSFQ",
        "NTRERSFQR",
        "NTREOFFDOM",
        "NTREOFFDOMR",
        "NTREOFFDOMQ",
        "NTREOFFDOMQR",
        "NTRFC",
        "NTRFCFG",
        "NTRFCFGR",
        "NTRFG",
        "NTRSMMDA",
        "NTRSMMDAR",
        "NTRSOTH",
        "NTRSOTHR",
        "OAIENC",
        "OALIFGEN",
        "OALIFGENR",
        "OALIFHYB",
        "OALIFHYBR",
        "OALIFINS",
        "OALIFINSR",
        "OALIFSEP",
        "OALIFSEPR",
        "OBSDIR",
        "OREAG",
        "OREAGR",
        "ORECONS",
        "ORECONSR",
        "OREGNMA",
        "OREINV",
        "OREINVR",
        "OREMULT",
        "OREMULTR",
        "ORENRES",
        "ORENRESR",
        "OREOTH",
        "OREOTHR",
        "OREOTHF",
        "OREOTHFR",
        "ORERES",
        "ORERESR",
        "OTHBORF",
        "OTHFFC",
        "OTHFFCR",
        "OTHNVS",
        "OTHOFFBS",
        "OTHOFFBSR",
        "OTHPOC",
        "OTHWOC",
        "OTSREGNM",
        "OWNCRCI",
        "OWNCRCRD",
        "OWNCRHEL",
        "OWNDRCI",
        "OWNDRCRD",
        "OWNDRHEL",
        "OWNLNCI",
        "OWNLNCRD",
        "OWNLNHEL",
        "OWNP3CI",
        "OWNP3CRD",
        "OWNP3HEL",
        "OWNP9CI",
        "OWNP9CRD",
        "OWNP9HEL",
        "OWNSCCI",
        "OWNSCCRD",
        "OWNSCHEL",
        "P3AG",
        "P3AGR",
        "P3AGSM",
        "P3AGSMR",
        "P3ASSET",
        "P3ASSETR",
        "P3AUTO",
        "P3AUTOR",
        "P3CI",
        "P3CIR",
        "P3CINUS",
        "P3CINUSR",
        "P3CON",
        "P3CONR",
        "P3CONOTH",
        "P3CONOTHR",
        "P3CRCD",
        "P3CRCDR",
        "P3DEP",
        "P3DEPR",
        "P3DEPNUS",
        "P3DEPNUSR",
        "P3FG",
        "P3FGR",
        "P3GTY",
        "P3GTYR",
        "P3GTYGNM",
        "P3GTYGNMR",
        "P3GTYPAR",
        "P3GTYPARR",
        "P3LAG",
        "P3LAGR",
        "P3LCI",
        "P3LCIR",
        "P3LCON",
        "P3LCONR",
        "P3LGTY",
        "P3LGTYR",
        "P3LNSALE",
        "P3LNSALER",
        "P3LOTH",
        "P3LOTHR",
        "P3LREAG",
        "P3LREAGR",
        "P3LRECON",
        "P3LRECONR",
        "P3LREMUL",
        "P3LREMULR",
        "P3LRENRS",
        "P3LRENRSR",
        "P3LRERES",
        "P3LRERESR",
        "P3LS",
        "P3LSR",
        "P3LTOT",
        "P3LTOTR",
        "P3OTHLN",
        "P3OTHLNR",
        "P3RE",
        "P3RER",
        "P3REAG",
        "P3REAGR",
        "P3RECNFM",
        "P3RECNFMR",
        "P3RECNOT",
        "P3RECNOTR",
        "P3RECONS",
        "P3RECONSR",
        "P3REFOR",
        "P3REFORR",
        "P3RELOC",
        "P3RELOCR",
        "P3REMULT",
        "P3REMULTR",
        "P3RENRES",
        "P3RENRESR",
        "P3RENROT",
        "P3RENROTR",
        "P3RENROW",
        "P3RENROWR",
        "P3RENUS",
        "P3RENUSR",
        "P3RERES",
        "P3RERESR",
        "P3RERSF2",
        "P3RERSF2R",
        "P3RERSFM",
        "P3RERSFMR",
        "P3RSCI",
        "P3RSCONS",
        "P3RSLNFM",
        "P3RSLNFMR",
        "P3RSLNLS",
        "P3RSLNLSR",
        "P3RSLNLT",
        "P3RSLNLTR",
        "P3RSMULT",
        "P3RSNRES",
        "P3RSOTH",
        "P3SCDEBT",
        "P3SCDEBTR",
        "P9AG",
        "P9AGR",
        "P9AGSM",
        "P9AGSMR",
        "P9ASSET",
        "P9ASSETR",
        "P9AUTO",
        "P9AUTOR",
        "P9CI",
        "P9CIR",
        "P9CINUS",
        "P9CINUSR",
        "P9CON",
        "P9CONR",
        "P9CONOTH",
        "P9CONOTHR",
        "P9CRCD",
        "P9CRCDR",
        "P9DEP",
        "P9DEPR",
        "P9DEPNUS",
        "P9DEPNUSR",
        "P9FG",
        "P9FGR",
        "P9GTY",
        "P9GTYR",
        "P9GTYGNM",
        "P9GTYGNMR",
        "P9GTYPAR",
        "P9GTYPARR",
        "P9LAG",
        "P9LAGR",
        "P9LCI",
        "P9LCIR",
        "P9LCON",
        "P9LCONR",
        "P9LGTY",
        "P9LGTYR",
        "P9LNSALE",
        "P9LNSALER",
        "P9LOTH",
        "P9LOTHR",
        "P9LREAG",
        "P9LREAGR",
        "P9LRECON",
        "P9LRECONR",
        "P9LREMUL",
        "P9LREMULR",
        "P9LRENRS",
        "P9LRENRSR",
        "P9LRERES",
        "P9LRERESR",
        "P9LS",
        "P9LSR",
        "P9LTOT",
        "P9LTOTR",
        "P9OTHLN",
        "P9OTHLNR",
        "P9RE",
        "P9RER",
        "P9REAG",
        "P9REAGR",
        "P9RECNFM",
        "P9RECNFMR",
        "P9RECNOT",
        "P9RECNOTR",
        "P9RECONS",
        "P9RECONSR",
        "P9REFOR",
        "P9REFORR",
        "P9RELOC",
        "P9RELOCR",
        "P9REMULT",
        "P9REMULTR",
        "P9RENRES",
        "P9RENRESR",
        "P9RENROT",
        "P9RENROTR",
        "P9RENROW",
        "P9RENROWR",
        "P9RENUS",
        "P9RENUSR",
        "P9RERES",
        "P9RERESR",
        "P9RERSF2",
        "P9RERSF2R",
        "P9RERSFM",
        "P9RERSFMR",
        "P9RSCI",
        "P9RSCONS",
        "P9RSLNFM",
        "P9RSLNFMR",
        "P9RSLNLS",
        "P9RSLNLSR",
        "P9RSLNLT",
        "P9RSLNLTR",
        "P9RSMULT",
        "P9RSNRES",
        "P9RSOTH",
        "P9SCDEBT",
        "P9SCDEBTR",
        "PARTACQU",
        "PARTCONV",
        "PARTCONVR",
        "RB2LNRES",
        "RB2LNRESR",
        "RBC",
        "RBCT1",
        "RBCT2",
        "RBCT2R",
        "RBCT1C",
        "RBCT1CER",
        "RBCT1J",
        "RBCT1JR",
        "RBC1AAJ",
        "RBC1RWAJ",
        "RBCRWAJ",
        "REPOPURF",
        "REPOSLDF",
        "ROEINJR",
        "RSCI",
        "RSCONS",
        "RSLNLS",
        "RSLNLSR",
        "RSLNLTOT",
        "RSLNLTOTR",
        "RSLNREFM",
        "RSLNREFMR",
        "RSMULT",
        "RSNRES",
        "RSOTHER",
        "RSSDID",
        "RT",
        "RTFFC",
        "RTNVS",
        "RTPOC",
        "RTWOC",
        "RWAJ",
        "RWAJT",
        "RWAJTR",
        "SCABS",
        "SCABSR",
        "SCAF",
        "SCAFR",
        "SCAOT",
        "SCCMMB",
        "SCCMOG",
        "SCCMOGR",
        "SCCMOT",
        "SCCMOTR",
        "SCCMPT",
        "SCCMPTR",
        "SCCOL",
        "SCCOLR",
        "SCCPTG",
        "SCCPTGR",
        "SCEQFV",
        "SCEQFVR",
        "SCFMN",
        "SCFMNR",
        "SCGNM",
        "SCGNMR",
        "SCGTY",
        "SCGTYR",
        "SCHA",
        "SCHAR",
        "SCHTMRES",
        "SCHTMRESR",
        "SCLENT",
        "SCLENTR",
        "SCNM1T3",
        "SCNM1T3R",
        "SCNM3LES",
        "SCNM3LESR",
        "SCNM3T5",
        "SCNM3T5R",
        "SCNM3T12",
        "SCNM3T12R",
        "SCNM5T15",
        "SCNM5T15R",
        "SCNMOV15",
        "SCNMOV15R",
        "SCO3YLES",
        "SCO3YLESR",
        "SC1LES",
        "SC1LESR",
        "SCODOT",
        "SCODOTR",
        "SCODPI",
        "SCODPIR",
        "SCOOV3Y",
        "SCOOV3YR",
        "SCPLEDGE",
        "SCPLEDGER",
        "SCPT1T3",
        "SCPT1T3R",
        "SCPT3LES",
        "SCPT3LESR",
        "SCPT3T5",
        "SCPT3T5R",
        "SCPT3T12",
        "SCPT3T12R",
        "SCPT5T15",
        "SCPT5T15R",
        "SCPTOV15",
        "SCPTOV15R",
        "SCRDEBT",
        "SCRDEBTR",
        "SCSFP",
        "SCSFPR",
        "SCSNHAA",
        "SCSNHAAR",
        "SCSNHAF",
        "SCSNHAFR",
        "SCSPN",
        "SZ30AUTO",
        "SZ30AUTOR",
        "SZ30CI",
        "SZ30CIR",
        "SZ30CON",
        "SZ30CONR",
        "SZ30CRCD",
        "SZ30CRCDR",
        "SZ30HEL",
        "SZ30HELR",
        "SZ30OTH",
        "SZ30OTHR",
        "SZ30RES",
        "SZ30RESR",
        "SZ90AUTO",
        "SZ90AUTOR",
        "SZ90CI",
        "SZ90CIR",
        "SZ90CON",
        "SZ90CONR",
        "SZ90CRCD",
        "SZ90CRCDR",
        "SZ90HEL",
        "SZ90HELR",
        "SZ90OTH",
        "SZ90OTHR",
        "SZ90RES",
        "SZ90RESR",
        "SZCRAUTO",
        "SZCRAUTOR",
        "SZCRCDFE",
        "SZCRCDFER",
        "SZCRCI",
        "SZCRCIR",
        "SZCRCON",
        "SZCRCONR",
        "SZCRCRCD",
        "SZCRCRCDR",
        "SZCRHEL",
        "SZCRHELR",
        "SZCROTH",
        "SZCROTHR",
        "SZCRRES",
        "SZCRRESR",
        "SZDRAUTO",
        "SZDRAUTOR",
        "SZDRCI",
        "SZDRCIR",
        "SZDRCON",
        "SZDRCONR",
        "SZDRCRCD",
        "SZDRCRCDR",
        "SZDRHEL",
        "SZDRHELR",
        "SZDROTH",
        "SZDROTHR",
        "SZDRRES",
        "SZISLAUT",
        "SZISLAUTR",
        "SZISLCCD",
        "SZISLCCDR",
        "SZISLCI",
        "SZISLCIR",
        "SZISLCON",
        "SZISLCONR",
        "SZISLHEL",
        "SZISLHELR",
        "SZISLOTH",
        "SZISLOTHR",
        "SZISLRES",
        "SZISLRESR",
        "SZLAUTO",
        "SZLAUTOR",
        "SZLNCI",
        "SZLNCIR",
        "SZLNCON",
        "SZLNCONR",
        "SZLNCRCD",
        "SZLNCRCDR",
        "SZLNHEL",
        "SZLNHELR",
        "SZLNOTH",
        "SZLNOTHR",
        "SZLNRES",
        "SZLNRESR",
        "SZUCAUTO",
        "SZUCCI",
        "SZUCCON",
        "SZUCCRCD",
        "SZUCHEL",
        "SZUCOTH",
        "SZUCRES",
        "TCAMA",
        "TCAMANUM",
        "TCANMA",
        "TCANMNUM",
        "TCANUM",
        "TCANUMD",
        "TCAPAO",
        "TCAPAOD",
        "TCATNUM",
        "TCDEMV",
        "TCDENUM",
        "TCIEMV",
        "TCIENUM",
        "TCMBMV",
        "TCMBNUM",
        "TCSBMV",
        "TCSBNUM",
        "TCSNMA",
        "TCSNMNUM",
        "TCSOMV",
        "TCSONUM",
        "TCSTMV",
        "TCSTNUM",
        "TCTBMV",
        "TCTBNUM",
        "TCTOTMV",
        "TCTOTNUM",
        "TEBMA",
        "TEBMANUM",
        "TEBNMA",
        "TEBNMNUM",
        "TECMA",
        "TECMANUM",
        "TECNMA",
        "TECNMNUM",
        "TECPS",
        "TEEQF",
        "TEI",
        "TEMATOT",
        "TEMISC",
        "TEMMF",
        "TENI",
        "TEOTHB",
        "TEOTHF",
        "TERE",
        "TEREMTG",
        "TESCMUN",
        "TESCUS",
        "TESTO",
        "TETOT",
        "TETRF",
        "TEUF",
        "TFEMA",
        "TFEMANUM",
        "TFENMA",
        "TFENMNUM",
        "TICA",
        "TICS",
        "TIEB",
        "TIEC",
        "TIFE",
        "TIMA",
        "TIMMA",
        "TIMMANUM",
        "TIMNMA",
        "TIMNMNUM",
        "TINTRA",
        "TIOF",
        "TIOR",
        "TIP",
        "TIR",
        "TITOTF",
        "TMAF",
        "TMAFNUM",
        "TMASMF",
        "TMASMFN",
        "TNI",
        "TNL",
        "TNMAF",
        "TNMNUMF",
        "TOCPS",
        "TOEQF",
        "TOFMA",
        "TOFMANUM",
        "TOFNMA",
        "TOFNMNUM",
        "TOI",
        "TOMATOT",
        "TOMISC",
        "TOMMF",
        "TONI",
        "TOOTHB",
        "TOOTHF",
        "TORE",
        "TOREMTG",
        "TORMA",
        "TORMANUM",
        "TORNMA",
        "TORNMNUM",
        "TOSCMUN",
        "TOSCUS",
        "TOSTO",
        "TOTRF",
        "TOUF",
        "TPICPS",
        "TPIEQF",
        "TPII",
        "TPIMATOT",
        "TPIMISC",
        "TPIMMF",
        "TPINI",
        "TPIOTHB",
        "TPIOTHF",
        "TPIRE",
        "TPIREMTG",
        "TPISCMUN",
        "TPISCUS",
        "TPISTO",
        "TPITRF",
        "TPIUF",
        "TPMA",
        "TPMANUM",
        "TPNMA",
        "TPNMNUM",
        "TREXER",
        "TRFOR",
        "TRHMA",
        "TRHMANUM",
        "TRHNMA",
        "TRHNMNUM",
        "TRLREVAL",
        "TRLREVALR",
        "TRNCBO",
        "TRNCBOR",
        "TRNFC",
        "TRNFCFG",
        "TRNFCFGR",
        "TRNFG",
        "TRNNIA",
        "TRNNIAR",
        "TRNNIN",
        "TRPOWER",
        "TRREVALD",
        "TRREVALF",
        "TRREVALSUM",
        "TRREVALSUMR",
        "TTMA",
        "TTNANUM",
        "TTNMA",
        "TTNMNUM",
        "UC",
        "UCR",
        "UCCOMRE",
        "UCCOMRER",
        "UCCOMRES",
        "UCCOMRESR",
        "UCCOMREU",
        "UCCOMREUR",
        "UCCRCD",
        "UCCRCDR",
        "UCLN",
        "UCLOC",
        "UCLOCR",
        "UCOTHER",
        "UCOTHERR",
        "UCOVER1",
        "UCOVER1R",
        "UCSC",
        "UCSCR",
        "UCSZAUTO",
        "UCSZCI",
        "UCSZCON",
        "UCSZCRCD",
        "UCSZHEL",
        "UCSZOTH",
        "UCSZRES",
        "UNINCFOR",
        "UNINCFORR",
        "VOLIAB",
        "VOLIABR",
        "ZIP",
        "LIPNMTG",
        "UYANMTG",
        "ILNLS",
        "UNIT",
        "PTAXNETINC",
        "PTAXNETINCR",
        "PTAXNETINCQ",
        "PTAXNETINCQR",
        "ADDNONII",
        "ADDNONIIR",
        "ADDNONIIQ",
        "ADDNONIIQR",
        "AVMMLF",
        "AVMMLFR",
        "AVPPPPLG",
        "AVPPPPLGR",
        "MMLFBAL",
        "MMLFBALR",
        "PPPLFOV1",
        "PPPLFOV1R",
        "PPPLNBAL",
        "PPPLNBALR",
        "PPPLNNUM",
        "PPPLNNUMR",
        "PPPLNPLG",
        "PPPLNPLGR",
        "PPPLF1LS",
        "PPPLF1LSR",
        "IDNTCIR",
        "IDNTCIQR",
        "IDNTCONR",
        "IDNTCRDR",
        "IDNTCOOR",
        "IDNTCOOQR",
        "IDNTCRDQR",
        "INSTCNT",
        "IDNTILR",
        "IDOTHNII",
        "NTAUTOPR",
        "NTCONOTR",
        "IDERNCVR",
        "IDERNCVQR",
        "EQCDIVNTINC",
        "NACDIR",
        "NTCOMREQR",
        "NTALLOTHNUM",
        "NTALLOTHDEN",
        "NTALLOTHR",
        "NTALLOTHQR",
        "IDNCCOOR",
        "IDNCOTHR",
        "IDNCCIR",
        "IDNCCONR",
        "IDNCCRDR",
        "IDNCATOR",
        "IDNTATOR",
        "IDNTCOTR",
        "IDDEPINR",
        "IDDIVNIR",
        "IDNCCOTR",
        "INTINCY",
        "IDNCGTPR",
        "IDLNCORR",
        "IDT1CNOCB",
        "IDT1JNOCB",
        "IDT1CER",
        "IDT1RWAJR",
        "SCEQNFT",
        "SCRMBPI",
        "SCRMBPIR",
        "SCUSO",
        "SCUSOR",
        "SCCMOS",
        "SCCMOSR",
        "SCTATFR",
        "LNLSGRS",
        "LNLSGRSR",
        "AOA",
        "AOAR",
        "ESTINS",
        "ESTINSR",
        "P3RELNDO",
        "P3RELNDOR",
        "P9RELNDO",
        "P9RELNDOR",
        "NARELNDO",
        "NARELNDOR",
        "STCNTY",
        "CBSA",
        "INSDATE",
        "UPDDATE",
        "ASSETR",
        "AVASSET",
        "BROINSLG",
        "CT1AJTOT",
        "CT1BADJ",
        "DEP2",
        "DEP5",
        "DEPIY1",
        "ECD100",
        "ECD100A",
        "ECD100Q",
        "EFREPPA",
        "EOTHTIMA",
        "EOTHTIME",
        "EOTHTIMQ",
        "EQUPGR",
        "ESAVDP",
        "ESAVDPA",
        "ESAVDPQ",
        "ESUBNDA",
        "ETRANDEP",
        "ETRANDPA",
        "ETRANDPQ",
        "ETTLOTBA",
        "ETTLOTBQ",
        "FFPUR",
        "IBEFTXA",
        "IGLSCA",
        "IGLSCAQ",
        "IGLSCH",
        "ILNA",
        "ILNLSA",
        "ILNLSQ",
        "ILNLSXA",
        "ILNLSXQ",
        "ILNMUNIQ",
        "ILNQ",
        "ISCA",
        "ISERCHGA",
        "ITAXA",
        "ITAXQA",
        "LNCDT1R",
        "LNCIT1R",
        "LNCONT1R",
        "LNLSRES",
        "LNREAG5",
        "LNRERT1R",
        "NCREAG",
        "NCRECNFM",
        "NCRECNOT",
        "NCRENROT",
        "NCRENROW",
        "NCRERS2R",
        "NCRERSF2",
        "NCRERSFM",
        "NCRERSFR",
        "NCRSLNLS",
        "NOIQ",
        "NTAGQA",
        "NTAGSMA",
        "NTAGSMQA",
        "NTCIQA",
        "NTCOMRE",
        "NTCOMRQA",
        "NTCONQA",
        "NTCRCDQA",
        "NTIRTQ",
        "NTRCDSMJ",
        "NTREAGA",
        "NTREAGQA",
        "OBOR",
        "OBOR2",
        "OBOR5",
        "OTHBFH03",
        "OTHBFH13",
        "P3COMRE",
        "P3RECONR",
        "P3RERS2R",
        "RBCEQUP",
        "RBCT1W",
        "REPOPUR",
        "SC2",
        "SC5",
        "SCMUNIAA",
        "SCMUNIAF",
        "SCMUNIHA",
        "SCMUNIHF",
    ];

    #[allow(unused_variables)] // the `query` parameter is unused if there are no endpoint-specific parameters
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
        
        if let Some(val) = &self.agg_by {
            query.insert("agg_by".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_term_fields {
            query.insert("agg_term_fields".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_sum_fields {
            query.insert("agg_sum_fields".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_limit {
            query.insert("agg_limit".to_string(), val.to_string());
        }
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/financials` endpoint.
/// Spec: risview_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct FinancialsProperties {
    #[schemars(description = r#"Activity Event Code - Structure activity event code. Merger or closing codes only."#)]
    pub actevt: Option<String>,

    #[schemars(description = r#"Total assets - The sum of all assets owned by the institution including cash, loans, securities, bank premises and other assets. This total does not include off-balance-sheet accounts."#)]
    pub asset: Option<f32>,

    #[schemars(description = r#"BRANCHING - A flag used to indicate whether an institution has branches. 0 means no branches and 1 means it has branches."#)]
    pub branch: Option<f32>,

    #[schemars(description = r#"Call Form Number - TBD"#)]
    pub callform: Option<f32>,

    #[schemars(description = r#"Community Bank - FDIC community banks are identified based on criteria defined in the FDIC Community Banking Study. Using detailed balance sheet and geographic data, the study defines communtiy banks in terms of their traditional relationship banking and limited geographic scope of operations"#)]
    pub cb: Option<String>,

    #[schemars(description = r#"Core Based Statistical Division Number - A metropolitan division is a county or group of counties within a core based statistical area that contains a population of at least 2.5 million. A metropolitan division consists of one or more main/secondary countues that represent an employment center or centers, plus adjacent conuties associated withe the main county or counties through commuting ties."#)]
    pub cbsadiv: Option<f32>,

    #[schemars(description = r#"Core Based Statistical Division Name - A statistical geographic entity consisting of the county or counties associated with at least one core (urbanized area or urban cluster) of at least 10,000 population, plus adjacent counties having a high degree of social and economic integration with the core as measured through commuting ties with the counties containing the core."#)]
    pub cbsaname: Option<String>,

    #[schemars(description = r#"Multi State Offices Flag - Multi State Offices Flag"#)]
    pub stmult: Option<String>,

    #[schemars(description = r#"ADDRESS - ADDRESS"#)]
    pub address: Option<String>,

    #[schemars(description = r#"Community Bank Ratio - "#)]
    pub cblrind: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 1-3 YEARS - "#)]
    pub cd1t3: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 1-3 YEARS RATIO - "#)]
    pub cd1t3r: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 3 MONTH OR LESS - "#)]
    pub cd3les: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 3 MONTH OR LESS RATIO - "#)]
    pub cd3lesr: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY REPRICING OF 3 MONTH OR LESS - "#)]
    pub cd3less: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY REPRICING OF 3 MONTH OR LESS RATIO - "#)]
    pub cd3lessr: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING OVER 3 YEARS - "#)]
    pub cdov3: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING OVER 3 YEARS RATIO - "#)]
    pub cdov3r: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING OVER 3 YEARS - "#)]
    pub cdov3s: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING OVER 3 YEARS RATIO - "#)]
    pub cdov3sr: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING 3-12 MONTHS - "#)]
    pub cd3t12: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING 3-12 MONTHS RATIO - "#)]
    pub cd3t12r: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 3-12 MONTHS - "#)]
    pub cd3t12s: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 3-12 MONTHS RATIO - "#)]
    pub cd3t12sr: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 1-3 YEARS - "#)]
    pub cd1t3s: Option<f32>,

    #[schemars(description = r#"TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 1-3 YEARS RATIO - "#)]
    pub cd1t3sr: Option<f32>,

    #[schemars(description = r#"FDIC Certificate # - A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#)]
    pub cert: Option<f32>,

    #[schemars(description = r#"Directly owned by another bank (CERT) - FDIC certificate number of the parent bank or savings institution with which the reported institution’s financial data has been consolidated. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports.  Click on the certificate number to identify the parent bank or thrift."#)]
    pub certcons: Option<String>,

    #[schemars(description = r#"City of High Holder (Search-Eligible) - City in which the headquarters of the institution's regulatory high holder are physically located. This field can be used for search and filtering."#)]
    pub cityhcr: Option<String>,

    #[schemars(description = r#"Classcode - A number that sub-categorizes a major class of institutions. 3 = National bank, Federal Reserve System (FRS) member; 13 = State commercial bank, FRS member; 15 = State savings, co-op, or insdustrial bank, FRS member; 21 = State commercial bank, not FRS member; 23 = State savings, co-op, or industrial bank, not FRS member; 25 = State mutual commercial bank, not FRS member; 33 =  Federal chartered stock savings bank; 34 = Federal chartered mutual savings bank; 35 = State chartered stock savings and loan association; 36 = State chartered mutual savings and loan association; 37 = Federal chartered stock savings and loan association; 38 = Federal chartered mutual savings and loan association; 41 = State chartered stock savings bank; 42 = State chartered mutual savings bank; 43 = Federal chartered stock savings bank (historical); 44 = Federal chartered mutual savings bank (historical); 50 = OCC chartered nondeposit and/or noninsured trust companies; 51 = Noninsured commercial bank; 52 = Noninsured domestic offices of foreign bank (International Banking Act); 53 = Noninsured industrial bank; 54 = State chartered nondeposit and/or noninsured trust company, not FRS member; 55 = State chartered domestic branches of foreign banks; 56 = OCC chartered domestic branches of foreign banks; 57 = New York investment company; 58 = State chartered nondeposit and/or noninsured trust company, FRS member; 59 = OTS chartered nondeposit and/or noninsured trust company, 61 = Noninsured private bank; 62 = Noninsured loan workout bank, OCC chartered; 63 = Noninsured loan workout bank, state chartered, FRS member; 64 = Noninsured loan workout bank, state chartered, not FRS member; 65 = Other holding company; 71 = Transfer agent; 81 = Noninsured stock savings bank; 82 = Noninsured mutual savings bank; 85 = Noninsured stock savings and loan association; 86 = Noninsured mutual savings and loan association; 89 = Noninsured insurance company; 91 = State chartered credit unions; 92 = Federal chartered credit unions; 93 = Privately insured state credit union."#)]
    pub clcode: Option<f32>,

    #[schemars(description = r#"Closed Institution Flag - A flag used to indicate whether an institution has been closed. 0 is institution not closed. 1 is institution closed."#)]
    pub closed: Option<f32>,

    #[schemars(description = r#"FIPS CMSA Code - The Federal Information Processing Standards Consolidated Metropolitan Statistical Area Code is a number representing the institution location. A CMSA consists of two or more contiguous MSAs with a combined population of over 1 million."#)]
    pub cmsa: Option<f32>,

    #[schemars(description = r#"FIPS Country Code - The Federal Information Processing Standards Alphabetic Code of the country in which the institution is physically located."#)]
    pub cntryalp: Option<String>,

    #[schemars(description = r#"FIPS Country Number - The Federal Information Processing Standards Numeric Code of the country in which the institution is physically located."#)]
    pub cntrynum: Option<f32>,

    #[schemars(description = r#"FIPS County Number - The Federal Information Processing Standards Numeric Code of the county in which the institution is physically located."#)]
    pub cntynum: Option<f32>,

    #[schemars(description = r#"Combined Statistical Area - U.S. CENSUS BUREAU OFFICE OF MANANGEMENT AND BUDGET DEFINES                                   THE COMBINED STATISTICAL AREA (CSA) AS A GEOGRAPHIC ENTITY                                         CONSISTING OF TWO OR MORE ADJACENT CORE BASED STATISTICAL AREAS                                  (CBSAS) WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 15.                                     PAIRS OF CBSAS WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST                                  25 COMBINE AUTOMATICALLY.  PAIRS OF CBSAS WITH EMPLOYMENT                                        INTERCHANGE MEASURES OF AT LEAST 15, BUT LESS THAN 25, MAY                                        COMBINE IF LOCAL OPTION IN BOTH AREAS FAVOR COMBINATION. "#)]
    pub csa: Option<String>,

    #[schemars(description = r#"Denovo Institution - A flag used to indicate whether an institution is a new institution (not a recharter). This flag is set quarterly. For instance, if REPDTE is 3/31/98 and DENOVO equals 1, the institution was a denovo during the first quarter of 1998."#)]
    pub denovo: Option<String>,

    #[schemars(description = r#"Total deposits - The sum of all deposits including demand deposits, money market deposits, other savings deposits, time deposits and deposits in foreign offices."#)]
    pub dep: Option<f32>,

    #[schemars(description = r#"TOTAL DEPOSITS RATIO - "#)]
    pub depr: Option<f32>,

    #[schemars(description = r#"Deposits held in domestic offices - The sum of all domestic office deposits, including demand deposits, money market deposits, other savings deposits and time deposits."#)]
    pub depdom: Option<f32>,

    #[schemars(description = r#"DEPOSITS HELD IN DOM OFF RATIO - "#)]
    pub depdomr: Option<f32>,

    #[schemars(description = r#"Division Flag - A flag used to indicate whether an institution is in a CBSA division. 0 is institution is not in a CBSA division. 1 is institution is in a CBSA division."#)]
    pub division: Option<f32>,

    #[schemars(description = r#"Docket Number - A unique identification number assigned to institutions chartered by the office of thrift supervision or that become members of the federal home loan system."#)]
    pub docket: Option<f32>,

    #[schemars(description = r#"International Activity Flag - A FLAG USED TO INDICATE WHETHER AN INSTITUTION OPERATES ONE OR                                   MORE EDGE ACT OR AGREEMENT CORPORATIONS.  AN EDGE ACT CORPORATION                                 IS A FEDERALLY CHARTERED DOMESTIC ORGANIZATION THAT IS ALLOWED TO                                ENGAGE ONLY IN INTERNATIONAL BANKING OR OTHER FINANCIAL                                          TRANSACTIONS RELATED TO INTERNATIONAL BUSINESS.  AN AGREEMENT CORPORATION IS RESTRICTED, IN GENERAL, TO INTERNATIONAL BANKING OPERATIONS. 0 = NO AFFILIATIONS WITH EDGE ACT CORPORATIONS.                                                                                    1 = AFFILIATED WITH EDGE ACT CORPORATIONS."#)]
    pub edgecode: Option<f32>,

    #[schemars(description = r#"Entity Type - A three digit number indicating the major type or category of an  institution. The entity code is used to categorize an institution by type of financial organization."#)]
    pub enttype: Option<f32>,

    #[schemars(description = r#"Equity capital - Total equity capital (includes preferred and common stock, surplus and undivided profits)."#)]
    pub eq: Option<f32>,

    #[schemars(description = r#"Equity capital - "#)]
    pub eq2: Option<f32>,

    #[schemars(description = r#"EQUITY CAPITAL RATIO - "#)]
    pub eqr: Option<f32>,

    #[schemars(description = r#"Failed Institution Flag - A flag used to indicate whether an institution has failed. Failures include assisted mergers and payoffs."#)]
    pub failed: Option<f32>,

    #[schemars(description = r#"FDIC Compliance Area - A number used to identify the compliance area in which an institution is located."#)]
    pub fdicarea: Option<f32>,

    #[schemars(description = r#"FDIC Compliance Territory - An abbreviation of the current compliance territory where an institution is located (FDIC Compliance Territory). All periods are displayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#)]
    pub fdicterr: Option<String>,

    #[schemars(description = r#"DCA Field Office - The name of the compliance field office to which an institution is assigned. All periods are diplayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#)]
    pub fldofdca: Option<String>,

    #[schemars(description = r#"FFIEC Call Report 31 Filer - A flag (1=yes,0=no) that indicates whether and institution filed an FFIEC 031 Call Report. Commercial banks with domestic and foreign offices are required to file such a report."#)]
    pub form31: Option<String>,

    #[schemars(description = r#"Bank Holding Company Type - A flag used to indicate whether an institution is a member of a multibank holding company 1=yes, 0=no"#)]
    pub hctmult: Option<String>,

    #[schemars(description = r#"Bank Not Member of Hold Company - A flag used to indicated whether an institution is an independent bank. 0 is member of a bank hold company. 1 is not a member of a bank holding company."#)]
    pub hctnone: Option<f32>,

    #[schemars(description = r#"Secondary Insurer - The secondary insurer, insurance agent, or insurance status of an institution."#)]
    pub insagnt2: Option<String>,

    #[schemars(description = r#"TBD - TBD"#)]
    pub insbif: Option<f32>,

    #[schemars(description = r#"Deposit Insurance Fund member - A flag used to indicate whether an institution is insured under the Deposit Insurance Fund (DIF).  As of April 1, 2006 the Bank Insurance Fund (BIF) was merged together with the Savings Institution Insurance Fund (SAIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured BIF and SAIF member institutions that are still active or open are now insured members of DIF.    0 = No, not DIF insured and 1 = Yes, DIF insured.  Note that institutions that became inactive prior to April 1006 will also have zero value."#)]
    pub insdif: Option<String>,

    #[schemars(description = r#"Agricultural lending institution indicator - An indicator specifying whether an institution is primarily an agricultural lending institution."#)]
    pub instag: Option<String>,

    #[schemars(description = r#"Credit Card Institutions - Institutions with total loans greater than 50% of total assets and credit card loans greater than 50% of total loans, including loans that have been securitized and sold."#)]
    pub instcrcd: Option<String>,

    #[schemars(description = r#"SAIF Insured - Institutions who are members of the Savings Association Insurance Fund. As of April 1, 2006 SAIF was merged together with the Bank Insurance Fund (BIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured SAIF member institutions, that are still active or open, are now insured members of DIF."#)]
    pub inssaif: Option<f32>,

    #[schemars(description = r#"MINORITY OWNED INSTITUTIONS - "#)]
    pub minority: Option<f32>,

    #[schemars(description = r#"Ownership Type - Banking institutions fall into one of two ownership types, stock or non-stock. An institution which sells stock to raise capital is called a stock institution. It is owned by the shareholders who benefit from profits earned by the institution. A non-stock institution, or mutual institution, is owned and controlled solely by its depositors. A mutual does not issue capital stock."#)]
    pub mutual: Option<f32>,

    #[schemars(description = r#"Bank Holding Company (Regulatory Top Holder) (Search-Eligible) - Regulatory top holder is assigned by the Federal Reserve Board based on ownership and control percentages. Note: Information on bank holding companies is only as of quarter-end. Regulatory top holder is any company that directly or indirectly owns, controls or has power to vote 25 percent or more of a bank's or direct holding company's shares or  controls in any manner the election of a majority of the directors or trustees of a bank or direct holding company or  exercises a controlling influence over the management or policies of a bank or direct holding company.   Information on Thrift Holding Companies that own Savings Associations but do not own banks is not currently available in the ID System.  Source: Federal Reserve Board National Information Center data base. This field can be used for search and filtering."#)]
    pub namehcr: Option<String>,

    #[schemars(description = r#"Net income - Net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#)]
    pub netinc: Option<f32>,

    #[schemars(description = r#"NET INCOME - RATIO - "#)]
    pub netincr: Option<f32>,

    #[schemars(description = r#"Net income - quarterly - Quarterly net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#)]
    pub netincq: Option<f32>,

    #[schemars(description = r#"Net income - quarterly - "#)]
    pub netincqa: Option<f32>,

    #[schemars(description = r#"NET INCOME - QUARTERLY RATIO - "#)]
    pub netincqr: Option<f32>,

    #[schemars(description = r#"Number of Domestic Offices - The number of domestic offices (including headquarters) operated by active institutions in the 50 states of the U.S.A."#)]
    pub offdom: Option<f32>,

    #[schemars(description = r#"Number of Foreign Offices - The number of foreign offices (outside the U.S.) operated by the institution."#)]
    pub offfor: Option<f32>,

    #[schemars(description = r#"Number of US Offices - The number of offices operated by an FDIC-insured institution in all commonwealths and terrirtories of the US, along with those in freely associated states under the Compact of Free Association"#)]
    pub offoa: Option<f32>,

    #[schemars(description = r#"Directly owned by another bank (CERT) - The PARCERT number identifies the subsidiary institutions parent certificate number. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports."#)]
    pub parcert: Option<String>,

    #[schemars(description = r#"Report Date (Search-Eligible) - The last day of the financial reporting period selected. This field can be used for search and filtering."#)]
    pub l_repdte: Option<String>,

    #[schemars(description = r#"Report Date (Search-Eligible) - The last day of the financial reporting period selected. This field can be used for search and filtering."#)]
    pub repdte_raw: Option<String>,

    #[schemars(description = r#"Report Date (Search-Eligible) - The last day of the financial reporting period selected. This field can be used for search and filtering."#)]
    pub repdte: Option<String>,

    #[schemars(description = r#"REPORT YEAR (Search-Eligible) - This field can be used for search and filtering."#)]
    pub repyear: Option<String>,

    #[schemars(description = r#"Report Date - The financial reporting period selected in CCYYMM format."#)]
    pub risdate: Option<String>,

    #[schemars(description = r#"Return on assets (ROA) - Net income after taxes and extraordinary items (annualized) as a percent of average total assets."#)]
    pub roa: Option<f32>,

    #[schemars(description = r#"Pretax return on assets - Annualized pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#)]
    pub roaptx: Option<f32>,

    #[schemars(description = r#"Quarterly Pretax return on assets - Quarterly pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#)]
    pub roaptxq: Option<f32>,

    #[schemars(description = r#"Quarterly return on assets - Quarterly net income after taxes and extraordinary items as a percent of average total assets."#)]
    pub roaq: Option<f32>,

    #[schemars(description = r#"Return on Equity (ROE) - Annualized net income as a percent of average equity on a consolidated basis.     Note: If retained earnings are  negative, the ratio is shown as NA."#)]
    pub roe: Option<f32>,

    #[schemars(description = r#"Quarterly return on equity - Quarterly net income (including gains or losses on securities and extraordinary items) as a percentage of average total equity capital."#)]
    pub roeq: Option<f32>,

    #[schemars(description = r#"RSSDID - High Regulatory Holder (Search-Eligible) - The unique number assigned by the Federal Reserve Board to the regulatory high holding company of the institution. This field can be used for search and filtering."#)]
    pub rssdhcr: Option<String>,

    #[schemars(description = r#"Asset Concentration Hierarchy - An indicator of an institution's primary specialization in terms of asset concentration"#)]
    pub specgrp: Option<f32>,

    #[schemars(description = r#"Asset Concentration Hierarchy Description - An indicator of an institution's primary specialization in terms of asset concentration Description"#)]
    pub specgrpdesc: Option<String>,

    #[schemars(description = r#"Regulatory holding company state location (Search-Eligible) - State location of the regulatory high holding company (either direct or indirect owner). This field can be used for search and filtering."#)]
    pub stalphcr: Option<String>,

    #[schemars(description = r#"Subchapter S Corporations - The Small Business Job Protection Act of 1996 changed the Internal Revenue Code to allow financial institutions to elect Subchapter S corporation status, beginning in 1997. Banks are required to indicate on the Call Report whether there is currently in effect an election to file under Subchapter S. Thrifts have a similar requirement as of March 1998.  The most important IRS requirements to elect and maintain Subchapter S status are: There can be no more than 75 eligible shareholders and no more than one class of stock. (In general, shareholders can only be individuals, estates, and certain types of trusts. Certain retirement plans and charitable organizations will be eligible in 1998.) All shareholders must consent.  Banks and thrifts converting to Subchapter S status must use the specific charge-off method for tax purposes rather than the reserve method of accounting for bad debts and recapture tax bad debt reserves over a period of six years, if the reserve method had been used prior to conversion. (Note: even though the specific charge-off method is required for tax purposes, an adequate allowance for loan and lease losses must still be maintained on the financial statements and Call Reports.) Banks and thrifts are subject to a built-in gains (BIG) tax, if the aggregate fair market value of assets is greater than their aggregate adjusted bases on the date of conversion to Subchapter S status.     [Banks are required to indicate separately on the Call Report in December of each year, the deferred portion of income taxes reported in net income. For Subchapter S banks, some or all of their deferred tax assets and liabilities may be eliminated upon conversion to Subchapter S status; however, deferred taxes related to the BIG tax and the recapture of bad debt reserves must be recognized.].   A Subchapter S corporation is treated as a pass-through entity, similar to a partnership, for federal income tax purposes. It is generally not subject to any federal income taxes at the corporate level. Its taxable income flows through to its shareholders in proportion to their stock ownership, and the shareholders generally pay federal income taxes on their share of this taxable income. This can have the effect of reducing institutions' reported income tax expense and increasing their after-tax earnings..   The election of Subchapter S status may result in an increase in shareholders' personal tax liabilities. Therefore, S corporations typically increase the amount of earnings distributed as dividends to compensate for higher personal taxes."#)]
    pub subchaps: Option<f32>,

    #[schemars(description = r#" - Beyond having trust powers granted and exercised, institutions with fiduciary assets accounts, income, or other reportable fiduciary related service"#)]
    pub tract: Option<f32>,

    #[schemars(description = r#"Trust Powers - A flag used to indicate an institution's Trust Powers Granted status. 0 = No Trust Power Granted 1 = Trust Power Granted Where Trust Power has been granted specific codes are: 00 - Trust powers not know 10 - Full trust powers granted 11 - Full trust powers granted, exercised 12 - Full trust powers granted, not exercised 20 - Limited trust powers granted 21 - Limited trust powers granted, exercised 22 - Limited trust powers granted, not exercised 30 - Trust powers not granted 31 - Trust powers not granted, but exercised"#)]
    pub trust: Option<String>,

    #[schemars(description = r#"BANKS LIABILITY ON ACCEPTANCES - "#)]
    pub acept: Option<f32>,

    #[schemars(description = r#"ACTIVE INSTITUTION FLAG - "#)]
    pub active: Option<f32>,

    #[schemars(description = r#"INSTITUTION CLASS (Search-Eligible) - A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA). This field can be used for search and filtering."#)]
    pub bkclass: Option<String>,

    #[schemars(description = r#"PREMISES AND FIXED ASSETS - "#)]
    pub bkprem: Option<f32>,

    #[schemars(description = r#"PREMISES AND FIXED ASSETS RATIO - "#)]
    pub bkpremr: Option<f32>,

    #[schemars(description = r#"BROKERED DEP - "#)]
    pub bro: Option<f32>,

    #[schemars(description = r#"BROKERED RATIO - "#)]
    pub bror: Option<f32>,

    #[schemars(description = r#"REPORT DATE (CCYYMM) - "#)]
    pub callym: Option<f32>,

    #[schemars(description = r#"CASH & DUE FROM DEPOSITORY INST - "#)]
    pub chbal: Option<f32>,

    #[schemars(description = r#"CASH & DUE FROM DEPOSITORY INST RATIO - "#)]
    pub chbalr: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING CASH & DUE - "#)]
    pub chbali: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING CASH & DUE RATIO - "#)]
    pub chbalir: Option<f32>,

    #[schemars(description = r#"CHARTER AGENT - "#)]
    pub chrtagnt: Option<String>,

    #[schemars(description = r#"RTC CONSERVATORSHIP FLAG - "#)]
    pub conserve: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS RECOVERIES - "#)]
    pub crlnls: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS RECOVERIES RATIO - "#)]
    pub crlnlsr: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS RECOVERIES QUARTERLY - "#)]
    pub crlnlsq: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS RECOVERIES QUARTERLY RATIO - "#)]
    pub crlnlsqr: Option<f32>,

    #[schemars(description = r#"CUSTOMERS ACCEPTANCES - "#)]
    pub cusli: Option<f32>,

    #[schemars(description = r#"DDA TRANS-TOTAL - "#)]
    pub ddt: Option<f32>,

    #[schemars(description = r#"DDA TRANS-TOTAL RATIO - "#)]
    pub ddtr: Option<f32>,

    #[schemars(description = r#"TOTAL DEPOSITS-FOR - "#)]
    pub depfor: Option<f32>,

    #[schemars(description = r#"TOTAL DEPOSITS-FOR RATIO - "#)]
    pub depforr: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING DEP - "#)]
    pub depi: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING DEP-FOR - "#)]
    pub depifor: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING DEP-FOR RATIO - "#)]
    pub depiforr: Option<f32>,

    #[schemars(description = r#"IPC & OFFICIAL CHECKS-FOR - "#)]
    pub depipccf: Option<f32>,

    #[schemars(description = r#"IPC & OFFICIAL CHECKS-FOR RATIO - "#)]
    pub depipccfr: Option<f32>,

    #[schemars(description = r#"IPC-FOR - "#)]
    pub depipcf: Option<f32>,

    #[schemars(description = r#"NONINTEREST-BEARING DEP - "#)]
    pub depni: Option<f32>,

    #[schemars(description = r#"NONINTEREST-BEARING DEP-FOR - "#)]
    pub depnifor: Option<f32>,

    #[schemars(description = r#"NONINTEREST-BEARING DEP-FOR RATIO - "#)]
    pub depniforr: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS CHARGE-OFFS - "#)]
    pub drlnls: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS CHARGE-OFFS RATIO - "#)]
    pub drlnlsr: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS CHARGE-OFFS QUARTERLY - "#)]
    pub drlnlsq: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drlnlsqr: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSS AST - "#)]
    pub eamintan: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSS AST RATIO - "#)]
    pub eamintanr: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSS AST QUARTERLY - "#)]
    pub eamintq: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSS AST QUARTERLY RATIO - "#)]
    pub eamintqr: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE - "#)]
    pub edep: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-DOM - "#)]
    pub edepdom: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-DOM RATIO - "#)]
    pub edepdomr: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-DOM QUARTERLY - "#)]
    pub edepdomq: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-DOM QUARTERLY RATIO - "#)]
    pub edepdomqr: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-FOR - "#)]
    pub edepfor: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-FOR RATIO - "#)]
    pub edepforr: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-FOR QUARTERLY - "#)]
    pub edepforq: Option<f32>,

    #[schemars(description = r#"DEPOSIT INTEREST EXPENSE-FOR QUARTERLY RATIO - "#)]
    pub edepforqr: Option<f32>,

    #[schemars(description = r#"ADVANCES FROM FHLBANK INT EXP - "#)]
    pub efhlbadv: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS INT EXPENSE - "#)]
    pub efrepp: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS INT EXPENSE RATIO - "#)]
    pub efreppr: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS INT EXPENSE QUARTERLY - "#)]
    pub efreppq: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS INT EXPENSE QUARTERLY RATIO - "#)]
    pub efreppqr: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST EXPENSE - "#)]
    pub eintexp: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST EXPENSE RATIO - "#)]
    pub eintexpr: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST EXPENSE QUARTERLY - "#)]
    pub eintxq: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST EXPENSE QUARTERLY - "#)]
    pub eintxqa: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST EXPENSE ANNUALLY - "#)]
    pub eintexpa: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST EXPENSE QUARTERLY RATIO - "#)]
    pub eintxqr: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR CREDIT LOSSES - "#)]
    pub elnatr: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR CREDIT LOSSES RATIO - "#)]
    pub elnatrr: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR CREDIT LOSSES QUARTERLY - "#)]
    pub elnatq: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR CREDIT LOSSES QUARTERLY - "#)]
    pub elnatqa: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO - "#)]
    pub elnatqr: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO - "#)]
    pub elnlosq: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO - "#)]
    pub nttotq: Option<f32>,

    #[schemars(description = r#"PROVISIONS FOR LN & LEASE LOSSES - "#)]
    pub elnlos: Option<f32>,

    #[schemars(description = r#"MORTGAGE DEBT INTEREST EXPENSE - "#)]
    pub emtgls: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST EXPENSE - "#)]
    pub addnonintexp: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST EXPENSE RATIO - "#)]
    pub addnonintexpr: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST EXPENSE QUARTERLY - "#)]
    pub addnonintexpq: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST EXPENSE QUARTERLY RATIO - "#)]
    pub addnonintexpqr: Option<f32>,

    #[schemars(description = r#"ALL OTHER NONINTEREST EXPENSE - "#)]
    pub eothnint: Option<f32>,

    #[schemars(description = r#"ALL OTHER NONINTEREST EXPENSE RATIO - "#)]
    pub eothnintr: Option<f32>,

    #[schemars(description = r#"ALL OTHER NONINTEREST EXPENSE QUARTERLY - "#)]
    pub eothninq: Option<f32>,

    #[schemars(description = r#"ALL OTHER NONINTEREST EXPENSE QUARTERLY RATIO - "#)]
    pub eothninqr: Option<f32>,

    #[schemars(description = r#"PREMISES & FIXED ASSETS EXPENSE - "#)]
    pub epremagg: Option<f32>,

    #[schemars(description = r#"PREMISES & EQUIPMENT EXPENSE RATIO - "#)]
    pub epremaggr: Option<f32>,

    #[schemars(description = r#"PREMISES & FIXED ASSETS EXPENSE QUARTERLY - "#)]
    pub epremagq: Option<f32>,

    #[schemars(description = r#"PREMISES & EQUIPMENT EXPENSE QUARTERLY RATIO - "#)]
    pub epremagqr: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON COMM & PREF - "#)]
    pub eqcdiv: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON COMM & PREF RATIO - "#)]
    pub eqcdivr: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON COMM STOCK - "#)]
    pub eqcdivc: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON COMM STOCK RATIO - "#)]
    pub eqcdivcr: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON PREF STOCK - "#)]
    pub eqcdivp: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON PREF STOCK RATIO - "#)]
    pub eqcdivpr: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON COMM & PREF QUARTERLY - "#)]
    pub eqcdivq: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON COMM & PREF QUARTERLY RATIO - "#)]
    pub eqcdivqr: Option<f32>,

    #[schemars(description = r#"EQCFCTA - "#)]
    pub eqcfcta: Option<f32>,

    #[schemars(description = r#"MINOR INT IN CONSOL SUBS-EQ - "#)]
    pub eqconsub: Option<f32>,

    #[schemars(description = r#"COMMON STOCK - "#)]
    pub eqcs: Option<f32>,

    #[schemars(description = r#"COMMON STOCK RATIO - "#)]
    pub eqcsr: Option<f32>,

    #[schemars(description = r#"NET WORTH CERTIFICATES - "#)]
    pub eqnwcert: Option<f32>,

    #[schemars(description = r#"OTHER EQUITY CAPITAL COMPONENTS - "#)]
    pub eqothcc: Option<f32>,

    #[schemars(description = r#"PERPETUAL PREFERRED STOCK - "#)]
    pub eqpp: Option<f32>,

    #[schemars(description = r#"PERPETUAL PREFERRED STOCK RATIO - "#)]
    pub eqppr: Option<f32>,

    #[schemars(description = r#"SURPLUS - "#)]
    pub eqsur: Option<f32>,

    #[schemars(description = r#"SURPLUS RATIO - "#)]
    pub eqsurr: Option<f32>,

    #[schemars(description = r#"EQUP - "#)]
    pub equp: Option<f32>,

    #[schemars(description = r#"UP-NET & OTHER CAPITAL COMP - "#)]
    pub equptot: Option<f32>,

    #[schemars(description = r#"UP-NET & OTHER CAPITAL RATIO - "#)]
    pub equptotr: Option<f32>,

    #[schemars(description = r#"SALARIES AND EMPLOYEE BENEFITS - "#)]
    pub esal: Option<f32>,

    #[schemars(description = r#"SALARIES AND EMPLOYEE BENEFITS RATIO - "#)]
    pub esalr: Option<f32>,

    #[schemars(description = r#"SALARIES AND EMPLOYEE BENEFITS QUARTERLY - "#)]
    pub esalq: Option<f32>,

    #[schemars(description = r#"SALARIES AND EMPLOYEE BENEFITS QUARTERLY RATIO - "#)]
    pub esalqr: Option<f32>,

    #[schemars(description = r#"SUBORDINATED NOTES INT EXPENSE - "#)]
    pub esubnd: Option<f32>,

    #[schemars(description = r#"TT&L & OTHER BORROWINGS INT EXP - "#)]
    pub ettlotbo: Option<f32>,

    #[schemars(description = r#"NET DISCONTINUED OPERATIONS - "#)]
    pub extra: Option<f32>,

    #[schemars(description = r#"NET DISCONTINUED RATIO - "#)]
    pub extrar: Option<f32>,

    #[schemars(description = r#"NET DISCONTINUED OPERATIONS QUARTERLY - "#)]
    pub extraq: Option<f32>,

    #[schemars(description = r#"NET DISCONTINUED OPERATIONS QUARTERLY RATIO - "#)]
    pub extraqr: Option<f32>,

    #[schemars(description = r#"FDIC REGION - "#)]
    pub fdicdbs: Option<f32>,

    #[schemars(description = r#"FDIC REGION DESC - "#)]
    pub fdicdbsdesc: Option<String>,

    #[schemars(description = r#"FDIC REGION - SUPERVISORY - "#)]
    pub fdicsupv: Option<f32>,

    #[schemars(description = r#"FDIC REGION - SUPERVISORY DESC - "#)]
    pub fdicsupvdesc: Option<String>,

    #[schemars(description = r#"FED DISTRICT - "#)]
    pub fed: Option<f32>,

    #[schemars(description = r#"FED DISTRICT DESC - "#)]
    pub feddesc: Option<String>,

    #[schemars(description = r#"FEDERAL CHARTER FLAG - "#)]
    pub fedchrtr: Option<f32>,

    #[schemars(description = r#"FDIC RISK MANAGEMENT FIELD OFFICE (Search-Eligible) - This field can be used for search and filtering."#)]
    pub fldoff: Option<String>,

    #[schemars(description = r#"FOREIGN CHARTER FLAG - "#)]
    pub forchrtr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL FINANCIAL REPORT FLAG - "#)]
    pub formcfr: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS SOLD - "#)]
    pub frepo: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS SOLD - "#)]
    pub frepor: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS PURCHASED - "#)]
    pub frepp: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPOS PURCHASED RATIO - "#)]
    pub freppr: Option<f32>,

    #[schemars(description = r#"FRS MEMBER FLAG - "#)]
    pub frsmem: Option<f32>,

    #[schemars(description = r#"MEMBER OF A ONE BANK HOLDING CO - "#)]
    pub hctone: Option<f32>,

    #[schemars(description = r#"INTL BANKING ACT ENTITY FLAG - "#)]
    pub iba: Option<f32>,

    #[schemars(description = r#"INCOME BEFORE INC TAXES & DISC - "#)]
    pub ibeftax: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INSTITUTIONS INT INC - "#)]
    pub ichbal: Option<f32>,

    #[schemars(description = r#"BALANCES FROM DEPOSITORY INSTITUTIONS YTD RATIO - "#)]
    pub ichbalr: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INSTITUTIONS INT INC QUARTERLY - "#)]
    pub ichbalq: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INSTITUTIONS INT INC QUARTERLY RATIO - "#)]
    pub ichbalqr: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPO INTEREST INCOME - "#)]
    pub ifrepo: Option<f32>,

    #[schemars(description = r#"FEDERAL FUNDS SOLD YTD RATIO - "#)]
    pub ifrepor: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPO INTEREST INCOME QUARTERLY - "#)]
    pub ifrepoq: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPO INTEREST INCOME QUARTERLY RATIO - "#)]
    pub ifrepoqr: Option<f32>,

    #[schemars(description = r#"SECURITIES GAINS AND LOSSES - "#)]
    pub iglsec: Option<f32>,

    #[schemars(description = r#"SECURITIES GAINS AND LOSSES RATIO - "#)]
    pub iglsecr: Option<f32>,

    #[schemars(description = r#"SECURITIES GAINS AND LOSSES QUARTERLY RATIO - "#)]
    pub iglsecqr: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-DOM - "#)]
    pub ilndom: Option<f32>,

    #[schemars(description = r#"DOMESTIC OFFICE LOANS YTD RATIO - "#)]
    pub ilndomr: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-DOM QUARTERLY - "#)]
    pub ilndomq: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-DOM QUARTERLY RATIO - "#)]
    pub ilndomqr: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-FOR - "#)]
    pub ilnfor: Option<f32>,

    #[schemars(description = r#"FOREIGN OFFICE LOANS YTD RATIO - "#)]
    pub ilnforr: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-FOR QUARTERLY - "#)]
    pub ilnforq: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-FOR QUARTERLY RATIO - "#)]
    pub ilnforqr: Option<f32>,

    #[schemars(description = r#"LEASE INCOME - "#)]
    pub ils: Option<f32>,

    #[schemars(description = r#"LEASE FINANCING RECEIVABLES YTD RATIO - "#)]
    pub ilsr: Option<f32>,

    #[schemars(description = r#"LEASE INCOME QUARTERLY - "#)]
    pub ilsq: Option<f32>,

    #[schemars(description = r#"LEASE INCOME QUARTERLY RATIO - "#)]
    pub ilsqr: Option<f32>,

    #[schemars(description = r#"INSURED INSTITUTION FLAG - "#)]
    pub insall: Option<f32>,

    #[schemars(description = r#"INSURED COMMERCIAL FLAG - "#)]
    pub inscoml: Option<f32>,

    #[schemars(description = r#"FDIC INSURED FLAG - "#)]
    pub insfdic: Option<f32>,

    #[schemars(description = r#"NOT FEDERALLY INSURED FLAG - "#)]
    pub insnone: Option<f32>,

    #[schemars(description = r#"INSURED SAVINGS INSTITUTION FLAG - "#)]
    pub inssave: Option<f32>,

    #[schemars(description = r#"COMMERCIAL INSTITUTION FLAG - "#)]
    pub instcoml: Option<f32>,

    #[schemars(description = r#"SAVING & S&L INSTITUTION FLAG - "#)]
    pub instsave: Option<f32>,

    #[schemars(description = r#"INSTITUTION TYPE - "#)]
    pub insttype: Option<String>,

    #[schemars(description = r#"INTANGIBLE ASSETS - "#)]
    pub intan: Option<f32>,

    #[schemars(description = r#"INTANGIBLE ASSETS RATIO - "#)]
    pub intanr: Option<f32>,

    #[schemars(description = r#"INTEREST EXPENSE TO EARNING ASSETS RATIO - "#)]
    pub intexpy: Option<f32>,

    #[schemars(description = r#"COST OF FUNDING EARNING ASSETS QUARTERLY - "#)]
    pub intexpyq: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST INCOME - "#)]
    pub intinc: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST INCOME YTD RATIO - "#)]
    pub intincr: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST INCOME QUARTERLY - "#)]
    pub intinq: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST INCOME QUARTERLY RATIO - "#)]
    pub intinqr: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub intinqa: Option<f32>,

    #[schemars(description = r#"INVEST IN UNCONSOLIDATED SUBS - "#)]
    pub invsub: Option<f32>,

    #[schemars(description = r#"INVESTMENTS IN RE - "#)]
    pub invsuore: Option<f32>,

    #[schemars(description = r#"OTHER FEE INCOME - "#)]
    pub iothfee: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST INCOME - "#)]
    pub iothii: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST INCOME YTD RATIO - "#)]
    pub iothiir: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST INCOME QUARTERLY - "#)]
    pub iothiiq: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST INCOME QUARTERLY RATIO - "#)]
    pub iothiiqr: Option<f32>,

    #[schemars(description = r#"IRAS AND KEOGH PLANS-DEPOSITS - "#)]
    pub irakeogh: Option<f32>,

    #[schemars(description = r#"IRAS AND KEOGH PLANS-DEPOSITS RATIO - "#)]
    pub irakeoghr: Option<f32>,

    #[schemars(description = r#"TOTAL SECURITY INCOME - "#)]
    pub isc: Option<f32>,

    #[schemars(description = r#"SECURITIES YTD RATIO - "#)]
    pub iscr: Option<f32>,

    #[schemars(description = r#"TOTAL SECURITY INCOME QUARTERLY - "#)]
    pub iscq: Option<f32>,

    #[schemars(description = r#"TOTAL SECURITY INCOME QUARTERLY RATIO - "#)]
    pub iscqr: Option<f32>,

    #[schemars(description = r#"SERVICE CHARGE ON DEPOSIT ACCTS - "#)]
    pub iserchg: Option<f32>,

    #[schemars(description = r#"SERVICE CHARGE ON DEPOSIT ACCTS RATIO - "#)]
    pub iserchgr: Option<f32>,

    #[schemars(description = r#"APPLICABLE INCOME TAXES - "#)]
    pub itax: Option<f32>,

    #[schemars(description = r#"APPLICABLE INCOME TAXES RATIO - "#)]
    pub itaxr: Option<f32>,

    #[schemars(description = r#"APPLICABLE INCOME TAXES QUARTERLY - "#)]
    pub itaxq: Option<f32>,

    #[schemars(description = r#"APPLICABLE INCOME TAXES QUARTERLY RATIO - "#)]
    pub itaxqr: Option<f32>,

    #[schemars(description = r#"INTEREST INCOME ON TRADING ACCTS - "#)]
    pub itrade: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNTS YTD RATIO - "#)]
    pub itrader: Option<f32>,

    #[schemars(description = r#"INTEREST INCOME ON TRADING ACCTS QUARTERLY - "#)]
    pub itradeq: Option<f32>,

    #[schemars(description = r#"INTEREST INCOME ON TRADING ACCTS QUARTERLY RATIO - "#)]
    pub itradeqr: Option<f32>,

    #[schemars(description = r#"TOTAL LIABILITIES - "#)]
    pub liab: Option<f32>,

    #[schemars(description = r#"TOTAL LIABILITIES RATIO - "#)]
    pub liabr: Option<f32>,

    #[schemars(description = r#"TOTAL LIABILITIES & CAPITAL - "#)]
    pub liabeq: Option<f32>,

    #[schemars(description = r#"TOTAL LIABILITIES & CAPITAL RATIO - "#)]
    pub liabeqr: Option<f32>,

    #[schemars(description = r#"MORTGAGE LOANS IN PROCESS - "#)]
    pub lipmtg: Option<f32>,

    #[schemars(description = r#"LIMITED-LIFE PREFERRED STOCK - "#)]
    pub llpfdstk: Option<f32>,

    #[schemars(description = r#"ACCEPTANCES OF OTHER BANKS - "#)]
    pub lnacoth: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS - "#)]
    pub lnag: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS RATIO - "#)]
    pub lnagr: Option<f32>,

    #[schemars(description = r#"ALLOW FOR LOANS LOSS ADJUSTED - "#)]
    pub lnatres: Option<f32>,

    #[schemars(description = r#"ALLOW FOR LOANS + ALLOC TRN RISK - "#)]
    pub lnatresj: Option<f32>,

    #[schemars(description = r#"ALLOW FOR LOANS + ALLOC TRN RISK RATIO - "#)]
    pub lnatresrr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS - AUTO - "#)]
    pub lnauto: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-AUTO RATIO - "#)]
    pub lnautor: Option<f32>,

    #[schemars(description = r#"C&I LOANS - "#)]
    pub lnci: Option<f32>,

    #[schemars(description = r#"C&I LOANS RATIO - "#)]
    pub lncir: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS - "#)]
    pub lncon: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS RATIO - "#)]
    pub lnconr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-HOME IMPROVEMENT - "#)]
    pub lnconot1: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-OTHER - "#)]
    pub lnconoth: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-OTHER RATIO - "#)]
    pub lnconothr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-CREDIT CARD PLAN - "#)]
    pub lncrcd: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-CREDIT CARD PLAN RATIO - "#)]
    pub lncrcdr: Option<f32>,

    #[schemars(description = r#"LNS-CREDIT CD & RELATED PLAN - "#)]
    pub lncrcdrp: Option<f32>,

    #[schemars(description = r#"DEP INSTITUTION LOANS - "#)]
    pub lndep: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVT LOANS - "#)]
    pub lnfg: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVT LOANS RATIO - "#)]
    pub lnfgr: Option<f32>,

    #[schemars(description = r#"LN&LS + UNEARNED INC - "#)]
    pub lnls: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-TOTAL - "#)]
    pub lnlsgr: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-TOTAL - "#)]
    pub lnlsgr2: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-TOTAL ADJUSTED - "#)]
    pub lnlsgrj: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-TOTAL RATIO - "#)]
    pub lnlsgrr: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-NET - "#)]
    pub lnlsnet: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-NET RATIO - "#)]
    pub lnlsnetr: Option<f32>,

    #[schemars(description = r#"MUNI LOANS - "#)]
    pub lnmuni: Option<f32>,

    #[schemars(description = r#"MUNI LOANS RATIO - "#)]
    pub lnmunir: Option<f32>,

    #[schemars(description = r#"OTHER LNS & LS-COMM-QBP - "#)]
    pub lnotci: Option<f32>,

    #[schemars(description = r#"OTHER LNS & LS-COMM-QBP RATIO - "#)]
    pub lnotcir: Option<f32>,

    #[schemars(description = r#"LN TO NONDEP FIN INST & OTH LN - "#)]
    pub lnother: Option<f32>,

    #[schemars(description = r#"OTHER LOANS - "#)]
    pub lnsother: Option<f32>,

    #[schemars(description = r#"OTHER LOANS - "#)]
    pub lnsotherr: Option<f32>,

    #[schemars(description = r#"RE LOANS - "#)]
    pub lnre: Option<f32>,

    #[schemars(description = r#"RE LOANS - "#)]
    pub lnre2: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub lnrecon2: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub lnremul2: Option<f32>,

    #[schemars(description = r#"RE LOANS ADJUSTED - "#)]
    pub lnrej: Option<f32>,

    #[schemars(description = r#"RE LOANS CAVG5 - "#)]
    pub lnre5: Option<f32>,

    #[schemars(description = r#"RE LOANS RATIO - "#)]
    pub lnrer: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL - "#)]
    pub lnreag: Option<f32>,

    #[schemars(description = r#"RE CONSTRUCTION & LAND DEV-CAV5 - "#)]
    pub lnrecon5: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL RATIO - "#)]
    pub lnreagr: Option<f32>,

    #[schemars(description = r#"RE CONSTRUCTION & LAND DEVELOP - "#)]
    pub lnrecons: Option<f32>,

    #[schemars(description = r#"RE CONSTRUCTION & LAND DEVELOP RATIO - "#)]
    pub lnreconsr: Option<f32>,

    #[schemars(description = r#"RE LOANS-DOM - "#)]
    pub lnredom: Option<f32>,

    #[schemars(description = r#"RE LOANS-DOM RATIO - "#)]
    pub lnredomr: Option<f32>,

    #[schemars(description = r#"RE LOANS-FOR - "#)]
    pub lnrefor: Option<f32>,

    #[schemars(description = r#"RE LOANS-FOR RATIO - "#)]
    pub lnreforr: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-LINE - "#)]
    pub lnreloc: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-LINE RATIO - "#)]
    pub lnrelocr: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-LINE2 - "#)]
    pub lnreloc2: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-LINE-CAVG5 - "#)]
    pub lnreloc5: Option<f32>,

    #[schemars(description = r#"RE MULTIFAMILY - "#)]
    pub lnremult: Option<f32>,

    #[schemars(description = r#"RE MULTIFAMILY-CAVG5 - "#)]
    pub lnremul5: Option<f32>,

    #[schemars(description = r#"RE MULTIFAMILY RATIO - "#)]
    pub lnremultr: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRESIDENTIAL PROP - "#)]
    pub lnrenres: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRESIDENTIAL CAVG5 - "#)]
    pub lnrenre5: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRESIDENTIAL CAVG5 - "#)]
    pub lnrenre2: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRESIDENTIAL PROP RATIO - "#)]
    pub lnrenresr: Option<f32>,

    #[schemars(description = r#"PREPAID TAXES & INS ON MTG LNS - "#)]
    pub lnrepp: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY - "#)]
    pub lnreres: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY RATIO - "#)]
    pub lnreresr: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY2 - "#)]
    pub lnreres2: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-CAVG5 - "#)]
    pub lnreres5: Option<f32>,

    #[schemars(description = r#"ALLOWANCE FOR RE LOAN - "#)]
    pub lnresre: Option<f32>,

    #[schemars(description = r#"LEASES - "#)]
    pub ls: Option<f32>,

    #[schemars(description = r#"LEASES RATIO - "#)]
    pub lsr: Option<f32>,

    #[schemars(description = r#"METROPOLITAN FLAG - A flag used to indicate whether an institution is in a metropolitan statistical area. The U.S census bureau office of management and budget defines the metropolitan statistical area. A core based statistical area associated with at least one urbanized area that has a population of at least 50,000. The metropolitan statistical area comprises the central county or counties containing the core, plus adjacent outlying counties having a high degree of social and economic integration with the central county as measured through commuting. 0=institution is not in a metropolitan statistical area. 1=institution is in a metropolitan statistical area."#)]
    pub metro: Option<f32>,

    #[schemars(description = r#"INSURED SAVINGS BANK FLAG - "#)]
    pub mi: Option<f32>,

    #[schemars(description = r#"MICROPOLITAN FLAG - A flag used to indicate whether an institution is in a micropolitan statistical area. The U.S census bureau office of management and budget defines the micropolitan statistical area. A core based statistical area associated with at least one urban cluster that has a population of at least 10,000 but less than 50,000. The micropolitan statistical area comprises the central county or counties containing the core, plus adjacent outlying counties having a high degree of social and economic integration with the central county as measured through commuting. 0=institution is not in a micropolitan statistical area. 1=institution is in a micropolitan statistical area."#)]
    pub micro: Option<f32>,

    #[schemars(description = r#"MINORITY CODE - A character field on the institution file corresponding to a type of minority ownership. .=NONE. 01=African American. 02=Hispanic American. 03=Asian or Pacific Islander Americans. 04=Native American or Native Alaskan American. 05=Multi-Racial American. 06=Minority Board and serving African American Community. 08=Minority Board and serving Asian/Pacific Islander Americans. 10=Minority Board and serving Multi-Racial community."#)]
    pub mnrtycde: Option<f32>,

    #[schemars(description = r#"EFFECTIVE DTE OF MINORITY STATUS - Represent the effective date on which an institution is assigned a minority status, transaction in dates. Format(DDMONCCYY) day, month abbrev, century, and year."#)]
    pub mnrtydte: Option<f32>,

    #[schemars(description = r#"MORTGAGE INDEBTEDNESS & CAP LS - "#)]
    pub mtgls: Option<f32>,

    #[schemars(description = r#"NATIONAL BANK FLAG - "#)]
    pub n: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-LOANS & LEASES - "#)]
    pub nalnls: Option<f32>,

    #[schemars(description = r#"NONINSURED COMMERCIAL INST FLAG - "#)]
    pub nc: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-LOANS & LEASES - "#)]
    pub nclnls: Option<f32>,

    #[schemars(description = r#"NET INC - ATTRIB TO MINORITY INT - "#)]
    pub netimin: Option<f32>,

    #[schemars(description = r#"NET INC - ATTRIB TO MINORITY INT RATIO - "#)]
    pub netiminr: Option<f32>,

    #[schemars(description = r#"NET INC - ATTRIB TO MINORITY INT QUARTERLY - "#)]
    pub netiminq: Option<f32>,

    #[schemars(description = r#"NET INC - ATTRIB TO MINORITY INT QUARTERLY RATIO - "#)]
    pub netiminqr: Option<f32>,

    #[schemars(description = r#"NET INC - BANK & MINORITY INT - "#)]
    pub netinbm: Option<f32>,

    #[schemars(description = r#"NET INC - BANK & MINORITY INT RATIO - "#)]
    pub netinbmr: Option<f32>,

    #[schemars(description = r#"NET INC - BANK & MINORITY INT QUARTERLY - "#)]
    pub netinbmq: Option<f32>,

    #[schemars(description = r#"NET INCOME BEFORE TAXES ANNUALLY - "#)]
    pub netinbxa: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub netibxqa: Option<f32>,

    #[schemars(description = r#"NET INC - BANK & MINORITY INT QUARTERLY RATIO - "#)]
    pub netinbmqr: Option<f32>,

    #[schemars(description = r#"NEW INSTITUTION FLAG - "#)]
    pub newinst: Option<f32>,

    #[schemars(description = r#"NUMBER OF FIDUCIARY ACCOUNTS AND RELATED ASSET ACCOUNTS - "#)]
    pub nfaa: Option<f32>,

    #[schemars(description = r#"NET INTEREST INCOME - "#)]
    pub nim: Option<f32>,

    #[schemars(description = r#"NET INTEREST INCOME RATIO - "#)]
    pub nimr: Option<f32>,

    #[schemars(description = r#"NET INTEREST INCOME QUARTERLY - "#)]
    pub nimq: Option<f32>,

    #[schemars(description = r#"NET INTEREST INCOME QUARTERLY - "#)]
    pub nimqa: Option<f32>,

    #[schemars(description = r#"NET INTEREST INCOME ANNUALLY - "#)]
    pub nima: Option<f32>,

    #[schemars(description = r#"NET INTEREST INCOME QUARTERLY RATIO - "#)]
    pub nimqr: Option<f32>,

    #[schemars(description = r#"NONMEMBER INSURED INST FLAG - "#)]
    pub nm: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST INCOME - "#)]
    pub nonii: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST INCOME RATIO - "#)]
    pub noniir: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST EXPENSE - "#)]
    pub nonix: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST EXPENSE RATIO - "#)]
    pub nonixr: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST EXPENSE QUARTERLY - "#)]
    pub nonixq: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST EXPENSE QUARTERLY - "#)]
    pub nonixqa: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST EXPENSE QUARTERLY RATIO - "#)]
    pub nonixqr: Option<f32>,

    #[schemars(description = r#"NONINSURED SAVINGS INST FLAG - "#)]
    pub ns: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS NET CHARGE-OFFS - "#)]
    pub ntlnls: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS NET CHARGE-OFFS RATIO - "#)]
    pub ntlnlscor: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS NET CHARGE-OFFS QUARTERLY - "#)]
    pub ntlnlsq: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS NET CHARGE-OFFS QUARTERLY - "#)]
    pub ntlnlsqa: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS NET CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub ntlnlscoqr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-TOTAL - "#)]
    pub ntr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-TOTAL RATIO - "#)]
    pub ntrr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-IPC - "#)]
    pub ntripc: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-IPC RATIO - "#)]
    pub ntripcr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-MUNI - "#)]
    pub ntrmuni: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-MUNI RATIO - "#)]
    pub ntrmunir: Option<f32>,

    #[schemars(description = r#"TIME DEPOSITS-TOTAL - "#)]
    pub ntrtime: Option<f32>,

    #[schemars(description = r#"TIME DEPOSITS OVER $100M - "#)]
    pub ntrtmlg: Option<f32>,

    #[schemars(description = r#"AMT TOTAL TIME DEP MORE THAN $250,000 - "#)]
    pub ntrtmlgj: Option<f32>,

    #[schemars(description = r#"AMT TOTAL TIME DEP MORE THAN $250,000 RATIO - "#)]
    pub ntrtmlgjr: Option<f32>,

    #[schemars(description = r#"AMT TIME DEP OF $250,000 OR LESS - "#)]
    pub ntrtmmed: Option<f32>,

    #[schemars(description = r#"AMT TIME DEP OF $250,000 OR LESS RATIO - "#)]
    pub ntrtmmedr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-U.S. GOVERNMENT - "#)]
    pub ntrusgov: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-U.S. GOVERNMENT RATIO - "#)]
    pub ntrusgovr: Option<f32>,

    #[schemars(description = r#"RETAINED EARNINGS ANUALLY - "#)]
    pub ntirta: Option<f32>,

    #[schemars(description = r#"TOTAL LN & LS LOSS NET CHG-OFFS - "#)]
    pub nttot: Option<f32>,

    #[schemars(description = r#"NUMBER OF FULL TIME EMPLOYEES - "#)]
    pub numemp: Option<f32>,

    #[schemars(description = r#"OTHER ASSETS - "#)]
    pub oa: Option<f32>,

    #[schemars(description = r#"OAKAR FLAG - A flag used to indicate whether an institution acquired deposits that were previously insured under a different insurance fund. 0=has no oakar deposits. 1=has oakar deposits."#)]
    pub oakar: Option<f32>,

    #[schemars(description = r#"OCC DISTRICT - "#)]
    pub occdist: Option<f32>,

    #[schemars(description = r#"OCC DISTRICT DESC - "#)]
    pub occdistdesc: Option<String>,

    #[schemars(description = r#"DOMESTIC MULTI-SERVICE OFFICES - The number of multiple service domestic offices operated by an institution."#)]
    pub offdmult: Option<f32>,

    #[schemars(description = r#"NONDOMESTIC OFFICES - The number of nondomestic offices operated by an institution."#)]
    pub offndom: Option<f32>,

    #[schemars(description = r#"DOMESTIC OTHER OFFICES - The number of domestic non-multiple service offices operated by an institution."#)]
    pub offoth: Option<f32>,

    #[schemars(description = r#"SOD OFFICES - The number of offices operated by an institution based on the summary of deposits definition of offices."#)]
    pub offsod: Option<f32>,

    #[schemars(description = r#"NUMBER OF STATES WITH OFFICES - The number of states with offices (including its main office)."#)]
    pub offstate: Option<f32>,

    #[schemars(description = r#"TOTAL OFFICES - The total number of offices operated by an institution."#)]
    pub offtot: Option<f32>,

    #[schemars(description = r#"U.S. AND OTHER AREA OFFICES - The number of domestic and U.S terrirtories offices operated by an institution."#)]
    pub offusoa: Option<f32>,

    #[schemars(description = r#"INSURED IBA OFFICE FLAG - "#)]
    pub oi: Option<f32>,

    #[schemars(description = r#"OTS DISTRICT - A number used to identify the office of thrift supervision district in which the institution is located. 01=Northeast. 02=Southeast. 03=Midwest. 04=West."#)]
    pub otsdist: Option<f32>,

    #[schemars(description = r#"OTS REGION NUMBER - A number used to identify the office of thrift supervision region in which the institution is located. 01=Northeast. 02=Southeast. 03=Midwest. 04=West."#)]
    pub otsregno: Option<f32>,

    #[schemars(description = r#"OTHER LIAB & MINOR IN SUBS - "#)]
    pub olmin: Option<f32>,

    #[schemars(description = r#"OTHER REAL ESTATE OWNED - "#)]
    pub ore: Option<f32>,

    #[schemars(description = r#"OTHER REAL ESTATE OWNED RATIO - "#)]
    pub orer: Option<f32>,

    #[schemars(description = r#"OTHER LIABILITIES-FHLB - "#)]
    pub othbfhlb: Option<f32>,

    #[schemars(description = r#"OTHER LIABILITIES-FHLB RATIO - "#)]
    pub othbfhlbr: Option<f32>,

    #[schemars(description = r#"OTHER BORROWED MONEY - "#)]
    pub othbor: Option<f32>,

    #[schemars(description = r#"OTH BORROWED FUNDS - "#)]
    pub othbrf: Option<f32>,

    #[schemars(description = r#"OTH BORROWED FUNDS RATIO - "#)]
    pub othbrfr: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP ONE YR OR LESS - "#)]
    pub otbfh1l: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP ONE YR OR LESS RATIO - "#)]
    pub otbfh1lr: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP ONE YR THROUGH THREE - "#)]
    pub otbfh1t3: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP ONE YR THROUGH THREE - "#)]
    pub otbfh1t3r: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP THREE THROUGH FIVE - "#)]
    pub otbfh3t5: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP THREE THROUGH FIVE RATIO - "#)]
    pub otbfh3t5r: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP OVER FIVE YEARS - "#)]
    pub otbfhov5: Option<f32>,

    #[schemars(description = r#"FHLB ADV MAT REP OVER FIVE YEARS RATIO - "#)]
    pub otbfhov5r: Option<f32>,

    #[schemars(description = r#"FHLB ADV WITH REMAINING MAT ONE YR OR LESS - "#)]
    pub othbfh1l: Option<f32>,

    #[schemars(description = r#"FHLB ADV WITH REMAINING MAT ONE YR OR LESS RATIO - "#)]
    pub othbfh1lr: Option<f32>,

    #[schemars(description = r#"FHLB STRUCTURED ADV - "#)]
    pub otbfhsta: Option<f32>,

    #[schemars(description = r#"FHLB STRUCTURED ADV - "#)]
    pub otbfhstar: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING ONE YR OR LESS - "#)]
    pub otbot1l: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING ONE YR OR LESS RATIO - "#)]
    pub otbot1lr: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING ONE YR THROUGH THREE - "#)]
    pub otbot1t3: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING ONE YR THROUGH THREE RATIO - "#)]
    pub otbot1t3r: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING THREE YR THROUGH FIVE - "#)]
    pub otbot3t5: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING THREE YR THROUGH FIVE RATIO - "#)]
    pub otbot3t5r: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING OVER FIVE YRS - "#)]
    pub otbotov5: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT OR NEXT REPRICING OVER FIVE YRS RATIO - "#)]
    pub otbotov5r: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT REMAINING MAT OF ONE YR OR LESS - "#)]
    pub othbot1l: Option<f32>,

    #[schemars(description = r#"OTH BORR MAT REMANING MAT OF ONE YR OR LESS RATIO - "#)]
    pub othbot1lr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LIABILITIES - "#)]
    pub allothl: Option<f32>,

    #[schemars(description = r#"ALL OTHER LIABILITIES RATIO - "#)]
    pub allothlr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-LOANS & LEASES - "#)]
    pub p3lnls: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-LOANS & LEASES - "#)]
    pub p9lnls: Option<f32>,

    #[schemars(description = r#"QBP COMMERCIAL BANK REGION - "#)]
    pub qbprcoml: Option<f32>,

    #[schemars(description = r#"QBP COMMERCIAL BANK REGION DESC - "#)]
    pub qbprcomldesc: Option<String>,

    #[schemars(description = r#"QBP BIF FUND SAVINGS REGION - "#)]
    pub qbprsavb: Option<f32>,

    #[schemars(description = r#"QBP SAVING SAIF FUND REGION - "#)]
    pub qbprsavs: Option<f32>,

    #[schemars(description = r#"QUARTER NUMBER - Identifies the calendar quarter. 1=March. 2=June. 3=September. 4=December."#)]
    pub qtrno: Option<f32>,

    #[schemars(description = r#"PRIMARY REGULATING AGENCY - "#)]
    pub regagnt: Option<String>,

    #[schemars(description = r#"FDIC RISK TERRITORY - An abbreviation of the current risk territory for an institution (FDIC Risk Territory). All periods are displayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#)]
    pub riskterr: Option<String>,

    #[schemars(description = r#"ASSETS 10B TO 250B FLAG - "#)]
    pub s10t250b: Option<f32>,

    #[schemars(description = r#"SASSER FLAG - A flag used to indicate whether an institution was a former savings association that has converted to a bank charter and is still a SAIF insured institution. 0=not a sasser institution. 1=is a sasser institution."#)]
    pub sasser: Option<f32>,

    #[schemars(description = r#"SAVINGS BANK FLAG - "#)]
    pub sb: Option<f32>,

    #[schemars(description = r#"SECURITIES - "#)]
    pub sc: Option<f32>,

    #[schemars(description = r#"SECURITIES RATIO - "#)]
    pub scr: Option<f32>,

    #[schemars(description = r#"TOTAL AVAILABLE-FOR-SALE AT AMORTIZED COST SECURITIES ON A CONSOLIDATED BASIS - "#)]
    pub scaa: Option<f32>,

    #[schemars(description = r#"TOTAL HELD-TO-MATURITY AT FAIR VALUE SECURITIES ON A CONSOLIDATED BASIS - "#)]
    pub schf: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY - "#)]
    pub scage: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY - "#)]
    pub scaspnha: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY - "#)]
    pub scaspnaf: Option<f32>,

    #[schemars(description = r#"NON-MORT BACKED ISSUES BY US GOVT OR SPONSORED AGENCIES - "#)]
    pub scaspnsum: Option<f32>,

    #[schemars(description = r#"NON-MORT BACKED ISSUES BY US GOVT OR SPONSORED AGENCIES RATIO - "#)]
    pub scaspnsumr: Option<f32>,

    #[schemars(description = r#"DOMESTIC SEC*DEBT & EQUITY - CON - "#)]
    pub scdeq: Option<f32>,

    #[schemars(description = r#"OTHER DOMESTIC DEBT - "#)]
    pub scdomo: Option<f32>,

    #[schemars(description = r#"OTHER DOMESTIC DEBT RATIO - "#)]
    pub scdomor: Option<f32>,

    #[schemars(description = r#"EQUITY SECURITIES - "#)]
    pub sceq: Option<f32>,

    #[schemars(description = r#"FOREIGN DEBT & EQUITY - "#)]
    pub scfdeq: Option<f32>,

    #[schemars(description = r#"FOREIGN DEBT SECURITIES - "#)]
    pub scford: Option<f32>,

    #[schemars(description = r#"FOREIGN DEBT SECURITIES RATIO - "#)]
    pub scfordr: Option<f32>,

    #[schemars(description = r#"MORTGAGE BACKED SECURITIES - "#)]
    pub scmtgbk: Option<f32>,

    #[schemars(description = r#"MORTGAGE BACKED SECURITIES RATIO - "#)]
    pub scmtgbkr: Option<f32>,

    #[schemars(description = r#"MUNICIPAL SECURITIES - "#)]
    pub scmuni: Option<f32>,

    #[schemars(description = r#"MUNICIPAL RATIO - "#)]
    pub scmunir: Option<f32>,

    #[schemars(description = r#"SECURITIES-MV - "#)]
    pub scmv: Option<f32>,

    #[schemars(description = r#"RES-OTH DOM DEBT*PRIV CERTS - "#)]
    pub scodpc: Option<f32>,

    #[schemars(description = r#"RES-OTH DOM DEBT*PRIV CERTS RATIO - "#)]
    pub scodpcr: Option<f32>,

    #[schemars(description = r#"CONTRA-ASSETS TO SECURITIES - "#)]
    pub scres: Option<f32>,

    #[schemars(description = r#"U.S. TREASURY & AGENCY - "#)]
    pub scus: Option<f32>,

    #[schemars(description = r#"U.S. TREASURY & AGENCY RATIO - "#)]
    pub scusr: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY ALL OTHER - "#)]
    pub scusa: Option<f32>,

    #[schemars(description = r#"U.S. TREASURY SECURITIES - "#)]
    pub scust: Option<f32>,

    #[schemars(description = r#"U.S. TREASURY SECURITIES RATIO - "#)]
    pub scustr: Option<f32>,

    #[schemars(description = r#"GEOGRAPHIC LATITUDE OF MAIN OFFICE - Geographic latitude of main office."#)]
    pub sims_lat: Option<f32>,

    #[schemars(description = r#"GEOGRAPHIC LONGITUDE OF MAIN OFFICE - Geographic longitude of main office"#)]
    pub sims_long: Option<f32>,

    #[schemars(description = r#"SAVINGS AND LOAN FLAG - "#)]
    pub sl: Option<f32>,

    #[schemars(description = r#"STATE MEMBER BANK FLAG - "#)]
    pub sm: Option<f32>,

    #[schemars(description = r#"FIPS STATE ALPHA CODE (Search-Eligible) - This field can be used for search and filtering."#)]
    pub stalp: Option<String>,

    #[schemars(description = r#"STATE CHARTER FLAG - "#)]
    pub stchrtr: Option<f32>,

    #[schemars(description = r#"STATE NAME (Search-Eligible) - This field can be used for search and filtering."#)]
    pub stname: Option<String>,

    #[schemars(description = r#"FIPS STATE NUMBER - "#)]
    pub stnum: Option<f32>,

    #[schemars(description = r#"SUB. DEBT & L/L PREFERRED STK - "#)]
    pub subllpf: Option<f32>,

    #[schemars(description = r#"SUBORDINATED NOTES & DEBENTURES - "#)]
    pub subnd: Option<f32>,

    #[schemars(description = r#"ASSETS UNDER 25M FLAG - "#)]
    pub sz25: Option<f32>,

    #[schemars(description = r#"ASSETS UNDER 100M FLAG - "#)]
    pub sz100: Option<f32>,

    #[schemars(description = r#"ASSETS OVER 100M FLAG - "#)]
    pub sz100mp: Option<f32>,

    #[schemars(description = r#"ASSETS 100M TO 300M FLAG - "#)]
    pub sz100t3: Option<f32>,

    #[schemars(description = r#"ASSETS 100M TO 500M FLAG - "#)]
    pub sz100t5: Option<f32>,

    #[schemars(description = r#"ASSETS 100M TO 1B FLAG - "#)]
    pub sz100t1b: Option<f32>,

    #[schemars(description = r#"ASSETS OVER 10B FLAG - "#)]
    pub sz10bp: Option<f32>,

    #[schemars(description = r#"ASSETS OVER 1B FLAG - "#)]
    pub sz1bp: Option<f32>,

    #[schemars(description = r#"ASSETS 1B TO 10B FLAG - "#)]
    pub sz1bt10b: Option<f32>,

    #[schemars(description = r#"ASSETS 1B TO 3B FLAG - "#)]
    pub sz1bt3b: Option<f32>,

    #[schemars(description = r#"ASSETS 1B TO 5B FLAG - "#)]
    pub sz1bt5b: Option<f32>,

    #[schemars(description = r#"ASSETS OVER 250B FLAG - "#)]
    pub sz250bp: Option<f32>,

    #[schemars(description = r#"ASSETS 25M TO 50M FLAG - "#)]
    pub sz25t50: Option<f32>,

    #[schemars(description = r#"ASSETS 300M TO 500M FLAG - "#)]
    pub sz300t5: Option<f32>,

    #[schemars(description = r#"ASSETS 3B TO 10B FLAG - "#)]
    pub sz3bt10b: Option<f32>,

    #[schemars(description = r#"ASSETS 500M TO 1B FLAG - "#)]
    pub sz500t1b: Option<f32>,

    #[schemars(description = r#"ASSETS 50M TO 100M FLAG - "#)]
    pub sz50t100: Option<f32>,

    #[schemars(description = r#"ASSETS OVER 5B FLAG - "#)]
    pub sz5bp: Option<f32>,

    #[schemars(description = r#"TOTAL FIDUCIARY AND RELATED ASSETS - "#)]
    pub tfra: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNTS - "#)]
    pub trade: Option<f32>,

    #[schemars(description = r#"TRADING LIABILITIES - "#)]
    pub tradel: Option<f32>,

    #[schemars(description = r#"TRADING LIABILITIES RATIO - "#)]
    pub tradelr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNTS RATIO - "#)]
    pub trader: Option<f32>,

    #[schemars(description = r#"TRANSACTION-TOTAL - "#)]
    pub trn: Option<f32>,

    #[schemars(description = r#"TRANSACTION-TOTAL RATIO - "#)]
    pub trnr: Option<f32>,

    #[schemars(description = r#"TRANSACTION-IPC - "#)]
    pub trnipc: Option<f32>,

    #[schemars(description = r#"TRAN-IPC-OFFICIAL CHECKS - "#)]
    pub trnipcoc: Option<f32>,

    #[schemars(description = r#"TRAN-IPC-OFFICIAL CHECKS RATIO - "#)]
    pub trnipcocr: Option<f32>,

    #[schemars(description = r#"TRANSACTION-MUNI - "#)]
    pub trnmuni: Option<f32>,

    #[schemars(description = r#"TRANSACTION-MUNI RATIO - "#)]
    pub trnmunir: Option<f32>,

    #[schemars(description = r#"TRANSACTION-U.S. GOVERNMENT - "#)]
    pub trnusgov: Option<f32>,

    #[schemars(description = r#"TRANSACTION-U.S. GOVERNMENT RATIO - "#)]
    pub trnusgovr: Option<f32>,

    #[schemars(description = r#"TRUST POWER GRANTED CODES - Is a two digit numeric code which identifies the trust power granted categories of an institution. 00 - Trust powers not known. 10 - Full trust powers granted. 11 - Full trust powers granted, exercised. 12 - Full trust powers granted, not exercised. 20 - Limited trust powers granted. 21 - Limited trust powers granted, exercised. 30 - Trust powers not granted. 31 - Trust powers not granted but exercised. 40 - Full trust powers grandfathered."#)]
    pub trustpwr: Option<f32>,

    #[schemars(description = r#"TIME & SAVINGS DEPOSITS-TOTAL - "#)]
    pub ts: Option<f32>,

    #[schemars(description = r#"TIME & SAVINGS DEPOSITS-TOTAL RATIO - "#)]
    pub tsr: Option<f32>,

    #[schemars(description = r#"TT&L NOTE OPTION - "#)]
    pub ttl: Option<f32>,

    #[schemars(description = r#"TT&L & OTHER BORROWINGS - "#)]
    pub ttlotbor: Option<f32>,

    #[schemars(description = r#"UNEARNED INCOME - "#)]
    pub uninc: Option<f32>,

    #[schemars(description = r#"BANK UNIQUE NUMBER - A unique identification number assigned to an institution by the FDIC."#)]
    pub uninum: Option<f32>,

    #[schemars(description = r#"USA LOCATED INSTITUTION - "#)]
    pub usa: Option<f32>,

    #[schemars(description = r#"UNAMORTIZED YIELD ADJ-MTG LOANS - "#)]
    pub uyamtg: Option<f32>,

    #[schemars(description = r#"ASST-BCK UNUSED COMMIT - RELATED - "#)]
    pub abcubk: Option<f32>,

    #[schemars(description = r#"ASST-BCK UNUSED COMMIT - RELATED RATIO - "#)]
    pub abcubkr: Option<f32>,

    #[schemars(description = r#"ASSET-BACK UNUSED COMMIT - OTHER - "#)]
    pub abcuoth: Option<f32>,

    #[schemars(description = r#"ASSET-BACK UNUSED COMMIT - OTHER RATIO - "#)]
    pub abcuothr: Option<f32>,

    #[schemars(description = r#"ASSET-BACK CREDIT EX-RELATED - "#)]
    pub abcxbk: Option<f32>,

    #[schemars(description = r#"ASSET-BACK CREDIT EX-RELATED RATIO - "#)]
    pub abcxbkr: Option<f32>,

    #[schemars(description = r#"ASSET-BACK CREDIT EX-OTHER - "#)]
    pub abcxoth: Option<f32>,

    #[schemars(description = r#"ASSET-BACK CREDIT EX-OTHER RATIO - "#)]
    pub abcxothr: Option<f32>,

    #[schemars(description = r#"C.E. RECOURSE NOT SECUR. - OTH - "#)]
    pub asceoth: Option<f32>,

    #[schemars(description = r#"C.E. RECOURSE NOT SECUR. - OTH RATIO - "#)]
    pub asceothr: Option<f32>,

    #[schemars(description = r#"C.E. RECOURSE NOT SECUR. - RES - "#)]
    pub asceres: Option<f32>,

    #[schemars(description = r#"C.E. RECOURSE NOT SECUR. - RES RATIO - "#)]
    pub asceresr: Option<f32>,

    #[schemars(description = r#"SOLD W/RECOURSE N/SECUR. - OTH - "#)]
    pub asdroth: Option<f32>,

    #[schemars(description = r#"SOLD W/RECOURSE N/SECUR. - OTH RATIO - "#)]
    pub asdrothr: Option<f32>,

    #[schemars(description = r#"SOLD W/RECOURSE N/SECUR.- RES - "#)]
    pub asdrres: Option<f32>,

    #[schemars(description = r#"SOLD W/RECOURSE N/SECUR.- RES RATIO - "#)]
    pub asdrresr: Option<f32>,

    #[schemars(description = r#"TOTAL ASSETS-CAVG2 - "#)]
    pub asset2: Option<f32>,

    #[schemars(description = r#"TOTAL ASSETS-CAVG5 - "#)]
    pub asset5: Option<f32>,

    #[schemars(description = r#"TOTAL ASSETS-FOR - "#)]
    pub assetfor: Option<f32>,

    #[schemars(description = r#"LONG-TERM ASSETS (5+ YEARS)-QBP - "#)]
    pub asstlt: Option<f32>,

    #[schemars(description = r#"LONG-TERM ASSETS (5+ YEARS) RATIO - "#)]
    pub asstltr: Option<f32>,

    #[schemars(description = r#"ASSETS PER EMPLOYEE IN MILLION - "#)]
    pub astempm: Option<f32>,

    #[schemars(description = r#"AVERAGE ASSETS-ADJUSTED-PCA - "#)]
    pub avassetj: Option<f32>,

    #[schemars(description = r#"AVERAGE ASSETS-ADJUSTED-PCA RATIO - "#)]
    pub avassetjr: Option<f32>,

    #[schemars(description = r#"BROKERED DEP-INSURED - "#)]
    pub broins: Option<f32>,

    #[schemars(description = r#"BROKERED DEP-INSURED RATIO - "#)]
    pub broinsr: Option<f32>,

    #[schemars(description = r#"REPORT DATE (CCYYMMDD) - "#)]
    pub callymd: Option<f32>,

    #[schemars(description = r#"CASH & DUE FROM DEP INST-FOR - "#)]
    pub chbalfor: Option<f32>,

    #[schemars(description = r#"NONINTEREST-BEARING CASH & DUE - "#)]
    pub chbalni: Option<f32>,

    #[schemars(description = r#"NONINTEREST-BEARING CASH & DUE RATIO - "#)]
    pub chbalnir: Option<f32>,

    #[schemars(description = r#"CASH ITEMS - "#)]
    pub chcic: Option<f32>,

    #[schemars(description = r#"CASH ITEMS RATIO - "#)]
    pub chcicr: Option<f32>,

    #[schemars(description = r#"CURRENCY & COIN - "#)]
    pub chcoin: Option<f32>,

    #[schemars(description = r#"CURRENCY & COIN RATIO - "#)]
    pub chcoinr: Option<f32>,

    #[schemars(description = r#"NET OPERATING CASH FLOW-ANN - "#)]
    pub chfla: Option<f32>,

    #[schemars(description = r#"NET OPERATING CASH FLOW-ANN Quarterly - "#)]
    pub chflq: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM FRB - "#)]
    pub chfrb: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM FRB RATIO - "#)]
    pub chfrbr: Option<f32>,

    #[schemars(description = r#"CASH ITEM COLLEC IN DOMESTIC OFFICES - "#)]
    pub chitem: Option<f32>,

    #[schemars(description = r#"CASH ITEMS COLLEC IN DOMESTIC OFFICES RATIO - "#)]
    pub chitemr: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM BK FOR COUNTRY - "#)]
    pub chnus: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM BK FOR COUNTRY RATIOS - "#)]
    pub chnusr: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM FOR BR OF OTH US BK - "#)]
    pub chnusfbk: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM DEP INST U.S. - "#)]
    pub chus: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM DEP INST U.S. RATIO - "#)]
    pub chusr: Option<f32>,

    #[schemars(description = r#"BAL DUE FROM U.S. BR OF FOR BKS - "#)]
    pub chusfbk: Option<f32>,

    #[schemars(description = r#"CITY (Search-Eligible) - This field can be used for search and filtering."#)]
    pub city: Option<String>,

    #[schemars(description = r#"CORE DEPOSITS - "#)]
    pub coredep: Option<f32>,

    #[schemars(description = r#"CORE DEPOSITS RATIO - "#)]
    pub coredepr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN RECOVERIES - "#)]
    pub crag: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN RECOVERIES RATIO - "#)]
    pub cragr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN RECOVERIES QUARTERLY - "#)]
    pub cragq: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN RECOVERIES QUARTERLY RATIO - "#)]
    pub cragqr: Option<f32>,

    #[schemars(description = r#"AG LOAN RECOVERIES*SMALL BKS - "#)]
    pub cragsm: Option<f32>,

    #[schemars(description = r#"AAG LOAN RECOVERIES*SMALL BKS RATIO - "#)]
    pub cragsmr: Option<f32>,

    #[schemars(description = r#"AG LOAN RECOVERIES*SMALL BKS QUARTERLY - "#)]
    pub cragsmq: Option<f32>,

    #[schemars(description = r#"AG LOAN RECOVERIES*SMALL BKS QUARTERLY RATIO - "#)]
    pub cragsmqr: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - RECOVERIES - "#)]
    pub crauto: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - RECOVERIES RATIO - "#)]
    pub crautor: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - RECOVERIES QUARTERLY - "#)]
    pub crautoq: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - RECOVERIES QUARTERLY RATIO - "#)]
    pub crautoqr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES - "#)]
    pub crci: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES RATIO - "#)]
    pub crcir: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES QUARTERLY - "#)]
    pub crciq: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES QUARTERLY RATIO - "#)]
    pub crciqr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES NON-U.S. - "#)]
    pub crcinus: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES NON-U.S. RATIO - "#)]
    pub crcinusr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES NON-U.S. QUARTERLY - "#)]
    pub crcinusq: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN RECOVERIES NON-U.S. QUARTERLY RATIO - "#)]
    pub crcinusqr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN RECOVERIES - "#)]
    pub crcon: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN RECOVERIES RATIO - "#)]
    pub crconr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN RECOVERIES QUARTERLY - "#)]
    pub crconq: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN RECOVERIES QUARTERLY RATIO - "#)]
    pub crconqr: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN RECOVERIES - "#)]
    pub crconoth: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN RECOVERIES RATIO - "#)]
    pub crconothr: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN RECOVERIES QUARTERLY - "#)]
    pub crconotq: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN RECOVERIES QUARTERLY RATIO - "#)]
    pub crconotqr: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN RECOVERIES - "#)]
    pub crcrcd: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN RECOVERIES RATIO - "#)]
    pub crcrcdr: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN RECOVERIES QUARTERLY - "#)]
    pub crcrcdq: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN RECOVERIES QUARTERLY RATIO - "#)]
    pub crcrcdqr: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN RECOVERIES - "#)]
    pub crdep: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN RECOVERIES RATIO - "#)]
    pub crdepr: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN RECOVERIES QUARTERLY - "#)]
    pub crdepq: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN RECOVERIES Quarterly RATIO - "#)]
    pub crdepqr: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN RECOVERIES - "#)]
    pub crdepnus: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN RECOVERIES RATIO - "#)]
    pub crdepnusr: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN RECOVERIES QUARTERLY - "#)]
    pub crdepnuq: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN RECOVERIES QUARTERLY RATIO - "#)]
    pub crdepnuqr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN RECOVERIES - "#)]
    pub crforgv: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN RECOVERIES RATIO - "#)]
    pub crforgvr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN RECOVERIES QUARTERLY - "#)]
    pub crforgvq: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN RECOVERIES QUARTERLY RATIO - "#)]
    pub crforgvqr: Option<f32>,

    #[schemars(description = r#"LEASE RECOVERIES - "#)]
    pub crls: Option<f32>,

    #[schemars(description = r#"LEASE RECOVERIES RATIO - "#)]
    pub crlsr: Option<f32>,

    #[schemars(description = r#"LEASE RECOVERIES QUARTERLY - "#)]
    pub crlsq: Option<f32>,

    #[schemars(description = r#"LEASE RECOVERIES QUARTERLY RATIO - "#)]
    pub crlsqr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN RECOVERIES - "#)]
    pub crother: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN RECOVERIES RATIO - "#)]
    pub crotherr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN RECOVERIES QUARTERLY - "#)]
    pub crothq: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN RECOVERIES QUARTERLY RATIO - "#)]
    pub crothqr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN RECOVERIES - "#)]
    pub crre: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN RECOVERIES RATIO - "#)]
    pub crrer: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN RECOVERIES QUARTERLY - "#)]
    pub crreq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN RECOVERIES QUARTERLY RATIO - "#)]
    pub crreqr: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN RECOVERIES - "#)]
    pub crreag: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN RECOVERIES RATIO - "#)]
    pub crreagr: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN RECOVERIES-QTR - "#)]
    pub crreagq: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN RECOVERIES QUARTERLY RATIO - "#)]
    pub crreagqr: Option<f32>,

    #[schemars(description = r#"1-4 FAM CONSTRUCT LN RECOVERIES - "#)]
    pub crrecnfm: Option<f32>,

    #[schemars(description = r#"OTHER CONSTRUCT LN RECOVERIES - "#)]
    pub crrecnot: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN RECOVER-QTR - "#)]
    pub crreconq: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN RECOVERIES QUARTERLY RATIO - "#)]
    pub crreconqr: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN RECOVERIES - "#)]
    pub crrecons: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN RECOVERIES RATIO - "#)]
    pub crreconsr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN RECOVERIES - FOR - "#)]
    pub crrefor: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN RECOVERIES - FOR RATIO - "#)]
    pub crreforr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN RECOVERIES - FOR QUARTERLY - "#)]
    pub crreforq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN RECOVERIES - FOR QUARTERLY RATIO - "#)]
    pub crreforqr: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN RECOVERIES - "#)]
    pub crreloc: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN RECOVERIES RATIO - "#)]
    pub crrelocr: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN RECOVERIES QUARTERLY - "#)]
    pub crrelocq: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN RECOVERIES QUARTERLY RATIO - "#)]
    pub crrelocqr: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RE LN RECOVERIES-QTR - "#)]
    pub crremulq: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RES RE LN RECOVERIES QUARTERLY RATIO - "#)]
    pub crremulqr: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RES RE LN RECOVERIES - "#)]
    pub crremult: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RES RE LN RECOVERIES RATIO - "#)]
    pub crremultr: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN RECOVERIES - "#)]
    pub crrenres: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN RECOVERIES RATIO - "#)]
    pub crrenresr: Option<f32>,

    #[schemars(description = r#"OTHER NONFARM NONRES RECOVERIES - "#)]
    pub crrenrot: Option<f32>,

    #[schemars(description = r#"OWN-OCCUP NONFARM NONRES RECOV - "#)]
    pub crrenrow: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN RECOVER-QTR - "#)]
    pub crrenrsq: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN RECOVER-QTR RATIO - "#)]
    pub crrenrsqr: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN RECOVERIES - "#)]
    pub crrenus: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN RECOVERIES RATIO - "#)]
    pub crrenusr: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN RECOVERIES QUARTERLY - "#)]
    pub crrenusq: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN RECOVERIES QUARTERLY RATIO - "#)]
    pub crrenusqr: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY RECOVERIES - "#)]
    pub crreres: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY RECOVERIES RATIO - "#)]
    pub crreresr: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY RECOVER-QTR - "#)]
    pub crreresq: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY RECOVERIES QUARTERLY RATIO - "#)]
    pub crreresqr: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM JR LIEN-RECOVER - "#)]
    pub crrersf2: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM JR LIEN-RECOVER RATIO - "#)]
    pub crrersf2r: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM JR LIEN-RECOVER QUARTERLY - "#)]
    pub crrers2q: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM JR LIEN-RECOVER QUARTERLY RATIO - "#)]
    pub crrers2qr: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM FIRST LIEN-RECOV - "#)]
    pub crrersfm: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM FIRST LIEN-RECOV RATIO - "#)]
    pub crrersfmr: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM FIRST LIEN-RECOV QUARTERLY - "#)]
    pub crrersfq: Option<f32>,

    #[schemars(description = r#"RE LOAN 1-4 FAM FIRST LIEN-RECOV QUARTERLY RATIO - "#)]
    pub crrersfqr: Option<f32>,

    #[schemars(description = r#"RE LOAN RECOVERIES DOMESTIC OFFICES - "#)]
    pub crreoffdom: Option<f32>,

    #[schemars(description = r#"RE LOAN RECOVERIES DOMESTIC OFFICES RATIO - "#)]
    pub crreoffdomr: Option<f32>,

    #[schemars(description = r#"RE LOAN RECOVERIES DOMESTIC OFFICES QUARTERLY - "#)]
    pub crreoffdomq: Option<f32>,

    #[schemars(description = r#"RE LOAN RECOVERIES DOMESTIC OFFICES QUARTERLY RATIO - "#)]
    pub crreoffdomqr: Option<f32>,

    #[schemars(description = r#"CR DER (NET)-PURCHASE PROTECT - "#)]
    pub ctderben: Option<f32>,

    #[schemars(description = r#"CR DER(NET) - SOLD PROTECTION - "#)]
    pub ctdergty: Option<f32>,

    #[schemars(description = r#"TOTAL DEPOSIT LIAB BEF EXCLUSION - "#)]
    pub depbefex: Option<f32>,

    #[schemars(description = r#"ESTIMATED ASSESSABLE DEPOSITS - "#)]
    pub depcsbq: Option<f32>,

    #[schemars(description = r#"ESTIMATED ASSESSABLE DEPOSITS RATIO - "#)]
    pub depcsbqr: Option<f32>,

    #[schemars(description = r#"TOT DOMESTIC DEPOSIT / ASSET - "#)]
    pub depdastr: Option<f32>,

    #[schemars(description = r#"FOREIGN BANKS-FOR - "#)]
    pub depfbkf: Option<f32>,

    #[schemars(description = r#"FOREIGN BANKS-FOR RATIO - "#)]
    pub depfbkfr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENTS-FOR - "#)]
    pub depfgovf: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENTS-FOR RATIO - "#)]
    pub depfgovfr: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING DEP-DOM - "#)]
    pub depidom: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING DEP-DOM RATIO - "#)]
    pub depidomr: Option<f32>,

    #[schemars(description = r#"ESTIMATED INSURED DEPOSITS - "#)]
    pub depins: Option<f32>,

    #[schemars(description = r#"ESTIMATED INSURED DEPOSITS RATIO - "#)]
    pub depinsr: Option<f32>,

    #[schemars(description = r#"AMT DEP ACC GREATER THAN $250,000 - "#)]
    pub deplgamt: Option<f32>,

    #[schemars(description = r#"AMT DEP ACC GREATER THAN $250,000 RATIO - "#)]
    pub deplgamtr: Option<f32>,

    #[schemars(description = r#"NUM DEP ACC GREATER THAN $250,000 - "#)]
    pub deplgb: Option<f32>,

    #[schemars(description = r#"AMT OF RETIREMENT DEP ACC OF MORE THAN $250,000 - "#)]
    pub deplgra: Option<f32>,

    #[schemars(description = r#"AMT OF RETIREMENT DEP ACC OF MORE THAN $250,000 RATIO - "#)]
    pub deplgrar: Option<f32>,

    #[schemars(description = r#"NUM OF RETIREMENT DEP ACC MORE THAN $250,000 - "#)]
    pub deplgrn: Option<f32>,

    #[schemars(description = r#"DEP THRU LIST SVC NOT BROKERED - "#)]
    pub deplsnb: Option<f32>,

    #[schemars(description = r#"DEP THRU LIST SVC NOT BROKERED RATIO - "#)]
    pub deplsnbr: Option<f32>,

    #[schemars(description = r#"NONINTEREST-BEARING DEP-DOM - "#)]
    pub depnidom: Option<f32>,

    #[schemars(description = r#"NONINTEREST-BEARING DEP-DOM RATIO - "#)]
    pub depnidomr: Option<f32>,

    #[schemars(description = r#"AMT DEP ACC AT $250,000 OR LESS - "#)]
    pub depsmamt: Option<f32>,

    #[schemars(description = r#"AMT DEP ACC AT $250,000 OR LESS RATIO - "#)]
    pub depsmamtr: Option<f32>,

    #[schemars(description = r#"NUM DEP ACC EQUAL OR LESS THAN EQUAL TO $250,000 - "#)]
    pub depsmb: Option<f32>,

    #[schemars(description = r#"AMT RETIREMENT DEP ACC OF $250,000 OR LESS - "#)]
    pub depsmra: Option<f32>,

    #[schemars(description = r#"AMT RETIREMENT DEP ACC OF $250,000 OR LESS RATIO - "#)]
    pub depsmrar: Option<f32>,

    #[schemars(description = r#"NUM RETIREMENT DEP ACC OF $250,000 - "#)]
    pub depsmrn: Option<f32>,

    #[schemars(description = r#"TOTAL ALLOWABLE EXCLUSIONS (INCLUDING FOREIGN DEPOSITS) - "#)]
    pub depallex: Option<f32>,

    #[schemars(description = r#"EST UNINSURED DEP IN DOM-OFF IN INSURED BRANCHES IN US TERR AND POSSESSIONS - "#)]
    pub depuna: Option<f32>,

    #[schemars(description = r#"EST UNINSURED DEP IN DOM-OFF IN INSURED BRANCHES IN US TERR AND POSSESSIONS - "#)]
    pub depunar: Option<f32>,

    #[schemars(description = r#"ESTIMATED UNINSURED DEPOSITS IN DOMESTIC OFFICES AND IN INSURED BRANCHES IN US TERRITORIES AND POSSESSIONS - "#)]
    pub depunins: Option<f32>,

    #[schemars(description = r#"U.S. BANKS&OTH.US INST-FOR - "#)]
    pub depusbkf: Option<f32>,

    #[schemars(description = r#"U.S. BANKS&OTH.US INST-FOR RATIO - "#)]
    pub depusbkfr: Option<f32>,

    #[schemars(description = r#"U.S.GOVT & ST & POL SUBS-FOR - "#)]
    pub depusmf: Option<f32>,

    #[schemars(description = r#"U.S.GOVT & ST & POL SUBS-FOR RATIO - "#)]
    pub depusmfr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN CHARGE-OFFS - "#)]
    pub drag: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN CHARGE-OFFS RATIO - "#)]
    pub dragr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub dragq: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub dragqr: Option<f32>,

    #[schemars(description = r#"AG LOAN CHARGE-OFFS*SMALL BKS - "#)]
    pub dragsm: Option<f32>,

    #[schemars(description = r#"AG LOAN CHARGE-OFFS*SMALL BKS RATIO - "#)]
    pub dragsmr: Option<f32>,

    #[schemars(description = r#"AG LOAN CHARGE-OFFS*SMALL BKS QUARTERLY - "#)]
    pub dragsmq: Option<f32>,

    #[schemars(description = r#"AG LOAN CHARGE-OFFS*SMALL BKS QUARTERLY RATIO - "#)]
    pub dragsmqr: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - CHARGE-OFFS - "#)]
    pub drauto: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - CHARGE-OFFS RATIO - "#)]
    pub drautor: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - CHARGE-OFFS QUARTERLY - "#)]
    pub drautoq: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drautoqr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS - "#)]
    pub drci: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS RATIO - "#)]
    pub drcir: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub drciq: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drciqr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS NON-U.S. - "#)]
    pub drcinus: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS NON-U.S. RATIO - "#)]
    pub drcinusr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS NON-U.S. QUARTERLY - "#)]
    pub drcinusq: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN CHARGE-OFFS NON-U.S. QUARTERLY RATIO - "#)]
    pub drcinusqr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN CHARGE-OFFS - "#)]
    pub drcon: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN CHARGE-OFFS RATIO - "#)]
    pub drconr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub drconq: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drconqr: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN CHARGE-OFFS - "#)]
    pub drconoth: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN CHARGE-OFFS RATIO - "#)]
    pub drconothr: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub drconotq: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drconotqr: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN CHARGE-OFFS - "#)]
    pub drcrcd: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN CHARGE-OFFS RATIO - "#)]
    pub drcrcdr: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub drcrcdq: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drcrcdqr: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN CHARGE-OFFS - "#)]
    pub drdep: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN CHARGE-OFFS RATIO - "#)]
    pub drdepr: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub drdepq: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drdepqr: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN CHG-OFFS - "#)]
    pub drdepnus: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN CHG-OFFS RATIO - "#)]
    pub drdepnusr: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN CHG-OFFS QUARTERLY - "#)]
    pub drdepnuq: Option<f32>,

    #[schemars(description = r#"FOREIGN DEPS INST LN CHG-OFFS QUARTERLY RATIO - "#)]
    pub drdepnuqr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN CHG-OFFS - "#)]
    pub drforgv: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN CHG-OFFS RATIO - "#)]
    pub drforgvr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN CHG-OFFS QUARTERLY - "#)]
    pub drforgvq: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVERNMENT LN CHG-OFFS QUARTERLY RATIO - "#)]
    pub drforgvqr: Option<f32>,

    #[schemars(description = r#"LEASE CHARGE-OFFS - "#)]
    pub drls: Option<f32>,

    #[schemars(description = r#"LEASE CHARGE-OFFS RATIO - "#)]
    pub drlsr: Option<f32>,

    #[schemars(description = r#"LEASE CHARGE-OFFS QUARTERLY - "#)]
    pub drlsq: Option<f32>,

    #[schemars(description = r#"LEASE CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drlsqr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN CHARGE-OFFS - "#)]
    pub drother: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN CHARGE-OFFS RATIO - "#)]
    pub drotherr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub drothq: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drothqr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS - "#)]
    pub drre: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS RATIO - "#)]
    pub drrer: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS QUARTERLY - "#)]
    pub drreq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drreqr: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN CHARGE-OFFS - "#)]
    pub drreag: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN CHARGE-OFFS RATIO - "#)]
    pub drreagr: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN CHG-OFFS-QTR - "#)]
    pub drreagq: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drreagqr: Option<f32>,

    #[schemars(description = r#"1-4 FAM CONSTRUCT LN CHARGE-OFFS - "#)]
    pub drrecnfm: Option<f32>,

    #[schemars(description = r#"OTHER CONSTRUCT LN CHARGE-OFFS - "#)]
    pub drrecnot: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN CHG-OFFS-QTR - "#)]
    pub drreconq: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drreconqr: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN CHARGE-OFFS - "#)]
    pub drrecons: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN CHARGE-OFFS RATIO - "#)]
    pub drreconsr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHRG-OFFS-FOR - "#)]
    pub drrefor: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHRG-OFFS-FOR RATIO - "#)]
    pub drreforr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHRG-OFFS-FOR QUARTERLY - "#)]
    pub drreforq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHRG-OFFS-FOR QUARTERLY RATIO - "#)]
    pub drreforqr: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN CHARGE-OFFS - "#)]
    pub drreloc: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN CHARGE-OFFS RATIO - "#)]
    pub drrelocr: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN CHARGE-OFFS QUARTERLY - "#)]
    pub drrelocq: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN CHARGE-OFFS RATIO - "#)]
    pub drrelocqr: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RE LN CHG-OFFS-QTR - "#)]
    pub drremulq: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RES RE LN CHARGE-OFF QUARTERLY RATIO - "#)]
    pub drremulqr: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RES RE LN CHARGE-OFF - "#)]
    pub drremult: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RES RE LN CHARGE-OFF RATIO - "#)]
    pub drremultr: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN CHARGE-OFFS - "#)]
    pub drrenres: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN CHARGE-OFFS RATIO - "#)]
    pub drrenresr: Option<f32>,

    #[schemars(description = r#"OTHER NONFARM NONRES RE CHG-OFF - "#)]
    pub drrenrot: Option<f32>,

    #[schemars(description = r#"OWN-OCCUP NONFARM NONRES CHG-OFF - "#)]
    pub drrenrow: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN CHG-OFF-QTR - "#)]
    pub drrenrsq: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drrenrsqr: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN CHARGE-OFFS - "#)]
    pub drrenus: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN CHARGE-OFFS RATIO - "#)]
    pub drrenusr: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN CHARGE-OFFS QUARTERLY - "#)]
    pub drrenusq: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN CHARGE-OFFS RATIO - "#)]
    pub drrenusqr: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY CHARGE-OFFS - "#)]
    pub drreres: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY CHARGE-OFFS RATIO - "#)]
    pub drreresr: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY CHG-OFFS-QTR - "#)]
    pub drreresq: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub drreresqr: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-CHG-OFF - "#)]
    pub drrersf2: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-CHG-OFF RATIO - "#)]
    pub drrersf2r: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-CHG-OFF QUARTERLY - "#)]
    pub drrers2q: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-CHG-OFF QUARTERLY RATIO - "#)]
    pub drrers2qr: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM FIRST LIEN-CHG-OFF - "#)]
    pub drrersfm: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM FIRST LIEN-CHG-OFF RATIO - "#)]
    pub drrersfmr: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM FIRST LIEN-CHG-OFF QUARTERLY - "#)]
    pub drrersfq: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM FIRST LIEN-CHG-OFF QUARTERLY RATIO - "#)]
    pub drrersfqr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES - "#)]
    pub drreoffdom: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES RATIO - "#)]
    pub drreoffdomr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES QUARTERLY - "#)]
    pub drreoffdomq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES QUARTERLY RATIO - "#)]
    pub drreoffdomqr: Option<f32>,

    #[schemars(description = r#"EQUITY - "#)]
    pub edcm: Option<f32>,

    #[schemars(description = r#"EFFICIENCY RATIO EXPENSE - "#)]
    pub eeff: Option<f32>,

    #[schemars(description = r#"EFFICIENCY RATIO EXPENSE QUARTERLY - "#)]
    pub eeffq: Option<f32>,

    #[schemars(description = r#"EFFICIENCY RATIO - "#)]
    pub eeffr: Option<f32>,

    #[schemars(description = r#"EFFICIENCY QUARTERLY RATIO - "#)]
    pub eeffqr: Option<f32>,

    #[schemars(description = r#"EFFECTIVE DATE - "#)]
    pub effdate: Option<f32>,

    #[schemars(description = r#"GOODWILL IMPAIRMENT LOSSES - "#)]
    pub eintgw: Option<f32>,

    #[schemars(description = r#"GOODWILL IMPAIRMENT LOSSES RATIO - "#)]
    pub eintgwr: Option<f32>,

    #[schemars(description = r#"GOODWILL IMPAIRMENT LOSSES QUARTERLY - "#)]
    pub eintgwq: Option<f32>,

    #[schemars(description = r#"GOODWILL IMPAIRMENT LOSSES QUARTERLY RATIO - "#)]
    pub eintgwqr: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSSES OTH INTAN - "#)]
    pub eintoth: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSSES OTH INTAN RATIO - "#)]
    pub eintothr: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSSES OTH INTAN QUARTERLY - "#)]
    pub eintothq: Option<f32>,

    #[schemars(description = r#"AMORT & IMPAIR LOSSES OTH INTAN QUARTERLY RATIO - "#)]
    pub eintothqr: Option<f32>,

    #[schemars(description = r#"LOAN LOSS PROV/NT CHG-OFFS - "#)]
    pub elnantr: Option<f32>,

    #[schemars(description = r#"ELNATRA - "#)]
    pub elnatra: Option<f32>,

    #[schemars(description = r#"CREDIT LOSS PROV/AVE ASSETS - "#)]
    pub elnatry: Option<f32>,

    #[schemars(description = r#"CREDIT LOSS PROV/AVE ASSETS QUARTERLY - "#)]
    pub elnatryq: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE-ENHANCEMENTS - AUTO - "#)]
    pub enceauto: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE-ENHANCEMENTS - AUTO RATIO - "#)]
    pub enceautor: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - CI - "#)]
    pub enceci: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - CI RATIO - "#)]
    pub encecir: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - CON - "#)]
    pub encecon: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - CON RATIO - "#)]
    pub enceconr: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - OTH - "#)]
    pub enceoth: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - OTH RATIO - "#)]
    pub enceothr: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - RES - "#)]
    pub enceres: Option<f32>,

    #[schemars(description = r#"CR EXPOSURE - ENHANCEMENTS - RES RATIO - "#)]
    pub enceresr: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST EXPENSE - "#)]
    pub eothint: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST EXPENSE RATIO - "#)]
    pub eothintr: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST EXPENSE QUARTERLY - "#)]
    pub eothintq: Option<f32>,

    #[schemars(description = r#"OTHER INTEREST EXPENSE QUARTERLY RATIO - "#)]
    pub eothintqr: Option<f32>,

    #[schemars(description = r#"TOTAL BANK EQUITY CAPITAL-CAVG5 - "#)]
    pub eq5: Option<f32>,

    #[schemars(description = r#"TRANSACTIONS WITH BHC - "#)]
    pub eqcbhctr: Option<f32>,

    #[schemars(description = r#"TRANSACTIONS WITH BHC RATIO - "#)]
    pub eqcbhctrr: Option<f32>,

    #[schemars(description = r#"OTHER COMPREHENSIVE INCOME - "#)]
    pub eqccompi: Option<f32>,

    #[schemars(description = r#"OTHER COMPREHENSIVE INCOME RATIO - "#)]
    pub eqccompir: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS ON COMM & PFD-ANN - "#)]
    pub eqcdiva: Option<f32>,

    #[schemars(description = r#"CHANGES DUE TO MERGERS - "#)]
    pub eqcmrg: Option<f32>,

    #[schemars(description = r#"CHANGES DUE TO MERGERS RATIO - "#)]
    pub eqcmrgr: Option<f32>,

    #[schemars(description = r#"BK EQ CAP MOST RECENTLY REPORTED - "#)]
    pub eqcprev: Option<f32>,

    #[schemars(description = r#"BK EQ CAP MOST RECENTLY REPORTED RATIO - "#)]
    pub eqcprevr: Option<f32>,

    #[schemars(description = r#"ACCOUNTING CHANGES & CORRECTIONS - "#)]
    pub eqcrest: Option<f32>,

    #[schemars(description = r#"ACCOUNTING CHANGES & CORRECTIONS RATIO - "#)]
    pub eqcrestr: Option<f32>,

    #[schemars(description = r#"SALE OF CAPITAL STOCK - "#)]
    pub eqcstkrx: Option<f32>,

    #[schemars(description = r#"SALE OF CAPITAL STOCK RATIO - "#)]
    pub eqcstkrxr: Option<f32>,

    #[schemars(description = r#"SALE OF CAPITAL STOCK QUARTERLY - "#)]
    pub eqcsxq: Option<f32>,

    #[schemars(description = r#"SALE OF CAPITAL STOCK QUARTERLY RATIO - "#)]
    pub eqcsxqr: Option<f32>,

    #[schemars(description = r#"TREASURY STOCK TRANSACTIONS - "#)]
    pub eqctrstx: Option<f32>,

    #[schemars(description = r#"TREASURY STOCK TRANSACTIONS RATIO - "#)]
    pub eqctrstxr: Option<f32>,

    #[schemars(description = r#"TOTAL EQUITY CAPITAL - "#)]
    pub eqtot: Option<f32>,

    #[schemars(description = r#"TOTAL EQUITY CAPITAL RATIO - "#)]
    pub eqtotr: Option<f32>,

    #[schemars(description = r#"BANK EQUITY CAPITAL/ASSETS - "#)]
    pub eqv: Option<f32>,

    #[schemars(description = r#"TOTAL EARNING ASSETS - "#)]
    pub ernast: Option<f32>,

    #[schemars(description = r#"TOTAL EARNING ASSETS - "#)]
    pub ernast2: Option<f32>,

    #[schemars(description = r#"TOTAL EARNING ASSETS-CAVG5I - "#)]
    pub ernast5: Option<f32>,

    #[schemars(description = r#"EARNING ASSETS / TOTAL ASSETS - "#)]
    pub ernastr: Option<f32>,

    #[schemars(description = r#"ESTABLISHED DATE - "#)]
    pub estymd: Option<f32>,

    #[schemars(description = r#"INACTIVE DATE - "#)]
    pub endefymd: Option<f32>,

    #[schemars(description = r#"INACTIVE DATE - "#)]
    pub org_end_num_dte: Option<f32>,

    #[schemars(description = r#"TT&L - "#)]
    pub ettlotmg: Option<f32>,

    #[schemars(description = r#"THRIFT FINANCIAL REPORT FLAG - "#)]
    pub formtfr: Option<f32>,

    #[schemars(description = r#"FOREIGN EXCHANGE-TOTAL CONTRACTS - "#)]
    pub fx: Option<f32>,

    #[schemars(description = r#"FOR EXCH-FUTURES & FORWARD CONTR - "#)]
    pub fxffc: Option<f32>,

    #[schemars(description = r#"FOR EXCHANGE-SWAPS - "#)]
    pub fxnvs: Option<f32>,

    #[schemars(description = r#"FOR EXCH-PUR OPTION CONTRACTS - "#)]
    pub fxpoc: Option<f32>,

    #[schemars(description = r#"SPOT FOREIGN EXCHANGE CONTRACTS - "#)]
    pub fxspot: Option<f32>,

    #[schemars(description = r#"FOR EXCH-WRITTEN OPTION CONTRACT - "#)]
    pub fxwoc: Option<f32>,

    #[schemars(description = r#"INC BEFORE INC TAXS & DISC-QTR - "#)]
    pub ibeftxq: Option<f32>,

    #[schemars(description = r#"INCOME BEFORE DISC OPR - "#)]
    pub ibefxtr: Option<f32>,

    #[schemars(description = r#"INCOME BEFORE DISC OPR RATIO - "#)]
    pub ibefxtrr: Option<f32>,

    #[schemars(description = r#"INCOME BEFORE DISC OPR QUARTERLY - "#)]
    pub ibefxtrq: Option<f32>,

    #[schemars(description = r#"EFFICIENCY RATIO INCOME - "#)]
    pub ieff: Option<f32>,

    #[schemars(description = r#"EFFICIENCY RATIO INCOME QUARTERLY - "#)]
    pub ieffq: Option<f32>,

    #[schemars(description = r#"INCOME BEFORE DISC OPR QUARTERLY RATIO - "#)]
    pub ibefxtrqr: Option<f32>,

    #[schemars(description = r#"FIDUCIARY ACTIVITIES INCOME - "#)]
    pub ifiduc: Option<f32>,

    #[schemars(description = r#"FIDUCIARY ACTIVITIES INCOME RATIO - "#)]
    pub ifiducr: Option<f32>,

    #[schemars(description = r#"FIDUCIARY ACTIVITIES INCOME-QTR - "#)]
    pub ifiducq: Option<f32>,

    #[schemars(description = r#"FIDUCIARY ACTIVITIES INCOME-QTR RATIO - "#)]
    pub ifiducqr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-COMMODITY - "#)]
    pub iglcmex: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-COMMODITY RATIO - "#)]
    pub iglcmexr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-COMMODITY QUARTERLY - "#)]
    pub iglcmexq: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-COMMODITY RATIO QUARTERLY - "#)]
    pub iglcmexqr: Option<f32>,

    #[schemars(description = r#"TRADING REVENUE- CREDIT EXPOSURE - "#)]
    pub iglcrex: Option<f32>,

    #[schemars(description = r#"TRADING REVENUE- CREDIT EXPOSURE RATIO - "#)]
    pub iglcrexr: Option<f32>,

    #[schemars(description = r#"TRADING REVENUE- CREDIT EXPOSURE QUARTERLY - "#)]
    pub iglcrexq: Option<f32>,

    #[schemars(description = r#"TRADING REVENUE- CREDIT EXPOSURE QUARTERLY RATIO - "#)]
    pub iglcrexqr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-EQ DERIVATIVE - "#)]
    pub igledex: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-EQ DERIVATIVE RATIO - "#)]
    pub igledexr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-EQ DERIVATIVE QUARTERLY - "#)]
    pub igledexq: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-EQ DERIVATIVE QUARTERLY RATIO - "#)]
    pub igledexqr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-FOREIGN EXCHANGE - "#)]
    pub iglfxex: Option<f32>,

    #[schemars(description = r#"RADING ACCOUNT-FOREIGN EXCHANGE RATIO - "#)]
    pub iglfxexr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-FOREIGN EXCHANGE QUARTERLY - "#)]
    pub iglfxexq: Option<f32>,

    #[schemars(description = r#"RADING ACCOUNT-FOREIGN EXCHANGE QUARTERLY RATIO - "#)]
    pub iglfxexqr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-INTEREST RATE - "#)]
    pub iglrtex: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-INTEREST RATE RATIO - "#)]
    pub iglrtexr: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-INTEREST RATE QUARTERLY - "#)]
    pub iglrtexq: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNT-INTEREST RATE QUARTERLY RATIO - "#)]
    pub iglrtexqr: Option<f32>,

    #[schemars(description = r#"SECURITIES GAINS AND LOSSES-QTR - "#)]
    pub iglsecq: Option<f32>,

    #[schemars(description = r#"TRADING REVENUES-TOTAL - "#)]
    pub igltrad: Option<f32>,

    #[schemars(description = r#"TRADING REVENUES-TOTAL RATIO - "#)]
    pub igltradr: Option<f32>,

    #[schemars(description = r#"TRADING REVENUE-QTR - "#)]
    pub igltrdq: Option<f32>,

    #[schemars(description = r#"TRADING REVENUE-QTR RATIO - "#)]
    pub igltrdqr: Option<f32>,

    #[schemars(description = r#"INSURANCE COMMISSIONS & FEES - "#)]
    pub iinscom: Option<f32>,

    #[schemars(description = r#"INSURANCE COMMISSIONS & FEES RATIO - "#)]
    pub iinscomr: Option<f32>,

    #[schemars(description = r#"INSURANCE COMMISSIONS & FEES QUARTERLY - "#)]
    pub iinscomq: Option<f32>,

    #[schemars(description = r#"INSURANCE COMMISSIONS & FEES QUARTERLY RATIO - "#)]
    pub iinscomqr: Option<f32>,

    #[schemars(description = r#"INSURANCE COM+FEES-OTHER - "#)]
    pub iinsoth: Option<f32>,

    #[schemars(description = r#"INSURANCE COM+FEES-OTHER RATIO - "#)]
    pub iinsothr: Option<f32>,

    #[schemars(description = r#"INSURANCE COM+FEES-OTHER QUARTERLY - "#)]
    pub iinsothq: Option<f32>,

    #[schemars(description = r#"INSURANCE COM+FEES-OTHER QUARTERLY RATIO - "#)]
    pub iinsothqr: Option<f32>,

    #[schemars(description = r#"INSURANCE UNDERWRITNG INCOME - "#)]
    pub iinsund: Option<f32>,

    #[schemars(description = r#"INSURANCE UNDERWRITNG INCOME RATIO - "#)]
    pub iinsundr: Option<f32>,

    #[schemars(description = r#"INSURANCE UNDERWRITNG INCOME QUARTERLY - "#)]
    pub iinsundq: Option<f32>,

    #[schemars(description = r#"INSURANCE UNDERWRITNG INCOME QUARTERLY RATIO - "#)]
    pub iinsundqr: Option<f32>,

    #[schemars(description = r#"INVEST BANK - "#)]
    pub iinvfee: Option<f32>,

    #[schemars(description = r#"INVEST BANK RATIO - "#)]
    pub iinvfeer: Option<f32>,

    #[schemars(description = r#"INVEST BANK QUARTERLY - "#)]
    pub iinvfeeq: Option<f32>,

    #[schemars(description = r#"INVEST BANK QUARTERLY RATIO - "#)]
    pub iinvfeeqr: Option<f32>,

    #[schemars(description = r#"PRIMARY INSURER (Search-Eligible) - This field can be used for search and filtering."#)]
    pub insagnt1: Option<String>,

    #[schemars(description = r#"PURCH CC REL & NONMTG SER ASTS - "#)]
    pub intangcc: Option<f32>,

    #[schemars(description = r#"GOODWILL - "#)]
    pub intangw: Option<f32>,

    #[schemars(description = r#"GOODWILL RATIO - "#)]
    pub intangwr: Option<f32>,

    #[schemars(description = r#"MORTGAGE SERVICING ASSETS - "#)]
    pub intanmsr: Option<f32>,

    #[schemars(description = r#"MORTGAGE SERVICING ASSETS RATIO - "#)]
    pub intanmsrr: Option<f32>,

    #[schemars(description = r#"OTHER IDENTIFIABLE INTANG ASSETS - "#)]
    pub intanoth: Option<f32>,

    #[schemars(description = r#"OTHER IDENTIFIABLE INTANG ASSETS RATIO - "#)]
    pub intanothr: Option<f32>,

    #[schemars(description = r#"INTEREST INCOME/EARNING ASSETS QUARTERLY - "#)]
    pub intincyq: Option<f32>,

    #[schemars(description = r#"TOTAL INTEREST INCOME ANNUAL - "#)]
    pub intinca: Option<f32>,

    #[schemars(description = r#"OTHER NONINTEREST INCOME - "#)]
    pub iotnii: Option<f32>,

    #[schemars(description = r#"OTHER NONINTEREST INCOME RATIO - "#)]
    pub iotniir: Option<f32>,

    #[schemars(description = r#"OTHER NONINTEREST INCOME QUARTERLY - "#)]
    pub iotniiq: Option<f32>,

    #[schemars(description = r#"OTHER NONINTEREST INCOME QUARTERLY RATIO - "#)]
    pub iotniiqr: Option<f32>,

    #[schemars(description = r#"SECURITIZATION INCOME - "#)]
    pub isecz: Option<f32>,

    #[schemars(description = r#"SECURITIZATION INCOME RATIO - "#)]
    pub iseczr: Option<f32>,

    #[schemars(description = r#"SECURITIZATION INCOME QUARTERLY - "#)]
    pub iseczq: Option<f32>,

    #[schemars(description = r#"SECURITIZATION INCOME QUARTERLY RATIO - "#)]
    pub iseczqr: Option<f32>,

    #[schemars(description = r#"SERVICE CHARGE ON DEP ACCTS-QTR - "#)]
    pub iserchgq: Option<f32>,

    #[schemars(description = r#"SERVICE CHARGE ON DEPOSIT ACCTS-QTR RATIO - "#)]
    pub iserchgqr: Option<f32>,

    #[schemars(description = r#"SERVICING FEES - "#)]
    pub iserfee: Option<f32>,

    #[schemars(description = r#"SERVICING FEES RATIO - "#)]
    pub iserfeer: Option<f32>,

    #[schemars(description = r#"SERVICING FEES QUARTERLY - "#)]
    pub iserfeeq: Option<f32>,

    #[schemars(description = r#"SERVICING FEES QUARTERLY RATIO - "#)]
    pub iserfeeqr: Option<f32>,

    #[schemars(description = r#"VENTURE CAPITAL REVENUE - "#)]
    pub ivencap: Option<f32>,

    #[schemars(description = r#"VENTURE CAPITAL REVENUE RATIO - "#)]
    pub ivencapr: Option<f32>,

    #[schemars(description = r#"VENTURE CAPITAL REVENUE QUARTERLY - "#)]
    pub ivencapq: Option<f32>,

    #[schemars(description = r#"VENTURE CAPITAL REVENUE QUARTERLY RATIO - "#)]
    pub ivencapqr: Option<f32>,

    #[schemars(description = r#"AG LOANS - LOSS SHARE - "#)]
    pub lag: Option<f32>,

    #[schemars(description = r#"AG LOANS - LOSS SHARE RATIO - "#)]
    pub lagr: Option<f32>,

    #[schemars(description = r#"C&I LOANS - LOSS SHARE - "#)]
    pub lci: Option<f32>,

    #[schemars(description = r#"C&I LOANS - LOSS SHARE RATIO - "#)]
    pub lcir: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS - LOSS SHARE - "#)]
    pub lcon: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS - LOSS SHARE RATIO - "#)]
    pub lconr: Option<f32>,

    #[schemars(description = r#"TOTAL LIABILITIES-FOR - "#)]
    pub liabfor: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 100-$ - "#)]
    pub lnag1: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 100-$ RATIO - "#)]
    pub lnag1r: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-100-250-$ - "#)]
    pub lnag2: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-100-250-$ RATIO - "#)]
    pub lnag2r: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-250-500-$ - "#)]
    pub lnag3: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-250-500-$ RATIO - "#)]
    pub lnag3r: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 500-$ - "#)]
    pub lnag4: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 500-$ RATIO - "#)]
    pub lnag4r: Option<f32>,

    #[schemars(description = r#"AG LOANS-CAVG5 - "#)]
    pub lnag5: Option<f32>,

    #[schemars(description = r#"AG LOANS-CAVG2 - "#)]
    pub lnag22: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 100-NUM - "#)]
    pub lnag1n: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 100-NUM RATIO - "#)]
    pub lnag1nr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-100-250-NUM - "#)]
    pub lnag2n: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-100-250-NUM RATIO - "#)]
    pub lnag2nr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-250-500-NUM - "#)]
    pub lnag3n: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-250-500-NUM RATIO - "#)]
    pub lnag3nr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 500-NUM - "#)]
    pub lnag4n: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-UNDER 500-NUM RATIO - "#)]
    pub lnag4nr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-FOR - "#)]
    pub lnagfor: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LOANS-FOR RATIO - "#)]
    pub lnagforr: Option<f32>,

    #[schemars(description = r#"LOAN LOSS RESERVE/GROSS LN&LS - "#)]
    pub lnatresr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS - AUTO - CAVG2 - "#)]
    pub lnauto2: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS - AUTO - CAVG5 - "#)]
    pub lnauto5: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-100-$ - "#)]
    pub lnci1: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-100-$ RATIO - "#)]
    pub lnci1r: Option<f32>,

    #[schemars(description = r#"C&I LOANS-100-250-$ - "#)]
    pub lnci2: Option<f32>,

    #[schemars(description = r#"C&I LOANS-100-250-$ RATIO - "#)]
    pub lnci2r: Option<f32>,

    #[schemars(description = r#"C&I LOANS-250-1M-$ - "#)]
    pub lnci3: Option<f32>,

    #[schemars(description = r#"C&I LOANS-250-1M-$ RATIO - "#)]
    pub lnci3r: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-1M-$ - "#)]
    pub lnci4: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-1M-$ RATIO - "#)]
    pub lnci4r: Option<f32>,

    #[schemars(description = r#"C&I LOANS-CAVG5 - "#)]
    pub lnci5: Option<f32>,

    #[schemars(description = r#"C&I LOANS-CAVG2 - "#)]
    pub lnci22: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-100-NUM - "#)]
    pub lnci1n: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-100-NUM RATIO - "#)]
    pub lnci1nr: Option<f32>,

    #[schemars(description = r#"C&I LOANS-100-250-NUM - "#)]
    pub lnci2n: Option<f32>,

    #[schemars(description = r#"C&I LOANS-250-1M-NUM RATIO - "#)]
    pub lnci2nr: Option<f32>,

    #[schemars(description = r#"C&I LOANS-250-1M-NUM - "#)]
    pub lnci3n: Option<f32>,

    #[schemars(description = r#"C&I LOANS-250-1M-NUM RATIO - "#)]
    pub lnci3nr: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-1M-NUM - "#)]
    pub lnci4n: Option<f32>,

    #[schemars(description = r#"C&I LOANS-UNDER-1M-NUM RATIO - "#)]
    pub lnci4nr: Option<f32>,

    #[schemars(description = r#"C&I LOANS-FOR - "#)]
    pub lncifor: Option<f32>,

    #[schemars(description = r#"C&I LOANS-FOR RATIO - "#)]
    pub lnciforr: Option<f32>,

    #[schemars(description = r#"C&I LOANS-NON-U.S. DOMICILE - "#)]
    pub lncinus: Option<f32>,

    #[schemars(description = r#"C&I LOANS-NON-U.S. DOMICILE-FOR - "#)]
    pub lncinusf: Option<f32>,

    #[schemars(description = r#"C&I LOANS-NON-U.S. DOMICILE-FOR RATIO - "#)]
    pub lncinusfr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE LOANS - "#)]
    pub lncomre: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE LOANS RATIO - "#)]
    pub lncomrer: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE LOANS2 - "#)]
    pub lncomre2: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE LOANS CAVG5 - "#)]
    pub lncomre5: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-CAVG2 - "#)]
    pub lncon2: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-CAVG5 - "#)]
    pub lncon5: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-FOR - "#)]
    pub lnconfor: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS-FOR RATIO - "#)]
    pub lnconforr: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER & RELATED PLANS - "#)]
    pub lnconorp: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOANS-CAVG2 - "#)]
    pub lnconot2: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOANS-CAVG5 - "#)]
    pub lnconot5: Option<f32>,

    #[schemars(description = r#"CONSUMER LNS-RELATED PLANS - "#)]
    pub lnconrp: Option<f32>,

    #[schemars(description = r#"CONSUMER LNS-RELATED PLANS RATIO - "#)]
    pub lnconrpr: Option<f32>,

    #[schemars(description = r#"OTHER CONTRA ACCOUNTS - "#)]
    pub lncontra: Option<f32>,

    #[schemars(description = r#"OTHER CONTRA ACCOUNTS RATIO - "#)]
    pub lncontrar: Option<f32>,

    #[schemars(description = r#"CREDIT CARD PLANS-CAVG2 - "#)]
    pub lncrcd2: Option<f32>,

    #[schemars(description = r#"CREDIT CARD PLANS-CAVG5 - "#)]
    pub lncrcd5: Option<f32>,

    #[schemars(description = r#"TOTAL DEP INST LNS & ACCEPT - "#)]
    pub lndepac: Option<f32>,

    #[schemars(description = r#"TOTAL DEP INST LNS & ACCEPT-DOM - "#)]
    pub lndepacd: Option<f32>,

    #[schemars(description = r#"LOANS TO DEPOSITORY INSTITUTIONS AND ACCEPTANCE OF OTHER BANKS - "#)]
    pub lndepaobk: Option<f32>,

    #[schemars(description = r#"LOANS TO DEPOSITORY INSTITUTIONS AND ACCEPTANCE OF OTHER BANKS RATIO - "#)]
    pub lndepaobkr: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-COMMERCIAL BANKS - "#)]
    pub lndepcb: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-COMMERCIAL BK-FOR - "#)]
    pub lndepcbf: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-COMMERCIAL BK-FOR RATIO - "#)]
    pub lndepcbfr: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-FOR COUNTRY - "#)]
    pub lndepfc: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-FOR COUNTRY-FOR - "#)]
    pub lndepfcf: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-FOR COUNTRY-FOR RATIO - "#)]
    pub lndepfcfr: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-FOR COUNTRY-U.S. BR - "#)]
    pub lndepfus: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-OTH U.S. INST - "#)]
    pub lndepus: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-COM BKS-U.S.BRANCH - "#)]
    pub lndepusb: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-OTH U.S. INST-FOR - "#)]
    pub lndepusf: Option<f32>,

    #[schemars(description = r#"DEP INST LNS-OTH U.S. INST-FOR RATIO - "#)]
    pub lndepusfr: Option<f32>,

    #[schemars(description = r#"EXECUTIVE OFFICER LOANS-AMOUNT - "#)]
    pub lnexamt: Option<f32>,

    #[schemars(description = r#"EXECUTIVE OFFICER LOANS-AMOUNT RATIO - "#)]
    pub lnexamtr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVT LOANS-FOR - "#)]
    pub lnfgfor: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVT LOANS-FOR RATIO - "#)]
    pub lnfgforr: Option<f32>,

    #[schemars(description = r#"NET LOANS & LEASES/DEPOSITS - "#)]
    pub lnlsdepr: Option<f32>,

    #[schemars(description = r#"LN&LS + UNEARNED INC-FOR - "#)]
    pub lnlsfor: Option<f32>,

    #[schemars(description = r#"LN&LS + UNEARNED INC-FOR RATIO - "#)]
    pub lnlsforr: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-TOTAL-CAVG5 - "#)]
    pub lnlsgr5: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-TOTAL-FOR - "#)]
    pub lnlsgrf: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES-TOTAL-FOR RATIO - "#)]
    pub lnlsgrfr: Option<f32>,

    #[schemars(description = r#"NET LOANS & LEASES/ASSETS - "#)]
    pub lnlsntv: Option<f32>,

    #[schemars(description = r#"NET LOANS & LEASES/ASSETS QUARTERLY RATIO - "#)]
    pub lnlsnqr: Option<f32>,

    #[schemars(description = r#"LOANS & LEASES HELD FOR RESALE - "#)]
    pub lnlssale: Option<f32>,

    #[schemars(description = r#"LOANS & LEASES HELD FOR RESALE RATIO - "#)]
    pub lnlssaler: Option<f32>,

    #[schemars(description = r#"PLEDGED LOANS AND LEASES - "#)]
    pub lnpledge: Option<f32>,

    #[schemars(description = r#"PLEDGED LOANS AND LEASES RATIO - "#)]
    pub lnpledger: Option<f32>,

    #[schemars(description = r#"MUNI LOANS-FOR - "#)]
    pub lnmunif: Option<f32>,

    #[schemars(description = r#"MUNI LOANS-FOR RATIO - "#)]
    pub lnmunifr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 1-3 YEARS - "#)]
    pub lnot1t3: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 1-3 YEARS RATIO - "#)]
    pub lnot1t3r: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS*3 MO OR LESS - "#)]
    pub lnot3les: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS*3 MO OR LESS RATIO - "#)]
    pub lnot3lesr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 3-5 YEARS - "#)]
    pub lnot3t5: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 3-5 YEARS RATIO - "#)]
    pub lnot3t5r: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 3-12 MONS - "#)]
    pub lnot3t12: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 3-12 MONS RATIO - "#)]
    pub lnot3t12r: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 5-15 YEARS - "#)]
    pub lnot5t15: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * 5-15 YEARS RATIO - "#)]
    pub lnot5t15r: Option<f32>,

    #[schemars(description = r#"OTHER LOANS & LEASES-QBP-CAVG2 - "#)]
    pub lnotci2: Option<f32>,

    #[schemars(description = r#"OTHER LOANS & LEASES-QBP-CAVG5 - "#)]
    pub lnotci5: Option<f32>,

    #[schemars(description = r#"LN TO NONDEP FIN INST & OTH-FGN - "#)]
    pub lnotherf: Option<f32>,

    #[schemars(description = r#"LN TO NONDEP FIN INST & OTH-FGN RATIO - "#)]
    pub lnotherfr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * OVER 15 YRS - "#)]
    pub lnotov15: Option<f32>,

    #[schemars(description = r#"ALL OTHER LNS & LS * OVER 15 YRS RATIO - "#)]
    pub lnotov15r: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 100-$ - "#)]
    pub lnreag1: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 100-$ RATIO - "#)]
    pub lnreag1r: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-100-250-$ - "#)]
    pub lnreag2: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-100-250-$ RATIO - "#)]
    pub lnreag2r: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-250-500-$ - "#)]
    pub lnreag3: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-250-500-$ RATIO - "#)]
    pub lnreag3r: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 500-$ - "#)]
    pub lnreag4: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 500-$ RATIO - "#)]
    pub lnreag4r: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 100-NUM - "#)]
    pub lnreag1n: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 100-NUM RATIO - "#)]
    pub lnreag1nr: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-100-250-NUM - "#)]
    pub lnreag2n: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-100-250-NUM RATIO - "#)]
    pub lnreag2nr: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-250-500-NUM - "#)]
    pub lnreag3n: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-250-500-NUM RATIO - "#)]
    pub lnreag3nr: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 500-NUM - "#)]
    pub lnreag4n: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-UNDER 500-NUM RATIO - "#)]
    pub lnreag4nr: Option<f32>,

    #[schemars(description = r#"1-4 FAM RE CONSTRUCTION LOANS - "#)]
    pub lnrecnfm: Option<f32>,

    #[schemars(description = r#"1-4 FAM RE CONSTRUCTION LOANS RATIO - "#)]
    pub lnrecnfmr: Option<f32>,

    #[schemars(description = r#"OTHER RE CONSTRUCTION & LAND LN - "#)]
    pub lnrecnot: Option<f32>,

    #[schemars(description = r#"OTHER RE CONSTRUCTION & LAND LN - "#)]
    pub lnrecnotr: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-1-4 FAMILY - "#)]
    pub lnreoth: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-1-4 FAMILY2 - "#)]
    pub lnreoth2: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY OTHER LOANS CAVG5 - "#)]
    pub lnreoth5: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 100-$ - "#)]
    pub lnrenr1: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 100-$ RATIO - "#)]
    pub lnrenr1r: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-100-250-$ - "#)]
    pub lnrenr2: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-100-250-$ RATIO - "#)]
    pub lnrenr2r: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-250-1M-$ - "#)]
    pub lnrenr3: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-250-1M-$ RATIO - "#)]
    pub lnrenr3r: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 1M-$ - "#)]
    pub lnrenr4: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 1M-$ RATIO - "#)]
    pub lnrenr4r: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 100-NUM - "#)]
    pub lnrenr1n: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 100-NUM RATIO - "#)]
    pub lnrenr1nr: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-100-250-NUM - "#)]
    pub lnrenr2n: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-100-250-NUM RATIO - "#)]
    pub lnrenr2nr: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-250-1M-NUM - "#)]
    pub lnrenr3n: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-250-1M-NUM RATIO - "#)]
    pub lnrenr3nr: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 1M-NUM - "#)]
    pub lnrenr4n: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES-UNDER 1M-NUM RATIO - "#)]
    pub lnrenr4nr: Option<f32>,

    #[schemars(description = r#"OTHER NONFARM NONRES RE LNS - "#)]
    pub lnrenrot: Option<f32>,

    #[schemars(description = r#"OTHER NONFARM NONRES RE LNS RATIO - "#)]
    pub lnrenrotr: Option<f32>,

    #[schemars(description = r#"OWNER-OCC NONFARM NONRES RE LNS - "#)]
    pub lnrenrow: Option<f32>,

    #[schemars(description = r#"OWNER-OCC NONFARM NONRES RE LNS - "#)]
    pub lnrenrowr: Option<f32>,

    #[schemars(description = r#"RE LNS-NON US ADDRESSEES - "#)]
    pub lnrenus: Option<f32>,

    #[schemars(description = r#"RE LNS-NON US ADDRESSEES RATIO - "#)]
    pub lnrenusr: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-FIRST LIENS-ADJUST - "#)]
    pub lnrersf1: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-FIRST LIENS-ADJUST RATIO - "#)]
    pub lnrersf1r: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-SECOND LIENS - "#)]
    pub lnrersf2: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-SECOND LIENS RATIO - "#)]
    pub lnrersf2r: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-FIRST LIENS - "#)]
    pub lnrersfm: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY-FIRST LIENS RATIO - "#)]
    pub lnrersfmr: Option<f32>,

    #[schemars(description = r#"LOAN LOSS RESERVE/N/C LOANS - "#)]
    pub lnresncr: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 1-3 YEARS - "#)]
    pub lnrs1t3: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 1-3 YEARS RATIO - "#)]
    pub lnrs1t3r: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 3 MONS OR LESS - "#)]
    pub lnrs3les: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 3 MONS OR LESS RATIO - "#)]
    pub lnrs3lesr: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 3-5 YEARS - "#)]
    pub lnrs3t5: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 3-5 YEARS RATIO - "#)]
    pub lnrs3t5r: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 3-12 MONTHS - "#)]
    pub lnrs3t12: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 3-12 MONTHS RATIO - "#)]
    pub lnrs3t12r: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 5-15 YEARS - "#)]
    pub lnrs5t15: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * 5-15 YEARS RATIO - "#)]
    pub lnrs5t15r: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * OVER 15 YEARS - "#)]
    pub lnrsov15: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY * OVER 15 YEARS RATIO - "#)]
    pub lnrsov15r: Option<f32>,

    #[schemars(description = r#"SMALL BUSINESS LNS SOLD-AMT - "#)]
    pub lnsb: Option<f32>,

    #[schemars(description = r#"SMALL BUSINESS LNS SOLD - "#)]
    pub lnsbr: Option<f32>,

    #[schemars(description = r#"PRIN BAL- LNS SERVICE FOR OTHERS - "#)]
    pub lnserv: Option<f32>,

    #[schemars(description = r#"PRIN BAL- LNS SERVICE FOR OTHERS RATIO - "#)]
    pub lnservr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LETTERS OF CREDIT - "#)]
    pub loccom: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LETTERS OF CREDIT RATIO - "#)]
    pub loccomr: Option<f32>,

    #[schemars(description = r#"FIN & PERFORM STANDBY LOC - "#)]
    pub locfpsb: Option<f32>,

    #[schemars(description = r#"FIN & PERFORM STANDBY LOC RATIO - "#)]
    pub locfpsbr: Option<f32>,

    #[schemars(description = r#"FIN & PERFORM STANDBY LOC-CONVEY - "#)]
    pub locfpsbk: Option<f32>,

    #[schemars(description = r#"FIN & PERFORM STANDBY LOC-CONVEY RATIO - "#)]
    pub locfpsbkr: Option<f32>,

    #[schemars(description = r#"FINANCIAL STANDBY LOC - "#)]
    pub locfsb: Option<f32>,

    #[schemars(description = r#"FINANCIAL STANDBY LOC RATIO - "#)]
    pub locfsbr: Option<f32>,

    #[schemars(description = r#"FINANCIAL STANDBY LOC-CONVEYED - "#)]
    pub locfsbk: Option<f32>,

    #[schemars(description = r#"FINANCIAL STANDBY LOC-CONVEYED RATIO - "#)]
    pub locfsbkr: Option<f32>,

    #[schemars(description = r#"PERFORMANCE STANDBY LOC - "#)]
    pub locpsb: Option<f32>,

    #[schemars(description = r#"PERFORMANCE STANDBY LOC RATIO - "#)]
    pub locpsbr: Option<f32>,

    #[schemars(description = r#"PERFORMANCE STANDBY LOC-CONVEYED - "#)]
    pub locpsbk: Option<f32>,

    #[schemars(description = r#"PERFORMANCE STANDBY LOC-CONVEYED RATIO - "#)]
    pub locpsbkr: Option<f32>,

    #[schemars(description = r#"ORE PROTECTED - LOSS SHARE - "#)]
    pub loregty: Option<f32>,

    #[schemars(description = r#"ORE PROTECTED - LOSS SHARE RATIO - "#)]
    pub loregtyr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LN & LS - LOSS SHARE - "#)]
    pub loth: Option<f32>,

    #[schemars(description = r#"ALL OTHER LN & LS - LOSS SHARE RATIO - "#)]
    pub lothr: Option<f32>,

    #[schemars(description = r#"RE FARMLAND LN - LOSS SH - "#)]
    pub lreag: Option<f32>,

    #[schemars(description = r#"RE FARMLAND LN - LOSS SH RATIO - "#)]
    pub lreagr: Option<f32>,

    #[schemars(description = r#"RE CONSTRUCT LN - LOSS SHARE - "#)]
    pub lrecons: Option<f32>,

    #[schemars(description = r#"RE CONSTRUCT LN - LOSS SHARE RATIO - "#)]
    pub lreconsr: Option<f32>,

    #[schemars(description = r#"RE MULTIFAMILY LN-LOSS SH - "#)]
    pub lremult: Option<f32>,

    #[schemars(description = r#"RE MULTIFAMILY LN-LOSS SH RATIO - "#)]
    pub lremultr: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES LN - LOSS SH - "#)]
    pub lrenres: Option<f32>,

    #[schemars(description = r#"RE NONFARM NONRES LN - LOSS SH RATIO - "#)]
    pub lrenresr: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY LNS - LOSS SHARE - "#)]
    pub lreres: Option<f32>,

    #[schemars(description = r#"RE 1-4 FAMILY LNS - LOSS SHARE RATIO - "#)]
    pub lreresr: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE-LNLS - "#)]
    pub lsalnls: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE-LNLS RATIO - "#)]
    pub lsalnlsr: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE -OTH ASSET - "#)]
    pub lsaoa: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE -OTH ASSET RATIO - "#)]
    pub lsaoar: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE- ORE - "#)]
    pub lsaore: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE- ORE RATIO - "#)]
    pub lsaorer: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE -DEBT SEC - "#)]
    pub lsascdbt: Option<f32>,

    #[schemars(description = r#"CARRY AMT LOSS SHARE -DEBT SEC RATIO - "#)]
    pub lsascdbtr: Option<f32>,

    #[schemars(description = r#"LEASES-FOR - "#)]
    pub lsfor: Option<f32>,

    #[schemars(description = r#"LEASES-FOR RATIO - "#)]
    pub lsforr: Option<f32>,

    #[schemars(description = r#"FIPS MSA CODE - "#)]
    pub msa: Option<f32>,

    #[schemars(description = r#"OUT PRIN BAL MORT W/ RECOURSE - "#)]
    pub msrece: Option<f32>,

    #[schemars(description = r#"OUT PRIN BAL MORT W/ RECOURSE RATIO - "#)]
    pub msrecer: Option<f32>,

    #[schemars(description = r#"1-4 FM SERVICED IN FORECLOSURE - "#)]
    pub msresfcl: Option<f32>,

    #[schemars(description = r#"1-4 FM SERVICED IN FORECLOSURE RATIO - "#)]
    pub msresfclr: Option<f32>,

    #[schemars(description = r#"OUT PRIN BAL MORT W/ NO RECOURSE - "#)]
    pub msrnrece: Option<f32>,

    #[schemars(description = r#"OUT PRIN BAL MORT W/ NO RECOURSE RATIO - "#)]
    pub msrnrecer: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-AGRICULTURAL LNS - "#)]
    pub naag: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-AGRICULTURAL LNS RATIO - "#)]
    pub naagr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-AG LNS*SMALL BKS - "#)]
    pub naagsm: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-AG LNS*SMALL BKS RATIO - "#)]
    pub naagsmr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-TOTAL ASSETS - "#)]
    pub naasset: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-AG LNS*SMALL BKS RATIO - "#)]
    pub naassetr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL AUTO LOANS - "#)]
    pub naauto: Option<f32>,

    #[schemars(description = r#"NONACCRUAL AUTO LOANS RATIO - "#)]
    pub naautor: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-C&I LOANS - "#)]
    pub naci: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-C&I LOANS RATIO - "#)]
    pub nacir: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-C&I*NON-U.S. - "#)]
    pub nacinus: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-C&I*NON-U.S. RATIO - "#)]
    pub nacinusr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-CONSUMER LOANS - "#)]
    pub nacon: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-CONSUMER LOANS RATIO - "#)]
    pub naconr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-OTHER CONSUMER - "#)]
    pub naconoth: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-OTHER CONSUMER RATIO - "#)]
    pub naconothr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-CREDIT CARD PLANS - "#)]
    pub nacrcd: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-CREDIT CARD PLANS RATIO - "#)]
    pub nacrcdr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-DEP INST LOANS - "#)]
    pub nadep: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-DEP INST LOANS RATIO - "#)]
    pub nadepr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-DEP INST*NON U.S. - "#)]
    pub nadepnus: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-DEP INST*NON U.S. RATIO - "#)]
    pub nadepnusr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-FOREIGN GOVT - "#)]
    pub nafg: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-FOREIGN GOVT RATIO - "#)]
    pub nafgr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-GTY LN&LS - "#)]
    pub nagty: Option<f32>,

    #[schemars(description = r#"NONACCRUAL -GTY LN&LS - "#)]
    pub nagtyr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL REBOOKED GNMA LOANS - "#)]
    pub nagtygnm: Option<f32>,

    #[schemars(description = r#"NONACCRUAL REBOOKED GNMA LNS - "#)]
    pub nagtygnmr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-PART GTY LN&LS - "#)]
    pub nagtypar: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-PART GTY LN&LS RATIO - "#)]
    pub nagtyparr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL AG LOANS-LOSS SH - "#)]
    pub nalag: Option<f32>,

    #[schemars(description = r#"NONACCRUAL AG LOANS-LOSS SH RATIO - "#)]
    pub nalagr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL C&I LNS-LOSS SH - "#)]
    pub nalci: Option<f32>,

    #[schemars(description = r#"NONACCRUAL C&I LNS-LOSS SH RATIO - "#)]
    pub nalcir: Option<f32>,

    #[schemars(description = r#"NONACCRUAL CONSUMER LN -LOSS SH - "#)]
    pub nalcon: Option<f32>,

    #[schemars(description = r#"NONACCRUAL CONSUMER LN -LOSS SH RATIO - "#)]
    pub nalconr: Option<f32>,

    #[schemars(description = r#"NONACCR PROTECT (GTY)-LOSS SH - "#)]
    pub nalgty: Option<f32>,

    #[schemars(description = r#"NONACCRUAL PROTECT (GTY)-LOSS SH RATIO - "#)]
    pub nalgtyr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-L&L HELD FOR SALE - "#)]
    pub nalnsale: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-L&L HELD FOR SALE RATIO - "#)]
    pub nalnsaler: Option<f32>,

    #[schemars(description = r#"NONACCRUAL OTHER LNS-LOSS SH - "#)]
    pub naloth: Option<f32>,

    #[schemars(description = r#"NONACCRUAL OTHER LNS-LOSS SH RATIO - "#)]
    pub nalothr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RE FARM-LOSS SH - "#)]
    pub nalreag: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RE FARM LOSS SH RATIO - "#)]
    pub nalreagr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL CONSTR LN -LOSS SH - "#)]
    pub nalrecon: Option<f32>,

    #[schemars(description = r#"NONACCRUAL CONSTR LN -LOSS SH RATIO - "#)]
    pub nalreconr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL MULTIFAM - LOSS SH - "#)]
    pub nalremul: Option<f32>,

    #[schemars(description = r#"NONACCRUAL MULTIFAM - LOSS SH RATIO - "#)]
    pub nalremulr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL NFNR LN - LOSS SH - "#)]
    pub nalrenrs: Option<f32>,

    #[schemars(description = r#"NONACCRUAL NFNR LN - LOSS SH RATIO - "#)]
    pub nalrenrsr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL 1-4 FM LN-LOSS SH - "#)]
    pub nalreres: Option<f32>,

    #[schemars(description = r#"NONACCRUAL 1-4 FM LN-LOSS SH RATIO - "#)]
    pub nalreresr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-LEASES - "#)]
    pub nals: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-LEASES RATIO - "#)]
    pub nalsr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL TOTAL LOANS - LOSS SH - "#)]
    pub naltot: Option<f32>,

    #[schemars(description = r#"NONACCRUAL TOTAL LOANS - LOSS SH RATIO - "#)]
    pub naltotr: Option<f32>,

    #[schemars(description = r#"INSTITUTION NAME (Search-Eligible) - This field can be used for search and filtering."#)]
    pub name: Option<String>,

    #[schemars(description = r#"INSTITUTION FULL NAME (Search-Eligible) - This field can be used for search and filtering."#)]
    pub namefull: Option<String>,

    #[schemars(description = r#"NONACCRUAL-ALL OTHER LOANS - "#)]
    pub naothln: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-ALL OTHER LOANS RATIO - "#)]
    pub naothlnr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-REAL ESTATE LOANS - "#)]
    pub nare: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-REAL ESTATE LOANS RATIO - "#)]
    pub narer: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*FARMLAND - "#)]
    pub nareag: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*FARMLAND RATIO - "#)]
    pub nareagr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL 1-4 FAM CONSTRUCT LN - "#)]
    pub narecnfm: Option<f32>,

    #[schemars(description = r#"NONACCRUAL 1-4 FAM CONSTRUCT LN RATIO - "#)]
    pub narecnfmr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL OTHER CONSTR & LAND - "#)]
    pub narecnot: Option<f32>,

    #[schemars(description = r#"NONACCRUAL OTHER CONSTR & LAND RATIO - "#)]
    pub narecnotr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*CONSTRUCTION - "#)]
    pub narecons: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*CONSTRUCTION RATIO - "#)]
    pub nareconsr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*FOREIGN - "#)]
    pub narefor: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*FOREIGN RATIO - "#)]
    pub nareforr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 FAM LINES - "#)]
    pub nareloc: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 FAM LINES RATIO - "#)]
    pub narelocr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*MULTIFAMILY - "#)]
    pub naremult: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*MULTIFAMILY RATIO - "#)]
    pub naremultr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*NONFARM NONRES - "#)]
    pub narenres: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*NONFARM NONRES RATIO - "#)]
    pub narenresr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL OTHER NONFARM NONRES - "#)]
    pub narenrot: Option<f32>,

    #[schemars(description = r#"NONACCRUAL OTHER NONFARM NONRES RATIO - "#)]
    pub narenrotr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL 0WN-OCC NONFRM NONRS - "#)]
    pub narenrow: Option<f32>,

    #[schemars(description = r#"NONACCRUAL OWN-OCC NONFRM NONRS RATIO - "#)]
    pub narenrowr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*NON-U.S. - "#)]
    pub narenus: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*NON-U.S. RATIO - "#)]
    pub narenusr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 FAMILY - "#)]
    pub nareres: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 FAMILY RATIO - "#)]
    pub nareresr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 JUNIOR LIEN - "#)]
    pub narersf2: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 JN LIEN RATIO - "#)]
    pub narersf2r: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 IST LIEN - "#)]
    pub narersfm: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-RE*1-4 IST LIEN RATIO - "#)]
    pub narersfmr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRUCT C&I LN - "#)]
    pub narsci: Option<f32>,

    #[schemars(description = r#"NONACCR RESTRUCT CONSTRUCTION - "#)]
    pub narscons: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRU LN- 1-4 FAM - "#)]
    pub narslnfm: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRU LN- 1-4 FAM RATIO - "#)]
    pub narslnfmr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRU LN EXCL 1-4 FM - "#)]
    pub narslnls: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRU LN EXCL 1-4 FM RATIO - "#)]
    pub narslnlsr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRUCT LN- TOTAL - "#)]
    pub narslnlt: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRUCT LN- TOTAL RATIO - "#)]
    pub narslnltr: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRUCT MULTIFAMILY - "#)]
    pub narsmult: Option<f32>,

    #[schemars(description = r#"NONACCR RESTRUCTURED NFNR LN - "#)]
    pub narsnres: Option<f32>,

    #[schemars(description = r#"NONACCRUAL RESTRUCT ALL OTH LN - "#)]
    pub narsoth: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-DEBT SECURITIES - "#)]
    pub nascdebt: Option<f32>,

    #[schemars(description = r#"NONACCRUAL-DEBT SECURITIES RATIO - "#)]
    pub nascdebtr: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-AGRICULTURAL LNS - "#)]
    pub ncag: Option<f32>,

    #[schemars(description = r#"N/C AUTO LOANS - "#)]
    pub ncauto: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-C&I LOANS - "#)]
    pub ncci: Option<f32>,

    #[schemars(description = r#"NC COMMERCIAL RE/COMMERCIAL RE - "#)]
    pub nccomrer: Option<f32>,

    #[schemars(description = r#"NC COMMERCIAL RE/COMMERCIAL RE - "#)]
    pub nccomre: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-CONSUMER LOANS - "#)]
    pub nccon: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-OTHER CONSUMER - "#)]
    pub ncconoth: Option<f32>,

    #[schemars(description = r#"TOTAL N/C CREDIT CARD PLANS - "#)]
    pub nccrcd: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-DEP INST LOANS - "#)]
    pub ncdep: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-FOREIGN GOVT - "#)]
    pub ncfg: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-PART GTY LN&LS - "#)]
    pub ncgtypar: Option<f32>,

    #[schemars(description = r#"N/C LNS & LS/GROSS LNS & LS - "#)]
    pub nclnlsr: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-LEASES - "#)]
    pub ncls: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-ALL OTHER LOANS - "#)]
    pub ncothln: Option<f32>,

    #[schemars(description = r#"TOTAL N/C REAL ESTATE LOANS - "#)]
    pub ncre: Option<f32>,

    #[schemars(description = r#"N/C CONST REAL ESTATE/CONST RE - "#)]
    pub ncreconr: Option<f32>,

    #[schemars(description = r#"TOTAL N/C CONST REAL ESTATE CONSTRUCTION - "#)]
    pub ncrecons: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-RE 1-4 FAMILY LINES - "#)]
    pub ncreloc: Option<f32>,

    #[schemars(description = r#"N/C HOME EQUITY/HOME EQUITY - "#)]
    pub ncrelocr: Option<f32>,

    #[schemars(description = r#"N/C MULTIFAMLY RE/MULTIFAMLY RE - "#)]
    pub ncremulr: Option<f32>,

    #[schemars(description = r#"TOTAL N/C MULTIFAMLY RE - "#)]
    pub ncremult: Option<f32>,

    #[schemars(description = r#"N/C NONFARM NONRES RE/NONRES RE - "#)]
    pub ncrenrer: Option<f32>,

    #[schemars(description = r#"TOTAL N/C NONFARM NONRES RE - "#)]
    pub ncrenres: Option<f32>,

    #[schemars(description = r#"N/C REAL ESTATE LNS/REAL ESTATE - "#)]
    pub ncrer: Option<f32>,

    #[schemars(description = r#"N/C 1-4 OTHER RE/1-4 OTHER RE - "#)]
    pub ncrereso: Option<f32>,

    #[schemars(description = r#"N/C 1-4 OTHER RE/1-4 OTHER RE - "#)]
    pub ncrereor: Option<f32>,

    #[schemars(description = r#"N/C 1-4 FAMILY RE - "#)]
    pub ncreres: Option<f32>,

    #[schemars(description = r#"N/C 1-4 FAMILY RE/1-4 FAMILY RE - "#)]
    pub ncreresr: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF FIX ASSETS - "#)]
    pub netgnast: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF FIX ASSETS RATIO - "#)]
    pub netgnastr: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF FIX ASSETS QUARTERLY - "#)]
    pub ntglfxaq: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF FIX ASSETS QUARTERLY RATIO - "#)]
    pub ntglfxaqr: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF LOANS - "#)]
    pub netgnsln: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF LOANS RATIO - "#)]
    pub netgnslnr: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF LOANS QUARTERLY - "#)]
    pub ntgllnq: Option<f32>,

    #[schemars(description = r#"NET G/L ON SALES OF LOANS QUARTERLY RATIO - "#)]
    pub ntgllnqr: Option<f32>,

    #[schemars(description = r#"NET G/L ON OTHER RE OWNED - "#)]
    pub netgnsre: Option<f32>,

    #[schemars(description = r#"NET G/L ON OTHER RE OWNED RATIO - "#)]
    pub netgnsrer: Option<f32>,

    #[schemars(description = r#"NET G/L ON OTHER RE OWNED QUARTERLY - "#)]
    pub ntglreq: Option<f32>,

    #[schemars(description = r#"NET G/L ON OTHER RE OWNED QUARTERLY RATIO - "#)]
    pub ntglreqr: Option<f32>,

    #[schemars(description = r#"NET INCOME- BANK- ANN - "#)]
    pub netinca: Option<f32>,

    #[schemars(description = r#"NET INTEREST MARGIN - "#)]
    pub nimy: Option<f32>,

    #[schemars(description = r#"NET INTEREST MARGIN QUARTERLY - "#)]
    pub nimyq: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ - "#)]
    pub noij: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ RATIO - "#)]
    pub noijr: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ/ASSETS - "#)]
    pub noijy: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ/ASSETS QUARTERLY - "#)]
    pub noijyq: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ ANNUALLY - "#)]
    pub noija: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ QUARTERLY - "#)]
    pub noijq: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ QUARTERLY - "#)]
    pub noijqa: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-ADJ QUARTERLY RATIO - "#)]
    pub noijqr: Option<f32>,

    #[schemars(description = r#"NONINTEREST INC/AVERAGE ASSETS - "#)]
    pub noniiay: Option<f32>,

    #[schemars(description = r#"NONINTEREST INC/AVERAGE ASSETS QUARTERLY - "#)]
    pub noniiayq: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST INCOME ANNUALLY - "#)]
    pub noniia: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST INCOME-QTR - "#)]
    pub noniiq: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST INCOME-QTR - "#)]
    pub noniiqa: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST INCOME-QTR RATIO - "#)]
    pub noniiqr: Option<f32>,

    #[schemars(description = r#"NONINTEREST EXP/AVERAGE ASSETS - "#)]
    pub nonixay: Option<f32>,

    #[schemars(description = r#"NONINTEREST EXP/AVERAGE ASSETS QUARTERLY - "#)]
    pub nonixayq: Option<f32>,

    #[schemars(description = r#"TOTAL NONINTEREST EXPENSE ANNUALLY - "#)]
    pub nonixa: Option<f32>,

    #[schemars(description = r#"NONPERF ASSETS/TOTAL ASSETS - "#)]
    pub nperf: Option<f32>,

    #[schemars(description = r#"NONPERF ASSETS/TOTAL ASSETS - "#)]
    pub nperfv: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LN NET CHARGE-OFFS - "#)]
    pub ntag: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LN NET CHARGE-OFFS RATIO - "#)]
    pub ntagr: Option<f32>,

    #[schemars(description = r#"AGRICULTURAL LN NET-CHG-ANN - "#)]
    pub ntaga: Option<f32>,

    #[schemars(description = r#"AG LOAN NET CHARGE-OFFS-QTR - "#)]
    pub ntagq: Option<f32>,

    #[schemars(description = r#"AG LOAN NET CHARGE-OFFS-QTR RATIO - "#)]
    pub ntagqr: Option<f32>,

    #[schemars(description = r#"AG LN NET CHARGE-OFFS*SMALL BKS - "#)]
    pub ntagsm: Option<f32>,

    #[schemars(description = r#"AG LN NET CHARGE-OFFS*SMALL BKS RATIO - "#)]
    pub ntagsmr: Option<f32>,

    #[schemars(description = r#"AG LN NET CHARGE-OFFS*SMALL BKS QUARTERLY - "#)]
    pub ntagsmq: Option<f32>,

    #[schemars(description = r#"AG LN NET CHARGE-OFFS*SMALL BKS QUARTERLY RATIO - "#)]
    pub ntagsmqr: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - NET CHARGE-OFFS - "#)]
    pub ntauto: Option<f32>,

    #[schemars(description = r#"AUTO LOANS - NET CHARGE-OFFS RATIO - "#)]
    pub ntautor: Option<f32>,

    #[schemars(description = r#"AUTO LNS - NET CHG-OFFS - ANN - "#)]
    pub ntautoa: Option<f32>,

    #[schemars(description = r#"AUTO LNS - NET CHG-OFFS - QTR - "#)]
    pub ntautoq: Option<f32>,

    #[schemars(description = r#"AUTO LNS - NET CHG-OFFS - QTR RATIO - "#)]
    pub ntautolnqr: Option<f32>,

    #[schemars(description = r#"AUTO LN-CHG-OFF- QTR/AUTO LN - "#)]
    pub ntautoqr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN NET CHARGE-OFFS - "#)]
    pub ntci: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN NET CHARGE-OFFS RATIO - "#)]
    pub ntcir: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN NET-CHG-ANN - "#)]
    pub ntcia: Option<f32>,

    #[schemars(description = r#"NON-U.S.COMMERCIAL LN NET CHG-OF - "#)]
    pub ntcinus: Option<f32>,

    #[schemars(description = r#"NON-U.S.COMMERCIAL LN NET CHG-OF RATIO - "#)]
    pub ntcinusr: Option<f32>,

    #[schemars(description = r#"NON-U.S.COMMERCIAL LN NET CHG-OF QUARTERLY - "#)]
    pub ntcinusq: Option<f32>,

    #[schemars(description = r#"NON-U.S.COMMERCIAL LN NET CHG-OF QUARTERLY RATIO - "#)]
    pub ntcinusqr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN NET-CHG-QTR - "#)]
    pub ntciq: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN NET-CHG-QTR RATIO - "#)]
    pub ntciqr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE CHG-OFF/COMM RE LN - "#)]
    pub ntcomrer: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE CHG-OFF/COMM RE LN QUARTERLY - "#)]
    pub ntcomreq: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE LN CHG-ANN - "#)]
    pub ntcomrea: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN NET CHARGE-OFFS - "#)]
    pub ntcon: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN NET CHARGE-OFFS RATIO - "#)]
    pub ntconr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN NET-CHG-ANN - "#)]
    pub ntcona: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOAN NET-CHG-ANN - "#)]
    pub ntconota: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LN NET CHARGE-OFF - "#)]
    pub ntconoth: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LN NET CHARGE-OFF RATIO - "#)]
    pub ntconothr: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LN NET-CHG-QTR - "#)]
    pub ntconotq: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LN NET-CHG-QTR RATIO - "#)]
    pub ntconotqr: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN NET-CHG-QTR - "#)]
    pub ntconq: Option<f32>,

    #[schemars(description = r#"CONSUMER LOAN NET-CHG-QTR RATIO - "#)]
    pub ntconqr: Option<f32>,

    #[schemars(description = r#"OTH.CONSUMER CHGOFF-QTR/OTH.CONS - "#)]
    pub ntcontqr: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN NET CHARGE-OFFS - "#)]
    pub ntcrcd: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN NET CHARGE-OFFS RATIO - "#)]
    pub ntcrcdr: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LOAN NET-CHG-ANN - "#)]
    pub ntcrcda: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LN NET-CHG-QTR - "#)]
    pub ntcrcdq: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LN NET-CHG-QTR RATIO - "#)]
    pub ntcrcdqr: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN NET CHG-OFF - "#)]
    pub ntdep: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN NET CHG-OFF RATIO - "#)]
    pub ntdepr: Option<f32>,

    #[schemars(description = r#"FOREIGN DEP INST LN NET CHG-OFFS - "#)]
    pub ntdepnus: Option<f32>,

    #[schemars(description = r#"FOREIGN DEP INST LN NET CHG-OFFS RATIO - "#)]
    pub ntdepnusr: Option<f32>,

    #[schemars(description = r#"FOREIGN DEP INST LN NET CHG-OFFS QUARTERLY - "#)]
    pub ntdepnuq: Option<f32>,

    #[schemars(description = r#"FOREIGN DEP INST LN NET CHG-OFFS QUARTERLY RATIO - "#)]
    pub ntdepnuqr: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN NET-CHG-QTR - "#)]
    pub ntdepq: Option<f32>,

    #[schemars(description = r#"DEPOSITORY INST LOAN NET-CHG-QTR RATIO - "#)]
    pub ntdepqr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVT LN NET CHG-OFFS - "#)]
    pub ntforgv: Option<f32>,

    #[schemars(description = r#"FOREIGN GOVT LN NET CHG-OFFS RATIO - "#)]
    pub ntforgvr: Option<f32>,

    #[schemars(description = r#"FOREIGN GOV LN NET-CHG-QTR - "#)]
    pub ntforgvq: Option<f32>,

    #[schemars(description = r#"FOREIGN GOV LN NET-CHG-QTR RATIO - "#)]
    pub ntforgvqr: Option<f32>,

    #[schemars(description = r#"NET INCOME-BK-HIGHER-PP - "#)]
    pub ntinchpp: Option<f32>,

    #[schemars(description = r#"NET INCOME-BANK- LOSERS - "#)]
    pub ntincl: Option<f32>,

    #[schemars(description = r#"NET INCOME-BK-LOSER-QTR - "#)]
    pub ntinclq: Option<f32>,

    #[schemars(description = r#"TOTAL LN&LS NET-CHG-ANN - "#)]
    pub ntlnlsa: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub ntinqhpp: Option<f32>,

    #[schemars(description = r#"NET CHARGE-OFFS/LOANS & LEASES - "#)]
    pub ntlnlsr: Option<f32>,

    #[schemars(description = r#"NET CHARGE-OFFS/LOANS & LEASES QUARTERLY - "#)]
    pub ntlnlsqr: Option<f32>,

    #[schemars(description = r#"LEASE NET CHARGE-OFFS - "#)]
    pub ntls: Option<f32>,

    #[schemars(description = r#"LEASE NET CHARGE-OFFS RATIO - "#)]
    pub ntlsr: Option<f32>,

    #[schemars(description = r#"LEASE NET CHARGE-OFFS-QTR - "#)]
    pub ntlsq: Option<f32>,

    #[schemars(description = r#"LEASE NET CHARGE-OFFS-QTR RATIO - "#)]
    pub ntlsqr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN NET CHARGE-OFFS - "#)]
    pub ntother: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOAN NET CHARGE-OFFS RATIO - "#)]
    pub ntotherr: Option<f32>,

    #[schemars(description = r#"ALL OTHER LN NET-CHG-QTR - "#)]
    pub ntothq: Option<f32>,

    #[schemars(description = r#"ALL OTHER LN NET-CHG-QTRS RATIO - "#)]
    pub ntothqr: Option<f32>,

    #[schemars(description = r#"AMT TIME DEP OF $100,000 OR LESS - "#)]
    pub ntrcdsm: Option<f32>,

    #[schemars(description = r#"AMT TIME DEP OF $100,000 OR LESS RATIO - "#)]
    pub ntrcdsmr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTN-COM BKS & OTH U.S. - "#)]
    pub ntrcomot: Option<f32>,

    #[schemars(description = r#"NONTRANSACTN-COM BKS & OTH U.S RATIO - "#)]
    pub ntrcomotr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS - "#)]
    pub ntre: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub ntremuqa: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub ntrecoqa: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS RATIO - "#)]
    pub ntrelnr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY - "#)]
    pub ntreq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY - "#)]
    pub ntreqa: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub ntrerq: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN NET CHARGE-OFFS - "#)]
    pub ntreag: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN NET CHARGE-OFFS RATIO - "#)]
    pub ntreagr: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN NET-CHG-QTR - "#)]
    pub ntreagq: Option<f32>,

    #[schemars(description = r#"RE LN NET-CHG-ANN - "#)]
    pub ntrea: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN NET-CHG-QTR RATIO - "#)]
    pub ntreagqr: Option<f32>,

    #[schemars(description = r#"1-4 FAM CONST LN NET-OFF - "#)]
    pub ntrecnfm: Option<f32>,

    #[schemars(description = r#"OTHER CONSTRUCT NET CHG-OFF - "#)]
    pub ntrecnot: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN NET-CHG-QTR - "#)]
    pub ntreconq: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN NET-CHG-QTR RATIO - "#)]
    pub ntreconqr: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN NET CHG-OFFS - "#)]
    pub ntrecons: Option<f32>,

    #[schemars(description = r#"CONST RE LOANS NET-CHG-ANN - "#)]
    pub ntrecosa: Option<f32>,

    #[schemars(description = r#"CONSTRUCTION RE LN NET CHG-OFFS RATIO - "#)]
    pub ntreconsr: Option<f32>,

    #[schemars(description = r#"CONST RE CHG-OFF/CONST RE LOANS - "#)]
    pub ntrecosr: Option<f32>,

    #[schemars(description = r#"CONST RE CHG-OFF/CONST RE LOANS QUARTERLY - "#)]
    pub ntrecoqr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN NET CHG-OFF-FOR - "#)]
    pub ntrefor: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN NET CHG-OFF-FOR RATIO - "#)]
    pub ntreforr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN NET CHG-OFF-FOR QUARTERLY - "#)]
    pub ntreforq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LN NET CHG-OFF-FOR QUARTERLY RATIO - "#)]
    pub ntreforqr: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN NET CHG-OFF - "#)]
    pub ntreloc: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN NET CHG-OFF RATIO - "#)]
    pub ntreloclnr: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN NET CHG-OFF QUARTERLY - "#)]
    pub ntrelocq: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN NET CHG-OFF ANNUALLY - "#)]
    pub ntreloca: Option<f32>,

    #[schemars(description = r#"LINE OF CREDIT RE LN NET CHG-OFF QUARTERLY RATIO - "#)]
    pub ntrelocqr: Option<f32>,

    #[schemars(description = r#"HOME EQUITY CHG-OFF/HOME EQ LNS QUARTERLY RATIO - "#)]
    pub ntrelocrq: Option<f32>,

    #[schemars(description = r#"HOME EQUITY CHG-OFF/HOME EQ LNS - "#)]
    pub ntrelocr: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RE LN NET-CHG-QTR - "#)]
    pub ntremulq: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RES RE LN NET-CHG-ANN - "#)]
    pub ntremula: Option<f32>,

    #[schemars(description = r#"MULTIFAMILY RE LN NET-CHG-QTR RATIO - "#)]
    pub ntremulqr: Option<f32>,

    #[schemars(description = r#"MULTIFAM RE CHG-OFF/MULTI RE LN - "#)]
    pub ntremulr: Option<f32>,

    #[schemars(description = r#"MULTIFAM RE CHG-OFF/MULTI RE LN QUARTERLY - "#)]
    pub ntremuqr: Option<f32>,

    #[schemars(description = r#"MULTIFAMLY RES RE LN NET CHG-OFF - "#)]
    pub ntremult: Option<f32>,

    #[schemars(description = r#"MULTIFAMLY RES RE LN NET CHG-OFF RATIO - "#)]
    pub ntremultr: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN NET CHG-OFF - "#)]
    pub ntrenres: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN NET CHG-OFF RATIO - "#)]
    pub ntrenresr: Option<f32>,

    #[schemars(description = r#"OTHER NONFARM NONRS NET CHG-OFF - "#)]
    pub ntrenrot: Option<f32>,

    #[schemars(description = r#"OWN OCC NONFRM NONRS NET CHG-OFF - "#)]
    pub ntrenrow: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN NET-CHG-ANN - "#)]
    pub ntrenrsa: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN NET-CHG-QTR - "#)]
    pub ntrenrsq: Option<f32>,

    #[schemars(description = r#"NONFARM NONRES RE LN NET-CHG-QTR RATIO - "#)]
    pub ntrenrsqr: Option<f32>,

    #[schemars(description = r#"NONRES CHG-OFF/NONRES LOANS - "#)]
    pub ntrenrsr: Option<f32>,

    #[schemars(description = r#"NONRES CHG-OFF/NONRES LOANS QUARTERLY - "#)]
    pub ntrenrqr: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN NET CHARGE-OFFS - "#)]
    pub ntrenus: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN NET CHARGE-OFFS RATIO - "#)]
    pub ntrenusr: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN NET CHARGE-OFFS QUARTERLY - "#)]
    pub ntrenusq: Option<f32>,

    #[schemars(description = r#"OTHER 1-4 FAM RE OTHER LN NET-CHG-ANN - "#)]
    pub ntreotha: Option<f32>,

    #[schemars(description = r#"NON-U.S. RE LN NET CHARGE-OFFS QUARTERLY RATIO - "#)]
    pub ntrenusqr: Option<f32>,

    #[schemars(description = r#"OTHER 1-4 FAM RE CHG-OFF/OTH 1-4 - "#)]
    pub ntreothr: Option<f32>,

    #[schemars(description = r#"OTHER 1-4 FAM RE CHG-OFF/OTH 1-4 QUARTERLY RATIO - "#)]
    pub ntreothrqr: Option<f32>,

    #[schemars(description = r#"OTHER 1-4 FAM RE CHG-OFF/OTH 1-4 QUARTERLY - "#)]
    pub ntreotqa: Option<f32>,

    #[schemars(description = r#"RE CHARGE-OFF/RE LOANS - "#)]
    pub ntrer: Option<f32>,

    #[schemars(description = r#"RE CHARGE-OFF/RE LOANS QUARTERLY - "#)]
    pub ntreqr: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY NET CHG-OFFS - "#)]
    pub ntreres: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY NET CHG-OFFS RATIO - "#)]
    pub ntrereslnr: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY NET-CHG-QTR - "#)]
    pub ntreresq: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY NET-CHG-ANN - "#)]
    pub ntreresa: Option<f32>,

    #[schemars(description = r#"RE LOANS 1-4 FAMILY NET-CHG-QTR RATIO - "#)]
    pub ntreresqr: Option<f32>,

    #[schemars(description = r#"1-4 FAM RE CHG-OFF/1-4 FAM LOANS - "#)]
    pub ntreresr: Option<f32>,

    #[schemars(description = r#"1-4 FAM RE CHG-OFF/1-4 FAM LOANS QUARTERLY RATIO - "#)]
    pub ntreresrq: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-NET C/OFF - "#)]
    pub ntrersf2: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-NET C/OFF RATIO - "#)]
    pub ntrersf2r: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-NET C/OFF QUARTERLY - "#)]
    pub ntrers2q: Option<f32>,

    #[schemars(description = r#"RE LN 1-4 FAM JR LIEN-NET C/OFF QUARTERLY RATIO - "#)]
    pub ntrers2qr: Option<f32>,

    #[schemars(description = r#"RE LN 1-4FAM IST LIEN-NET C/OFF - "#)]
    pub ntrersfm: Option<f32>,

    #[schemars(description = r#"RE LN 1-4FAM IST LIEN-NET C/OFF RATIO - "#)]
    pub ntrersfmr: Option<f32>,

    #[schemars(description = r#"RE LN 1-4FAM IST LIEN-NET C/OFF QUARTERLY - "#)]
    pub ntrersfq: Option<f32>,

    #[schemars(description = r#"RE LN 1-4FAM IST LIEN-NET C/OFF QUARTERLY RATIO - "#)]
    pub ntrersfqr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES - "#)]
    pub ntreoffdom: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES RATIO - "#)]
    pub ntreoffdomr: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES QUARTERLY - "#)]
    pub ntreoffdomq: Option<f32>,

    #[schemars(description = r#"REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES QUARTERLY RATIO - "#)]
    pub ntreoffdomqr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-FOR COUNTRY - "#)]
    pub ntrfc: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-FOR CNTRY & GOVT - "#)]
    pub ntrfcfg: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-FOR CNTRY & GOVT RATIO - "#)]
    pub ntrfcfgr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION-FOR GOVERNMENT - "#)]
    pub ntrfg: Option<f32>,

    #[schemars(description = r#"SAVINGS DEP-MMDA - "#)]
    pub ntrsmmda: Option<f32>,

    #[schemars(description = r#"SAVINGS DEP-MMDA RATIO - "#)]
    pub ntrsmmdar: Option<f32>,

    #[schemars(description = r#"SAVINGS DEP-OTHER - "#)]
    pub ntrsoth: Option<f32>,

    #[schemars(description = r#"SAVINGS DEP-OTHER RATIO - "#)]
    pub ntrsothr: Option<f32>,

    #[schemars(description = r#"INCOME EARNED NOT COLLECTED - "#)]
    pub oaienc: Option<f32>,

    #[schemars(description = r#"LIFE INS ASSETS - GENERAL ACC - "#)]
    pub oalifgen: Option<f32>,

    #[schemars(description = r#"LIFE INS ASSETS - GENERAL ACC RATIO - "#)]
    pub oalifgenr: Option<f32>,

    #[schemars(description = r#"LIFE INS ASSETS - HYBRID ACC - "#)]
    pub oalifhyb: Option<f32>,

    #[schemars(description = r#"LIFE INS ASSETS - HYBRID ACC RATIO - "#)]
    pub oalifhybr: Option<f32>,

    #[schemars(description = r#"LIFE INSURANCE ASSETS - "#)]
    pub oalifins: Option<f32>,

    #[schemars(description = r#"LIFE INSURANCE RATIO - "#)]
    pub oalifinsr: Option<f32>,

    #[schemars(description = r#"LIFE INS ASSETS - SEPARATE ACC - "#)]
    pub oalifsep: Option<f32>,

    #[schemars(description = r#"LIFE INS ASSETS - SEPARATE ACC RATIO - "#)]
    pub oalifsepr: Option<f32>,

    #[schemars(description = r#"OFF-BALANCE SHEET DERIVATIVES - "#)]
    pub obsdir: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-FARMLAND - "#)]
    pub oreag: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-FARMLAND RATIO - "#)]
    pub oreagr: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-CONST - "#)]
    pub orecons: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-CONST RATIO - "#)]
    pub oreconsr: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-GNMA LOANS - "#)]
    pub oregnma: Option<f32>,

    #[schemars(description = r#"DIRECT & INDIRECT INVEST IN ORE - "#)]
    pub oreinv: Option<f32>,

    #[schemars(description = r#"DIRECT & INDIRECT INVEST IN ORE RATIO - "#)]
    pub oreinvr: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-MULTI - "#)]
    pub oremult: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-MULTI RATIO - "#)]
    pub oremultr: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-NONFARM - "#)]
    pub orenres: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-NONFARM RATIO - "#)]
    pub orenresr: Option<f32>,

    #[schemars(description = r#"OTHER REAL ESTATE OWNED - "#)]
    pub oreoth: Option<f32>,

    #[schemars(description = r#"OTHER REAL ESTATE OWNED RATIO - "#)]
    pub oreothr: Option<f32>,

    #[schemars(description = r#"OTHER REAL ESTATE OWNED-FOR - "#)]
    pub oreothf: Option<f32>,

    #[schemars(description = r#"OTHER REAL ESTATE OWNED-FOR RATIO - "#)]
    pub oreothfr: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED-1-4 FAMILY - "#)]
    pub oreres: Option<f32>,

    #[schemars(description = r#"ALL OTHER RE OWNED 1-4 FAMILIY RATIO - "#)]
    pub oreresr: Option<f32>,

    #[schemars(description = r#"OTHER BORROWED MONEY-FOR - "#)]
    pub othborf: Option<f32>,

    #[schemars(description = r#"OTHER-FUTURES & FORWARD CONTRACT - "#)]
    pub othffc: Option<f32>,

    #[schemars(description = r#"OTHER-FUTURES & FORWARD CONTRACT RATIO - "#)]
    pub othffcr: Option<f32>,

    #[schemars(description = r#"OTHER-NOTIONAL VALUE SWAPS - "#)]
    pub othnvs: Option<f32>,

    #[schemars(description = r#"ALL OTH OFF-BALANCE SHEET LIAB - "#)]
    pub othoffbs: Option<f32>,

    #[schemars(description = r#"ALL OTH OFF-BALANCE SHEET LIAB RATIO - "#)]
    pub othoffbsr: Option<f32>,

    #[schemars(description = r#"OTHER-PURCHASED OPTION CONTRACTS - "#)]
    pub othpoc: Option<f32>,

    #[schemars(description = r#"OTHER-WRITTEN OPTION CONTRACTS - "#)]
    pub othwoc: Option<f32>,

    #[schemars(description = r#"OTS REGION NAME - "#)]
    pub otsregnm: Option<String>,

    #[schemars(description = r#"REC OWN INTEREST SEC - CI - "#)]
    pub owncrci: Option<f32>,

    #[schemars(description = r#"REC OWN INTEREST SEC - CRCD - "#)]
    pub owncrcrd: Option<f32>,

    #[schemars(description = r#"REC OWN INTEREST SEC - HEL - "#)]
    pub owncrhel: Option<f32>,

    #[schemars(description = r#"C/O OWN INTEREST SEC - CI - "#)]
    pub owndrci: Option<f32>,

    #[schemars(description = r#"C/O OWN INTEREST SEC - CRCD - "#)]
    pub owndrcrd: Option<f32>,

    #[schemars(description = r#"C/O OWN INTEREST SEC - HEL - "#)]
    pub owndrhel: Option<f32>,

    #[schemars(description = r#"LN SECURE HELD IN SEC - CI - "#)]
    pub ownlnci: Option<f32>,

    #[schemars(description = r#"LN SECURE HELD IN SEC - CRCD - "#)]
    pub ownlncrd: Option<f32>,

    #[schemars(description = r#"LN SECURE HELD IN SEC - HEL - "#)]
    pub ownlnhel: Option<f32>,

    #[schemars(description = r#"PD 30-89 OWN INTEREST SEC - CI - "#)]
    pub ownp3ci: Option<f32>,

    #[schemars(description = r#"PD 30-89 OWN INTEREST SEC - CRCD - "#)]
    pub ownp3crd: Option<f32>,

    #[schemars(description = r#"PD30-89 OWN INTEREST SEC - HEL - "#)]
    pub ownp3hel: Option<f32>,

    #[schemars(description = r#"PD 90 + OWN INTEREST SEC - CI - "#)]
    pub ownp9ci: Option<f32>,

    #[schemars(description = r#"PD 90 + OWN INTEREST SEC - CRCD - "#)]
    pub ownp9crd: Option<f32>,

    #[schemars(description = r#"PD 90 + OWN INTEREST SEC - HEL - "#)]
    pub ownp9hel: Option<f32>,

    #[schemars(description = r#"SEC. SECURE HELD IN RC-B - CI - "#)]
    pub ownscci: Option<f32>,

    #[schemars(description = r#"SEC. SECURE HELD IN RC-B - CRCD - "#)]
    pub ownsccrd: Option<f32>,

    #[schemars(description = r#"SEC. SECURE HELD IN RC-B - HEL - "#)]
    pub ownschel: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-AGRICULTURAL LNS - "#)]
    pub p3ag: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-AGRICULTURAL LNS RATIO - "#)]
    pub p3agr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-AG LNS*SMALL BKS - "#)]
    pub p3agsm: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-AG LNS*SMALL BKS RATIO - "#)]
    pub p3agsmr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-TOTAL ASSETS - "#)]
    pub p3asset: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D TOTAL ASSETS RATIO - "#)]
    pub p3assetr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D AUTO LOANS - "#)]
    pub p3auto: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D AUTO LOANS RATIO - "#)]
    pub p3autor: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-C&I LOANS - "#)]
    pub p3ci: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-C&I LOANS RATIO - "#)]
    pub p3cir: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-C&I*NON-U.S. - "#)]
    pub p3cinus: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-C&I*NON-U.S. RATIO - "#)]
    pub p3cinusr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-CONSUMER LOANS - "#)]
    pub p3con: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-CONSUMER LOANS RATIO - "#)]
    pub p3conr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-OTHER CONSUMER - "#)]
    pub p3conoth: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-OTHER CONSUMER RATIO - "#)]
    pub p3conothr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-CREDIT CARD PLANS - "#)]
    pub p3crcd: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-CREDIT CARD PLANS RATIO - "#)]
    pub p3crcdr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-DEP INST LOANS - "#)]
    pub p3dep: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-DEP INST LOANS - "#)]
    pub p3depr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-DEP INST*NON U.S. - "#)]
    pub p3depnus: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-DEP INST*NON U.S. - "#)]
    pub p3depnusr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-FOREIGN GOVT - "#)]
    pub p3fg: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-FOREIGN GOVT RATIO - "#)]
    pub p3fgr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-GTY LN&LS - "#)]
    pub p3gty: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-GTY LN&LS - "#)]
    pub p3gtyr: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D-REBOOKED GNMA LNS - "#)]
    pub p3gtygnm: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D-REBOOKED GNMA LNS - "#)]
    pub p3gtygnmr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-PART GTY LN&LS - "#)]
    pub p3gtypar: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-PART GTY LN&LS RATIO - "#)]
    pub p3gtyparr: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D AG LOANS-LOSS SH - "#)]
    pub p3lag: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D AG LOANS-LOSS SH RATIO - "#)]
    pub p3lagr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D C&I LNS-LOSS SH - "#)]
    pub p3lci: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D C&I LNS-LOSS SH RATIO - "#)]
    pub p3lcir: Option<f32>,

    #[schemars(description = r#"30-89 D P/D CONSUMER -LOSS SH - "#)]
    pub p3lcon: Option<f32>,

    #[schemars(description = r#"30-89 D P/D CONSUMER -LOSS SH RATIO - "#)]
    pub p3lconr: Option<f32>,

    #[schemars(description = r#"30-89 P/D PROTECT (GTY)-LOSS SH - "#)]
    pub p3lgty: Option<f32>,

    #[schemars(description = r#"30-89 P/D PROTECT (GTY)-LOSS SH RATIO - "#)]
    pub p3lgtyr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-L&L HELD FOR SALE - "#)]
    pub p3lnsale: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-L&L HELD FOR SALE RATIO - "#)]
    pub p3lnsaler: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D OTH LNS-LOSS SH - "#)]
    pub p3loth: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D OTH LNS-LOSS SH RATIO - "#)]
    pub p3lothr: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RE FARM-LOSS SH - "#)]
    pub p3lreag: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RE FARM-LOSS SH RATIO - "#)]
    pub p3lreagr: Option<f32>,

    #[schemars(description = r#"30-89 P/D CONSTRUCTION -LOSS SH - "#)]
    pub p3lrecon: Option<f32>,

    #[schemars(description = r#"30-89 P/D CONSTRUCTION -LOSS SH RATIO - "#)]
    pub p3lreconr: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D MULTIFAM -LOSS SH - "#)]
    pub p3lremul: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D MULTIFAM -LOSS SH RATIO - "#)]
    pub p3lremulr: Option<f32>,

    #[schemars(description = r#"30-89 P/D NONFRM NONRS -LOSS SH - "#)]
    pub p3lrenrs: Option<f32>,

    #[schemars(description = r#"30-89 P/D NONFRM NONRS -LOSS SH RATIO - "#)]
    pub p3lrenrsr: Option<f32>,

    #[schemars(description = r#"30-89 D P/D 1-4 FAMILY -LOSS SH - "#)]
    pub p3lreres: Option<f32>,

    #[schemars(description = r#"30-89 P/D 1-4 FAMILY -LOSS SH RATIO - "#)]
    pub p3lreresr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-LEASES - "#)]
    pub p3ls: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-LEASES RATIO - "#)]
    pub p3lsr: Option<f32>,

    #[schemars(description = r#"30-89 D P/D TOTAL LOANS-LOSS SH - "#)]
    pub p3ltot: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-TOTAL LOANS-LOSS SH RATIO - "#)]
    pub p3ltotr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-ALL OTHER LOANS - "#)]
    pub p3othln: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-ALL OTHER LOANS RATIO - "#)]
    pub p3othlnr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-REAL ESTATE LOANS - "#)]
    pub p3re: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-REAL ESTATE LOANS RATIO - "#)]
    pub p3rer: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*FARMLAND - "#)]
    pub p3reag: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*FARMLAND - "#)]
    pub p3reagr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D 1-4 FAM CONSTR LN - "#)]
    pub p3recnfm: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D 1-4 FAM CONSTR LN - "#)]
    pub p3recnfmr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D OTH CONSTR & LAND - "#)]
    pub p3recnot: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D OTH CONSTR & LAND - "#)]
    pub p3recnotr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*CONSTRUCTION - "#)]
    pub p3recons: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*CONSTRUCTION - "#)]
    pub p3reconsr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*FOREIGN - "#)]
    pub p3refor: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*FOREIGN RATIO - "#)]
    pub p3reforr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 FAM LINES - "#)]
    pub p3reloc: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 FAM LINES RATIO - "#)]
    pub p3relocr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*MULTIFAMILY - "#)]
    pub p3remult: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*MULTIFAMILY - "#)]
    pub p3remultr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*NONFARM NONRES - "#)]
    pub p3renres: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*NONFARM NONRES - "#)]
    pub p3renresr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D OTH NONFRM NONRES - "#)]
    pub p3renrot: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D OTH NONFRM NONRES - "#)]
    pub p3renrotr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D 0WN-OCC NONF NONRS - "#)]
    pub p3renrow: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D OWN-OCC NONF NONRS RATIO - "#)]
    pub p3renrowr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*NON-U.S. - "#)]
    pub p3renus: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*NON-U.S. - "#)]
    pub p3renusr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 FAMILY - "#)]
    pub p3reres: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 FAMILY - "#)]
    pub p3reresr: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 JN LIEN - "#)]
    pub p3rersf2: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 JN LIEN RATIO - "#)]
    pub p3rersf2r: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 IST LIEN - "#)]
    pub p3rersfm: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-RE*1-4 IST LIEN RATIO - "#)]
    pub p3rersfmr: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RESTRUCT C&I LN - "#)]
    pub p3rsci: Option<f32>,

    #[schemars(description = r#"30-89 P/D RESTRUCT CONSTRUCTION - "#)]
    pub p3rscons: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RESTR LN- 1-4 FAM - "#)]
    pub p3rslnfm: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RESTR LN- 1-4 FAM RATIO - "#)]
    pub p3rslnfmr: Option<f32>,

    #[schemars(description = r#"30-89 D P/D RESTR LN EXCL1-4 FM - "#)]
    pub p3rslnls: Option<f32>,

    #[schemars(description = r#"30-89 D P/D RESTR LN EXCL1-4 FM RATIO - "#)]
    pub p3rslnlsr: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RESTR LN- TOTAL - "#)]
    pub p3rslnlt: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RESTR LN- TOTAL RATIO - "#)]
    pub p3rslnltr: Option<f32>,

    #[schemars(description = r#"30-89 D P/D RESTRUCT MULTIFAM - "#)]
    pub p3rsmult: Option<f32>,

    #[schemars(description = r#"30-89 DAY P/D RESTRUCT NFNR LN - "#)]
    pub p3rsnres: Option<f32>,

    #[schemars(description = r#"30-89 D P/D RESTRUCT ALL OTH LN - "#)]
    pub p3rsoth: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-DEBT SECURITIES - "#)]
    pub p3scdebt: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-DEBT SECURITIES RATIO - "#)]
    pub p3scdebtr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-AGRICULTURAL LNS - "#)]
    pub p9ag: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-AGRICULTURAL LNS RATIO - "#)]
    pub p9agr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-AG LNS*SMALL BKS - "#)]
    pub p9agsm: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-AG LNS*SMALL BKS RATIO - "#)]
    pub p9agsmr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-TOTAL ASSETS - "#)]
    pub p9asset: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-TOTAL ASSETS RATIO - "#)]
    pub p9assetr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D AUTO LOANS - "#)]
    pub p9auto: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D AUTO LOANS RATIO - "#)]
    pub p9autor: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-C&I LOANS - "#)]
    pub p9ci: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-C&I LOANS RATIO - "#)]
    pub p9cir: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-C&I*NON-U.S. - "#)]
    pub p9cinus: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-C&I*NON-U.S. RATIO - "#)]
    pub p9cinusr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-CONSUMER LOANS - "#)]
    pub p9con: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-CONSUMER LOANS RATIO - "#)]
    pub p9conr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-OTHER CONSUMER - "#)]
    pub p9conoth: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-OTHER CONSUMER RATIO - "#)]
    pub p9conothr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-CREDIT CARD PLANS - "#)]
    pub p9crcd: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-CREDIT CARD PLANS RATIO - "#)]
    pub p9crcdr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-DEP INST LOANS - "#)]
    pub p9dep: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-DEP INST LOANS RATIO - "#)]
    pub p9depr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-DEP INST*NON U.S. - "#)]
    pub p9depnus: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-DEP INST*NON U.S. RATIO - "#)]
    pub p9depnusr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-FOREIGN GOVT - "#)]
    pub p9fg: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-FOREIGN GOVT RATIO - "#)]
    pub p9fgr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-GTY LN&LS - "#)]
    pub p9gty: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-GTY LN&LS - "#)]
    pub p9gtyr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-REBOOKED GNMA LNS - "#)]
    pub p9gtygnm: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D-REBOOKED GNMA LNS - "#)]
    pub p9gtygnmr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-PART GTY LN&LS - "#)]
    pub p9gtypar: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-PART GTY LN&LS RATIO - "#)]
    pub p9gtyparr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D AG LOANS-LOSS SH - "#)]
    pub p9lag: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D AG LOANS-LOSS SH RATIO - "#)]
    pub p9lagr: Option<f32>,

    #[schemars(description = r#"90+DAYS P/D C&I LNS-LOSS SH - "#)]
    pub p9lci: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D C&I LNS-LOSS SH RATIO - "#)]
    pub p9lcir: Option<f32>,

    #[schemars(description = r#"90+ D P/D CONSUMER LN - LOSS SH - "#)]
    pub p9lcon: Option<f32>,

    #[schemars(description = r#"90+ D P/D CONSUMER LN - LOSS SH RATIO - "#)]
    pub p9lconr: Option<f32>,

    #[schemars(description = r#"90+ D P/D PROTECT (GTY)-LOSS SH - "#)]
    pub p9lgty: Option<f32>,

    #[schemars(description = r#"90+ D P/D PROTECT (GTY)-LOSS SH RATIO - "#)]
    pub p9lgtyr: Option<f32>,

    #[schemars(description = r#"90 DAYS P/D-L&L HELD FOR SALE - "#)]
    pub p9lnsale: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-L&L HELD FOR SALE RATIO - "#)]
    pub p9lnsaler: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D OTHER LNS-LOSS SH - "#)]
    pub p9loth: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D OTHER LNS-LOSS SH RATIO - "#)]
    pub p9lothr: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RE FARM-LOSS SH - "#)]
    pub p9lreag: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RE FARM-LOSS SH RATIO - "#)]
    pub p9lreagr: Option<f32>,

    #[schemars(description = r#"90+ D P/D CONSTRUCTION -LOSS SH - "#)]
    pub p9lrecon: Option<f32>,

    #[schemars(description = r#"90+ D P/D CONSTRUCTION -LOSS SH RATIO - "#)]
    pub p9lreconr: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D MULTIFAM - LOSS SH - "#)]
    pub p9lremul: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D MULTIFAM - LOSS SH RATIO - "#)]
    pub p9lremulr: Option<f32>,

    #[schemars(description = r#"90+ D P/D NFNR - LOSS SHARE - "#)]
    pub p9lrenrs: Option<f32>,

    #[schemars(description = r#"90+ D P/D NFNR - LOSS SH RATIO - "#)]
    pub p9lrenrsr: Option<f32>,

    #[schemars(description = r#"90+ D P/D 1-4 FAMILY - LOSS SH - "#)]
    pub p9lreres: Option<f32>,

    #[schemars(description = r#"90+ D P/D 1-4 FAMILY - LOSS SH RATIO - "#)]
    pub p9lreresr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-LEASES - "#)]
    pub p9ls: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-LEASES RATIO - "#)]
    pub p9lsr: Option<f32>,

    #[schemars(description = r#"90+ D P/D TOTAL LOANS - LOSS SH - "#)]
    pub p9ltot: Option<f32>,

    #[schemars(description = r#"90+ D P/D TOTAL LOANS - LOSS SH RATIO - "#)]
    pub p9ltotr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-ALL OTHER LOANS - "#)]
    pub p9othln: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-ALL OTHER LOANS RATIO - "#)]
    pub p9othlnr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-REAL ESTATE LOANS - "#)]
    pub p9re: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-REAL ESTATE RATIO - "#)]
    pub p9rer: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*FARMLAND - "#)]
    pub p9reag: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*FARMLAND - "#)]
    pub p9reagr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D 1-4 FAM CONSTRUC LN - "#)]
    pub p9recnfm: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D 1-4 FAM CONSTRUC LN RATIO - "#)]
    pub p9recnfmr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D OTHER CONSTR & LAND - "#)]
    pub p9recnot: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D OTHER CONSTR & LAND RATIO - "#)]
    pub p9recnotr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*CONSTRUCTION - "#)]
    pub p9recons: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*CONSTRUCTION RATIO - "#)]
    pub p9reconsr: Option<f32>,

    #[schemars(description = r#"90 + DAYS P/D-RE*FOREIGN - "#)]
    pub p9refor: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*FOREIGN RATIO - "#)]
    pub p9reforr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 FAM LINES - "#)]
    pub p9reloc: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 FAM LINES RATIO - "#)]
    pub p9relocr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*MULTIFAMILY - "#)]
    pub p9remult: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*MULTIFAMILY RATIO - "#)]
    pub p9remultr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*NONFARM NONRES - "#)]
    pub p9renres: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*NONFARM NONRES RATIO - "#)]
    pub p9renresr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D OTHER NONFRM NONRES - "#)]
    pub p9renrot: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D OTHER NONFRM NONRES RATIO - "#)]
    pub p9renrotr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D 0WN-OCC NONFR NONRS - "#)]
    pub p9renrow: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D OWN-OCC NONFR NONRS RATIO - "#)]
    pub p9renrowr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*NON-U.S. - "#)]
    pub p9renus: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*NON-U.S. - "#)]
    pub p9renusr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 FAMILY - "#)]
    pub p9reres: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 FAMILY RATIO - "#)]
    pub p9reresr: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 JN LIEN - "#)]
    pub p9rersf2: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 JN LIEN RATIO - "#)]
    pub p9rersf2r: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 IST LIEN - "#)]
    pub p9rersfm: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-RE*1-4 IST LIEN RATIO - "#)]
    pub p9rersfmr: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RESTRUCT C&I LN - "#)]
    pub p9rsci: Option<f32>,

    #[schemars(description = r#"90+ D P/D RESTRUCT CONSTRUCTION - "#)]
    pub p9rscons: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D RESTR LN- 1-4 FAM - "#)]
    pub p9rslnfm: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D RESTR LN- 1-4 FAM RATIO - "#)]
    pub p9rslnfmr: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RESTRU LN EXCL 1-4 FM - "#)]
    pub p9rslnls: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RESTRU LN EXCL 1-4 FM RATIO - "#)]
    pub p9rslnlsr: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RESTR LN- TOTAL - "#)]
    pub p9rslnlt: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RESTR LN- TOTAL RATIO - "#)]
    pub p9rslnltr: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RESTRUCT MULTIFAM - "#)]
    pub p9rsmult: Option<f32>,

    #[schemars(description = r#"90+ DAY P/D RESTRUCT NFNR LN - "#)]
    pub p9rsnres: Option<f32>,

    #[schemars(description = r#"90+ D P/D RESTRUCT ALL OTH LN - "#)]
    pub p9rsoth: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-DEBT SECURITIES - "#)]
    pub p9scdebt: Option<f32>,

    #[schemars(description = r#"90+ DAYS P/D-DEBT SECURITIES RATIO - "#)]
    pub p9scdebtr: Option<f32>,

    #[schemars(description = r#"PARTICIPATIONS ACQUIRED - "#)]
    pub partacqu: Option<f32>,

    #[schemars(description = r#"PARTICIPATIONS CONVEYED - "#)]
    pub partconv: Option<f32>,

    #[schemars(description = r#"PARTICIPATIONS CONVEYED RATIO - "#)]
    pub partconvr: Option<f32>,

    #[schemars(description = r#"ALLOWANCE FOR L&L IN TIER 2 - "#)]
    pub rb2lnres: Option<f32>,

    #[schemars(description = r#"ALLOWANCE FOR L&L IN TIER 2 RATIO - "#)]
    pub rb2lnresr: Option<f32>,

    #[schemars(description = r#"RBC-TOTAL-PCA - "#)]
    pub rbc: Option<f32>,

    #[schemars(description = r#"TIER 1 RBC-PCA - "#)]
    pub rbct1: Option<f32>,

    #[schemars(description = r#"RBC-TIER2-PCA - "#)]
    pub rbct2: Option<f32>,

    #[schemars(description = r#"RBC-TIER2-PCA RATIO - "#)]
    pub rbct2r: Option<f32>,

    #[schemars(description = r#"RC-R COMMON EQ TIER 1 CAPITAL - "#)]
    pub rbct1c: Option<f32>,

    #[schemars(description = r#"COMMON EQUITY TIER 1 RATIO - "#)]
    pub rbct1cer: Option<f32>,

    #[schemars(description = r#"TIER 1 RBC ADJUSTED LLR - PCA - "#)]
    pub rbct1j: Option<f32>,

    #[schemars(description = r#"TIER 1 RBC ADJUSTED LLR - PCA RATIO - "#)]
    pub rbct1jr: Option<f32>,

    #[schemars(description = r#"LEVERAGE RATIO-PCA - "#)]
    pub rbc1aaj: Option<f32>,

    #[schemars(description = r#"TIER 1 RBC RATIO-PCA - "#)]
    pub rbc1rwaj: Option<f32>,

    #[schemars(description = r#"TOTAL RBC RATIO-PCA - "#)]
    pub rbcrwaj: Option<f32>,

    #[schemars(description = r#"REPURCHASE AGREEMENT-FOR - "#)]
    pub repopurf: Option<f32>,

    #[schemars(description = r#"REVERSE REPURCHASE AGREEMENT-FOR - "#)]
    pub reposldf: Option<f32>,

    #[schemars(description = r#"RETAINED EARNINGS/AVG BK EQUITY - "#)]
    pub roeinjr: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LN - C&I - "#)]
    pub rsci: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LN - CONSTRUCTION - "#)]
    pub rscons: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LN EXCL 1-4 FM - "#)]
    pub rslnls: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LN EXCL 1-4 FM RATIO - "#)]
    pub rslnlsr: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LOANS - TOTAL - "#)]
    pub rslnltot: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LOANS - TOTAL RATIO - "#)]
    pub rslnltotr: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LOANS - 1-4 FAMILY - "#)]
    pub rslnrefm: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LOANS - 1-4 FAMILY RATIO - "#)]
    pub rslnrefmr: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LN - MULTIFAMILY - "#)]
    pub rsmult: Option<f32>,

    #[schemars(description = r#"RESTRUCT LN - NONFARM NONRES - "#)]
    pub rsnres: Option<f32>,

    #[schemars(description = r#"RESTRUCTURED LN - ALL OTHER - "#)]
    pub rsother: Option<f32>,

    #[schemars(description = r#"FEDERAL RESERVE ID NUMBER - "#)]
    pub rssdid: Option<f32>,

    #[schemars(description = r#"INTEREST RATE-TOTAL CONTRACTS - "#)]
    pub rt: Option<f32>,

    #[schemars(description = r#"INT RATE-FUTURES & FORWARD CONTR - "#)]
    pub rtffc: Option<f32>,

    #[schemars(description = r#"INT RATE-SWAPS - "#)]
    pub rtnvs: Option<f32>,

    #[schemars(description = r#"INT RATE-PUR OPTION CONTRACTS - "#)]
    pub rtpoc: Option<f32>,

    #[schemars(description = r#"INT RATE-WRITTEN OPTION CONTRACT - "#)]
    pub rtwoc: Option<f32>,

    #[schemars(description = r#"RWA-ADJUST-PCA-T1 & CET1 RATIO - "#)]
    pub rwaj: Option<f32>,

    #[schemars(description = r#"RWA-ADJUSTED-PCA-TOTAL RBC RAT - "#)]
    pub rwajt: Option<f32>,

    #[schemars(description = r#"RWA-ADJUSTED-PCA-TOTAL RBC RAT RATIO - "#)]
    pub rwajtr: Option<f32>,

    #[schemars(description = r#"ABS-TOTAL-B/S - "#)]
    pub scabs: Option<f32>,

    #[schemars(description = r#"ABS-TOTAL-B/S RATIO - "#)]
    pub scabsr: Option<f32>,

    #[schemars(description = r#"SECURITIES-AF - "#)]
    pub scaf: Option<f32>,

    #[schemars(description = r#"SECURITIES-AF RATIO - "#)]
    pub scafr: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY ALL OTH - "#)]
    pub scaot: Option<f32>,

    #[schemars(description = r#"COMMERCIAL MBS - TOTAL - "#)]
    pub sccmmb: Option<f32>,

    #[schemars(description = r#"OTHER COMMERCIAL MBS-GOVT - "#)]
    pub sccmog: Option<f32>,

    #[schemars(description = r#"OTHER COMMERCIAL MBS-GOVT RATIO - "#)]
    pub sccmogr: Option<f32>,

    #[schemars(description = r#"OTHER COMMERCIAL MBS - "#)]
    pub sccmot: Option<f32>,

    #[schemars(description = r#"OTHER COMMERCIAL MBS RATIO - "#)]
    pub sccmotr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL MBS PASS-THROUGH - "#)]
    pub sccmpt: Option<f32>,

    #[schemars(description = r#"COMMERCIAL MBS PASS-THROUGH RATIO - "#)]
    pub sccmptr: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY COLLATERAL MTG-RES - "#)]
    pub sccol: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY COLLATERAL MTG-RES RATIO - "#)]
    pub sccolr: Option<f32>,

    #[schemars(description = r#"COMM MBS PASS-THRU-GOVT - "#)]
    pub sccptg: Option<f32>,

    #[schemars(description = r#"COMM MBS PASS-THRU-GOVT RATIO - "#)]
    pub sccptgr: Option<f32>,

    #[schemars(description = r#"EQ SEC READILY DET FV - "#)]
    pub sceqfv: Option<f32>,

    #[schemars(description = r#"EQ SEC READILY DET FV RATIO - "#)]
    pub sceqfvr: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY ISSUED*FNMA-RES - "#)]
    pub scfmn: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY ISSUED*FNMA-RES RATIO - "#)]
    pub scfmnr: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY GTY BY GNMA - "#)]
    pub scgnm: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY GTY BY GNMA RATIO - "#)]
    pub scgnmr: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY ISSUED OR GTY-RES - "#)]
    pub scgty: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY ISSUED OR GTY-RES RATIO - "#)]
    pub scgtyr: Option<f32>,

    #[schemars(description = r#"SECURITIES-HA - "#)]
    pub scha: Option<f32>,

    #[schemars(description = r#"SECURITIES-HA RATIO - "#)]
    pub schar: Option<f32>,

    #[schemars(description = r#"LESS ALLOW FOR CREDIT LOSSES ON HELD TO MATURITY DEBT SECURITIES - "#)]
    pub schtmres: Option<f32>,

    #[schemars(description = r#"LESS ALLOW FOR CREDIT LOSSES ON HELD TO MATURITY DEBT SECURITIES RATIO - "#)]
    pub schtmresr: Option<f32>,

    #[schemars(description = r#"SECURITIES LENT - "#)]
    pub sclent: Option<f32>,

    #[schemars(description = r#"SECURITIES LENT RATIO - "#)]
    pub sclentr: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 1-3 YEARS - "#)]
    pub scnm1t3: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 1-3 YEARS RATIO - "#)]
    pub scnm1t3r: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC*3 MONS OR LESS - "#)]
    pub scnm3les: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC*3 MONS OR LESS RATIO - "#)]
    pub scnm3lesr: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 3-5 YEARS - "#)]
    pub scnm3t5: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 3-5 YEARS RATIO - "#)]
    pub scnm3t5r: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 3-12 MONTHS - "#)]
    pub scnm3t12: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 3-12 MONTHS RATIO - "#)]
    pub scnm3t12r: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 5-15 YEARS - "#)]
    pub scnm5t15: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * 5-15 YEARS RATIO - "#)]
    pub scnm5t15r: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * OVER 15 YEARS - "#)]
    pub scnmov15: Option<f32>,

    #[schemars(description = r#"NONMTG DEBT SEC * OVER 15 YEARS RATIO - "#)]
    pub scnmov15r: Option<f32>,

    #[schemars(description = r#"OTH MORTGAGE SEC * 3 YR OR LESS - "#)]
    pub sco3yles: Option<f32>,

    #[schemars(description = r#"OTH MORTGAGE SEC * 3 YR OR LESS RATIO - "#)]
    pub sco3ylesr: Option<f32>,

    #[schemars(description = r#"Fixed and floating rate debt securities (included above) with remaining maturity of one year or less - "#)]
    pub sc1les: Option<f32>,

    #[schemars(description = r#"Fixed and floating rate debt securities (included above) with remaining maturity of one year or less ratio - "#)]
    pub sc1lesr: Option<f32>,

    #[schemars(description = r#"OTH DOM DEBT*ALL OTHER - "#)]
    pub scodot: Option<f32>,

    #[schemars(description = r#"OTH DOM DEBT*ALL OTHER RATIO - "#)]
    pub scodotr: Option<f32>,

    #[schemars(description = r#"CMO PRIV ISSUED - "#)]
    pub scodpi: Option<f32>,

    #[schemars(description = r#"CMO PRIV ISSUED RATIO - "#)]
    pub scodpir: Option<f32>,

    #[schemars(description = r#"OTH MORTGAGE SEC * OVER 3 YRS - "#)]
    pub scoov3y: Option<f32>,

    #[schemars(description = r#"OTH MORTGAGE SEC * OVER 3 YRS RATIO - "#)]
    pub scoov3yr: Option<f32>,

    #[schemars(description = r#"PLEDGED SECURITIES - "#)]
    pub scpledge: Option<f32>,

    #[schemars(description = r#"PLEDGED SECURITIES RATIO - "#)]
    pub scpledger: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 1-3 YEARS - "#)]
    pub scpt1t3: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 1-3 YEARS RATIO - "#)]
    pub scpt1t3r: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC*3 MON OR LESS - "#)]
    pub scpt3les: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC*3 MON OR LESS RATIO - "#)]
    pub scpt3lesr: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 3-5 YEARS - "#)]
    pub scpt3t5: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 3-5 YEARS RATIO - "#)]
    pub scpt3t5r: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 3-12 MONTHS - "#)]
    pub scpt3t12: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 3-12 MONTHS RATIO - "#)]
    pub scpt3t12r: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 5-15 YEARS - "#)]
    pub scpt5t15: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * 5-15 YEARS RATIO - "#)]
    pub scpt5t15r: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * OVER 15 YRS - "#)]
    pub scptov15: Option<f32>,

    #[schemars(description = r#"MTG PASS-THRU SEC * OVER 15 YRS RATIO - "#)]
    pub scptov15r: Option<f32>,

    #[schemars(description = r#"DEBT SECURITIES - "#)]
    pub scrdebt: Option<f32>,

    #[schemars(description = r#"DEBT SECURITIES RATIO - "#)]
    pub scrdebtr: Option<f32>,

    #[schemars(description = r#"STRUCTURED FIN PROD - TOTAL - "#)]
    pub scsfp: Option<f32>,

    #[schemars(description = r#"STRUCTURED FIN PROD - TOTAL RATIO - "#)]
    pub scsfpr: Option<f32>,

    #[schemars(description = r#"STRUCTURED NOTES AMORTIZED COST - "#)]
    pub scsnhaa: Option<f32>,

    #[schemars(description = r#"STRUCTURED NOTES AMORTIZED COST RATIO - "#)]
    pub scsnhaar: Option<f32>,

    #[schemars(description = r#"STRUCTURED NOTES-FAIR VALUE - "#)]
    pub scsnhaf: Option<f32>,

    #[schemars(description = r#"STRUCTURED NOTES-FAIR VALUE RATIO - "#)]
    pub scsnhafr: Option<f32>,

    #[schemars(description = r#"U.S. AGENCY GOVT SPONSORED - "#)]
    pub scspn: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-AUTO - "#)]
    pub sz30auto: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-AUTO RATIO - "#)]
    pub sz30autor: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-CI - "#)]
    pub sz30ci: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-CI RATIO - "#)]
    pub sz30cir: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-CON - "#)]
    pub sz30con: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-CON RATIO - "#)]
    pub sz30conr: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-CRCD - "#)]
    pub sz30crcd: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-CRCD RATIO - "#)]
    pub sz30crcdr: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-HEL - "#)]
    pub sz30hel: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-HEL RATIO - "#)]
    pub sz30helr: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-OTH - "#)]
    pub sz30oth: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION-OTH RATIO - "#)]
    pub sz30othr: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION -RES - "#)]
    pub sz30res: Option<f32>,

    #[schemars(description = r#"30-89 PD LN-SECURITIZATION -RES RATIO - "#)]
    pub sz30resr: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-AUTO - "#)]
    pub sz90auto: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-AUTO RATIO - "#)]
    pub sz90autor: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-CI - "#)]
    pub sz90ci: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-CI RATIO - "#)]
    pub sz90cir: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-CON - "#)]
    pub sz90con: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-CON RATIO - "#)]
    pub sz90conr: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-CRCD - "#)]
    pub sz90crcd: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-CRCD RATIO - "#)]
    pub sz90crcdr: Option<f32>,

    #[schemars(description = r#"90+ PD LN-SECURITIZATION-HEL - "#)]
    pub sz90hel: Option<f32>,

    #[schemars(description = r#"90+ PD LN-SECURITIZATION-HEL RATIO - "#)]
    pub sz90helr: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-OTH - "#)]
    pub sz90oth: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-OTH RATIO - "#)]
    pub sz90othr: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-RES - "#)]
    pub sz90res: Option<f32>,

    #[schemars(description = r#"90 + PD LN-SECURITIZATION-RES RATION - "#)]
    pub sz90resr: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-AUTO - "#)]
    pub szcrauto: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-AUTO - "#)]
    pub szcrautor: Option<f32>,

    #[schemars(description = r#"OUTSTDG CC FEES IN SECURITZD CC - "#)]
    pub szcrcdfe: Option<f32>,

    #[schemars(description = r#"OUTSTDG CC FEES IN SECURITZD CC RATIO - "#)]
    pub szcrcdfer: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-CI - "#)]
    pub szcrci: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-CI RATIO - "#)]
    pub szcrcir: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-CON - "#)]
    pub szcrcon: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-CON RATIO - "#)]
    pub szcrconr: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION - CRCD - "#)]
    pub szcrcrcd: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION - CRCD RATIO - "#)]
    pub szcrcrcdr: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD-HEL - "#)]
    pub szcrhel: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD-HEL RATIO - "#)]
    pub szcrhelr: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION- - "#)]
    pub szcroth: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION- RATIO - "#)]
    pub szcrothr: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-RES - "#)]
    pub szcrres: Option<f32>,

    #[schemars(description = r#"REC ASSET SECURITIZATION-RES RATIO - "#)]
    pub szcrresr: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-AUTO - "#)]
    pub szdrauto: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-AUTO RATIO - "#)]
    pub szdrautor: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-CI - "#)]
    pub szdrci: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-CI RATIO - "#)]
    pub szdrcir: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-CON - "#)]
    pub szdrcon: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-CON RATIO - "#)]
    pub szdrconr: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-CRCD - "#)]
    pub szdrcrcd: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-CRCD RATIO - "#)]
    pub szdrcrcdr: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-HEL - "#)]
    pub szdrhel: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-HEL RATIO - "#)]
    pub szdrhelr: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-OTH - "#)]
    pub szdroth: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-OTH RATIO - "#)]
    pub szdrothr: Option<f32>,

    #[schemars(description = r#"C/O ON ASSET SECURITIZATION-RES - "#)]
    pub szdrres: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - AUTO - "#)]
    pub szislaut: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - AUTO RATIO - "#)]
    pub szislautr: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - CRCD - "#)]
    pub szislccd: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - CRCD RATIO - "#)]
    pub szislccdr: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN -CI - "#)]
    pub szislci: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN -CI RATIO - "#)]
    pub szislcir: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - CON - "#)]
    pub szislcon: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - CON RATIO - "#)]
    pub szislconr: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - HEL - "#)]
    pub szislhel: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN - HEL RATIO - "#)]
    pub szislhelr: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN -OTH - "#)]
    pub szisloth: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATN -OTH RATIO - "#)]
    pub szislothr: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATION RES - "#)]
    pub szislres: Option<f32>,

    #[schemars(description = r#"CR EXP ON SECURITIZATION RES RATIO - "#)]
    pub szislresr: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - AUTO - "#)]
    pub szlauto: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - AUTO RATIO - "#)]
    pub szlautor: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - CI - "#)]
    pub szlnci: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - CI RATIO - "#)]
    pub szlncir: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - CONS - "#)]
    pub szlncon: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - CONS RATIO - "#)]
    pub szlnconr: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - CRCD - "#)]
    pub szlncrcd: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - CRCD RATIO - "#)]
    pub szlncrcdr: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - HEL - "#)]
    pub szlnhel: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - HEL RATIO - "#)]
    pub szlnhelr: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - OTH - "#)]
    pub szlnoth: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD - OTH RATIO - "#)]
    pub szlnothr: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD-RES - "#)]
    pub szlnres: Option<f32>,

    #[schemars(description = r#"RE PRIN SEC ASSET SOLD-RES RATIO - "#)]
    pub szlnresr: Option<f32>,

    #[schemars(description = r#"COMMITS FOR LIQUIDITY  - AUTO - "#)]
    pub szucauto: Option<f32>,

    #[schemars(description = r#"COMMITS FOR LIQUIDITY  - CI - "#)]
    pub szucci: Option<f32>,

    #[schemars(description = r#"COMMITS FOR LIQUIDITY  - CON - "#)]
    pub szuccon: Option<f32>,

    #[schemars(description = r#"COMMITS FOR LIQUIDITY  - CRCD - "#)]
    pub szuccrcd: Option<f32>,

    #[schemars(description = r#"COMMITS FOR LIQUIDITY - HEL - "#)]
    pub szuchel: Option<f32>,

    #[schemars(description = r#"COMMITS FOR LIQUIDITY  - OTH - "#)]
    pub szucoth: Option<f32>,

    #[schemars(description = r#"COMMITS FOR LIQUIDITY  - RES - "#)]
    pub szucres: Option<f32>,

    #[schemars(description = r#"CORP TRUST-MANAGED-AMT - "#)]
    pub tcama: Option<f32>,

    #[schemars(description = r#"CORP TRUST-MANAGED-NUM - "#)]
    pub tcamanum: Option<f32>,

    #[schemars(description = r#"CORP TRUST-NON-MANAGED-AMT - "#)]
    pub tcanma: Option<f32>,

    #[schemars(description = r#"CORP TRUST-NON-MANAGED-NUM - "#)]
    pub tcanmnum: Option<f32>,

    #[schemars(description = r#"CORP TRUST-TRUSTEESHIPS-NUM - "#)]
    pub tcanum: Option<f32>,

    #[schemars(description = r#"CORP & MUNI-TRUSTEE-DEFAULT-NUM - "#)]
    pub tcanumd: Option<f32>,

    #[schemars(description = r#"CORP TRUST-TRUSTEESHIPS-AMT - "#)]
    pub tcapao: Option<f32>,

    #[schemars(description = r#"CORP & MUNI-TRUSTEE-DEFAULT-AMT - "#)]
    pub tcapaod: Option<f32>,

    #[schemars(description = r#"CORP TRUST-TRANSFER-NUM - "#)]
    pub tcatnum: Option<f32>,

    #[schemars(description = r#"CIFS -DOM EQUITY-AMT - "#)]
    pub tcdemv: Option<f32>,

    #[schemars(description = r#"CIFS -DOM EQUITY-NUM - "#)]
    pub tcdenum: Option<f32>,

    #[schemars(description = r#"CIFS -INTL/GLOBAL-EQ-AMT - "#)]
    pub tciemv: Option<f32>,

    #[schemars(description = r#"CIFS -INTL/GLOBAL-EQ-NUM - "#)]
    pub tcienum: Option<f32>,

    #[schemars(description = r#"CIFS-MUNICIPAL BOND-AMT - "#)]
    pub tcmbmv: Option<f32>,

    #[schemars(description = r#"CIFS-MUNICIPAL BOND-NUM - "#)]
    pub tcmbnum: Option<f32>,

    #[schemars(description = r#"CIFS -STOCK/BOND-AMT - "#)]
    pub tcsbmv: Option<f32>,

    #[schemars(description = r#"CIFS -STOCK/BOND-NUM - "#)]
    pub tcsbnum: Option<f32>,

    #[schemars(description = r#"CUST AND SAFE ACCT-NON-MAN-AMT - "#)]
    pub tcsnma: Option<f32>,

    #[schemars(description = r#"CUST AND SAFE ACCT-NON-MAN-NUM - "#)]
    pub tcsnmnum: Option<f32>,

    #[schemars(description = r#"CIFS-SPECIALTY/OTHER-AMT - "#)]
    pub tcsomv: Option<f32>,

    #[schemars(description = r#"CIFS-SPECIALTY/OTHER-NUM - "#)]
    pub tcsonum: Option<f32>,

    #[schemars(description = r#"CIFS-SHORT TERM INV-AMT - "#)]
    pub tcstmv: Option<f32>,

    #[schemars(description = r#"CIFS-SHORT TERM INV-NUM - "#)]
    pub tcstnum: Option<f32>,

    #[schemars(description = r#"CIFS - TAXABLE BOND-AMT - "#)]
    pub tctbmv: Option<f32>,

    #[schemars(description = r#"CIFS - TAXABLE BOND-NUM - "#)]
    pub tctbnum: Option<f32>,

    #[schemars(description = r#"CIFS-TOTAL-AMT - "#)]
    pub tctotmv: Option<f32>,

    #[schemars(description = r#"CIFS-TOTAL-NUM - "#)]
    pub tctotnum: Option<f32>,

    #[schemars(description = r#"EMP BENE-DEF BENE-MANAGE-AMT - "#)]
    pub tebma: Option<f32>,

    #[schemars(description = r#"EMP BENE-DEF BENE-MANAGED-NUM - "#)]
    pub tebmanum: Option<f32>,

    #[schemars(description = r#"EMP BENE-DEF BENE-NON-MAN-AMT - "#)]
    pub tebnma: Option<f32>,

    #[schemars(description = r#"EMP BENE-DEF BENE-NON-MAN-NUM - "#)]
    pub tebnmnum: Option<f32>,

    #[schemars(description = r#"EMP BENE-CONTRIB-MANAGED-AMT - "#)]
    pub tecma: Option<f32>,

    #[schemars(description = r#"EMP BENE-CONTRI-MANAGED-NUM - "#)]
    pub tecmanum: Option<f32>,

    #[schemars(description = r#"EMP BENE-CONTRI-NON-MAN-AMT - "#)]
    pub tecnma: Option<f32>,

    #[schemars(description = r#"EMP BENE-CONTRI-NON-MANAGE-NUM - "#)]
    pub tecnmnum: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - COM & PF STK - "#)]
    pub tecps: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - EQ MUT FUND - "#)]
    pub teeqf: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - INT BEARING - "#)]
    pub tei: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR-TOT MANAGE AST - "#)]
    pub tematot: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - MISC ASSET - "#)]
    pub temisc: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - MONEY MKT - "#)]
    pub temmf: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - NONINT BEAR - "#)]
    pub teni: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR-OTH NOTE & BND - "#)]
    pub teothb: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - OTH MUT FUND - "#)]
    pub teothf: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - REAL ESTATE - "#)]
    pub tere: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - RE MTG - "#)]
    pub teremtg: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - MUNI - "#)]
    pub tescmun: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR -U.S TREAS & OB - "#)]
    pub tescus: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - SHRT TERM OB - "#)]
    pub testo: Option<f32>,

    #[schemars(description = r#"EXPENSE FIDUCIARY - YTD - "#)]
    pub tetot: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - TRUST FUND - "#)]
    pub tetrf: Option<f32>,

    #[schemars(description = r#"EMP BEN & RET TR - UNREG FUNDS - "#)]
    pub teuf: Option<f32>,

    #[schemars(description = r#"FOUNDATION & ENDOW-MANAGED-AMT - "#)]
    pub tfema: Option<f32>,

    #[schemars(description = r#"FOUNDATION & ENDOW-MANAGED-NUM - "#)]
    pub tfemanum: Option<f32>,

    #[schemars(description = r#"FOUNDATION & END-NON-MANAGE-AMT - "#)]
    pub tfenma: Option<f32>,

    #[schemars(description = r#"FOUNDATION & END-NON-MANAGE-NUM - "#)]
    pub tfenmnum: Option<f32>,

    #[schemars(description = r#"GR.INC-CORP TRUST & AGENCY-YTD - "#)]
    pub tica: Option<f32>,

    #[schemars(description = r#"GR.INC-CUSTODY-YTD - "#)]
    pub tics: Option<f32>,

    #[schemars(description = r#"GR.INC-EMP. BENEFIT-BENEFIT-YTD - "#)]
    pub tieb: Option<f32>,

    #[schemars(description = r#"GR.INC-EMP. BENEFIT- CONTRI-YTD - "#)]
    pub tiec: Option<f32>,

    #[schemars(description = r#"GR. INC- FOUNDATION & ENDOW-YTD - "#)]
    pub tife: Option<f32>,

    #[schemars(description = r#"GR.INC - INVESTMENT AGCY - YTD - "#)]
    pub tima: Option<f32>,

    #[schemars(description = r#"INVESTMENT AGENCY-MANAGED-AMT - "#)]
    pub timma: Option<f32>,

    #[schemars(description = r#"INVESTMENT AGENCY-MANAGED-NUM - "#)]
    pub timmanum: Option<f32>,

    #[schemars(description = r#"INVESTMENT AGCY NON-MANAGED-AMT - "#)]
    pub timnma: Option<f32>,

    #[schemars(description = r#"INVESTMENT AGCY NON-MANAGED-NUM - "#)]
    pub timnmnum: Option<f32>,

    #[schemars(description = r#"INTRACOMPANY INC FIDUCIARY-YTD - "#)]
    pub tintra: Option<f32>,

    #[schemars(description = r#"GR.INC-OTHER FIDUCIARY-YTD - "#)]
    pub tiof: Option<f32>,

    #[schemars(description = r#"GR.INC-OTHER RETIREMENT -YTD - "#)]
    pub tior: Option<f32>,

    #[schemars(description = r#"GR.INC-PERSONAL & AG ACCTS-YTD - "#)]
    pub tip: Option<f32>,

    #[schemars(description = r#"GR.INC-RELATED SERV-YTD - "#)]
    pub tir: Option<f32>,

    #[schemars(description = r#"TOT FOREIGN OFF GROSS FIDUC-YTD - "#)]
    pub titotf: Option<f32>,

    #[schemars(description = r#"FIDUCIARY FGN OFF-MANAGED-AMT - "#)]
    pub tmaf: Option<f32>,

    #[schemars(description = r#"FIDUCIARY FGN OFF-MANAGED-AMT - "#)]
    pub tmafnum: Option<f32>,

    #[schemars(description = r#"ADVISED/SPONSORED MUT FND -AMT - "#)]
    pub tmasmf: Option<f32>,

    #[schemars(description = r#"ADVISED/SPONSORED MUTAL FND-NUM - "#)]
    pub tmasmfn: Option<f32>,

    #[schemars(description = r#"NET FIDUCIARY INCOME -YTD - "#)]
    pub tni: Option<f32>,

    #[schemars(description = r#"NET LOSS FROM FIDUCIARY-YTD - "#)]
    pub tnl: Option<f32>,

    #[schemars(description = r#"FIDUCIARY FGN OFF-NON-MAN-AMT - "#)]
    pub tnmaf: Option<f32>,

    #[schemars(description = r#"FIDUCIARY FGN OFF-NON-MAN-NUM - "#)]
    pub tnmnumf: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN ASSET-COM & PFD STK - "#)]
    pub tocps: Option<f32>,

    #[schemars(description = r#"ALL OTH MANAGE AST - EQ MUT FND - "#)]
    pub toeqf: Option<f32>,

    #[schemars(description = r#"OTH FIDUCIARY-MANAGED-AMT - "#)]
    pub tofma: Option<f32>,

    #[schemars(description = r#"OTH FIDUCIARY-MANAGED-NUM - "#)]
    pub tofmanum: Option<f32>,

    #[schemars(description = r#"OTH FIDUCIARY NON-MANAGED-AMT - "#)]
    pub tofnma: Option<f32>,

    #[schemars(description = r#"OTH FIDUCIARY-NON-MANAGED-NUM - "#)]
    pub tofnmnum: Option<f32>,

    #[schemars(description = r#"ALL OTH MANAGE ASSET - INT BEAR - "#)]
    pub toi: Option<f32>,

    #[schemars(description = r#"ALL OTHER MANAGED ASSET- TOTAL - "#)]
    pub tomatot: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN ASSET - MISC ASSET - "#)]
    pub tomisc: Option<f32>,

    #[schemars(description = r#"ALL OTH MANAGE AST - MONEY MKT - "#)]
    pub tommf: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN ASSET - NONINT BEAR - "#)]
    pub toni: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN AST -OTH NOTE & BND - "#)]
    pub toothb: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN ASSET - OTH MUT FND - "#)]
    pub toothf: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN ASSET - REAL ESTATE - "#)]
    pub tore: Option<f32>,

    #[schemars(description = r#"ALL OTHER MANAGE ASSET - RE MTG - "#)]
    pub toremtg: Option<f32>,

    #[schemars(description = r#"OTH RETIREMENT-MANAGED-AMT - "#)]
    pub torma: Option<f32>,

    #[schemars(description = r#"OTH RETIREMENT-MANAGED-NUM - "#)]
    pub tormanum: Option<f32>,

    #[schemars(description = r#"OTH RETIREMENT-NON-MAN-AMT - "#)]
    pub tornma: Option<f32>,

    #[schemars(description = r#"OTH RETIREMENT-NON-MAN-NUM - "#)]
    pub tornmnum: Option<f32>,

    #[schemars(description = r#"ALL OTHER MANAGED ASSET - MUNI - "#)]
    pub toscmun: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN AST-U.S. TREAS & OB - "#)]
    pub toscus: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN AST - SHRT TERM OBL - "#)]
    pub tosto: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN ASSET - TRUST FUND - "#)]
    pub totrf: Option<f32>,

    #[schemars(description = r#"ALL OTH MAN ASSET - UNREG FUNDS - "#)]
    pub touf: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY- COM & PRF STK - "#)]
    pub tpicps: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - EQ MUT FUND - "#)]
    pub tpieqf: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - INT BEARING - "#)]
    pub tpii: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY-TOT MANAGE AST - "#)]
    pub tpimatot: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - MISC - "#)]
    pub tpimisc: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - MONEY MKT - "#)]
    pub tpimmf: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY-NONINT BEARING - "#)]
    pub tpini: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY-OTH NOTE & BND - "#)]
    pub tpiothb: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - OTH MUT FUND - "#)]
    pub tpiothf: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - REAL ESTATE - "#)]
    pub tpire: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - RE MTG - "#)]
    pub tpiremtg: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - MUNI - "#)]
    pub tpiscmun: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY-U.S TREAS & OB - "#)]
    pub tpiscus: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - SHRT TERM OB - "#)]
    pub tpisto: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY - TRUST FUND - "#)]
    pub tpitrf: Option<f32>,

    #[schemars(description = r#"PER TR & INV AGY- UNREG FUNDS - "#)]
    pub tpiuf: Option<f32>,

    #[schemars(description = r#"MANAGED ASSET-PER & AGEN-AMT - "#)]
    pub tpma: Option<f32>,

    #[schemars(description = r#"MANAGED ASSET - PER&AGEN-NUM - "#)]
    pub tpmanum: Option<f32>,

    #[schemars(description = r#"NON-MANAGED - PER&AGEN-AMT - "#)]
    pub tpnma: Option<f32>,

    #[schemars(description = r#"NON-MANAGED ASSET-PER&AGEN-NUM - "#)]
    pub tpnmnum: Option<f32>,

    #[schemars(description = r#"TRUST POWERS EXERCISED - "#)]
    pub trexer: Option<f32>,

    #[schemars(description = r#"TRADING ACCOUNTS-FOR - "#)]
    pub trfor: Option<f32>,

    #[schemars(description = r#"IRA - "#)]
    pub trhma: Option<f32>,

    #[schemars(description = r#"IRA - "#)]
    pub trhmanum: Option<f32>,

    #[schemars(description = r#"IRA - "#)]
    pub trhnma: Option<f32>,

    #[schemars(description = r#"IRA - "#)]
    pub trhnmnum: Option<f32>,

    #[schemars(description = r#"TRADE-DERIVATIVES NEG VAL - "#)]
    pub trlreval: Option<f32>,

    #[schemars(description = r#"TRADE-DERIVATED NEG VAL RATIO - "#)]
    pub trlrevalr: Option<f32>,

    #[schemars(description = r#"TRANSACTION-COM BKS& OTHER - "#)]
    pub trncbo: Option<f32>,

    #[schemars(description = r#"TRANSACTION-COM BKS& OTHER RATIO - "#)]
    pub trncbor: Option<f32>,

    #[schemars(description = r#"TRANSACTION-FOR COUNTRY - "#)]
    pub trnfc: Option<f32>,

    #[schemars(description = r#"TRANSACTION-FOR COUNTRY & GOVT - "#)]
    pub trnfcfg: Option<f32>,

    #[schemars(description = r#"TRANSACTION-FOR COUNTRY & GOVT RATIO - "#)]
    pub trnfcfgr: Option<f32>,

    #[schemars(description = r#"TRANSACTION-FOREIGN GOVERNMENT - "#)]
    pub trnfg: Option<f32>,

    #[schemars(description = r#"AMT NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000 - "#)]
    pub trnnia: Option<f32>,

    #[schemars(description = r#"AMT NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000 - "#)]
    pub trnniar: Option<f32>,

    #[schemars(description = r#"NUM NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000 - "#)]
    pub trnnin: Option<f32>,

    #[schemars(description = r#"INSTITUTION HAS TRUST POWER - "#)]
    pub trpower: Option<f32>,

    #[schemars(description = r#"TRADE-DERIV POS VAL-DOM - "#)]
    pub trrevald: Option<f32>,

    #[schemars(description = r#"TRADE-DERIV POS VALUE-FOR - "#)]
    pub trrevalf: Option<f32>,

    #[schemars(description = r#"REVALUATION GAINS ON OFF-BALANCE SHEET CONTRACTS - "#)]
    pub trrevalsum: Option<f32>,

    #[schemars(description = r#"REVALUATION GAINS ON OFF-BALANCE SHEET CONTRACTS RATIO - "#)]
    pub trrevalsumr: Option<f32>,

    #[schemars(description = r#"TOT FIDUCIARY ACCTS-MAN-AMT - "#)]
    pub ttma: Option<f32>,

    #[schemars(description = r#"TOT FIDUCIARY ACCTS-MAN-NUM - "#)]
    pub ttnanum: Option<f32>,

    #[schemars(description = r#"TOT FIDUCIARY ACCTS-NON-MAN-AMT - "#)]
    pub ttnma: Option<f32>,

    #[schemars(description = r#"TOT FIDUCIARY ACCTS-NON-MAN-NUM - "#)]
    pub ttnmnum: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-TOTAL - "#)]
    pub uc: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-TOTAL RATIO - "#)]
    pub ucr: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-COM RE - "#)]
    pub uccomre: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-COM RE RATIO - "#)]
    pub uccomrer: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-SECURED COM RE - "#)]
    pub uccomres: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-SECURED COM RE RATIO - "#)]
    pub uccomresr: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-UNSECURED COM RE - "#)]
    pub uccomreu: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-UNSECURED COM RE RATIO - "#)]
    pub uccomreur: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-CREDIT CARD LINES - "#)]
    pub uccrcd: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-CREDIT CARD LINES RATIO - "#)]
    pub uccrcdr: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-TOTAL LOANS - "#)]
    pub ucln: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-HOME EQUITY LINES - "#)]
    pub ucloc: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-HOME EQUITY LINES RATIO - "#)]
    pub uclocr: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-ALL OTHER - "#)]
    pub ucother: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-ALL OTHER RATIO - "#)]
    pub ucotherr: Option<f32>,

    #[schemars(description = r#"UNUSED COM-OVER 1 YR-RC-R COL A - "#)]
    pub ucover1: Option<f32>,

    #[schemars(description = r#"UNUSED COM-OVER 1 YR-RC-R COL A RATIO - "#)]
    pub ucover1r: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-SEC UNDERWRITING - "#)]
    pub ucsc: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT-SEC UNDERWRITING RATIO - "#)]
    pub ucscr: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT FOR SECUR. - AUTO - "#)]
    pub ucszauto: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT FOR SECUR. - CI - "#)]
    pub ucszci: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT FOR SECUR. - CON - "#)]
    pub ucszcon: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT FOR SECUR. - CRCD - "#)]
    pub ucszcrcd: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT FOR SECUR. - HEL - "#)]
    pub ucszhel: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT FOR SECUR. - OTH - "#)]
    pub ucszoth: Option<f32>,

    #[schemars(description = r#"UNUSED COMMIT FOR SECUR. - RES - "#)]
    pub ucszres: Option<f32>,

    #[schemars(description = r#"UNEARNED INCOME-FOR - "#)]
    pub unincfor: Option<f32>,

    #[schemars(description = r#"UNEARNED INCOME-FOR RATIO - "#)]
    pub unincforr: Option<f32>,

    #[schemars(description = r#"VOLATILE LIABILITIES - "#)]
    pub voliab: Option<f32>,

    #[schemars(description = r#"VOLATILE LIABILITIES RATIO - "#)]
    pub voliabr: Option<f32>,

    #[schemars(description = r#"ZIP CODE - "#)]
    pub zip: Option<f32>,

    #[schemars(description = r#"NONMORTGAGE LOANS IN PROCESS - "#)]
    pub lipnmtg: Option<f32>,

    #[schemars(description = r#"UNAMORTIZED YIELD ADJ-NONMTG LNS - "#)]
    pub uyanmtg: Option<f32>,

    #[schemars(description = r#"LOAN & LEASE INCOME - "#)]
    pub ilnls: Option<f32>,

    #[schemars(description = r#"BANKS UNIT - "#)]
    pub unit: Option<f32>,

    #[schemars(description = r#"PRE-TAX NET INCOME OPERATING INCOME - "#)]
    pub ptaxnetinc: Option<f32>,

    #[schemars(description = r#"PRE-TAX NET INCOME OPERATING INCOME RATIO - "#)]
    pub ptaxnetincr: Option<f32>,

    #[schemars(description = r#"PRE-TAX NET INCOME OPERATING INCOME QUARTERLY - "#)]
    pub ptaxnetincq: Option<f32>,

    #[schemars(description = r#"PRE-TAX NET INCOME OPERATING INCOME QUARTERLY RATIO - "#)]
    pub ptaxnetincqr: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST INCOME - "#)]
    pub addnonii: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST INCOME RATIO - "#)]
    pub addnoniir: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST INCOME QUARTERLY - "#)]
    pub addnoniiq: Option<f32>,

    #[schemars(description = r#"ADDITIONAL NONINTEREST INCOME QUARTERLY RATIO - "#)]
    pub addnoniiqr: Option<f32>,

    #[schemars(description = r#"Quarterly average amount of assets purchased under the MMLF and excluded from “Total assets for the leverage ratio.” - "#)]
    pub avmmlf: Option<f32>,

    #[schemars(description = r#"Quarterly average amount of assets purchased under the MMLF and excluded from “Total assets for the leverage ratio.” ratio - "#)]
    pub avmmlfr: Option<f32>,

    #[schemars(description = r#"Quarterly average amount of PPP loans pledged to the PPPLF and excluded from “Total assets for the leverage ratio.” - "#)]
    pub avpppplg: Option<f32>,

    #[schemars(description = r#"Quarterly average amount of PPP loans pledged to the PPPLF and excluded from “Total assets for the leverage ratio.” ratio - "#)]
    pub avpppplgr: Option<f32>,

    #[schemars(description = r#"Outstanding balance of assets purchased under the Money Market Mutual Fund Liquidity Facility (MMLF). - "#)]
    pub mmlfbal: Option<f32>,

    #[schemars(description = r#"Outstanding balance of assets purchased under the Money Market Mutual Fund Liquidity Facility (MMLF) ratio - "#)]
    pub mmlfbalr: Option<f32>,

    #[schemars(description = r#"Outstanding balance under the PPPLF with a remaining maturity of more than one year - "#)]
    pub ppplfov1: Option<f32>,

    #[schemars(description = r#"Outstanding balance under the PPPLF with a remaining maturity of more than one year ratio - "#)]
    pub ppplfov1r: Option<f32>,

    #[schemars(description = r#"Outstanding balance of PPP loans - "#)]
    pub ppplnbal: Option<f32>,

    #[schemars(description = r#"Outstanding balance of PPP loans ratio - "#)]
    pub ppplnbalr: Option<f32>,

    #[schemars(description = r#"Number of PPP loans outstanding - "#)]
    pub ppplnnum: Option<f32>,

    #[schemars(description = r#"Number of PPP loans outstanding ratio - "#)]
    pub ppplnnumr: Option<f32>,

    #[schemars(description = r#"Outstanding balance of PPP loans pledged to the PPPLF - "#)]
    pub ppplnplg: Option<f32>,

    #[schemars(description = r#"Outstanding balance of PPP loans pledged to the PPPLF ratio - "#)]
    pub ppplnplgr: Option<f32>,

    #[schemars(description = r#"Outstanding balance under the PPPLF with a remaining maturity of one year or less - "#)]
    pub ppplf1ls: Option<f32>,

    #[schemars(description = r#"Outstanding balance under the PPPLF with a remaining maturity of one year or less ratio - "#)]
    pub ppplf1lsr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL & INDUSTRIAL LOANS - "#)]
    pub idntcir: Option<f32>,

    #[schemars(description = r#"COMMERCIAL & INDUSTRIAL LOANS QUARTERLY - "#)]
    pub idntciqr: Option<f32>,

    #[schemars(description = r#"LOANS TO INDIVIDUALS - "#)]
    pub idntconr: Option<f32>,

    #[schemars(description = r#"CREDIT CARDS & RELATED PLANS - "#)]
    pub idntcrdr: Option<f32>,

    #[schemars(description = r#"OTHER LOANS TO INDIVIDUALS - "#)]
    pub idntcoor: Option<f32>,

    #[schemars(description = r#"OTHER LOANS TO INDIVIDUALS - "#)]
    pub idntcooqr: Option<f32>,

    #[schemars(description = r#"CREDIT CARDS & RELATED PLANS QUARTERLY - "#)]
    pub idntcrdqr: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub instcnt: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub idntilr: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub idothnii: Option<f32>,

    #[schemars(description = r#"AUTOMOBILE LOANS - "#)]
    pub ntautopr: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOANS - "#)]
    pub ntconotr: Option<f32>,

    #[schemars(description = r#"EARNINGS COVERAGE OF NET LOAN CHARGE-OFFS (X) - "#)]
    pub iderncvr: Option<f32>,

    #[schemars(description = r#"Earnings coverage of net loan charge-offs - "#)]
    pub iderncvqr: Option<f32>,

    #[schemars(description = r#"CASH DIVIDENDS TO NET INCOME (YTD ONLY) - "#)]
    pub eqcdivntinc: Option<f32>,

    #[schemars(description = r#"NOTIONAL AMOUNT OF CREDIT DERIVATIVES - "#)]
    pub nacdir: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE CHG-OFF/COMM RE LN QUARTERLY RATIO - "#)]
    pub ntcomreqr: Option<f32>,

    #[schemars(description = r#"Net Charge-offs All other loans & leases (including farm) Numerator - "#)]
    pub ntallothnum: Option<f32>,

    #[schemars(description = r#"Net Charge-offs All other loans & leases (including farm) denominator - "#)]
    pub ntallothden: Option<f32>,

    #[schemars(description = r#"ALL OTHER LOANS & LEASES (INCLUDING FARM) - "#)]
    pub ntallothr: Option<f32>,

    #[schemars(description = r#"Net Charge-offs All other loans & leases (including farm) - "#)]
    pub ntallothqr: Option<f32>,

    #[schemars(description = r#"Other loans to individuals - "#)]
    pub idnccoor: Option<f32>,

    #[schemars(description = r#"All other loans & leases (including farm ) - "#)]
    pub idncothr: Option<f32>,

    #[schemars(description = r#"COMMERCIAL & INDUSTRIAL LOANS RATIO - "#)]
    pub idnccir: Option<f32>,

    #[schemars(description = r#"LOANS TO INDIVIDUALS RATIO - "#)]
    pub idncconr: Option<f32>,

    #[schemars(description = r#"CREDIT CARDS & RELATED PLANS RATIO - "#)]
    pub idnccrdr: Option<f32>,

    #[schemars(description = r#"AUTOMOBILE LOANS RATIO - "#)]
    pub idncator: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub idntator: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub idntcotr: Option<f32>,

    #[schemars(description = r#"IDDEPINR - "#)]
    pub iddepinr: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub iddivnir: Option<f32>,

    #[schemars(description = r#"OTHER CONSUMER LOANS RATIO - "#)]
    pub idnccotr: Option<f32>,

    #[schemars(description = r#"INTEREST INCOME TO EARNING ASSETS RATIO - "#)]
    pub intincy: Option<f32>,

    #[schemars(description = r#"NONCURRENT LOANS WHICH ARE WHOLLY OR PARTIALLY GUARANTEED BY THE U.S. GOVERNMENT RATIO - "#)]
    pub idncgtpr: Option<f32>,

    #[schemars(description = r#"NET LOANS AND LEASES TO CORE DEPOSITS RATIO - "#)]
    pub idlncorr: Option<f32>,

    #[schemars(description = r#"ID NO CB FLAG - "#)]
    pub idt1cnocb: Option<f32>,

    #[schemars(description = r#"ID NO J CB FLAG - "#)]
    pub idt1jnocb: Option<f32>,

    #[schemars(description = r#"COMMON EQUITY TIER 1 CAPITAL RATIO - "#)]
    pub idt1cer: Option<f32>,

    #[schemars(description = r#"TIER 1 RISK-BASED CAPITAL RATIO - "#)]
    pub idt1rwajr: Option<f32>,

    #[schemars(description = r#"EQUITY SECURITIES NOT HELD FOR TRADING - "#)]
    pub sceqnft: Option<f32>,

    #[schemars(description = r#"PRIV ISSUED RES MORTGAGE-BACKED SECURITIES - "#)]
    pub scrmbpi: Option<f32>,

    #[schemars(description = r#"PRIV ISSUED RES MORTGAGE-BACKED SECURITIES RATIO - "#)]
    pub scrmbpir: Option<f32>,

    #[schemars(description = r#"U.S GOVERNMENT OBLIGATIONS - "#)]
    pub scuso: Option<f32>,

    #[schemars(description = r#"U.S GOVERNMENT OBLIGATIONS RATIOS - "#)]
    pub scusor: Option<f32>,

    #[schemars(description = r#"OTHER COMM MORTGAGE-BACKED SEC - "#)]
    pub sccmos: Option<f32>,

    #[schemars(description = r#"OTHER COMM MORTGAGE-BACKED SEC - "#)]
    pub sccmosr: Option<f32>,

    #[schemars(description = r#"ASSETS HELD IN TRADING ACCOUNTS FOR TFR REPORTERS - "#)]
    pub sctatfr: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES, GROSS - "#)]
    pub lnlsgrs: Option<f32>,

    #[schemars(description = r#"LOANS AND LEASES, GROSS RATIO - "#)]
    pub lnlsgrsr: Option<f32>,

    #[schemars(description = r#"ALL OTH ASSETS - "#)]
    pub aoa: Option<f32>,

    #[schemars(description = r#"ALL OTH ASSETS RATIO - "#)]
    pub aoar: Option<f32>,

    #[schemars(description = r#"PERCENTAGE INSURED ESTIMATED - "#)]
    pub estins: Option<f32>,

    #[schemars(description = r#"PERCENTAGE INSURED ESTIMATED RATIO - "#)]
    pub estinsr: Option<f32>,

    #[schemars(description = r#"P/D 30-89 REAL ESTATE LOANS IN DOMESTIC OFFICES - "#)]
    pub p3relndo: Option<f32>,

    #[schemars(description = r#"P/D 30-89 REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO - "#)]
    pub p3relndor: Option<f32>,

    #[schemars(description = r#"90+ REAL ESTATE LOANS IN DOMESTIC OFFICES - "#)]
    pub p9relndo: Option<f32>,

    #[schemars(description = r#"90+ REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO - "#)]
    pub p9relndor: Option<f32>,

    #[schemars(description = r#"90+ REAL ESTATE LOANS IN DOMESTIC OFFICES - "#)]
    pub narelndo: Option<f32>,

    #[schemars(description = r#"90+ REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO - "#)]
    pub narelndor: Option<f32>,

    #[schemars(description = r#"State and County Nunber - "#)]
    pub stcnty: Option<String>,

    #[schemars(description = r#"Metropolitan Statistical Area - "#)]
    pub cbsa: Option<String>,

    #[schemars(description = r#"Date of Deposit Insurance (Search-Eligible) - This field can be used for search and filtering."#)]
    pub insdate: Option<String>,

    #[schemars(description = r#"Last Structure Change Process Date (Search-Eligible) - This field can be used for search and filtering."#)]
    pub upddate: Option<String>,

    #[schemars(description = r#"Total Assets Ratio - "#)]
    pub assetr: Option<f32>,

    #[schemars(description = r#"AVG TOTAL ASSETS - "#)]
    pub avasset: Option<f32>,

    #[schemars(description = r#"BROKERED DEP-INSURED-LARGE - "#)]
    pub broinslg: Option<f32>,

    #[schemars(description = r#"RC-R TOTAL ADJ & DED COM EQ T1 - "#)]
    pub ct1ajtot: Option<f32>,

    #[schemars(description = r#"RC-R COM EQUITY T1 BEFORE ADJ - "#)]
    pub ct1badj: Option<f32>,

    #[schemars(description = r#"TOTAL DEPOSITS-CAVG2 - "#)]
    pub dep2: Option<f32>,

    #[schemars(description = r#"TOTAL DEPOSITS-CAVG5 - "#)]
    pub dep5: Option<f32>,

    #[schemars(description = r#"INTEREST-BEARING-DEP-Y1 - "#)]
    pub depiy1: Option<f32>,

    #[schemars(description = r#"INT EXPENSE TIME CD GT $250 - "#)]
    pub ecd100: Option<f32>,

    #[schemars(description = r#"INT EXP TIME CD GT $250 - "#)]
    pub ecd100a: Option<f32>,

    #[schemars(description = r#"INT EXP TIME CD GT $250 - "#)]
    pub ecd100q: Option<f32>,

    #[schemars(description = r#"FED FUNDS & REPO INT EXPENSE-ANN - "#)]
    pub efreppa: Option<f32>,

    #[schemars(description = r#"INT EXP TIME CD LE $250 - "#)]
    pub eothtima: Option<f32>,

    #[schemars(description = r#"INT EXPENSE TIME CD LE $250 - "#)]
    pub eothtime: Option<f32>,

    #[schemars(description = r#"INT EXP TIME CD LE $250 - "#)]
    pub eothtimq: Option<f32>,

    #[schemars(description = r#"UNDIVIDED PROFITS - "#)]
    pub equpgr: Option<f32>,

    #[schemars(description = r#"NONTRANSACTION SAV ACCTS INT EXP - "#)]
    pub esavdp: Option<f32>,

    #[schemars(description = r#"NONTRANSACT SAV ACCT INT EXT-ANN - "#)]
    pub esavdpa: Option<f32>,

    #[schemars(description = r#"NONTRANSACT SAV ACCT INT EXP-QTR - "#)]
    pub esavdpq: Option<f32>,

    #[schemars(description = r#"SUBORDINATED NOTES INT EXP-ANN - "#)]
    pub esubnda: Option<f32>,

    #[schemars(description = r#"TRANSACTION ACCOUNTS INT EXPENSE - "#)]
    pub etrandep: Option<f32>,

    #[schemars(description = r#"TRANSACTION ACCOUNTS INT EXP-ANN - "#)]
    pub etrandpa: Option<f32>,

    #[schemars(description = r#"TRANSACTION ACCOUNTS INT EXP-QTR - "#)]
    pub etrandpq: Option<f32>,

    #[schemars(description = r#"TT&L & OTHER BORROW INT EXP-ANN - "#)]
    pub ettlotba: Option<f32>,

    #[schemars(description = r#"TT&L & OTHER BORROW INT EXP-QTR - "#)]
    pub ettlotbq: Option<f32>,

    #[schemars(description = r#"FEDERAL FUNDS PURCHASED - "#)]
    pub ffpur: Option<f32>,

    #[schemars(description = r#"INC BEFORE INC TAXS & DISC-ANN - "#)]
    pub ibeftxa: Option<f32>,

    #[schemars(description = r#"AVAILABLE-FOR-SALE SECS G/L - "#)]
    pub iglsca: Option<f32>,

    #[schemars(description = r#"AVAILABLE-FOR-SALE SEC G/L-QTR - "#)]
    pub iglscaq: Option<f32>,

    #[schemars(description = r#"HELD-TO-MATURITY SECS G/L - "#)]
    pub iglsch: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-ANN - "#)]
    pub ilna: Option<f32>,

    #[schemars(description = r#"LOAN & LEASE INCOME-ANN - "#)]
    pub ilnlsa: Option<f32>,

    #[schemars(description = r#"LOAN & LEASE INCOME-QTR - "#)]
    pub ilnlsq: Option<f32>,

    #[schemars(description = r#"TAX-EXEMPT LN & LS INT INC-ANN - "#)]
    pub ilnlsxa: Option<f32>,

    #[schemars(description = r#"TAX-EXEMPT LN & LS INT INC-QTR - "#)]
    pub ilnlsxq: Option<f32>,

    #[schemars(description = r#"MUNICIPAL LOAN INCOME-QTR - "#)]
    pub ilnmuniq: Option<f32>,

    #[schemars(description = r#"LOAN INCOME-QTR - "#)]
    pub ilnq: Option<f32>,

    #[schemars(description = r#"TOTAL SECURITY INCOME-ANN - "#)]
    pub isca: Option<f32>,

    #[schemars(description = r#"SERVICE CHARGE ON DEP ACCTS-ANN - "#)]
    pub iserchga: Option<f32>,

    #[schemars(description = r#"APPLICABLE INCOME TAXES-ANN - "#)]
    pub itaxa: Option<f32>,

    #[schemars(description = r#"APPLICABLE INCOME TAXES-QTR-ANN - "#)]
    pub itaxqa: Option<f32>,

    #[schemars(description = r#"CONSTR & LAND DEV LNS/TIER 1 - "#)]
    pub lncdt1r: Option<f32>,

    #[schemars(description = r#"C&I LOANS/TIER 1 - "#)]
    pub lncit1r: Option<f32>,

    #[schemars(description = r#"CONSUMER LOANS/TIER 1 - "#)]
    pub lncont1r: Option<f32>,

    #[schemars(description = r#"ALLOWANCE FOR LOAN AND LEASES - "#)]
    pub lnlsres: Option<f32>,

    #[schemars(description = r#"RE AGRICULTURAL-CAVG5 - "#)]
    pub lnreag5: Option<f32>,

    #[schemars(description = r#"RE LOANS/TIER 1 - "#)]
    pub lnrert1r: Option<f32>,

    #[schemars(description = r#"TOTAL N/C-RE*FARMLAND - "#)]
    pub ncreag: Option<f32>,

    #[schemars(description = r#"N/C 1-4 FAMILY CONSTRUCTION LOAN - "#)]
    pub ncrecnfm: Option<f32>,

    #[schemars(description = r#"N/C OTHER CONSTRUCT & LAND DEV - "#)]
    pub ncrecnot: Option<f32>,

    #[schemars(description = r#"N/C OTHER NONFARM NONRES RE LN - "#)]
    pub ncrenrot: Option<f32>,

    #[schemars(description = r#"N/C OWN-OCCUPIED NONFARM NONRES - "#)]
    pub ncrenrow: Option<f32>,

    #[schemars(description = r#"N/C 1-4 FAM JR LN/1-4 FAM JR LN - "#)]
    pub ncrers2r: Option<f32>,

    #[schemars(description = r#"N/C RE 1-4 FAM JUNIOR LIEN - "#)]
    pub ncrersf2: Option<f32>,

    #[schemars(description = r#"N/C RE 1-4 FAM FIRST LIEN - "#)]
    pub ncrersfm: Option<f32>,

    #[schemars(description = r#"N/C 1-4 FAM 1STLN/1-4 FAM IST LN - "#)]
    pub ncrersfr: Option<f32>,

    #[schemars(description = r#"NC RESTRUCT LOANS EXCL 1-4 FM - "#)]
    pub ncrslnls: Option<f32>,

    #[schemars(description = r#"NET OPERATING INCOME-QTR - "#)]
    pub noiq: Option<f32>,

    #[schemars(description = r#"AG LOAN NET CHARGE-OFFS-QTR-ANN - "#)]
    pub ntagqa: Option<f32>,

    #[schemars(description = r#"AG LN NET CHARGE-OFFS ANN*SM BKS - "#)]
    pub ntagsma: Option<f32>,

    #[schemars(description = r#"AG LOAN NET-CHG-QTR-ANN*SMALL BK - "#)]
    pub ntagsmqa: Option<f32>,

    #[schemars(description = r#"COMMERCIAL LOAN NET-CHG-QTR-ANN - "#)]
    pub ntciqa: Option<f32>,

    #[schemars(description = r#"COMMERCIAL RE LN NET CHARGE-OFFS - "#)]
    pub ntcomre: Option<f32>,

    #[schemars(description = r#"COMML RE NET-CHARGE-OFF-QTR-ANN - "#)]
    pub ntcomrqa: Option<f32>,

    #[schemars(description = r#"CONSUMER LN NET-CHG-QTR-ANN - "#)]
    pub ntconqa: Option<f32>,

    #[schemars(description = r#"CREDIT CARD LN NET-CHG-QTR-ANN - "#)]
    pub ntcrcdqa: Option<f32>,

    #[schemars(description = r#"RETAINED EARNINGS- BANK- QTR - "#)]
    pub ntirtq: Option<f32>,

    #[schemars(description = r#"Time Deposits Less Than Or Equal To insurance Limit - "#)]
    pub ntrcdsmj: Option<f32>,

    #[schemars(description = r#"FARMLAND RE LN NET-CHG-ANN - "#)]
    pub ntreaga: Option<f32>,

    #[schemars(description = r#"FARM RE LN NET CHRG-OFF-QTR-ANN - "#)]
    pub ntreagqa: Option<f32>,

    #[schemars(description = r#"OTHER BORROWED FUNDS - "#)]
    pub obor: Option<f32>,

    #[schemars(description = r#"OTHER BORROWED FUNDS-CAVG2 - "#)]
    pub obor2: Option<f32>,

    #[schemars(description = r#"OTHER BORROWED FUNDS-CAVG5 - "#)]
    pub obor5: Option<f32>,

    #[schemars(description = r#"OTH BOR FHLB-OVER 3 YRS - "#)]
    pub othbfh03: Option<f32>,

    #[schemars(description = r#"OTH BOR. FHLB-1 TO 3 YRS - "#)]
    pub othbfh13: Option<f32>,

    #[schemars(description = r#"30-89 DAYS P/D-COMMERCIAL RE - "#)]
    pub p3comre: Option<f32>,

    #[schemars(description = r#"30-89 PAST DUE CONST RE/CONST RE - "#)]
    pub p3reconr: Option<f32>,

    #[schemars(description = r#"30-89 P/D 1-4FAM JR/1-4 FAM JR - "#)]
    pub p3rers2r: Option<f32>,

    #[schemars(description = r#"RETAINED EARNINGS - RBC - "#)]
    pub rbcequp: Option<f32>,

    #[schemars(description = r#"TIER 1 CAPITAL - REPORTED - "#)]
    pub rbct1w: Option<f32>,

    #[schemars(description = r#"REPURCHASE AGREEMENTS - "#)]
    pub repopur: Option<f32>,

    #[schemars(description = r#"SECURITIES-CAVG2 - "#)]
    pub sc2: Option<f32>,

    #[schemars(description = r#"SECURITIES-CAVG5 - "#)]
    pub sc5: Option<f32>,

    #[schemars(description = r#"MUNICIPAL SECURITIES -AA - "#)]
    pub scmuniaa: Option<f32>,

    #[schemars(description = r#"MUNICIPAL SECURITIES -AF - "#)]
    pub scmuniaf: Option<f32>,

    #[schemars(description = r#"MUNICIPAL SECURITIES -HA - "#)]
    pub scmuniha: Option<f32>,

    #[schemars(description = r#"MUNICIPAL SECURITIES -HF - "#)]
    pub scmunihf: Option<f32>,

}

#[derive(Clone,Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct FinancialsResponse {
    pub data: Vec<FinancialsProperties>,
    pub meta: ResponseMeta,
    pub total: u64,
}

impl IntoContents for FinancialsResponse {
    fn into_contents(self) -> Vec<Content> {
        // Convert the response into a Vec<Content> as expected by MCP
        // Panics only if serialization fails, which should be impossible for valid structs
        vec![Content::json(self).expect("Failed to serialize FinancialsResponse to Content")]
    }
}

/// FDIC BankFind API `/financials` endpoint handler
/// Get Financial Information for FDIC Insured Institutions
/// Returns financial information for financial institutions
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter criteria that refines the records included in the result. All values must be entered in UPPERCASE.
Examples:  
* Filter data by the numeric range  
`ASSET:&#91;1000 TO 9999&#93;`
CERT:14 - `fields` (String, optional): Comma delimited list of fields with quarterly financial data to return. All values must be entered in UPPERCASE.
CERT,REPDTE,ASSET,DEP - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
REPDTE - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
DESC - `limit` (i32, optional): The number of records to return. Default is 10 and maximum is 10,000. However, if the fields request is for more than 250 fields (variables), the maximum limit is 500 to ensure the request is successful. - `offset` (i32, optional): The offset of page to return. - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE.
CERT - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE.
REPDTE - `agg_sum_fields` (String, optional): The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE.
ASSET - `agg_limit` (i32, optional): The limit on how many aggregated results will be displayed - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /financials
Parameters: FinancialsParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: Financials"#]
pub async fn financials_handler(config: &FdicApiConfig, params: &FinancialsParameters) -> Result<CallToolResult, rmcp::Error> {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "financials",
        method = "GET",
        path = "/financials",
        params = serde_json::to_string(params).unwrap()
    );

    let resp = get_fdic_bank_find_mcp_response::<_, FinancialsResponse>(config, params).await;

    // Log outgoing FDIC API request as structured JSON
    resp.and_then(|r| r.into_call_tool_result())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_parameters_struct_serialization() {
        let params = FinancialsParameters {
            common: CommonParameters::default(),
            agg_by: None,
            agg_term_fields: None,
            agg_sum_fields: None,
            agg_limit: None,
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }

    #[test]
    fn test_properties_struct_serialization() {
        let props = FinancialsProperties {
            actevt: None,
            asset: None,
            branch: None,
            callform: None,
            cb: None,
            cbsadiv: None,
            cbsaname: None,
            stmult: None,
            address: None,
            cblrind: None,
            cd1t3: None,
            cd1t3r: None,
            cd3les: None,
            cd3lesr: None,
            cd3less: None,
            cd3lessr: None,
            cdov3: None,
            cdov3r: None,
            cdov3s: None,
            cdov3sr: None,
            cd3t12: None,
            cd3t12r: None,
            cd3t12s: None,
            cd3t12sr: None,
            cd1t3s: None,
            cd1t3sr: None,
            cert: None,
            certcons: None,
            cityhcr: None,
            clcode: None,
            closed: None,
            cmsa: None,
            cntryalp: None,
            cntrynum: None,
            cntynum: None,
            csa: None,
            denovo: None,
            dep: None,
            depr: None,
            depdom: None,
            depdomr: None,
            division: None,
            docket: None,
            edgecode: None,
            enttype: None,
            eq: None,
            eq2: None,
            eqr: None,
            failed: None,
            fdicarea: None,
            fdicterr: None,
            fldofdca: None,
            form31: None,
            hctmult: None,
            hctnone: None,
            insagnt2: None,
            insbif: None,
            insdif: None,
            instag: None,
            instcrcd: None,
            inssaif: None,
            minority: None,
            mutual: None,
            namehcr: None,
            netinc: None,
            netincr: None,
            netincq: None,
            netincqa: None,
            netincqr: None,
            offdom: None,
            offfor: None,
            offoa: None,
            parcert: None,
            l_repdte: None,
            repdte_raw: None,
            repdte: None,
            repyear: None,
            risdate: None,
            roa: None,
            roaptx: None,
            roaptxq: None,
            roaq: None,
            roe: None,
            roeq: None,
            rssdhcr: None,
            specgrp: None,
            specgrpdesc: None,
            stalphcr: None,
            subchaps: None,
            tract: None,
            trust: None,
            acept: None,
            active: None,
            bkclass: None,
            bkprem: None,
            bkpremr: None,
            bro: None,
            bror: None,
            callym: None,
            chbal: None,
            chbalr: None,
            chbali: None,
            chbalir: None,
            chrtagnt: None,
            conserve: None,
            crlnls: None,
            crlnlsr: None,
            crlnlsq: None,
            crlnlsqr: None,
            cusli: None,
            ddt: None,
            ddtr: None,
            depfor: None,
            depforr: None,
            depi: None,
            depifor: None,
            depiforr: None,
            depipccf: None,
            depipccfr: None,
            depipcf: None,
            depni: None,
            depnifor: None,
            depniforr: None,
            drlnls: None,
            drlnlsr: None,
            drlnlsq: None,
            drlnlsqr: None,
            eamintan: None,
            eamintanr: None,
            eamintq: None,
            eamintqr: None,
            edep: None,
            edepdom: None,
            edepdomr: None,
            edepdomq: None,
            edepdomqr: None,
            edepfor: None,
            edepforr: None,
            edepforq: None,
            edepforqr: None,
            efhlbadv: None,
            efrepp: None,
            efreppr: None,
            efreppq: None,
            efreppqr: None,
            eintexp: None,
            eintexpr: None,
            eintxq: None,
            eintxqa: None,
            eintexpa: None,
            eintxqr: None,
            elnatr: None,
            elnatrr: None,
            elnatq: None,
            elnatqa: None,
            elnatqr: None,
            elnlosq: None,
            nttotq: None,
            elnlos: None,
            emtgls: None,
            addnonintexp: None,
            addnonintexpr: None,
            addnonintexpq: None,
            addnonintexpqr: None,
            eothnint: None,
            eothnintr: None,
            eothninq: None,
            eothninqr: None,
            epremagg: None,
            epremaggr: None,
            epremagq: None,
            epremagqr: None,
            eqcdiv: None,
            eqcdivr: None,
            eqcdivc: None,
            eqcdivcr: None,
            eqcdivp: None,
            eqcdivpr: None,
            eqcdivq: None,
            eqcdivqr: None,
            eqcfcta: None,
            eqconsub: None,
            eqcs: None,
            eqcsr: None,
            eqnwcert: None,
            eqothcc: None,
            eqpp: None,
            eqppr: None,
            eqsur: None,
            eqsurr: None,
            equp: None,
            equptot: None,
            equptotr: None,
            esal: None,
            esalr: None,
            esalq: None,
            esalqr: None,
            esubnd: None,
            ettlotbo: None,
            extra: None,
            extrar: None,
            extraq: None,
            extraqr: None,
            fdicdbs: None,
            fdicdbsdesc: None,
            fdicsupv: None,
            fdicsupvdesc: None,
            fed: None,
            feddesc: None,
            fedchrtr: None,
            fldoff: None,
            forchrtr: None,
            formcfr: None,
            frepo: None,
            frepor: None,
            frepp: None,
            freppr: None,
            frsmem: None,
            hctone: None,
            iba: None,
            ibeftax: None,
            ichbal: None,
            ichbalr: None,
            ichbalq: None,
            ichbalqr: None,
            ifrepo: None,
            ifrepor: None,
            ifrepoq: None,
            ifrepoqr: None,
            iglsec: None,
            iglsecr: None,
            iglsecqr: None,
            ilndom: None,
            ilndomr: None,
            ilndomq: None,
            ilndomqr: None,
            ilnfor: None,
            ilnforr: None,
            ilnforq: None,
            ilnforqr: None,
            ils: None,
            ilsr: None,
            ilsq: None,
            ilsqr: None,
            insall: None,
            inscoml: None,
            insfdic: None,
            insnone: None,
            inssave: None,
            instcoml: None,
            instsave: None,
            insttype: None,
            intan: None,
            intanr: None,
            intexpy: None,
            intexpyq: None,
            intinc: None,
            intincr: None,
            intinq: None,
            intinqr: None,
            intinqa: None,
            invsub: None,
            invsuore: None,
            iothfee: None,
            iothii: None,
            iothiir: None,
            iothiiq: None,
            iothiiqr: None,
            irakeogh: None,
            irakeoghr: None,
            isc: None,
            iscr: None,
            iscq: None,
            iscqr: None,
            iserchg: None,
            iserchgr: None,
            itax: None,
            itaxr: None,
            itaxq: None,
            itaxqr: None,
            itrade: None,
            itrader: None,
            itradeq: None,
            itradeqr: None,
            liab: None,
            liabr: None,
            liabeq: None,
            liabeqr: None,
            lipmtg: None,
            llpfdstk: None,
            lnacoth: None,
            lnag: None,
            lnagr: None,
            lnatres: None,
            lnatresj: None,
            lnatresrr: None,
            lnauto: None,
            lnautor: None,
            lnci: None,
            lncir: None,
            lncon: None,
            lnconr: None,
            lnconot1: None,
            lnconoth: None,
            lnconothr: None,
            lncrcd: None,
            lncrcdr: None,
            lncrcdrp: None,
            lndep: None,
            lnfg: None,
            lnfgr: None,
            lnls: None,
            lnlsgr: None,
            lnlsgr2: None,
            lnlsgrj: None,
            lnlsgrr: None,
            lnlsnet: None,
            lnlsnetr: None,
            lnmuni: None,
            lnmunir: None,
            lnotci: None,
            lnotcir: None,
            lnother: None,
            lnsother: None,
            lnsotherr: None,
            lnre: None,
            lnre2: None,
            lnrecon2: None,
            lnremul2: None,
            lnrej: None,
            lnre5: None,
            lnrer: None,
            lnreag: None,
            lnrecon5: None,
            lnreagr: None,
            lnrecons: None,
            lnreconsr: None,
            lnredom: None,
            lnredomr: None,
            lnrefor: None,
            lnreforr: None,
            lnreloc: None,
            lnrelocr: None,
            lnreloc2: None,
            lnreloc5: None,
            lnremult: None,
            lnremul5: None,
            lnremultr: None,
            lnrenres: None,
            lnrenre5: None,
            lnrenre2: None,
            lnrenresr: None,
            lnrepp: None,
            lnreres: None,
            lnreresr: None,
            lnreres2: None,
            lnreres5: None,
            lnresre: None,
            ls: None,
            lsr: None,
            metro: None,
            mi: None,
            micro: None,
            mnrtycde: None,
            mnrtydte: None,
            mtgls: None,
            n: None,
            nalnls: None,
            nc: None,
            nclnls: None,
            netimin: None,
            netiminr: None,
            netiminq: None,
            netiminqr: None,
            netinbm: None,
            netinbmr: None,
            netinbmq: None,
            netinbxa: None,
            netibxqa: None,
            netinbmqr: None,
            newinst: None,
            nfaa: None,
            nim: None,
            nimr: None,
            nimq: None,
            nimqa: None,
            nima: None,
            nimqr: None,
            nm: None,
            nonii: None,
            noniir: None,
            nonix: None,
            nonixr: None,
            nonixq: None,
            nonixqa: None,
            nonixqr: None,
            ns: None,
            ntlnls: None,
            ntlnlscor: None,
            ntlnlsq: None,
            ntlnlsqa: None,
            ntlnlscoqr: None,
            ntr: None,
            ntrr: None,
            ntripc: None,
            ntripcr: None,
            ntrmuni: None,
            ntrmunir: None,
            ntrtime: None,
            ntrtmlg: None,
            ntrtmlgj: None,
            ntrtmlgjr: None,
            ntrtmmed: None,
            ntrtmmedr: None,
            ntrusgov: None,
            ntrusgovr: None,
            ntirta: None,
            nttot: None,
            numemp: None,
            oa: None,
            oakar: None,
            occdist: None,
            occdistdesc: None,
            offdmult: None,
            offndom: None,
            offoth: None,
            offsod: None,
            offstate: None,
            offtot: None,
            offusoa: None,
            oi: None,
            otsdist: None,
            otsregno: None,
            olmin: None,
            ore: None,
            orer: None,
            othbfhlb: None,
            othbfhlbr: None,
            othbor: None,
            othbrf: None,
            othbrfr: None,
            otbfh1l: None,
            otbfh1lr: None,
            otbfh1t3: None,
            otbfh1t3r: None,
            otbfh3t5: None,
            otbfh3t5r: None,
            otbfhov5: None,
            otbfhov5r: None,
            othbfh1l: None,
            othbfh1lr: None,
            otbfhsta: None,
            otbfhstar: None,
            otbot1l: None,
            otbot1lr: None,
            otbot1t3: None,
            otbot1t3r: None,
            otbot3t5: None,
            otbot3t5r: None,
            otbotov5: None,
            otbotov5r: None,
            othbot1l: None,
            othbot1lr: None,
            allothl: None,
            allothlr: None,
            p3lnls: None,
            p9lnls: None,
            qbprcoml: None,
            qbprcomldesc: None,
            qbprsavb: None,
            qbprsavs: None,
            qtrno: None,
            regagnt: None,
            riskterr: None,
            s10t250b: None,
            sasser: None,
            sb: None,
            sc: None,
            scr: None,
            scaa: None,
            schf: None,
            scage: None,
            scaspnha: None,
            scaspnaf: None,
            scaspnsum: None,
            scaspnsumr: None,
            scdeq: None,
            scdomo: None,
            scdomor: None,
            sceq: None,
            scfdeq: None,
            scford: None,
            scfordr: None,
            scmtgbk: None,
            scmtgbkr: None,
            scmuni: None,
            scmunir: None,
            scmv: None,
            scodpc: None,
            scodpcr: None,
            scres: None,
            scus: None,
            scusr: None,
            scusa: None,
            scust: None,
            scustr: None,
            sims_lat: None,
            sims_long: None,
            sl: None,
            sm: None,
            stalp: None,
            stchrtr: None,
            stname: None,
            stnum: None,
            subllpf: None,
            subnd: None,
            sz25: None,
            sz100: None,
            sz100mp: None,
            sz100t3: None,
            sz100t5: None,
            sz100t1b: None,
            sz10bp: None,
            sz1bp: None,
            sz1bt10b: None,
            sz1bt3b: None,
            sz1bt5b: None,
            sz250bp: None,
            sz25t50: None,
            sz300t5: None,
            sz3bt10b: None,
            sz500t1b: None,
            sz50t100: None,
            sz5bp: None,
            tfra: None,
            trade: None,
            tradel: None,
            tradelr: None,
            trader: None,
            trn: None,
            trnr: None,
            trnipc: None,
            trnipcoc: None,
            trnipcocr: None,
            trnmuni: None,
            trnmunir: None,
            trnusgov: None,
            trnusgovr: None,
            trustpwr: None,
            ts: None,
            tsr: None,
            ttl: None,
            ttlotbor: None,
            uninc: None,
            uninum: None,
            usa: None,
            uyamtg: None,
            abcubk: None,
            abcubkr: None,
            abcuoth: None,
            abcuothr: None,
            abcxbk: None,
            abcxbkr: None,
            abcxoth: None,
            abcxothr: None,
            asceoth: None,
            asceothr: None,
            asceres: None,
            asceresr: None,
            asdroth: None,
            asdrothr: None,
            asdrres: None,
            asdrresr: None,
            asset2: None,
            asset5: None,
            assetfor: None,
            asstlt: None,
            asstltr: None,
            astempm: None,
            avassetj: None,
            avassetjr: None,
            broins: None,
            broinsr: None,
            callymd: None,
            chbalfor: None,
            chbalni: None,
            chbalnir: None,
            chcic: None,
            chcicr: None,
            chcoin: None,
            chcoinr: None,
            chfla: None,
            chflq: None,
            chfrb: None,
            chfrbr: None,
            chitem: None,
            chitemr: None,
            chnus: None,
            chnusr: None,
            chnusfbk: None,
            chus: None,
            chusr: None,
            chusfbk: None,
            city: None,
            coredep: None,
            coredepr: None,
            crag: None,
            cragr: None,
            cragq: None,
            cragqr: None,
            cragsm: None,
            cragsmr: None,
            cragsmq: None,
            cragsmqr: None,
            crauto: None,
            crautor: None,
            crautoq: None,
            crautoqr: None,
            crci: None,
            crcir: None,
            crciq: None,
            crciqr: None,
            crcinus: None,
            crcinusr: None,
            crcinusq: None,
            crcinusqr: None,
            crcon: None,
            crconr: None,
            crconq: None,
            crconqr: None,
            crconoth: None,
            crconothr: None,
            crconotq: None,
            crconotqr: None,
            crcrcd: None,
            crcrcdr: None,
            crcrcdq: None,
            crcrcdqr: None,
            crdep: None,
            crdepr: None,
            crdepq: None,
            crdepqr: None,
            crdepnus: None,
            crdepnusr: None,
            crdepnuq: None,
            crdepnuqr: None,
            crforgv: None,
            crforgvr: None,
            crforgvq: None,
            crforgvqr: None,
            crls: None,
            crlsr: None,
            crlsq: None,
            crlsqr: None,
            crother: None,
            crotherr: None,
            crothq: None,
            crothqr: None,
            crre: None,
            crrer: None,
            crreq: None,
            crreqr: None,
            crreag: None,
            crreagr: None,
            crreagq: None,
            crreagqr: None,
            crrecnfm: None,
            crrecnot: None,
            crreconq: None,
            crreconqr: None,
            crrecons: None,
            crreconsr: None,
            crrefor: None,
            crreforr: None,
            crreforq: None,
            crreforqr: None,
            crreloc: None,
            crrelocr: None,
            crrelocq: None,
            crrelocqr: None,
            crremulq: None,
            crremulqr: None,
            crremult: None,
            crremultr: None,
            crrenres: None,
            crrenresr: None,
            crrenrot: None,
            crrenrow: None,
            crrenrsq: None,
            crrenrsqr: None,
            crrenus: None,
            crrenusr: None,
            crrenusq: None,
            crrenusqr: None,
            crreres: None,
            crreresr: None,
            crreresq: None,
            crreresqr: None,
            crrersf2: None,
            crrersf2r: None,
            crrers2q: None,
            crrers2qr: None,
            crrersfm: None,
            crrersfmr: None,
            crrersfq: None,
            crrersfqr: None,
            crreoffdom: None,
            crreoffdomr: None,
            crreoffdomq: None,
            crreoffdomqr: None,
            ctderben: None,
            ctdergty: None,
            depbefex: None,
            depcsbq: None,
            depcsbqr: None,
            depdastr: None,
            depfbkf: None,
            depfbkfr: None,
            depfgovf: None,
            depfgovfr: None,
            depidom: None,
            depidomr: None,
            depins: None,
            depinsr: None,
            deplgamt: None,
            deplgamtr: None,
            deplgb: None,
            deplgra: None,
            deplgrar: None,
            deplgrn: None,
            deplsnb: None,
            deplsnbr: None,
            depnidom: None,
            depnidomr: None,
            depsmamt: None,
            depsmamtr: None,
            depsmb: None,
            depsmra: None,
            depsmrar: None,
            depsmrn: None,
            depallex: None,
            depuna: None,
            depunar: None,
            depunins: None,
            depusbkf: None,
            depusbkfr: None,
            depusmf: None,
            depusmfr: None,
            drag: None,
            dragr: None,
            dragq: None,
            dragqr: None,
            dragsm: None,
            dragsmr: None,
            dragsmq: None,
            dragsmqr: None,
            drauto: None,
            drautor: None,
            drautoq: None,
            drautoqr: None,
            drci: None,
            drcir: None,
            drciq: None,
            drciqr: None,
            drcinus: None,
            drcinusr: None,
            drcinusq: None,
            drcinusqr: None,
            drcon: None,
            drconr: None,
            drconq: None,
            drconqr: None,
            drconoth: None,
            drconothr: None,
            drconotq: None,
            drconotqr: None,
            drcrcd: None,
            drcrcdr: None,
            drcrcdq: None,
            drcrcdqr: None,
            drdep: None,
            drdepr: None,
            drdepq: None,
            drdepqr: None,
            drdepnus: None,
            drdepnusr: None,
            drdepnuq: None,
            drdepnuqr: None,
            drforgv: None,
            drforgvr: None,
            drforgvq: None,
            drforgvqr: None,
            drls: None,
            drlsr: None,
            drlsq: None,
            drlsqr: None,
            drother: None,
            drotherr: None,
            drothq: None,
            drothqr: None,
            drre: None,
            drrer: None,
            drreq: None,
            drreqr: None,
            drreag: None,
            drreagr: None,
            drreagq: None,
            drreagqr: None,
            drrecnfm: None,
            drrecnot: None,
            drreconq: None,
            drreconqr: None,
            drrecons: None,
            drreconsr: None,
            drrefor: None,
            drreforr: None,
            drreforq: None,
            drreforqr: None,
            drreloc: None,
            drrelocr: None,
            drrelocq: None,
            drrelocqr: None,
            drremulq: None,
            drremulqr: None,
            drremult: None,
            drremultr: None,
            drrenres: None,
            drrenresr: None,
            drrenrot: None,
            drrenrow: None,
            drrenrsq: None,
            drrenrsqr: None,
            drrenus: None,
            drrenusr: None,
            drrenusq: None,
            drrenusqr: None,
            drreres: None,
            drreresr: None,
            drreresq: None,
            drreresqr: None,
            drrersf2: None,
            drrersf2r: None,
            drrers2q: None,
            drrers2qr: None,
            drrersfm: None,
            drrersfmr: None,
            drrersfq: None,
            drrersfqr: None,
            drreoffdom: None,
            drreoffdomr: None,
            drreoffdomq: None,
            drreoffdomqr: None,
            edcm: None,
            eeff: None,
            eeffq: None,
            eeffr: None,
            eeffqr: None,
            effdate: None,
            eintgw: None,
            eintgwr: None,
            eintgwq: None,
            eintgwqr: None,
            eintoth: None,
            eintothr: None,
            eintothq: None,
            eintothqr: None,
            elnantr: None,
            elnatra: None,
            elnatry: None,
            elnatryq: None,
            enceauto: None,
            enceautor: None,
            enceci: None,
            encecir: None,
            encecon: None,
            enceconr: None,
            enceoth: None,
            enceothr: None,
            enceres: None,
            enceresr: None,
            eothint: None,
            eothintr: None,
            eothintq: None,
            eothintqr: None,
            eq5: None,
            eqcbhctr: None,
            eqcbhctrr: None,
            eqccompi: None,
            eqccompir: None,
            eqcdiva: None,
            eqcmrg: None,
            eqcmrgr: None,
            eqcprev: None,
            eqcprevr: None,
            eqcrest: None,
            eqcrestr: None,
            eqcstkrx: None,
            eqcstkrxr: None,
            eqcsxq: None,
            eqcsxqr: None,
            eqctrstx: None,
            eqctrstxr: None,
            eqtot: None,
            eqtotr: None,
            eqv: None,
            ernast: None,
            ernast2: None,
            ernast5: None,
            ernastr: None,
            estymd: None,
            endefymd: None,
            org_end_num_dte: None,
            ettlotmg: None,
            formtfr: None,
            fx: None,
            fxffc: None,
            fxnvs: None,
            fxpoc: None,
            fxspot: None,
            fxwoc: None,
            ibeftxq: None,
            ibefxtr: None,
            ibefxtrr: None,
            ibefxtrq: None,
            ieff: None,
            ieffq: None,
            ibefxtrqr: None,
            ifiduc: None,
            ifiducr: None,
            ifiducq: None,
            ifiducqr: None,
            iglcmex: None,
            iglcmexr: None,
            iglcmexq: None,
            iglcmexqr: None,
            iglcrex: None,
            iglcrexr: None,
            iglcrexq: None,
            iglcrexqr: None,
            igledex: None,
            igledexr: None,
            igledexq: None,
            igledexqr: None,
            iglfxex: None,
            iglfxexr: None,
            iglfxexq: None,
            iglfxexqr: None,
            iglrtex: None,
            iglrtexr: None,
            iglrtexq: None,
            iglrtexqr: None,
            iglsecq: None,
            igltrad: None,
            igltradr: None,
            igltrdq: None,
            igltrdqr: None,
            iinscom: None,
            iinscomr: None,
            iinscomq: None,
            iinscomqr: None,
            iinsoth: None,
            iinsothr: None,
            iinsothq: None,
            iinsothqr: None,
            iinsund: None,
            iinsundr: None,
            iinsundq: None,
            iinsundqr: None,
            iinvfee: None,
            iinvfeer: None,
            iinvfeeq: None,
            iinvfeeqr: None,
            insagnt1: None,
            intangcc: None,
            intangw: None,
            intangwr: None,
            intanmsr: None,
            intanmsrr: None,
            intanoth: None,
            intanothr: None,
            intincyq: None,
            intinca: None,
            iotnii: None,
            iotniir: None,
            iotniiq: None,
            iotniiqr: None,
            isecz: None,
            iseczr: None,
            iseczq: None,
            iseczqr: None,
            iserchgq: None,
            iserchgqr: None,
            iserfee: None,
            iserfeer: None,
            iserfeeq: None,
            iserfeeqr: None,
            ivencap: None,
            ivencapr: None,
            ivencapq: None,
            ivencapqr: None,
            lag: None,
            lagr: None,
            lci: None,
            lcir: None,
            lcon: None,
            lconr: None,
            liabfor: None,
            lnag1: None,
            lnag1r: None,
            lnag2: None,
            lnag2r: None,
            lnag3: None,
            lnag3r: None,
            lnag4: None,
            lnag4r: None,
            lnag5: None,
            lnag22: None,
            lnag1n: None,
            lnag1nr: None,
            lnag2n: None,
            lnag2nr: None,
            lnag3n: None,
            lnag3nr: None,
            lnag4n: None,
            lnag4nr: None,
            lnagfor: None,
            lnagforr: None,
            lnatresr: None,
            lnauto2: None,
            lnauto5: None,
            lnci1: None,
            lnci1r: None,
            lnci2: None,
            lnci2r: None,
            lnci3: None,
            lnci3r: None,
            lnci4: None,
            lnci4r: None,
            lnci5: None,
            lnci22: None,
            lnci1n: None,
            lnci1nr: None,
            lnci2n: None,
            lnci2nr: None,
            lnci3n: None,
            lnci3nr: None,
            lnci4n: None,
            lnci4nr: None,
            lncifor: None,
            lnciforr: None,
            lncinus: None,
            lncinusf: None,
            lncinusfr: None,
            lncomre: None,
            lncomrer: None,
            lncomre2: None,
            lncomre5: None,
            lncon2: None,
            lncon5: None,
            lnconfor: None,
            lnconforr: None,
            lnconorp: None,
            lnconot2: None,
            lnconot5: None,
            lnconrp: None,
            lnconrpr: None,
            lncontra: None,
            lncontrar: None,
            lncrcd2: None,
            lncrcd5: None,
            lndepac: None,
            lndepacd: None,
            lndepaobk: None,
            lndepaobkr: None,
            lndepcb: None,
            lndepcbf: None,
            lndepcbfr: None,
            lndepfc: None,
            lndepfcf: None,
            lndepfcfr: None,
            lndepfus: None,
            lndepus: None,
            lndepusb: None,
            lndepusf: None,
            lndepusfr: None,
            lnexamt: None,
            lnexamtr: None,
            lnfgfor: None,
            lnfgforr: None,
            lnlsdepr: None,
            lnlsfor: None,
            lnlsforr: None,
            lnlsgr5: None,
            lnlsgrf: None,
            lnlsgrfr: None,
            lnlsntv: None,
            lnlsnqr: None,
            lnlssale: None,
            lnlssaler: None,
            lnpledge: None,
            lnpledger: None,
            lnmunif: None,
            lnmunifr: None,
            lnot1t3: None,
            lnot1t3r: None,
            lnot3les: None,
            lnot3lesr: None,
            lnot3t5: None,
            lnot3t5r: None,
            lnot3t12: None,
            lnot3t12r: None,
            lnot5t15: None,
            lnot5t15r: None,
            lnotci2: None,
            lnotci5: None,
            lnotherf: None,
            lnotherfr: None,
            lnotov15: None,
            lnotov15r: None,
            lnreag1: None,
            lnreag1r: None,
            lnreag2: None,
            lnreag2r: None,
            lnreag3: None,
            lnreag3r: None,
            lnreag4: None,
            lnreag4r: None,
            lnreag1n: None,
            lnreag1nr: None,
            lnreag2n: None,
            lnreag2nr: None,
            lnreag3n: None,
            lnreag3nr: None,
            lnreag4n: None,
            lnreag4nr: None,
            lnrecnfm: None,
            lnrecnfmr: None,
            lnrecnot: None,
            lnrecnotr: None,
            lnreoth: None,
            lnreoth2: None,
            lnreoth5: None,
            lnrenr1: None,
            lnrenr1r: None,
            lnrenr2: None,
            lnrenr2r: None,
            lnrenr3: None,
            lnrenr3r: None,
            lnrenr4: None,
            lnrenr4r: None,
            lnrenr1n: None,
            lnrenr1nr: None,
            lnrenr2n: None,
            lnrenr2nr: None,
            lnrenr3n: None,
            lnrenr3nr: None,
            lnrenr4n: None,
            lnrenr4nr: None,
            lnrenrot: None,
            lnrenrotr: None,
            lnrenrow: None,
            lnrenrowr: None,
            lnrenus: None,
            lnrenusr: None,
            lnrersf1: None,
            lnrersf1r: None,
            lnrersf2: None,
            lnrersf2r: None,
            lnrersfm: None,
            lnrersfmr: None,
            lnresncr: None,
            lnrs1t3: None,
            lnrs1t3r: None,
            lnrs3les: None,
            lnrs3lesr: None,
            lnrs3t5: None,
            lnrs3t5r: None,
            lnrs3t12: None,
            lnrs3t12r: None,
            lnrs5t15: None,
            lnrs5t15r: None,
            lnrsov15: None,
            lnrsov15r: None,
            lnsb: None,
            lnsbr: None,
            lnserv: None,
            lnservr: None,
            loccom: None,
            loccomr: None,
            locfpsb: None,
            locfpsbr: None,
            locfpsbk: None,
            locfpsbkr: None,
            locfsb: None,
            locfsbr: None,
            locfsbk: None,
            locfsbkr: None,
            locpsb: None,
            locpsbr: None,
            locpsbk: None,
            locpsbkr: None,
            loregty: None,
            loregtyr: None,
            loth: None,
            lothr: None,
            lreag: None,
            lreagr: None,
            lrecons: None,
            lreconsr: None,
            lremult: None,
            lremultr: None,
            lrenres: None,
            lrenresr: None,
            lreres: None,
            lreresr: None,
            lsalnls: None,
            lsalnlsr: None,
            lsaoa: None,
            lsaoar: None,
            lsaore: None,
            lsaorer: None,
            lsascdbt: None,
            lsascdbtr: None,
            lsfor: None,
            lsforr: None,
            msa: None,
            msrece: None,
            msrecer: None,
            msresfcl: None,
            msresfclr: None,
            msrnrece: None,
            msrnrecer: None,
            naag: None,
            naagr: None,
            naagsm: None,
            naagsmr: None,
            naasset: None,
            naassetr: None,
            naauto: None,
            naautor: None,
            naci: None,
            nacir: None,
            nacinus: None,
            nacinusr: None,
            nacon: None,
            naconr: None,
            naconoth: None,
            naconothr: None,
            nacrcd: None,
            nacrcdr: None,
            nadep: None,
            nadepr: None,
            nadepnus: None,
            nadepnusr: None,
            nafg: None,
            nafgr: None,
            nagty: None,
            nagtyr: None,
            nagtygnm: None,
            nagtygnmr: None,
            nagtypar: None,
            nagtyparr: None,
            nalag: None,
            nalagr: None,
            nalci: None,
            nalcir: None,
            nalcon: None,
            nalconr: None,
            nalgty: None,
            nalgtyr: None,
            nalnsale: None,
            nalnsaler: None,
            naloth: None,
            nalothr: None,
            nalreag: None,
            nalreagr: None,
            nalrecon: None,
            nalreconr: None,
            nalremul: None,
            nalremulr: None,
            nalrenrs: None,
            nalrenrsr: None,
            nalreres: None,
            nalreresr: None,
            nals: None,
            nalsr: None,
            naltot: None,
            naltotr: None,
            name: None,
            namefull: None,
            naothln: None,
            naothlnr: None,
            nare: None,
            narer: None,
            nareag: None,
            nareagr: None,
            narecnfm: None,
            narecnfmr: None,
            narecnot: None,
            narecnotr: None,
            narecons: None,
            nareconsr: None,
            narefor: None,
            nareforr: None,
            nareloc: None,
            narelocr: None,
            naremult: None,
            naremultr: None,
            narenres: None,
            narenresr: None,
            narenrot: None,
            narenrotr: None,
            narenrow: None,
            narenrowr: None,
            narenus: None,
            narenusr: None,
            nareres: None,
            nareresr: None,
            narersf2: None,
            narersf2r: None,
            narersfm: None,
            narersfmr: None,
            narsci: None,
            narscons: None,
            narslnfm: None,
            narslnfmr: None,
            narslnls: None,
            narslnlsr: None,
            narslnlt: None,
            narslnltr: None,
            narsmult: None,
            narsnres: None,
            narsoth: None,
            nascdebt: None,
            nascdebtr: None,
            ncag: None,
            ncauto: None,
            ncci: None,
            nccomrer: None,
            nccomre: None,
            nccon: None,
            ncconoth: None,
            nccrcd: None,
            ncdep: None,
            ncfg: None,
            ncgtypar: None,
            nclnlsr: None,
            ncls: None,
            ncothln: None,
            ncre: None,
            ncreconr: None,
            ncrecons: None,
            ncreloc: None,
            ncrelocr: None,
            ncremulr: None,
            ncremult: None,
            ncrenrer: None,
            ncrenres: None,
            ncrer: None,
            ncrereso: None,
            ncrereor: None,
            ncreres: None,
            ncreresr: None,
            netgnast: None,
            netgnastr: None,
            ntglfxaq: None,
            ntglfxaqr: None,
            netgnsln: None,
            netgnslnr: None,
            ntgllnq: None,
            ntgllnqr: None,
            netgnsre: None,
            netgnsrer: None,
            ntglreq: None,
            ntglreqr: None,
            netinca: None,
            nimy: None,
            nimyq: None,
            noij: None,
            noijr: None,
            noijy: None,
            noijyq: None,
            noija: None,
            noijq: None,
            noijqa: None,
            noijqr: None,
            noniiay: None,
            noniiayq: None,
            noniia: None,
            noniiq: None,
            noniiqa: None,
            noniiqr: None,
            nonixay: None,
            nonixayq: None,
            nonixa: None,
            nperf: None,
            nperfv: None,
            ntag: None,
            ntagr: None,
            ntaga: None,
            ntagq: None,
            ntagqr: None,
            ntagsm: None,
            ntagsmr: None,
            ntagsmq: None,
            ntagsmqr: None,
            ntauto: None,
            ntautor: None,
            ntautoa: None,
            ntautoq: None,
            ntautolnqr: None,
            ntautoqr: None,
            ntci: None,
            ntcir: None,
            ntcia: None,
            ntcinus: None,
            ntcinusr: None,
            ntcinusq: None,
            ntcinusqr: None,
            ntciq: None,
            ntciqr: None,
            ntcomrer: None,
            ntcomreq: None,
            ntcomrea: None,
            ntcon: None,
            ntconr: None,
            ntcona: None,
            ntconota: None,
            ntconoth: None,
            ntconothr: None,
            ntconotq: None,
            ntconotqr: None,
            ntconq: None,
            ntconqr: None,
            ntcontqr: None,
            ntcrcd: None,
            ntcrcdr: None,
            ntcrcda: None,
            ntcrcdq: None,
            ntcrcdqr: None,
            ntdep: None,
            ntdepr: None,
            ntdepnus: None,
            ntdepnusr: None,
            ntdepnuq: None,
            ntdepnuqr: None,
            ntdepq: None,
            ntdepqr: None,
            ntforgv: None,
            ntforgvr: None,
            ntforgvq: None,
            ntforgvqr: None,
            ntinchpp: None,
            ntincl: None,
            ntinclq: None,
            ntlnlsa: None,
            ntinqhpp: None,
            ntlnlsr: None,
            ntlnlsqr: None,
            ntls: None,
            ntlsr: None,
            ntlsq: None,
            ntlsqr: None,
            ntother: None,
            ntotherr: None,
            ntothq: None,
            ntothqr: None,
            ntrcdsm: None,
            ntrcdsmr: None,
            ntrcomot: None,
            ntrcomotr: None,
            ntre: None,
            ntremuqa: None,
            ntrecoqa: None,
            ntrelnr: None,
            ntreq: None,
            ntreqa: None,
            ntrerq: None,
            ntreag: None,
            ntreagr: None,
            ntreagq: None,
            ntrea: None,
            ntreagqr: None,
            ntrecnfm: None,
            ntrecnot: None,
            ntreconq: None,
            ntreconqr: None,
            ntrecons: None,
            ntrecosa: None,
            ntreconsr: None,
            ntrecosr: None,
            ntrecoqr: None,
            ntrefor: None,
            ntreforr: None,
            ntreforq: None,
            ntreforqr: None,
            ntreloc: None,
            ntreloclnr: None,
            ntrelocq: None,
            ntreloca: None,
            ntrelocqr: None,
            ntrelocrq: None,
            ntrelocr: None,
            ntremulq: None,
            ntremula: None,
            ntremulqr: None,
            ntremulr: None,
            ntremuqr: None,
            ntremult: None,
            ntremultr: None,
            ntrenres: None,
            ntrenresr: None,
            ntrenrot: None,
            ntrenrow: None,
            ntrenrsa: None,
            ntrenrsq: None,
            ntrenrsqr: None,
            ntrenrsr: None,
            ntrenrqr: None,
            ntrenus: None,
            ntrenusr: None,
            ntrenusq: None,
            ntreotha: None,
            ntrenusqr: None,
            ntreothr: None,
            ntreothrqr: None,
            ntreotqa: None,
            ntrer: None,
            ntreqr: None,
            ntreres: None,
            ntrereslnr: None,
            ntreresq: None,
            ntreresa: None,
            ntreresqr: None,
            ntreresr: None,
            ntreresrq: None,
            ntrersf2: None,
            ntrersf2r: None,
            ntrers2q: None,
            ntrers2qr: None,
            ntrersfm: None,
            ntrersfmr: None,
            ntrersfq: None,
            ntrersfqr: None,
            ntreoffdom: None,
            ntreoffdomr: None,
            ntreoffdomq: None,
            ntreoffdomqr: None,
            ntrfc: None,
            ntrfcfg: None,
            ntrfcfgr: None,
            ntrfg: None,
            ntrsmmda: None,
            ntrsmmdar: None,
            ntrsoth: None,
            ntrsothr: None,
            oaienc: None,
            oalifgen: None,
            oalifgenr: None,
            oalifhyb: None,
            oalifhybr: None,
            oalifins: None,
            oalifinsr: None,
            oalifsep: None,
            oalifsepr: None,
            obsdir: None,
            oreag: None,
            oreagr: None,
            orecons: None,
            oreconsr: None,
            oregnma: None,
            oreinv: None,
            oreinvr: None,
            oremult: None,
            oremultr: None,
            orenres: None,
            orenresr: None,
            oreoth: None,
            oreothr: None,
            oreothf: None,
            oreothfr: None,
            oreres: None,
            oreresr: None,
            othborf: None,
            othffc: None,
            othffcr: None,
            othnvs: None,
            othoffbs: None,
            othoffbsr: None,
            othpoc: None,
            othwoc: None,
            otsregnm: None,
            owncrci: None,
            owncrcrd: None,
            owncrhel: None,
            owndrci: None,
            owndrcrd: None,
            owndrhel: None,
            ownlnci: None,
            ownlncrd: None,
            ownlnhel: None,
            ownp3ci: None,
            ownp3crd: None,
            ownp3hel: None,
            ownp9ci: None,
            ownp9crd: None,
            ownp9hel: None,
            ownscci: None,
            ownsccrd: None,
            ownschel: None,
            p3ag: None,
            p3agr: None,
            p3agsm: None,
            p3agsmr: None,
            p3asset: None,
            p3assetr: None,
            p3auto: None,
            p3autor: None,
            p3ci: None,
            p3cir: None,
            p3cinus: None,
            p3cinusr: None,
            p3con: None,
            p3conr: None,
            p3conoth: None,
            p3conothr: None,
            p3crcd: None,
            p3crcdr: None,
            p3dep: None,
            p3depr: None,
            p3depnus: None,
            p3depnusr: None,
            p3fg: None,
            p3fgr: None,
            p3gty: None,
            p3gtyr: None,
            p3gtygnm: None,
            p3gtygnmr: None,
            p3gtypar: None,
            p3gtyparr: None,
            p3lag: None,
            p3lagr: None,
            p3lci: None,
            p3lcir: None,
            p3lcon: None,
            p3lconr: None,
            p3lgty: None,
            p3lgtyr: None,
            p3lnsale: None,
            p3lnsaler: None,
            p3loth: None,
            p3lothr: None,
            p3lreag: None,
            p3lreagr: None,
            p3lrecon: None,
            p3lreconr: None,
            p3lremul: None,
            p3lremulr: None,
            p3lrenrs: None,
            p3lrenrsr: None,
            p3lreres: None,
            p3lreresr: None,
            p3ls: None,
            p3lsr: None,
            p3ltot: None,
            p3ltotr: None,
            p3othln: None,
            p3othlnr: None,
            p3re: None,
            p3rer: None,
            p3reag: None,
            p3reagr: None,
            p3recnfm: None,
            p3recnfmr: None,
            p3recnot: None,
            p3recnotr: None,
            p3recons: None,
            p3reconsr: None,
            p3refor: None,
            p3reforr: None,
            p3reloc: None,
            p3relocr: None,
            p3remult: None,
            p3remultr: None,
            p3renres: None,
            p3renresr: None,
            p3renrot: None,
            p3renrotr: None,
            p3renrow: None,
            p3renrowr: None,
            p3renus: None,
            p3renusr: None,
            p3reres: None,
            p3reresr: None,
            p3rersf2: None,
            p3rersf2r: None,
            p3rersfm: None,
            p3rersfmr: None,
            p3rsci: None,
            p3rscons: None,
            p3rslnfm: None,
            p3rslnfmr: None,
            p3rslnls: None,
            p3rslnlsr: None,
            p3rslnlt: None,
            p3rslnltr: None,
            p3rsmult: None,
            p3rsnres: None,
            p3rsoth: None,
            p3scdebt: None,
            p3scdebtr: None,
            p9ag: None,
            p9agr: None,
            p9agsm: None,
            p9agsmr: None,
            p9asset: None,
            p9assetr: None,
            p9auto: None,
            p9autor: None,
            p9ci: None,
            p9cir: None,
            p9cinus: None,
            p9cinusr: None,
            p9con: None,
            p9conr: None,
            p9conoth: None,
            p9conothr: None,
            p9crcd: None,
            p9crcdr: None,
            p9dep: None,
            p9depr: None,
            p9depnus: None,
            p9depnusr: None,
            p9fg: None,
            p9fgr: None,
            p9gty: None,
            p9gtyr: None,
            p9gtygnm: None,
            p9gtygnmr: None,
            p9gtypar: None,
            p9gtyparr: None,
            p9lag: None,
            p9lagr: None,
            p9lci: None,
            p9lcir: None,
            p9lcon: None,
            p9lconr: None,
            p9lgty: None,
            p9lgtyr: None,
            p9lnsale: None,
            p9lnsaler: None,
            p9loth: None,
            p9lothr: None,
            p9lreag: None,
            p9lreagr: None,
            p9lrecon: None,
            p9lreconr: None,
            p9lremul: None,
            p9lremulr: None,
            p9lrenrs: None,
            p9lrenrsr: None,
            p9lreres: None,
            p9lreresr: None,
            p9ls: None,
            p9lsr: None,
            p9ltot: None,
            p9ltotr: None,
            p9othln: None,
            p9othlnr: None,
            p9re: None,
            p9rer: None,
            p9reag: None,
            p9reagr: None,
            p9recnfm: None,
            p9recnfmr: None,
            p9recnot: None,
            p9recnotr: None,
            p9recons: None,
            p9reconsr: None,
            p9refor: None,
            p9reforr: None,
            p9reloc: None,
            p9relocr: None,
            p9remult: None,
            p9remultr: None,
            p9renres: None,
            p9renresr: None,
            p9renrot: None,
            p9renrotr: None,
            p9renrow: None,
            p9renrowr: None,
            p9renus: None,
            p9renusr: None,
            p9reres: None,
            p9reresr: None,
            p9rersf2: None,
            p9rersf2r: None,
            p9rersfm: None,
            p9rersfmr: None,
            p9rsci: None,
            p9rscons: None,
            p9rslnfm: None,
            p9rslnfmr: None,
            p9rslnls: None,
            p9rslnlsr: None,
            p9rslnlt: None,
            p9rslnltr: None,
            p9rsmult: None,
            p9rsnres: None,
            p9rsoth: None,
            p9scdebt: None,
            p9scdebtr: None,
            partacqu: None,
            partconv: None,
            partconvr: None,
            rb2lnres: None,
            rb2lnresr: None,
            rbc: None,
            rbct1: None,
            rbct2: None,
            rbct2r: None,
            rbct1c: None,
            rbct1cer: None,
            rbct1j: None,
            rbct1jr: None,
            rbc1aaj: None,
            rbc1rwaj: None,
            rbcrwaj: None,
            repopurf: None,
            reposldf: None,
            roeinjr: None,
            rsci: None,
            rscons: None,
            rslnls: None,
            rslnlsr: None,
            rslnltot: None,
            rslnltotr: None,
            rslnrefm: None,
            rslnrefmr: None,
            rsmult: None,
            rsnres: None,
            rsother: None,
            rssdid: None,
            rt: None,
            rtffc: None,
            rtnvs: None,
            rtpoc: None,
            rtwoc: None,
            rwaj: None,
            rwajt: None,
            rwajtr: None,
            scabs: None,
            scabsr: None,
            scaf: None,
            scafr: None,
            scaot: None,
            sccmmb: None,
            sccmog: None,
            sccmogr: None,
            sccmot: None,
            sccmotr: None,
            sccmpt: None,
            sccmptr: None,
            sccol: None,
            sccolr: None,
            sccptg: None,
            sccptgr: None,
            sceqfv: None,
            sceqfvr: None,
            scfmn: None,
            scfmnr: None,
            scgnm: None,
            scgnmr: None,
            scgty: None,
            scgtyr: None,
            scha: None,
            schar: None,
            schtmres: None,
            schtmresr: None,
            sclent: None,
            sclentr: None,
            scnm1t3: None,
            scnm1t3r: None,
            scnm3les: None,
            scnm3lesr: None,
            scnm3t5: None,
            scnm3t5r: None,
            scnm3t12: None,
            scnm3t12r: None,
            scnm5t15: None,
            scnm5t15r: None,
            scnmov15: None,
            scnmov15r: None,
            sco3yles: None,
            sco3ylesr: None,
            sc1les: None,
            sc1lesr: None,
            scodot: None,
            scodotr: None,
            scodpi: None,
            scodpir: None,
            scoov3y: None,
            scoov3yr: None,
            scpledge: None,
            scpledger: None,
            scpt1t3: None,
            scpt1t3r: None,
            scpt3les: None,
            scpt3lesr: None,
            scpt3t5: None,
            scpt3t5r: None,
            scpt3t12: None,
            scpt3t12r: None,
            scpt5t15: None,
            scpt5t15r: None,
            scptov15: None,
            scptov15r: None,
            scrdebt: None,
            scrdebtr: None,
            scsfp: None,
            scsfpr: None,
            scsnhaa: None,
            scsnhaar: None,
            scsnhaf: None,
            scsnhafr: None,
            scspn: None,
            sz30auto: None,
            sz30autor: None,
            sz30ci: None,
            sz30cir: None,
            sz30con: None,
            sz30conr: None,
            sz30crcd: None,
            sz30crcdr: None,
            sz30hel: None,
            sz30helr: None,
            sz30oth: None,
            sz30othr: None,
            sz30res: None,
            sz30resr: None,
            sz90auto: None,
            sz90autor: None,
            sz90ci: None,
            sz90cir: None,
            sz90con: None,
            sz90conr: None,
            sz90crcd: None,
            sz90crcdr: None,
            sz90hel: None,
            sz90helr: None,
            sz90oth: None,
            sz90othr: None,
            sz90res: None,
            sz90resr: None,
            szcrauto: None,
            szcrautor: None,
            szcrcdfe: None,
            szcrcdfer: None,
            szcrci: None,
            szcrcir: None,
            szcrcon: None,
            szcrconr: None,
            szcrcrcd: None,
            szcrcrcdr: None,
            szcrhel: None,
            szcrhelr: None,
            szcroth: None,
            szcrothr: None,
            szcrres: None,
            szcrresr: None,
            szdrauto: None,
            szdrautor: None,
            szdrci: None,
            szdrcir: None,
            szdrcon: None,
            szdrconr: None,
            szdrcrcd: None,
            szdrcrcdr: None,
            szdrhel: None,
            szdrhelr: None,
            szdroth: None,
            szdrothr: None,
            szdrres: None,
            szislaut: None,
            szislautr: None,
            szislccd: None,
            szislccdr: None,
            szislci: None,
            szislcir: None,
            szislcon: None,
            szislconr: None,
            szislhel: None,
            szislhelr: None,
            szisloth: None,
            szislothr: None,
            szislres: None,
            szislresr: None,
            szlauto: None,
            szlautor: None,
            szlnci: None,
            szlncir: None,
            szlncon: None,
            szlnconr: None,
            szlncrcd: None,
            szlncrcdr: None,
            szlnhel: None,
            szlnhelr: None,
            szlnoth: None,
            szlnothr: None,
            szlnres: None,
            szlnresr: None,
            szucauto: None,
            szucci: None,
            szuccon: None,
            szuccrcd: None,
            szuchel: None,
            szucoth: None,
            szucres: None,
            tcama: None,
            tcamanum: None,
            tcanma: None,
            tcanmnum: None,
            tcanum: None,
            tcanumd: None,
            tcapao: None,
            tcapaod: None,
            tcatnum: None,
            tcdemv: None,
            tcdenum: None,
            tciemv: None,
            tcienum: None,
            tcmbmv: None,
            tcmbnum: None,
            tcsbmv: None,
            tcsbnum: None,
            tcsnma: None,
            tcsnmnum: None,
            tcsomv: None,
            tcsonum: None,
            tcstmv: None,
            tcstnum: None,
            tctbmv: None,
            tctbnum: None,
            tctotmv: None,
            tctotnum: None,
            tebma: None,
            tebmanum: None,
            tebnma: None,
            tebnmnum: None,
            tecma: None,
            tecmanum: None,
            tecnma: None,
            tecnmnum: None,
            tecps: None,
            teeqf: None,
            tei: None,
            tematot: None,
            temisc: None,
            temmf: None,
            teni: None,
            teothb: None,
            teothf: None,
            tere: None,
            teremtg: None,
            tescmun: None,
            tescus: None,
            testo: None,
            tetot: None,
            tetrf: None,
            teuf: None,
            tfema: None,
            tfemanum: None,
            tfenma: None,
            tfenmnum: None,
            tica: None,
            tics: None,
            tieb: None,
            tiec: None,
            tife: None,
            tima: None,
            timma: None,
            timmanum: None,
            timnma: None,
            timnmnum: None,
            tintra: None,
            tiof: None,
            tior: None,
            tip: None,
            tir: None,
            titotf: None,
            tmaf: None,
            tmafnum: None,
            tmasmf: None,
            tmasmfn: None,
            tni: None,
            tnl: None,
            tnmaf: None,
            tnmnumf: None,
            tocps: None,
            toeqf: None,
            tofma: None,
            tofmanum: None,
            tofnma: None,
            tofnmnum: None,
            toi: None,
            tomatot: None,
            tomisc: None,
            tommf: None,
            toni: None,
            toothb: None,
            toothf: None,
            tore: None,
            toremtg: None,
            torma: None,
            tormanum: None,
            tornma: None,
            tornmnum: None,
            toscmun: None,
            toscus: None,
            tosto: None,
            totrf: None,
            touf: None,
            tpicps: None,
            tpieqf: None,
            tpii: None,
            tpimatot: None,
            tpimisc: None,
            tpimmf: None,
            tpini: None,
            tpiothb: None,
            tpiothf: None,
            tpire: None,
            tpiremtg: None,
            tpiscmun: None,
            tpiscus: None,
            tpisto: None,
            tpitrf: None,
            tpiuf: None,
            tpma: None,
            tpmanum: None,
            tpnma: None,
            tpnmnum: None,
            trexer: None,
            trfor: None,
            trhma: None,
            trhmanum: None,
            trhnma: None,
            trhnmnum: None,
            trlreval: None,
            trlrevalr: None,
            trncbo: None,
            trncbor: None,
            trnfc: None,
            trnfcfg: None,
            trnfcfgr: None,
            trnfg: None,
            trnnia: None,
            trnniar: None,
            trnnin: None,
            trpower: None,
            trrevald: None,
            trrevalf: None,
            trrevalsum: None,
            trrevalsumr: None,
            ttma: None,
            ttnanum: None,
            ttnma: None,
            ttnmnum: None,
            uc: None,
            ucr: None,
            uccomre: None,
            uccomrer: None,
            uccomres: None,
            uccomresr: None,
            uccomreu: None,
            uccomreur: None,
            uccrcd: None,
            uccrcdr: None,
            ucln: None,
            ucloc: None,
            uclocr: None,
            ucother: None,
            ucotherr: None,
            ucover1: None,
            ucover1r: None,
            ucsc: None,
            ucscr: None,
            ucszauto: None,
            ucszci: None,
            ucszcon: None,
            ucszcrcd: None,
            ucszhel: None,
            ucszoth: None,
            ucszres: None,
            unincfor: None,
            unincforr: None,
            voliab: None,
            voliabr: None,
            zip: None,
            lipnmtg: None,
            uyanmtg: None,
            ilnls: None,
            unit: None,
            ptaxnetinc: None,
            ptaxnetincr: None,
            ptaxnetincq: None,
            ptaxnetincqr: None,
            addnonii: None,
            addnoniir: None,
            addnoniiq: None,
            addnoniiqr: None,
            avmmlf: None,
            avmmlfr: None,
            avpppplg: None,
            avpppplgr: None,
            mmlfbal: None,
            mmlfbalr: None,
            ppplfov1: None,
            ppplfov1r: None,
            ppplnbal: None,
            ppplnbalr: None,
            ppplnnum: None,
            ppplnnumr: None,
            ppplnplg: None,
            ppplnplgr: None,
            ppplf1ls: None,
            ppplf1lsr: None,
            idntcir: None,
            idntciqr: None,
            idntconr: None,
            idntcrdr: None,
            idntcoor: None,
            idntcooqr: None,
            idntcrdqr: None,
            instcnt: None,
            idntilr: None,
            idothnii: None,
            ntautopr: None,
            ntconotr: None,
            iderncvr: None,
            iderncvqr: None,
            eqcdivntinc: None,
            nacdir: None,
            ntcomreqr: None,
            ntallothnum: None,
            ntallothden: None,
            ntallothr: None,
            ntallothqr: None,
            idnccoor: None,
            idncothr: None,
            idnccir: None,
            idncconr: None,
            idnccrdr: None,
            idncator: None,
            idntator: None,
            idntcotr: None,
            iddepinr: None,
            iddivnir: None,
            idnccotr: None,
            intincy: None,
            idncgtpr: None,
            idlncorr: None,
            idt1cnocb: None,
            idt1jnocb: None,
            idt1cer: None,
            idt1rwajr: None,
            sceqnft: None,
            scrmbpi: None,
            scrmbpir: None,
            scuso: None,
            scusor: None,
            sccmos: None,
            sccmosr: None,
            sctatfr: None,
            lnlsgrs: None,
            lnlsgrsr: None,
            aoa: None,
            aoar: None,
            estins: None,
            estinsr: None,
            p3relndo: None,
            p3relndor: None,
            p9relndo: None,
            p9relndor: None,
            narelndo: None,
            narelndor: None,
            stcnty: None,
            cbsa: None,
            insdate: None,
            upddate: None,
            assetr: None,
            avasset: None,
            broinslg: None,
            ct1ajtot: None,
            ct1badj: None,
            dep2: None,
            dep5: None,
            depiy1: None,
            ecd100: None,
            ecd100a: None,
            ecd100q: None,
            efreppa: None,
            eothtima: None,
            eothtime: None,
            eothtimq: None,
            equpgr: None,
            esavdp: None,
            esavdpa: None,
            esavdpq: None,
            esubnda: None,
            etrandep: None,
            etrandpa: None,
            etrandpq: None,
            ettlotba: None,
            ettlotbq: None,
            ffpur: None,
            ibeftxa: None,
            iglsca: None,
            iglscaq: None,
            iglsch: None,
            ilna: None,
            ilnlsa: None,
            ilnlsq: None,
            ilnlsxa: None,
            ilnlsxq: None,
            ilnmuniq: None,
            ilnq: None,
            isca: None,
            iserchga: None,
            itaxa: None,
            itaxqa: None,
            lncdt1r: None,
            lncit1r: None,
            lncont1r: None,
            lnlsres: None,
            lnreag5: None,
            lnrert1r: None,
            ncreag: None,
            ncrecnfm: None,
            ncrecnot: None,
            ncrenrot: None,
            ncrenrow: None,
            ncrers2r: None,
            ncrersf2: None,
            ncrersfm: None,
            ncrersfr: None,
            ncrslnls: None,
            noiq: None,
            ntagqa: None,
            ntagsma: None,
            ntagsmqa: None,
            ntciqa: None,
            ntcomre: None,
            ntcomrqa: None,
            ntconqa: None,
            ntcrcdqa: None,
            ntirtq: None,
            ntrcdsmj: None,
            ntreaga: None,
            ntreagqa: None,
            obor: None,
            obor2: None,
            obor5: None,
            othbfh03: None,
            othbfh13: None,
            p3comre: None,
            p3reconr: None,
            p3rers2r: None,
            rbcequp: None,
            rbct1w: None,
            repopur: None,
            sc2: None,
            sc5: None,
            scmuniaa: None,
            scmuniaf: None,
            scmuniha: None,
            scmunihf: None,
            };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
