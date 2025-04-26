//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/financials` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/financials` endpoint.
/// Spec: risview_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct FinancialsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[doc = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_by")]
    pub agg_by: Option<String>,
    #[doc = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_term_fields")]
    pub agg_term_fields: Option<String>,
    #[doc = r#"The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_sum_fields")]
    pub agg_sum_fields: Option<String>,
    #[doc = r#"The limit on how many aggregated results will be displayed"#]
    #[param(rename = "agg_limit")]
    pub agg_limit: Option<u32>,
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

    #[allow(unused_variables)]
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
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct FinancialsProperties {
    #[doc = r#"## FDIC Field:: `ACTEVT`
    Title: Activity Event Code
    Description: Structure activity event code. Merger or closing codes only."#]
    #[serde(rename="ACTEVT")]
    pub activity_event_code: Option<String>,

    #[doc = r#"## FDIC Field:: `ASSET`
    Title: Total assets
    Description: The sum of all assets owned by the institution including cash, loans, securities, bank premises and other assets. This total does not include off-balance-sheet accounts."#]
    #[serde(rename="ASSET")]
    pub total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `BRANCH`
    Title: BRANCHING
    Description: A flag used to indicate whether an institution has branches. 0 means no branches and 1 means it has branches."#]
    #[serde(rename="BRANCH")]
    pub branching: Option<f64>,

    #[doc = r#"## FDIC Field:: `CALLFORM`
    Title: Call Form Number
    Description: TBD"#]
    #[serde(rename="CALLFORM")]
    pub call_form_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CB`
    Title: Community Bank
    Description: FDIC community banks are identified based on criteria defined in the FDIC Community Banking Study. Using detailed balance sheet and geographic data, the study defines communtiy banks in terms of their traditional relationship banking and limited geographic scope of operations"#]
    #[serde(rename="CB")]
    pub community_bank: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSADIV`
    Title: Core Based Statistical Division Number
    Description: A metropolitan division is a county or group of counties within a core based statistical area that contains a population of at least 2.5 million. A metropolitan division consists of one or more main/secondary countues that represent an employment center or centers, plus adjacent conuties associated withe the main county or counties through commuting ties."#]
    #[serde(rename="CBSADIV")]
    pub core_based_statistical_division_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CBSANAME`
    Title: Core Based Statistical Division Name
    Description: A statistical geographic entity consisting of the county or counties associated with at least one core (urbanized area or urban cluster) of at least 10,000 population, plus adjacent counties having a high degree of social and economic integration with the core as measured through commuting ties with the counties containing the core."#]
    #[serde(rename="CBSANAME")]
    pub core_based_statistical_division_name: Option<String>,

    #[doc = r#"## FDIC Field:: `STMULT`
    Title: Multi State Offices Flag
    Description: Multi State Offices Flag"#]
    #[serde(rename="STMULT")]
    pub multi_state_offices_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `ADDRESS`
    Title: ADDRESS
    Description: ADDRESS"#]
    #[serde(rename="ADDRESS")]
    pub address: Option<String>,

    #[doc = r#"## FDIC Field:: `CBLRIND`
    Title: Community Bank Ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CBLRIND")]
    pub community_bank_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD1T3`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 1-3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD1T3")]
    pub time_dep_250_000_or_more_remaining_maturity_repricing_of_1_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD1T3R`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 1-3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD1T3R")]
    pub time_dep_250_000_or_more_remaining_maturity_repricing_of_1_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3LES`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 3 MONTH OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3LES")]
    pub time_dep_250_000_or_more_remaining_maturity_repricing_of_3_month_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3LESR`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 3 MONTH OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3LESR")]
    pub time_dep_250_000_or_more_remaining_maturity_repricing_of_3_month_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3LESS`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY REPRICING OF 3 MONTH OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3LESS")]
    pub time_dep_250_000_or_less_remaining_maturity_repricing_of_3_month_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3LESSR`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY REPRICING OF 3 MONTH OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3LESSR")]
    pub time_dep_250_000_or_less_remaining_maturity_repricing_of_3_month_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CDOV3`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING OVER 3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CDOV3")]
    pub time_dep_250_000_or_more_remaining_maturity_or_repricing_over_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `CDOV3R`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING OVER 3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CDOV3R")]
    pub time_dep_250_000_or_more_remaining_maturity_or_repricing_over_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CDOV3S`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING OVER 3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CDOV3S")]
    pub time_dep_250_000_or_less_remaining_maturity_or_repricing_over_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `CDOV3SR`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING OVER 3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CDOV3SR")]
    pub time_dep_250_000_or_less_remaining_maturity_or_repricing_over_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3T12`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING 3-12 MONTHS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3T12")]
    pub time_dep_250_000_or_more_remaining_maturity_or_repricing_3_12_months: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3T12R`
    Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING 3-12 MONTHS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3T12R")]
    pub time_dep_250_000_or_more_remaining_maturity_or_repricing_3_12_months_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3T12S`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 3-12 MONTHS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3T12S")]
    pub time_dep_250_000_or_less_remaining_maturity_or_repricing_3_12_months: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD3T12SR`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 3-12 MONTHS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD3T12SR")]
    pub time_dep_250_000_or_less_remaining_maturity_or_repricing_3_12_months_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD1T3S`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 1-3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD1T3S")]
    pub time_dep_250_000_or_less_remaining_maturity_or_repricing_1_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `CD1T3SR`
    Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 1-3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CD1T3SR")]
    pub time_dep_250_000_or_less_remaining_maturity_or_repricing_1_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CERT`
    Title: FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="CERT")]
    pub fdic_certificate: Option<f64>,

    #[doc = r#"## FDIC Field:: `CERTCONS`
    Title: Directly owned by another bank (CERT)
    Description: FDIC certificate number of the parent bank or savings institution with which the reported institution’s financial data has been consolidated. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports.  Click on the certificate number to identify the parent bank or thrift."#]
    #[serde(rename="CERTCONS")]
    pub directly_owned_by_another_bank_cert: Option<String>,

    #[doc = r#"## FDIC Field:: `CITYHCR`
    Title: City of High Holder
    Description: City in which the headquarters of the institution's regulatory high holder are physically located."#]
    #[serde(rename="CITYHCR")]
    pub city_of_high_holder: Option<String>,

    #[doc = r#"## FDIC Field:: `CLCODE`
    Title: Classcode
    Description: A number that sub-categorizes a major class of institutions. 3 = National bank, Federal Reserve System (FRS) member; 13 = State commercial bank, FRS member; 15 = State savings, co-op, or insdustrial bank, FRS member; 21 = State commercial bank, not FRS member; 23 = State savings, co-op, or industrial bank, not FRS member; 25 = State mutual commercial bank, not FRS member; 33 =  Federal chartered stock savings bank; 34 = Federal chartered mutual savings bank; 35 = State chartered stock savings and loan association; 36 = State chartered mutual savings and loan association; 37 = Federal chartered stock savings and loan association; 38 = Federal chartered mutual savings and loan association; 41 = State chartered stock savings bank; 42 = State chartered mutual savings bank; 43 = Federal chartered stock savings bank (historical); 44 = Federal chartered mutual savings bank (historical); 50 = OCC chartered nondeposit and/or noninsured trust companies; 51 = Noninsured commercial bank; 52 = Noninsured domestic offices of foreign bank (International Banking Act); 53 = Noninsured industrial bank; 54 = State chartered nondeposit and/or noninsured trust company, not FRS member; 55 = State chartered domestic branches of foreign banks; 56 = OCC chartered domestic branches of foreign banks; 57 = New York investment company; 58 = State chartered nondeposit and/or noninsured trust company, FRS member; 59 = OTS chartered nondeposit and/or noninsured trust company, 61 = Noninsured private bank; 62 = Noninsured loan workout bank, OCC chartered; 63 = Noninsured loan workout bank, state chartered, FRS member; 64 = Noninsured loan workout bank, state chartered, not FRS member; 65 = Other holding company; 71 = Transfer agent; 81 = Noninsured stock savings bank; 82 = Noninsured mutual savings bank; 85 = Noninsured stock savings and loan association; 86 = Noninsured mutual savings and loan association; 89 = Noninsured insurance company; 91 = State chartered credit unions; 92 = Federal chartered credit unions; 93 = Privately insured state credit union."#]
    #[serde(rename="CLCODE")]
    pub classcode: Option<f64>,

    #[doc = r#"## FDIC Field:: `CLOSED`
    Title: Closed Institution Flag
    Description: A flag used to indicate whether an institution has been closed. 0 is institution not closed. 1 is institution closed."#]
    #[serde(rename="CLOSED")]
    pub closed_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `CMSA`
    Title: FIPS CMSA Code
    Description: The Federal Information Processing Standards Consolidated Metropolitan Statistical Area Code is a number representing the institution location. A CMSA consists of two or more contiguous MSAs with a combined population of over 1 million."#]
    #[serde(rename="CMSA")]
    pub fips_cmsa_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `CNTRYALP`
    Title: FIPS Country Code
    Description: The Federal Information Processing Standards Alphabetic Code of the country in which the institution is physically located."#]
    #[serde(rename="CNTRYALP")]
    pub fips_country_code: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTRYNUM`
    Title: FIPS Country Number
    Description: The Federal Information Processing Standards Numeric Code of the country in which the institution is physically located."#]
    #[serde(rename="CNTRYNUM")]
    pub fips_country_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CNTYNUM`
    Title: FIPS County Number
    Description: The Federal Information Processing Standards Numeric Code of the county in which the institution is physically located."#]
    #[serde(rename="CNTYNUM")]
    pub fips_county_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CSA`
    Title: Combined Statistical Area
    Description: U.S. CENSUS BUREAU OFFICE OF MANANGEMENT AND BUDGET DEFINES                                   THE COMBINED STATISTICAL AREA (CSA) AS A GEOGRAPHIC ENTITY                                         CONSISTING OF TWO OR MORE ADJACENT CORE BASED STATISTICAL AREAS                                  (CBSAS) WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 15.                                     PAIRS OF CBSAS WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST                                  25 COMBINE AUTOMATICALLY.  PAIRS OF CBSAS WITH EMPLOYMENT                                        INTERCHANGE MEASURES OF AT LEAST 15, BUT LESS THAN 25, MAY                                        COMBINE IF LOCAL OPTION IN BOTH AREAS FAVOR COMBINATION. "#]
    #[serde(rename="CSA")]
    pub combined_statistical_area: Option<String>,

    #[doc = r#"## FDIC Field:: `DENOVO`
    Title: Denovo Institution
    Description: A flag used to indicate whether an institution is a new institution (not a recharter). This flag is set quarterly. For instance, if REPDTE is 3/31/98 and DENOVO equals 1, the institution was a denovo during the first quarter of 1998."#]
    #[serde(rename="DENOVO")]
    pub denovo_institution: Option<String>,

    #[doc = r#"## FDIC Field:: `DEP`
    Title: Total deposits
    Description: The sum of all deposits including demand deposits, money market deposits, other savings deposits, time deposits and deposits in foreign offices."#]
    #[serde(rename="DEP")]
    pub total_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPR`
    Title: TOTAL DEPOSITS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPR")]
    pub total_deposits_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPDOM`
    Title: Deposits held in domestic offices
    Description: The sum of all domestic office deposits, including demand deposits, money market deposits, other savings deposits and time deposits."#]
    #[serde(rename="DEPDOM")]
    pub deposits_held_in_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPDOMR`
    Title: DEPOSITS HELD IN DOM OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPDOMR")]
    pub deposits_held_in_dom_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DIVISION`
    Title: Division Flag
    Description: A flag used to indicate whether an institution is in a CBSA division. 0 is institution is not in a CBSA division. 1 is institution is in a CBSA division."#]
    #[serde(rename="DIVISION")]
    pub division_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `DOCKET`
    Title: Docket Number
    Description: A unique identification number assigned to institutions chartered by the office of thrift supervision or that become members of the federal home loan system."#]
    #[serde(rename="DOCKET")]
    pub docket_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDGECODE`
    Title: International Activity Flag
    Description: A FLAG USED TO INDICATE WHETHER AN INSTITUTION OPERATES ONE OR                                   MORE EDGE ACT OR AGREEMENT CORPORATIONS.  AN EDGE ACT CORPORATION                                 IS A FEDERALLY CHARTERED DOMESTIC ORGANIZATION THAT IS ALLOWED TO                                ENGAGE ONLY IN INTERNATIONAL BANKING OR OTHER FINANCIAL                                          TRANSACTIONS RELATED TO INTERNATIONAL BUSINESS.  AN AGREEMENT CORPORATION IS RESTRICTED, IN GENERAL, TO INTERNATIONAL BANKING OPERATIONS. 0 = NO AFFILIATIONS WITH EDGE ACT CORPORATIONS.                                                                                    1 = AFFILIATED WITH EDGE ACT CORPORATIONS."#]
    #[serde(rename="EDGECODE")]
    pub international_activity_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENTTYPE`
    Title: Entity Type
    Description: A three digit number indicating the major type or category of an  institution. The entity code is used to categorize an institution by type of financial organization."#]
    #[serde(rename="ENTTYPE")]
    pub entity_type: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQ`
    Title: Equity capital
    Description: Total equity capital (includes preferred and common stock, surplus and undivided profits)."#]
    #[serde(rename="EQ")]
    pub equity_capital: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQ2`
    Title: Equity capital
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQ2")]
    pub equity_capital_eq2: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQR`
    Title: EQUITY CAPITAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQR")]
    pub equity_capital_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `FAILED`
    Title: Failed Institution Flag
    Description: A flag used to indicate whether an institution has failed. Failures include assisted mergers and payoffs."#]
    #[serde(rename="FAILED")]
    pub failed_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICAREA`
    Title: FDIC Compliance Area
    Description: A number used to identify the compliance area in which an institution is located."#]
    #[serde(rename="FDICAREA")]
    pub fdic_compliance_area: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICTERR`
    Title: FDIC Compliance Territory
    Description: An abbreviation of the current compliance territory where an institution is located (FDIC Compliance Territory). All periods are displayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#]
    #[serde(rename="FDICTERR")]
    pub fdic_compliance_territory: Option<String>,

    #[doc = r#"## FDIC Field:: `FLDOFDCA`
    Title: DCA Field Office
    Description: The name of the compliance field office to which an institution is assigned. All periods are diplayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#]
    #[serde(rename="FLDOFDCA")]
    pub dca_field_office: Option<String>,

    #[doc = r#"## FDIC Field:: `FORM31`
    Title: FFIEC Call Report 31 Filer
    Description: A flag (1=yes,0=no) that indicates whether and institution filed an FFIEC 031 Call Report. Commercial banks with domestic and foreign offices are required to file such a report."#]
    #[serde(rename="FORM31")]
    pub ffiec_call_report_31_filer: Option<String>,

    #[doc = r#"## FDIC Field:: `HCTMULT`
    Title: Bank Holding Company Type
    Description: A flag used to indicate whether an institution is a member of a multibank holding company 1=yes, 0=no"#]
    #[serde(rename="HCTMULT")]
    pub bank_holding_company_type: Option<String>,

    #[doc = r#"## FDIC Field:: `HCTNONE`
    Title: Bank Not Member of Hold Company
    Description: A flag used to indicated whether an institution is an independent bank. 0 is member of a bank hold company. 1 is not a member of a bank holding company."#]
    #[serde(rename="HCTNONE")]
    pub bank_not_member_of_hold_company: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSAGNT2`
    Title: Secondary Insurer
    Description: The secondary insurer, insurance agent, or insurance status of an institution."#]
    #[serde(rename="INSAGNT2")]
    pub secondary_insurer: Option<String>,

    #[doc = r#"## FDIC Field:: `INSBIF`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="INSBIF")]
    pub tbd: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSDIF`
    Title: Deposit Insurance Fund member
    Description: A flag used to indicate whether an institution is insured under the Deposit Insurance Fund (DIF).  As of April 1, 2006 the Bank Insurance Fund (BIF) was merged together with the Savings Institution Insurance Fund (SAIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured BIF and SAIF member institutions that are still active or open are now insured members of DIF.    0 = No, not DIF insured and 1 = Yes, DIF insured.  Note that institutions that became inactive prior to April 1006 will also have zero value."#]
    #[serde(rename="INSDIF")]
    pub deposit_insurance_fund_member: Option<String>,

    #[doc = r#"## FDIC Field:: `INSTAG`
    Title: Agricultural lending institution indicator
    Description: An indicator specifying whether an institution is primarily an agricultural lending institution."#]
    #[serde(rename="INSTAG")]
    pub agricultural_lending_institution_indicator: Option<String>,

    #[doc = r#"## FDIC Field:: `INSTCRCD`
    Title: Credit Card Institutions
    Description: Institutions with total loans greater than 50% of total assets and credit card loans greater than 50% of total loans, including loans that have been securitized and sold."#]
    #[serde(rename="INSTCRCD")]
    pub credit_card_institutions: Option<String>,

    #[doc = r#"## FDIC Field:: `INSSAIF`
    Title: SAIF Insured
    Description: Institutions who are members of the Savings Association Insurance Fund. As of April 1, 2006 SAIF was merged together with the Bank Insurance Fund (BIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured SAIF member institutions, that are still active or open, are now insured members of DIF."#]
    #[serde(rename="INSSAIF")]
    pub saif_insured: Option<f64>,

    #[doc = r#"## FDIC Field:: `MINORITY`
    Title: MINORITY OWNED INSTITUTIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MINORITY")]
    pub minority_owned_institutions: Option<f64>,

    #[doc = r#"## FDIC Field:: `MUTUAL`
    Title: Ownership Type
    Description: Banking institutions fall into one of two ownership types, stock or non-stock. An institution which sells stock to raise capital is called a stock institution. It is owned by the shareholders who benefit from profits earned by the institution. A non-stock institution, or mutual institution, is owned and controlled solely by its depositors. A mutual does not issue capital stock."#]
    #[serde(rename="MUTUAL")]
    pub ownership_type: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAMEHCR`
    Title: Bank Holding Company (Regulatory Top Holder)
    Description: Regulatory top holder is assigned by the Federal Reserve Board based on ownership and control percentages. Note: Information on bank holding companies is only as of quarter-end. Regulatory top holder is any company that directly or indirectly owns, controls or has power to vote 25 percent or more of a bank's or direct holding company's shares or  controls in any manner the election of a majority of the directors or trustees of a bank or direct holding company or  exercises a controlling influence over the management or policies of a bank or direct holding company.   Information on Thrift Holding Companies that own Savings Associations but do not own banks is not currently available in the ID System.  Source: Federal Reserve Board National Information Center data base."#]
    #[serde(rename="NAMEHCR")]
    pub bank_holding_company_regulatory_top_holder: Option<String>,

    #[doc = r#"## FDIC Field:: `NETINC`
    Title: Net income
    Description: Net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#]
    #[serde(rename="NETINC")]
    pub net_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINCR`
    Title: NET INCOME - RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINCR")]
    pub net_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINCQ`
    Title: Net income - quarterly
    Description: Quarterly net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#]
    #[serde(rename="NETINCQ")]
    pub net_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINCQA`
    Title: Net income - quarterly
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINCQA")]
    pub net_income_quarterly_netincqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINCQR`
    Title: NET INCOME - QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINCQR")]
    pub net_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFDOM`
    Title: Number of Domestic Offices
    Description: The number of domestic offices (including headquarters) operated by active institutions in the 50 states of the U.S.A."#]
    #[serde(rename="OFFDOM")]
    pub number_of_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFFOR`
    Title: Number of Foreign Offices
    Description: The number of foreign offices (outside the U.S.) operated by the institution."#]
    #[serde(rename="OFFFOR")]
    pub number_of_foreign_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFOA`
    Title: Number of US Offices
    Description: The number of offices operated by an FDIC-insured institution in all commonwealths and terrirtories of the US, along with those in freely associated states under the Compact of Free Association"#]
    #[serde(rename="OFFOA")]
    pub number_of_us_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `PARCERT`
    Title: Directly owned by another bank (CERT)
    Description: The PARCERT number identifies the subsidiary institutions parent certificate number. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports."#]
    #[serde(rename="PARCERT")]
    pub directly_owned_by_another_bank_cert_parcert: Option<String>,

    #[doc = r#"## FDIC Field:: `L_REPDTE`
    Title: Report Date
    Description: The last day of the financial reporting period selected."#]
    #[serde(rename="L_REPDTE")]
    pub report_date: Option<String>,

    #[doc = r#"## FDIC Field:: `REPDTE_RAW`
    Title: Report Date
    Description: The last day of the financial reporting period selected."#]
    #[serde(rename="REPDTE_RAW")]
    pub report_date_repdte_raw: Option<String>,

    #[doc = r#"## FDIC Field:: `REPDTE`
    Title: Report Date
    Description: The last day of the financial reporting period selected."#]
    #[serde(rename="REPDTE")]
    pub report_date_repdte: Option<String>,

    #[doc = r#"## FDIC Field:: `REPYEAR`
    Title: REPORT YEAR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="REPYEAR")]
    pub report_year: Option<String>,

    #[doc = r#"## FDIC Field:: `RISDATE`
    Title: Report Date
    Description: The financial reporting period selected in CCYYMM format."#]
    #[serde(rename="RISDATE")]
    pub report_date_risdate: Option<String>,

    #[doc = r#"## FDIC Field:: `ROA`
    Title: Return on assets (ROA)
    Description: Net income after taxes and extraordinary items (annualized) as a percent of average total assets."#]
    #[serde(rename="ROA")]
    pub return_on_assets_roa: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROAPTX`
    Title: Pretax return on assets
    Description: Annualized pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#]
    #[serde(rename="ROAPTX")]
    pub pretax_return_on_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROAPTXQ`
    Title: Quarterly Pretax return on assets
    Description: Quarterly pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#]
    #[serde(rename="ROAPTXQ")]
    pub quarterly_pretax_return_on_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROAQ`
    Title: Quarterly return on assets
    Description: Quarterly net income after taxes and extraordinary items as a percent of average total assets."#]
    #[serde(rename="ROAQ")]
    pub quarterly_return_on_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROE`
    Title: Return on Equity (ROE)
    Description: Annualized net income as a percent of average equity on a consolidated basis.     Note: If retained earnings are  negative, the ratio is shown as NA."#]
    #[serde(rename="ROE")]
    pub return_on_equity_roe: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROEQ`
    Title: Quarterly return on equity
    Description: Quarterly net income (including gains or losses on securities and extraordinary items) as a percentage of average total equity capital."#]
    #[serde(rename="ROEQ")]
    pub quarterly_return_on_equity: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSSDHCR`
    Title: RSSDID - High Regulatory Holder
    Description: The unique number assigned by the Federal Reserve Board to the regulatory high holding company of the institution."#]
    #[serde(rename="RSSDHCR")]
    pub rssdid_high_regulatory_holder: Option<String>,

    #[doc = r#"## FDIC Field:: `SPECGRP`
    Title: Asset Concentration Hierarchy
    Description: An indicator of an institution's primary specialization in terms of asset concentration"#]
    #[serde(rename="SPECGRP")]
    pub asset_concentration_hierarchy: Option<f64>,

    #[doc = r#"## FDIC Field:: `SPECGRPDESC`
    Title: Asset Concentration Hierarchy Description
    Description: An indicator of an institution's primary specialization in terms of asset concentration Description"#]
    #[serde(rename="SPECGRPDESC")]
    pub asset_concentration_hierarchy_description: Option<String>,

    #[doc = r#"## FDIC Field:: `STALPHCR`
    Title: Regulatory holding company state location
    Description: State location of the regulatory high holding company (either direct or indirect owner)."#]
    #[serde(rename="STALPHCR")]
    pub regulatory_holding_company_state_location: Option<String>,

    #[doc = r#"## FDIC Field:: `SUBCHAPS`
    Title: Subchapter S Corporations
    Description: The Small Business Job Protection Act of 1996 changed the Internal Revenue Code to allow financial institutions to elect Subchapter S corporation status, beginning in 1997. Banks are required to indicate on the Call Report whether there is currently in effect an election to file under Subchapter S. Thrifts have a similar requirement as of March 1998.  The most important IRS requirements to elect and maintain Subchapter S status are: There can be no more than 75 eligible shareholders and no more than one class of stock. (In general, shareholders can only be individuals, estates, and certain types of trusts. Certain retirement plans and charitable organizations will be eligible in 1998.) All shareholders must consent.  Banks and thrifts converting to Subchapter S status must use the specific charge-off method for tax purposes rather than the reserve method of accounting for bad debts and recapture tax bad debt reserves over a period of six years, if the reserve method had been used prior to conversion. (Note: even though the specific charge-off method is required for tax purposes, an adequate allowance for loan and lease losses must still be maintained on the financial statements and Call Reports.) Banks and thrifts are subject to a built-in gains (BIG) tax, if the aggregate fair market value of assets is greater than their aggregate adjusted bases on the date of conversion to Subchapter S status.     [Banks are required to indicate separately on the Call Report in December of each year, the deferred portion of income taxes reported in net income. For Subchapter S banks, some or all of their deferred tax assets and liabilities may be eliminated upon conversion to Subchapter S status; however, deferred taxes related to the BIG tax and the recapture of bad debt reserves must be recognized.].   A Subchapter S corporation is treated as a pass-through entity, similar to a partnership, for federal income tax purposes. It is generally not subject to any federal income taxes at the corporate level. Its taxable income flows through to its shareholders in proportion to their stock ownership, and the shareholders generally pay federal income taxes on their share of this taxable income. This can have the effect of reducing institutions' reported income tax expense and increasing their after-tax earnings..   The election of Subchapter S status may result in an increase in shareholders' personal tax liabilities. Therefore, S corporations typically increase the amount of earnings distributed as dividends to compensate for higher personal taxes."#]
    #[serde(rename="SUBCHAPS")]
    pub subchapter_s_corporations: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRACT`
    Title: 
    Description: Beyond having trust powers granted and exercised, institutions with fiduciary assets accounts, income, or other reportable fiduciary related service"#]
    #[serde(rename="TRACT")]
    pub tract: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRUST`
    Title: Trust Powers
    Description: A flag used to indicate an institution's Trust Powers Granted status. 0 = No Trust Power Granted 1 = Trust Power Granted Where Trust Power has been granted specific codes are: 00 - Trust powers not know 10 - Full trust powers granted 11 - Full trust powers granted, exercised 12 - Full trust powers granted, not exercised 20 - Limited trust powers granted 21 - Limited trust powers granted, exercised 22 - Limited trust powers granted, not exercised 30 - Trust powers not granted 31 - Trust powers not granted, but exercised"#]
    #[serde(rename="TRUST")]
    pub trust_powers: Option<String>,

    #[doc = r#"## FDIC Field:: `ACEPT`
    Title: BANKS LIABILITY ON ACCEPTANCES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ACEPT")]
    pub banks_liability_on_acceptances: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACTIVE`
    Title: ACTIVE INSTITUTION FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ACTIVE")]
    pub active_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `BKCLASS`
    Title: INSTITUTION CLASS
    Description: A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA)."#]
    #[serde(rename="BKCLASS")]
    pub institution_class: Option<String>,

    #[doc = r#"## FDIC Field:: `BKPREM`
    Title: PREMISES AND FIXED ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BKPREM")]
    pub premises_and_fixed_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `BKPREMR`
    Title: PREMISES AND FIXED ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BKPREMR")]
    pub premises_and_fixed_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `BRO`
    Title: BROKERED DEP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BRO")]
    pub brokered_dep: Option<f64>,

    #[doc = r#"## FDIC Field:: `BROR`
    Title: BROKERED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BROR")]
    pub brokered_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CALLYM`
    Title: REPORT DATE (CCYYMM)
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CALLYM")]
    pub report_date_ccyymm: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHBAL`
    Title: CASH & DUE FROM DEPOSITORY INST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHBAL")]
    pub cash_due_from_depository_inst: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHBALR`
    Title: CASH & DUE FROM DEPOSITORY INST RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHBALR")]
    pub cash_due_from_depository_inst_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHBALI`
    Title: INTEREST-BEARING CASH & DUE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHBALI")]
    pub interest_bearing_cash_due: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHBALIR`
    Title: INTEREST-BEARING CASH & DUE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHBALIR")]
    pub interest_bearing_cash_due_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHRTAGNT`
    Title: CHARTER AGENT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHRTAGNT")]
    pub charter_agent: Option<String>,

    #[doc = r#"## FDIC Field:: `CONSERVE`
    Title: RTC CONSERVATORSHIP FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CONSERVE")]
    pub rtc_conservatorship_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLNLS`
    Title: TOTAL LN&LS RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLNLS")]
    pub total_ln_ls_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLNLSR`
    Title: TOTAL LN&LS RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLNLSR")]
    pub total_ln_ls_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLNLSQ`
    Title: TOTAL LN&LS RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLNLSQ")]
    pub total_ln_ls_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLNLSQR`
    Title: TOTAL LN&LS RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLNLSQR")]
    pub total_ln_ls_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CUSLI`
    Title: CUSTOMERS ACCEPTANCES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CUSLI")]
    pub customers_acceptances: Option<f64>,

    #[doc = r#"## FDIC Field:: `DDT`
    Title: DDA TRANS-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DDT")]
    pub dda_trans_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `DDTR`
    Title: DDA TRANS-TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DDTR")]
    pub dda_trans_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPFOR`
    Title: TOTAL DEPOSITS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPFOR")]
    pub total_deposits_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPFORR`
    Title: TOTAL DEPOSITS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPFORR")]
    pub total_deposits_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPI`
    Title: INTEREST-BEARING DEP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPI")]
    pub interest_bearing_dep: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIFOR`
    Title: INTEREST-BEARING DEP-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIFOR")]
    pub interest_bearing_dep_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIFORR`
    Title: INTEREST-BEARING DEP-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIFORR")]
    pub interest_bearing_dep_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIPCCF`
    Title: IPC & OFFICIAL CHECKS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIPCCF")]
    pub ipc_official_checks_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIPCCFR`
    Title: IPC & OFFICIAL CHECKS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIPCCFR")]
    pub ipc_official_checks_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIPCF`
    Title: IPC-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIPCF")]
    pub ipc_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPNI`
    Title: NONINTEREST-BEARING DEP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPNI")]
    pub noninterest_bearing_dep: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPNIFOR`
    Title: NONINTEREST-BEARING DEP-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPNIFOR")]
    pub noninterest_bearing_dep_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPNIFORR`
    Title: NONINTEREST-BEARING DEP-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPNIFORR")]
    pub noninterest_bearing_dep_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLNLS`
    Title: TOTAL LN&LS CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLNLS")]
    pub total_ln_ls_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLNLSR`
    Title: TOTAL LN&LS CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLNLSR")]
    pub total_ln_ls_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLNLSQ`
    Title: TOTAL LN&LS CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLNLSQ")]
    pub total_ln_ls_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLNLSQR`
    Title: TOTAL LN&LS CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLNLSQR")]
    pub total_ln_ls_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EAMINTAN`
    Title: AMORT & IMPAIR LOSS AST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EAMINTAN")]
    pub amort_impair_loss_ast: Option<f64>,

    #[doc = r#"## FDIC Field:: `EAMINTANR`
    Title: AMORT & IMPAIR LOSS AST RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EAMINTANR")]
    pub amort_impair_loss_ast_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EAMINTQ`
    Title: AMORT & IMPAIR LOSS AST QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EAMINTQ")]
    pub amort_impair_loss_ast_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EAMINTQR`
    Title: AMORT & IMPAIR LOSS AST QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EAMINTQR")]
    pub amort_impair_loss_ast_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEP`
    Title: DEPOSIT INTEREST EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEP")]
    pub deposit_interest_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPDOM`
    Title: DEPOSIT INTEREST EXPENSE-DOM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPDOM")]
    pub deposit_interest_expense_dom: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPDOMR`
    Title: DEPOSIT INTEREST EXPENSE-DOM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPDOMR")]
    pub deposit_interest_expense_dom_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPDOMQ`
    Title: DEPOSIT INTEREST EXPENSE-DOM QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPDOMQ")]
    pub deposit_interest_expense_dom_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPDOMQR`
    Title: DEPOSIT INTEREST EXPENSE-DOM QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPDOMQR")]
    pub deposit_interest_expense_dom_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPFOR`
    Title: DEPOSIT INTEREST EXPENSE-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPFOR")]
    pub deposit_interest_expense_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPFORR`
    Title: DEPOSIT INTEREST EXPENSE-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPFORR")]
    pub deposit_interest_expense_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPFORQ`
    Title: DEPOSIT INTEREST EXPENSE-FOR QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPFORQ")]
    pub deposit_interest_expense_for_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDEPFORQR`
    Title: DEPOSIT INTEREST EXPENSE-FOR QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDEPFORQR")]
    pub deposit_interest_expense_for_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EFHLBADV`
    Title: ADVANCES FROM FHLBANK INT EXP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EFHLBADV")]
    pub advances_from_fhlbank_int_exp: Option<f64>,

    #[doc = r#"## FDIC Field:: `EFREPP`
    Title: FED FUNDS & REPOS INT EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EFREPP")]
    pub fed_funds_repos_int_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `EFREPPR`
    Title: FED FUNDS & REPOS INT EXPENSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EFREPPR")]
    pub fed_funds_repos_int_expense_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EFREPPQ`
    Title: FED FUNDS & REPOS INT EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EFREPPQ")]
    pub fed_funds_repos_int_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EFREPPQR`
    Title: FED FUNDS & REPOS INT EXPENSE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EFREPPQR")]
    pub fed_funds_repos_int_expense_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTEXP`
    Title: TOTAL INTEREST EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTEXP")]
    pub total_interest_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTEXPR`
    Title: TOTAL INTEREST EXPENSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTEXPR")]
    pub total_interest_expense_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTXQ`
    Title: TOTAL INTEREST EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTXQ")]
    pub total_interest_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTXQA`
    Title: TOTAL INTEREST EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTXQA")]
    pub total_interest_expense_quarterly_eintxqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTEXPA`
    Title: TOTAL INTEREST EXPENSE ANNUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTEXPA")]
    pub total_interest_expense_annually: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTXQR`
    Title: TOTAL INTEREST EXPENSE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTXQR")]
    pub total_interest_expense_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATR`
    Title: PROVISIONS FOR CREDIT LOSSES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATR")]
    pub provisions_for_credit_losses: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATRR`
    Title: PROVISIONS FOR CREDIT LOSSES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATRR")]
    pub provisions_for_credit_losses_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATQ`
    Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATQ")]
    pub provisions_for_credit_losses_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATQA`
    Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATQA")]
    pub provisions_for_credit_losses_quarterly_elnatqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATQR`
    Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATQR")]
    pub provisions_for_credit_losses_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNLOSQ`
    Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNLOSQ")]
    pub provisions_for_credit_losses_quarterly_ratio_elnlosq: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTTOTQ`
    Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTTOTQ")]
    pub provisions_for_credit_losses_quarterly_ratio_nttotq: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNLOS`
    Title: PROVISIONS FOR LN & LEASE LOSSES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNLOS")]
    pub provisions_for_ln_lease_losses: Option<f64>,

    #[doc = r#"## FDIC Field:: `EMTGLS`
    Title: MORTGAGE DEBT INTEREST EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EMTGLS")]
    pub mortgage_debt_interest_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONINTEXP`
    Title: ADDITIONAL NONINTEREST EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONINTEXP")]
    pub additional_noninterest_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONINTEXPR`
    Title: ADDITIONAL NONINTEREST EXPENSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONINTEXPR")]
    pub additional_noninterest_expense_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONINTEXPQ`
    Title: ADDITIONAL NONINTEREST EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONINTEXPQ")]
    pub additional_noninterest_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONINTEXPQR`
    Title: ADDITIONAL NONINTEREST EXPENSE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONINTEXPQR")]
    pub additional_noninterest_expense_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHNINT`
    Title: ALL OTHER NONINTEREST EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHNINT")]
    pub all_other_noninterest_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHNINTR`
    Title: ALL OTHER NONINTEREST EXPENSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHNINTR")]
    pub all_other_noninterest_expense_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHNINQ`
    Title: ALL OTHER NONINTEREST EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHNINQ")]
    pub all_other_noninterest_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHNINQR`
    Title: ALL OTHER NONINTEREST EXPENSE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHNINQR")]
    pub all_other_noninterest_expense_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EPREMAGG`
    Title: PREMISES & FIXED ASSETS EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EPREMAGG")]
    pub premises_fixed_assets_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `EPREMAGGR`
    Title: PREMISES & EQUIPMENT EXPENSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EPREMAGGR")]
    pub premises_equipment_expense_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EPREMAGQ`
    Title: PREMISES & FIXED ASSETS EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EPREMAGQ")]
    pub premises_fixed_assets_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EPREMAGQR`
    Title: PREMISES & EQUIPMENT EXPENSE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EPREMAGQR")]
    pub premises_equipment_expense_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIV`
    Title: CASH DIVIDENDS ON COMM & PREF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIV")]
    pub cash_dividends_on_comm_pref: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVR`
    Title: CASH DIVIDENDS ON COMM & PREF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVR")]
    pub cash_dividends_on_comm_pref_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVC`
    Title: CASH DIVIDENDS ON COMM STOCK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVC")]
    pub cash_dividends_on_comm_stock: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVCR`
    Title: CASH DIVIDENDS ON COMM STOCK RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVCR")]
    pub cash_dividends_on_comm_stock_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVP`
    Title: CASH DIVIDENDS ON PREF STOCK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVP")]
    pub cash_dividends_on_pref_stock: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVPR`
    Title: CASH DIVIDENDS ON PREF STOCK RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVPR")]
    pub cash_dividends_on_pref_stock_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVQ`
    Title: CASH DIVIDENDS ON COMM & PREF QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVQ")]
    pub cash_dividends_on_comm_pref_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVQR`
    Title: CASH DIVIDENDS ON COMM & PREF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVQR")]
    pub cash_dividends_on_comm_pref_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCFCTA`
    Title: EQCFCTA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCFCTA")]
    pub eqcfcta: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCONSUB`
    Title: MINOR INT IN CONSOL SUBS-EQ
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCONSUB")]
    pub minor_int_in_consol_subs_eq: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCS`
    Title: COMMON STOCK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCS")]
    pub common_stock: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCSR`
    Title: COMMON STOCK RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCSR")]
    pub common_stock_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQNWCERT`
    Title: NET WORTH CERTIFICATES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQNWCERT")]
    pub net_worth_certificates: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQOTHCC`
    Title: OTHER EQUITY CAPITAL COMPONENTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQOTHCC")]
    pub other_equity_capital_components: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQPP`
    Title: PERPETUAL PREFERRED STOCK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQPP")]
    pub perpetual_preferred_stock: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQPPR`
    Title: PERPETUAL PREFERRED STOCK RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQPPR")]
    pub perpetual_preferred_stock_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQSUR`
    Title: SURPLUS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQSUR")]
    pub surplus: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQSURR`
    Title: SURPLUS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQSURR")]
    pub surplus_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQUP`
    Title: EQUP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQUP")]
    pub equp: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQUPTOT`
    Title: UP-NET & OTHER CAPITAL COMP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQUPTOT")]
    pub up_net_other_capital_comp: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQUPTOTR`
    Title: UP-NET & OTHER CAPITAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQUPTOTR")]
    pub up_net_other_capital_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESAL`
    Title: SALARIES AND EMPLOYEE BENEFITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESAL")]
    pub salaries_and_employee_benefits: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESALR`
    Title: SALARIES AND EMPLOYEE BENEFITS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESALR")]
    pub salaries_and_employee_benefits_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESALQ`
    Title: SALARIES AND EMPLOYEE BENEFITS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESALQ")]
    pub salaries_and_employee_benefits_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESALQR`
    Title: SALARIES AND EMPLOYEE BENEFITS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESALQR")]
    pub salaries_and_employee_benefits_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESUBND`
    Title: SUBORDINATED NOTES INT EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESUBND")]
    pub subordinated_notes_int_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `ETTLOTBO`
    Title: TT&L & OTHER BORROWINGS INT EXP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ETTLOTBO")]
    pub tt_l_other_borrowings_int_exp: Option<f64>,

    #[doc = r#"## FDIC Field:: `EXTRA`
    Title: NET DISCONTINUED OPERATIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EXTRA")]
    pub net_discontinued_operations: Option<f64>,

    #[doc = r#"## FDIC Field:: `EXTRAR`
    Title: NET DISCONTINUED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EXTRAR")]
    pub net_discontinued_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EXTRAQ`
    Title: NET DISCONTINUED OPERATIONS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EXTRAQ")]
    pub net_discontinued_operations_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EXTRAQR`
    Title: NET DISCONTINUED OPERATIONS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EXTRAQR")]
    pub net_discontinued_operations_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICDBS`
    Title: FDIC REGION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FDICDBS")]
    pub fdic_region: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICDBSDESC`
    Title: FDIC REGION DESC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FDICDBSDESC")]
    pub fdic_region_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `FDICSUPV`
    Title: FDIC REGION - SUPERVISORY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FDICSUPV")]
    pub fdic_region_supervisory: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICSUPVDESC`
    Title: FDIC REGION - SUPERVISORY DESC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FDICSUPVDESC")]
    pub fdic_region_supervisory_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `FED`
    Title: FED DISTRICT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FED")]
    pub fed_district: Option<f64>,

    #[doc = r#"## FDIC Field:: `FEDDESC`
    Title: FED DISTRICT DESC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FEDDESC")]
    pub fed_district_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `FEDCHRTR`
    Title: FEDERAL CHARTER FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FEDCHRTR")]
    pub federal_charter_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `FLDOFF`
    Title: FDIC RISK MANAGEMENT FIELD OFFICE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FLDOFF")]
    pub fdic_risk_management_field_office: Option<String>,

    #[doc = r#"## FDIC Field:: `FORCHRTR`
    Title: FOREIGN CHARTER FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FORCHRTR")]
    pub foreign_charter_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `FORMCFR`
    Title: COMMERCIAL FINANCIAL REPORT FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FORMCFR")]
    pub commercial_financial_report_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `FREPO`
    Title: FED FUNDS & REPOS SOLD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FREPO")]
    pub fed_funds_repos_sold: Option<f64>,

    #[doc = r#"## FDIC Field:: `FREPOR`
    Title: FED FUNDS & REPOS SOLD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FREPOR")]
    pub fed_funds_repos_sold_frepor: Option<f64>,

    #[doc = r#"## FDIC Field:: `FREPP`
    Title: FED FUNDS & REPOS PURCHASED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FREPP")]
    pub fed_funds_repos_purchased: Option<f64>,

    #[doc = r#"## FDIC Field:: `FREPPR`
    Title: FED FUNDS & REPOS PURCHASED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FREPPR")]
    pub fed_funds_repos_purchased_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRSMEM`
    Title: FRS MEMBER FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FRSMEM")]
    pub frs_member_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `HCTONE`
    Title: MEMBER OF A ONE BANK HOLDING CO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="HCTONE")]
    pub member_of_a_one_bank_holding_co: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBA`
    Title: INTL BANKING ACT ENTITY FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBA")]
    pub intl_banking_act_entity_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBEFTAX`
    Title: INCOME BEFORE INC TAXES & DISC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBEFTAX")]
    pub income_before_inc_taxes_disc: Option<f64>,

    #[doc = r#"## FDIC Field:: `ICHBAL`
    Title: DEPOSITORY INSTITUTIONS INT INC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ICHBAL")]
    pub depository_institutions_int_inc: Option<f64>,

    #[doc = r#"## FDIC Field:: `ICHBALR`
    Title: BALANCES FROM DEPOSITORY INSTITUTIONS YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ICHBALR")]
    pub balances_from_depository_institutions_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ICHBALQ`
    Title: DEPOSITORY INSTITUTIONS INT INC QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ICHBALQ")]
    pub depository_institutions_int_inc_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ICHBALQR`
    Title: DEPOSITORY INSTITUTIONS INT INC QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ICHBALQR")]
    pub depository_institutions_int_inc_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFREPO`
    Title: FED FUNDS & REPO INTEREST INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFREPO")]
    pub fed_funds_repo_interest_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFREPOR`
    Title: FEDERAL FUNDS SOLD YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFREPOR")]
    pub federal_funds_sold_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFREPOQ`
    Title: FED FUNDS & REPO INTEREST INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFREPOQ")]
    pub fed_funds_repo_interest_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFREPOQR`
    Title: FED FUNDS & REPO INTEREST INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFREPOQR")]
    pub fed_funds_repo_interest_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLSEC`
    Title: SECURITIES GAINS AND LOSSES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLSEC")]
    pub securities_gains_and_losses: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLSECR`
    Title: SECURITIES GAINS AND LOSSES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLSECR")]
    pub securities_gains_and_losses_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLSECQR`
    Title: SECURITIES GAINS AND LOSSES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLSECQR")]
    pub securities_gains_and_losses_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNDOM`
    Title: LOAN INCOME-DOM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNDOM")]
    pub loan_income_dom: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNDOMR`
    Title: DOMESTIC OFFICE LOANS YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNDOMR")]
    pub domestic_office_loans_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNDOMQ`
    Title: LOAN INCOME-DOM QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNDOMQ")]
    pub loan_income_dom_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNDOMQR`
    Title: LOAN INCOME-DOM QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNDOMQR")]
    pub loan_income_dom_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNFOR`
    Title: LOAN INCOME-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNFOR")]
    pub loan_income_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNFORR`
    Title: FOREIGN OFFICE LOANS YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNFORR")]
    pub foreign_office_loans_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNFORQ`
    Title: LOAN INCOME-FOR QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNFORQ")]
    pub loan_income_for_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNFORQR`
    Title: LOAN INCOME-FOR QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNFORQR")]
    pub loan_income_for_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILS`
    Title: LEASE INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILS")]
    pub lease_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILSR`
    Title: LEASE FINANCING RECEIVABLES YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILSR")]
    pub lease_financing_receivables_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILSQ`
    Title: LEASE INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILSQ")]
    pub lease_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILSQR`
    Title: LEASE INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILSQR")]
    pub lease_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSALL`
    Title: INSURED INSTITUTION FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSALL")]
    pub insured_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSCOML`
    Title: INSURED COMMERCIAL FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSCOML")]
    pub insured_commercial_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSFDIC`
    Title: FDIC INSURED FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSFDIC")]
    pub fdic_insured_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSNONE`
    Title: NOT FEDERALLY INSURED FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSNONE")]
    pub not_federally_insured_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSSAVE`
    Title: INSURED SAVINGS INSTITUTION FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSSAVE")]
    pub insured_savings_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSTCOML`
    Title: COMMERCIAL INSTITUTION FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSTCOML")]
    pub commercial_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSTSAVE`
    Title: SAVING & S&L INSTITUTION FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSTSAVE")]
    pub saving_s_l_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSTTYPE`
    Title: INSTITUTION TYPE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSTTYPE")]
    pub institution_type: Option<String>,

    #[doc = r#"## FDIC Field:: `INTAN`
    Title: INTANGIBLE ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTAN")]
    pub intangible_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTANR`
    Title: INTANGIBLE ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANR")]
    pub intangible_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTEXPY`
    Title: INTEREST EXPENSE TO EARNING ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTEXPY")]
    pub interest_expense_to_earning_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTEXPYQ`
    Title: COST OF FUNDING EARNING ASSETS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTEXPYQ")]
    pub cost_of_funding_earning_assets_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINC`
    Title: TOTAL INTEREST INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINC")]
    pub total_interest_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINCR`
    Title: TOTAL INTEREST INCOME YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINCR")]
    pub total_interest_income_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINQ`
    Title: TOTAL INTEREST INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINQ")]
    pub total_interest_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINQR`
    Title: TOTAL INTEREST INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINQR")]
    pub total_interest_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINQA`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINQA")]
    pub missing_title: Option<f64>,

    #[doc = r#"## FDIC Field:: `INVSUB`
    Title: INVEST IN UNCONSOLIDATED SUBS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INVSUB")]
    pub invest_in_unconsolidated_subs: Option<f64>,

    #[doc = r#"## FDIC Field:: `INVSUORE`
    Title: INVESTMENTS IN RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INVSUORE")]
    pub investments_in_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTHFEE`
    Title: OTHER FEE INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTHFEE")]
    pub other_fee_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTHII`
    Title: OTHER INTEREST INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTHII")]
    pub other_interest_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTHIIR`
    Title: OTHER INTEREST INCOME YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTHIIR")]
    pub other_interest_income_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTHIIQ`
    Title: OTHER INTEREST INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTHIIQ")]
    pub other_interest_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTHIIQR`
    Title: OTHER INTEREST INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTHIIQR")]
    pub other_interest_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IRAKEOGH`
    Title: IRAS AND KEOGH PLANS-DEPOSITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IRAKEOGH")]
    pub iras_and_keogh_plans_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `IRAKEOGHR`
    Title: IRAS AND KEOGH PLANS-DEPOSITS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IRAKEOGHR")]
    pub iras_and_keogh_plans_deposits_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISC`
    Title: TOTAL SECURITY INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISC")]
    pub total_security_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISCR`
    Title: SECURITIES YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISCR")]
    pub securities_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISCQ`
    Title: TOTAL SECURITY INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISCQ")]
    pub total_security_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISCQR`
    Title: TOTAL SECURITY INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISCQR")]
    pub total_security_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERCHG`
    Title: SERVICE CHARGE ON DEPOSIT ACCTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERCHG")]
    pub service_charge_on_deposit_accts: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERCHGR`
    Title: SERVICE CHARGE ON DEPOSIT ACCTS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERCHGR")]
    pub service_charge_on_deposit_accts_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITAX`
    Title: APPLICABLE INCOME TAXES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITAX")]
    pub applicable_income_taxes: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITAXR`
    Title: APPLICABLE INCOME TAXES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITAXR")]
    pub applicable_income_taxes_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITAXQ`
    Title: APPLICABLE INCOME TAXES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITAXQ")]
    pub applicable_income_taxes_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITAXQR`
    Title: APPLICABLE INCOME TAXES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITAXQR")]
    pub applicable_income_taxes_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITRADE`
    Title: INTEREST INCOME ON TRADING ACCTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITRADE")]
    pub interest_income_on_trading_accts: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITRADER`
    Title: TRADING ACCOUNTS YTD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITRADER")]
    pub trading_accounts_ytd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITRADEQ`
    Title: INTEREST INCOME ON TRADING ACCTS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITRADEQ")]
    pub interest_income_on_trading_accts_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITRADEQR`
    Title: INTEREST INCOME ON TRADING ACCTS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITRADEQR")]
    pub interest_income_on_trading_accts_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LIAB`
    Title: TOTAL LIABILITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LIAB")]
    pub total_liabilities: Option<f64>,

    #[doc = r#"## FDIC Field:: `LIABR`
    Title: TOTAL LIABILITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LIABR")]
    pub total_liabilities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LIABEQ`
    Title: TOTAL LIABILITIES & CAPITAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LIABEQ")]
    pub total_liabilities_capital: Option<f64>,

    #[doc = r#"## FDIC Field:: `LIABEQR`
    Title: TOTAL LIABILITIES & CAPITAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LIABEQR")]
    pub total_liabilities_capital_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LIPMTG`
    Title: MORTGAGE LOANS IN PROCESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LIPMTG")]
    pub mortgage_loans_in_process: Option<f64>,

    #[doc = r#"## FDIC Field:: `LLPFDSTK`
    Title: LIMITED-LIFE PREFERRED STOCK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LLPFDSTK")]
    pub limited_life_preferred_stock: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNACOTH`
    Title: ACCEPTANCES OF OTHER BANKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNACOTH")]
    pub acceptances_of_other_banks: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG`
    Title: AGRICULTURAL LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG")]
    pub agricultural_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAGR`
    Title: AGRICULTURAL LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAGR")]
    pub agricultural_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNATRES`
    Title: ALLOW FOR LOANS LOSS ADJUSTED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNATRES")]
    pub allow_for_loans_loss_adjusted: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNATRESJ`
    Title: ALLOW FOR LOANS + ALLOC TRN RISK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNATRESJ")]
    pub allow_for_loans_alloc_trn_risk: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNATRESRR`
    Title: ALLOW FOR LOANS + ALLOC TRN RISK RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNATRESRR")]
    pub allow_for_loans_alloc_trn_risk_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAUTO`
    Title: CONSUMER LOANS - AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAUTO")]
    pub consumer_loans_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAUTOR`
    Title: CONSUMER LOANS-AUTO RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAUTOR")]
    pub consumer_loans_auto_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI`
    Title: C&I LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI")]
    pub c_i_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCIR`
    Title: C&I LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCIR")]
    pub c_i_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCON`
    Title: CONSUMER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCON")]
    pub consumer_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONR`
    Title: CONSUMER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONR")]
    pub consumer_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONOT1`
    Title: CONSUMER LOANS-HOME IMPROVEMENT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONOT1")]
    pub consumer_loans_home_improvement: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONOTH`
    Title: CONSUMER LOANS-OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONOTH")]
    pub consumer_loans_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONOTHR`
    Title: CONSUMER LOANS-OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONOTHR")]
    pub consumer_loans_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCRCD`
    Title: CONSUMER LOANS-CREDIT CARD PLAN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCRCD")]
    pub consumer_loans_credit_card_plan: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCRCDR`
    Title: CONSUMER LOANS-CREDIT CARD PLAN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCRCDR")]
    pub consumer_loans_credit_card_plan_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCRCDRP`
    Title: LNS-CREDIT CD & RELATED PLAN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCRCDRP")]
    pub lns_credit_cd_related_plan: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEP`
    Title: DEP INSTITUTION LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEP")]
    pub dep_institution_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNFG`
    Title: FOREIGN GOVT LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNFG")]
    pub foreign_govt_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNFGR`
    Title: FOREIGN GOVT LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNFGR")]
    pub foreign_govt_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLS`
    Title: LN&LS + UNEARNED INC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLS")]
    pub ln_ls_unearned_inc: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGR`
    Title: LOANS AND LEASES-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGR")]
    pub loans_and_leases_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGR2`
    Title: LOANS AND LEASES-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGR2")]
    pub loans_and_leases_total_lnlsgr2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGRJ`
    Title: LOANS AND LEASES-TOTAL ADJUSTED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGRJ")]
    pub loans_and_leases_total_adjusted: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGRR`
    Title: LOANS AND LEASES-TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGRR")]
    pub loans_and_leases_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSNET`
    Title: LOANS AND LEASES-NET
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSNET")]
    pub loans_and_leases_net: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSNETR`
    Title: LOANS AND LEASES-NET RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSNETR")]
    pub loans_and_leases_net_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNMUNI`
    Title: MUNI LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNMUNI")]
    pub muni_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNMUNIR`
    Title: MUNI LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNMUNIR")]
    pub muni_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTCI`
    Title: OTHER LNS & LS-COMM-QBP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTCI")]
    pub other_lns_ls_comm_qbp: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTCIR`
    Title: OTHER LNS & LS-COMM-QBP RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTCIR")]
    pub other_lns_ls_comm_qbp_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTHER`
    Title: LN TO NONDEP FIN INST & OTH LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTHER")]
    pub ln_to_nondep_fin_inst_oth_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNSOTHER`
    Title: OTHER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNSOTHER")]
    pub other_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNSOTHERR`
    Title: OTHER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNSOTHERR")]
    pub other_loans_lnsotherr: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRE`
    Title: RE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRE")]
    pub re_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRE2`
    Title: RE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRE2")]
    pub re_loans_lnre2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECON2`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECON2")]
    pub missing_title_lnrecon2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREMUL2`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREMUL2")]
    pub missing_title_lnremul2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREJ`
    Title: RE LOANS ADJUSTED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREJ")]
    pub re_loans_adjusted: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRE5`
    Title: RE LOANS CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRE5")]
    pub re_loans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRER`
    Title: RE LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRER")]
    pub re_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG`
    Title: RE AGRICULTURAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG")]
    pub re_agricultural: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECON5`
    Title: RE CONSTRUCTION & LAND DEV-CAV5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECON5")]
    pub re_construction_land_dev_cav5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAGR`
    Title: RE AGRICULTURAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAGR")]
    pub re_agricultural_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECONS`
    Title: RE CONSTRUCTION & LAND DEVELOP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECONS")]
    pub re_construction_land_develop: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECONSR`
    Title: RE CONSTRUCTION & LAND DEVELOP RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECONSR")]
    pub re_construction_land_develop_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREDOM`
    Title: RE LOANS-DOM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREDOM")]
    pub re_loans_dom: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREDOMR`
    Title: RE LOANS-DOM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREDOMR")]
    pub re_loans_dom_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREFOR`
    Title: RE LOANS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREFOR")]
    pub re_loans_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREFORR`
    Title: RE LOANS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREFORR")]
    pub re_loans_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRELOC`
    Title: RE 1-4 FAMILY-LINE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRELOC")]
    pub re_1_4_family_line: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRELOCR`
    Title: RE 1-4 FAMILY-LINE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRELOCR")]
    pub re_1_4_family_line_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRELOC2`
    Title: RE 1-4 FAMILY-LINE2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRELOC2")]
    pub re_1_4_family_line2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRELOC5`
    Title: RE 1-4 FAMILY-LINE-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRELOC5")]
    pub re_1_4_family_line_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREMULT`
    Title: RE MULTIFAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREMULT")]
    pub re_multifamily: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREMUL5`
    Title: RE MULTIFAMILY-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREMUL5")]
    pub re_multifamily_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREMULTR`
    Title: RE MULTIFAMILY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREMULTR")]
    pub re_multifamily_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENRES`
    Title: RE NONFARM NONRESIDENTIAL PROP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENRES")]
    pub re_nonfarm_nonresidential_prop: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENRE5`
    Title: RE NONFARM NONRESIDENTIAL CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENRE5")]
    pub re_nonfarm_nonresidential_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENRE2`
    Title: RE NONFARM NONRESIDENTIAL CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENRE2")]
    pub re_nonfarm_nonresidential_cavg5_lnrenre2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENRESR`
    Title: RE NONFARM NONRESIDENTIAL PROP RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENRESR")]
    pub re_nonfarm_nonresidential_prop_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREPP`
    Title: PREPAID TAXES & INS ON MTG LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREPP")]
    pub prepaid_taxes_ins_on_mtg_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERES`
    Title: RE 1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERES")]
    pub re_1_4_family: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERESR`
    Title: RE 1-4 FAMILY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERESR")]
    pub re_1_4_family_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERES2`
    Title: RE 1-4 FAMILY2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERES2")]
    pub re_1_4_family2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERES5`
    Title: RE 1-4 FAMILY-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERES5")]
    pub re_1_4_family_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRESRE`
    Title: ALLOWANCE FOR RE LOAN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRESRE")]
    pub allowance_for_re_loan: Option<f64>,

    #[doc = r#"## FDIC Field:: `LS`
    Title: LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LS")]
    pub leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSR`
    Title: LEASES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSR")]
    pub leases_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `METRO`
    Title: METROPOLITAN FLAG
    Description: A flag used to indicate whether an institution is in a metropolitan statistical area. The U.S census bureau office of management and budget defines the metropolitan statistical area. A core based statistical area associated with at least one urbanized area that has a population of at least 50,000. The metropolitan statistical area comprises the central county or counties containing the core, plus adjacent outlying counties having a high degree of social and economic integration with the central county as measured through commuting. 0=institution is not in a metropolitan statistical area. 1=institution is in a metropolitan statistical area."#]
    #[serde(rename="METRO")]
    pub metropolitan_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `MI`
    Title: INSURED SAVINGS BANK FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MI")]
    pub insured_savings_bank_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `MICRO`
    Title: MICROPOLITAN FLAG
    Description: A flag used to indicate whether an institution is in a micropolitan statistical area. The U.S census bureau office of management and budget defines the micropolitan statistical area. A core based statistical area associated with at least one urban cluster that has a population of at least 10,000 but less than 50,000. The micropolitan statistical area comprises the central county or counties containing the core, plus adjacent outlying counties having a high degree of social and economic integration with the central county as measured through commuting. 0=institution is not in a micropolitan statistical area. 1=institution is in a micropolitan statistical area."#]
    #[serde(rename="MICRO")]
    pub micropolitan_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `MNRTYCDE`
    Title: MINORITY CODE
    Description: A character field on the institution file corresponding to a type of minority ownership. .=NONE. 01=African American. 02=Hispanic American. 03=Asian or Pacific Islander Americans. 04=Native American or Native Alaskan American. 05=Multi-Racial American. 06=Minority Board and serving African American Community. 08=Minority Board and serving Asian/Pacific Islander Americans. 10=Minority Board and serving Multi-Racial community."#]
    #[serde(rename="MNRTYCDE")]
    pub minority_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `MNRTYDTE`
    Title: EFFECTIVE DTE OF MINORITY STATUS
    Description: Represent the effective date on which an institution is assigned a minority status, transaction in dates. Format(DDMONCCYY) day, month abbrev, century, and year."#]
    #[serde(rename="MNRTYDTE")]
    pub effective_dte_of_minority_status: Option<f64>,

    #[doc = r#"## FDIC Field:: `MTGLS`
    Title: MORTGAGE INDEBTEDNESS & CAP LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MTGLS")]
    pub mortgage_indebtedness_cap_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `N`
    Title: NATIONAL BANK FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="N")]
    pub national_bank_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALNLS`
    Title: NONACCRUAL-LOANS & LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALNLS")]
    pub nonaccrual_loans_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `NC`
    Title: NONINSURED COMMERCIAL INST FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NC")]
    pub noninsured_commercial_inst_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCLNLS`
    Title: TOTAL N/C-LOANS & LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCLNLS")]
    pub total_n_c_loans_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETIMIN`
    Title: NET INC - ATTRIB TO MINORITY INT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETIMIN")]
    pub net_inc_attrib_to_minority_int: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETIMINR`
    Title: NET INC - ATTRIB TO MINORITY INT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETIMINR")]
    pub net_inc_attrib_to_minority_int_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETIMINQ`
    Title: NET INC - ATTRIB TO MINORITY INT QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETIMINQ")]
    pub net_inc_attrib_to_minority_int_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETIMINQR`
    Title: NET INC - ATTRIB TO MINORITY INT QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETIMINQR")]
    pub net_inc_attrib_to_minority_int_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINBM`
    Title: NET INC - BANK & MINORITY INT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINBM")]
    pub net_inc_bank_minority_int: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINBMR`
    Title: NET INC - BANK & MINORITY INT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINBMR")]
    pub net_inc_bank_minority_int_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINBMQ`
    Title: NET INC - BANK & MINORITY INT QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINBMQ")]
    pub net_inc_bank_minority_int_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINBXA`
    Title: NET INCOME BEFORE TAXES ANNUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINBXA")]
    pub net_income_before_taxes_annually: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETIBXQA`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETIBXQA")]
    pub missing_title_netibxqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINBMQR`
    Title: NET INC - BANK & MINORITY INT QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINBMQR")]
    pub net_inc_bank_minority_int_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NEWINST`
    Title: NEW INSTITUTION FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NEWINST")]
    pub new_institution_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `NFAA`
    Title: NUMBER OF FIDUCIARY ACCOUNTS AND RELATED ASSET ACCOUNTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NFAA")]
    pub number_of_fiduciary_accounts_and_related_asset_accounts: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIM`
    Title: NET INTEREST INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIM")]
    pub net_interest_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIMR`
    Title: NET INTEREST INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIMR")]
    pub net_interest_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIMQ`
    Title: NET INTEREST INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIMQ")]
    pub net_interest_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIMQA`
    Title: NET INTEREST INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIMQA")]
    pub net_interest_income_quarterly_nimqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIMA`
    Title: NET INTEREST INCOME ANNUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIMA")]
    pub net_interest_income_annually: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIMQR`
    Title: NET INTEREST INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIMQR")]
    pub net_interest_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NM`
    Title: NONMEMBER INSURED INST FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NM")]
    pub nonmember_insured_inst_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONII`
    Title: TOTAL NONINTEREST INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONII")]
    pub total_noninterest_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIIR`
    Title: TOTAL NONINTEREST INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIIR")]
    pub total_noninterest_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIX`
    Title: TOTAL NONINTEREST EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIX")]
    pub total_noninterest_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIXR`
    Title: TOTAL NONINTEREST EXPENSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIXR")]
    pub total_noninterest_expense_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIXQ`
    Title: TOTAL NONINTEREST EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIXQ")]
    pub total_noninterest_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIXQA`
    Title: TOTAL NONINTEREST EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIXQA")]
    pub total_noninterest_expense_quarterly_nonixqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIXQR`
    Title: TOTAL NONINTEREST EXPENSE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIXQR")]
    pub total_noninterest_expense_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NS`
    Title: NONINSURED SAVINGS INST FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NS")]
    pub noninsured_savings_inst_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLS`
    Title: TOTAL LN&LS NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLS")]
    pub total_ln_ls_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLSCOR`
    Title: TOTAL LN&LS NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLSCOR")]
    pub total_ln_ls_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLSQ`
    Title: TOTAL LN&LS NET CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLSQ")]
    pub total_ln_ls_net_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLSQA`
    Title: TOTAL LN&LS NET CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLSQA")]
    pub total_ln_ls_net_charge_offs_quarterly_ntlnlsqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLSCOQR`
    Title: TOTAL LN&LS NET CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLSCOQR")]
    pub total_ln_ls_net_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTR`
    Title: NONTRANSACTION-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTR")]
    pub nontransaction_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRR`
    Title: NONTRANSACTION-TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRR")]
    pub nontransaction_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRIPC`
    Title: NONTRANSACTION-IPC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRIPC")]
    pub nontransaction_ipc: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRIPCR`
    Title: NONTRANSACTION-IPC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRIPCR")]
    pub nontransaction_ipc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRMUNI`
    Title: NONTRANSACTION-MUNI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRMUNI")]
    pub nontransaction_muni: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRMUNIR`
    Title: NONTRANSACTION-MUNI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRMUNIR")]
    pub nontransaction_muni_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRTIME`
    Title: TIME DEPOSITS-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRTIME")]
    pub time_deposits_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRTMLG`
    Title: TIME DEPOSITS OVER $100M
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRTMLG")]
    pub time_deposits_over_100m: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRTMLGJ`
    Title: AMT TOTAL TIME DEP MORE THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRTMLGJ")]
    pub amt_total_time_dep_more_than_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRTMLGJR`
    Title: AMT TOTAL TIME DEP MORE THAN $250,000 RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRTMLGJR")]
    pub amt_total_time_dep_more_than_250_000_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRTMMED`
    Title: AMT TIME DEP OF $250,000 OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRTMMED")]
    pub amt_time_dep_of_250_000_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRTMMEDR`
    Title: AMT TIME DEP OF $250,000 OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRTMMEDR")]
    pub amt_time_dep_of_250_000_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRUSGOV`
    Title: NONTRANSACTION-U.S. GOVERNMENT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRUSGOV")]
    pub nontransaction_u_s_government: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRUSGOVR`
    Title: NONTRANSACTION-U.S. GOVERNMENT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRUSGOVR")]
    pub nontransaction_u_s_government_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTIRTA`
    Title: RETAINED EARNINGS ANUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTIRTA")]
    pub retained_earnings_anually: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTTOT`
    Title: TOTAL LN & LS LOSS NET CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTTOT")]
    pub total_ln_ls_loss_net_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NUMEMP`
    Title: NUMBER OF FULL TIME EMPLOYEES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NUMEMP")]
    pub number_of_full_time_employees: Option<f64>,

    #[doc = r#"## FDIC Field:: `OA`
    Title: OTHER ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OA")]
    pub other_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `OAKAR`
    Title: OAKAR FLAG
    Description: A flag used to indicate whether an institution acquired deposits that were previously insured under a different insurance fund. 0=has no oakar deposits. 1=has oakar deposits."#]
    #[serde(rename="OAKAR")]
    pub oakar_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `OCCDIST`
    Title: OCC DISTRICT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OCCDIST")]
    pub occ_district: Option<f64>,

    #[doc = r#"## FDIC Field:: `OCCDISTDESC`
    Title: OCC DISTRICT DESC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OCCDISTDESC")]
    pub occ_district_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `OFFDMULT`
    Title: DOMESTIC MULTI-SERVICE OFFICES
    Description: The number of multiple service domestic offices operated by an institution."#]
    #[serde(rename="OFFDMULT")]
    pub domestic_multi_service_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFNDOM`
    Title: NONDOMESTIC OFFICES
    Description: The number of nondomestic offices operated by an institution."#]
    #[serde(rename="OFFNDOM")]
    pub nondomestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFOTH`
    Title: DOMESTIC OTHER OFFICES
    Description: The number of domestic non-multiple service offices operated by an institution."#]
    #[serde(rename="OFFOTH")]
    pub domestic_other_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFSOD`
    Title: SOD OFFICES
    Description: The number of offices operated by an institution based on the summary of deposits definition of offices."#]
    #[serde(rename="OFFSOD")]
    pub sod_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFSTATE`
    Title: NUMBER OF STATES WITH OFFICES
    Description: The number of states with offices (including its main office)."#]
    #[serde(rename="OFFSTATE")]
    pub number_of_states_with_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFTOT`
    Title: TOTAL OFFICES
    Description: The total number of offices operated by an institution."#]
    #[serde(rename="OFFTOT")]
    pub total_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFUSOA`
    Title: U.S. AND OTHER AREA OFFICES
    Description: The number of domestic and U.S terrirtories offices operated by an institution."#]
    #[serde(rename="OFFUSOA")]
    pub u_s_and_other_area_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OI`
    Title: INSURED IBA OFFICE FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OI")]
    pub insured_iba_office_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTSDIST`
    Title: OTS DISTRICT
    Description: A number used to identify the office of thrift supervision district in which the institution is located. 01=Northeast. 02=Southeast. 03=Midwest. 04=West."#]
    #[serde(rename="OTSDIST")]
    pub ots_district: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTSREGNO`
    Title: OTS REGION NUMBER
    Description: A number used to identify the office of thrift supervision region in which the institution is located. 01=Northeast. 02=Southeast. 03=Midwest. 04=West."#]
    #[serde(rename="OTSREGNO")]
    pub ots_region_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `OLMIN`
    Title: OTHER LIAB & MINOR IN SUBS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OLMIN")]
    pub other_liab_minor_in_subs: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORE`
    Title: OTHER REAL ESTATE OWNED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORE")]
    pub other_real_estate_owned: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORER`
    Title: OTHER REAL ESTATE OWNED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORER")]
    pub other_real_estate_owned_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBFHLB`
    Title: OTHER LIABILITIES-FHLB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBFHLB")]
    pub other_liabilities_fhlb: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBFHLBR`
    Title: OTHER LIABILITIES-FHLB RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBFHLBR")]
    pub other_liabilities_fhlb_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBOR`
    Title: OTHER BORROWED MONEY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBOR")]
    pub other_borrowed_money: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBRF`
    Title: OTH BORROWED FUNDS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBRF")]
    pub oth_borrowed_funds: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBRFR`
    Title: OTH BORROWED FUNDS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBRFR")]
    pub oth_borrowed_funds_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFH1L`
    Title: FHLB ADV MAT REP ONE YR OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFH1L")]
    pub fhlb_adv_mat_rep_one_yr_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFH1LR`
    Title: FHLB ADV MAT REP ONE YR OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFH1LR")]
    pub fhlb_adv_mat_rep_one_yr_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFH1T3`
    Title: FHLB ADV MAT REP ONE YR THROUGH THREE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFH1T3")]
    pub fhlb_adv_mat_rep_one_yr_through_three: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFH1T3R`
    Title: FHLB ADV MAT REP ONE YR THROUGH THREE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFH1T3R")]
    pub fhlb_adv_mat_rep_one_yr_through_three_otbfh1t3r: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFH3T5`
    Title: FHLB ADV MAT REP THREE THROUGH FIVE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFH3T5")]
    pub fhlb_adv_mat_rep_three_through_five: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFH3T5R`
    Title: FHLB ADV MAT REP THREE THROUGH FIVE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFH3T5R")]
    pub fhlb_adv_mat_rep_three_through_five_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFHOV5`
    Title: FHLB ADV MAT REP OVER FIVE YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFHOV5")]
    pub fhlb_adv_mat_rep_over_five_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFHOV5R`
    Title: FHLB ADV MAT REP OVER FIVE YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFHOV5R")]
    pub fhlb_adv_mat_rep_over_five_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBFH1L`
    Title: FHLB ADV WITH REMAINING MAT ONE YR OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBFH1L")]
    pub fhlb_adv_with_remaining_mat_one_yr_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBFH1LR`
    Title: FHLB ADV WITH REMAINING MAT ONE YR OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBFH1LR")]
    pub fhlb_adv_with_remaining_mat_one_yr_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFHSTA`
    Title: FHLB STRUCTURED ADV
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFHSTA")]
    pub fhlb_structured_adv: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBFHSTAR`
    Title: FHLB STRUCTURED ADV
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBFHSTAR")]
    pub fhlb_structured_adv_otbfhstar: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOT1L`
    Title: OTH BORR MAT OR NEXT REPRICING ONE YR OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOT1L")]
    pub oth_borr_mat_or_next_repricing_one_yr_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOT1LR`
    Title: OTH BORR MAT OR NEXT REPRICING ONE YR OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOT1LR")]
    pub oth_borr_mat_or_next_repricing_one_yr_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOT1T3`
    Title: OTH BORR MAT OR NEXT REPRICING ONE YR THROUGH THREE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOT1T3")]
    pub oth_borr_mat_or_next_repricing_one_yr_through_three: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOT1T3R`
    Title: OTH BORR MAT OR NEXT REPRICING ONE YR THROUGH THREE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOT1T3R")]
    pub oth_borr_mat_or_next_repricing_one_yr_through_three_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOT3T5`
    Title: OTH BORR MAT OR NEXT REPRICING THREE YR THROUGH FIVE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOT3T5")]
    pub oth_borr_mat_or_next_repricing_three_yr_through_five: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOT3T5R`
    Title: OTH BORR MAT OR NEXT REPRICING THREE YR THROUGH FIVE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOT3T5R")]
    pub oth_borr_mat_or_next_repricing_three_yr_through_five_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOTOV5`
    Title: OTH BORR MAT OR NEXT REPRICING OVER FIVE YRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOTOV5")]
    pub oth_borr_mat_or_next_repricing_over_five_yrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTBOTOV5R`
    Title: OTH BORR MAT OR NEXT REPRICING OVER FIVE YRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTBOTOV5R")]
    pub oth_borr_mat_or_next_repricing_over_five_yrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBOT1L`
    Title: OTH BORR MAT REMAINING MAT OF ONE YR OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBOT1L")]
    pub oth_borr_mat_remaining_mat_of_one_yr_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBOT1LR`
    Title: OTH BORR MAT REMANING MAT OF ONE YR OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBOT1LR")]
    pub oth_borr_mat_remaning_mat_of_one_yr_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ALLOTHL`
    Title: ALL OTHER LIABILITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ALLOTHL")]
    pub all_other_liabilities: Option<f64>,

    #[doc = r#"## FDIC Field:: `ALLOTHLR`
    Title: ALL OTHER LIABILITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ALLOTHLR")]
    pub all_other_liabilities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LNLS`
    Title: 30-89 DAYS P/D-LOANS & LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LNLS")]
    pub _30_89_days_p_d_loans_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LNLS`
    Title: 90+ DAYS P/D-LOANS & LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LNLS")]
    pub _90_days_p_d_loans_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `QBPRCOML`
    Title: QBP COMMERCIAL BANK REGION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="QBPRCOML")]
    pub qbp_commercial_bank_region: Option<f64>,

    #[doc = r#"## FDIC Field:: `QBPRCOMLDESC`
    Title: QBP COMMERCIAL BANK REGION DESC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="QBPRCOMLDESC")]
    pub qbp_commercial_bank_region_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `QBPRSAVB`
    Title: QBP BIF FUND SAVINGS REGION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="QBPRSAVB")]
    pub qbp_bif_fund_savings_region: Option<f64>,

    #[doc = r#"## FDIC Field:: `QBPRSAVS`
    Title: QBP SAVING SAIF FUND REGION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="QBPRSAVS")]
    pub qbp_saving_saif_fund_region: Option<f64>,

    #[doc = r#"## FDIC Field:: `QTRNO`
    Title: QUARTER NUMBER
    Description: Identifies the calendar quarter. 1=March. 2=June. 3=September. 4=December."#]
    #[serde(rename="QTRNO")]
    pub quarter_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `REGAGNT`
    Title: PRIMARY REGULATING AGENCY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="REGAGNT")]
    pub primary_regulating_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `RISKTERR`
    Title: FDIC RISK TERRITORY
    Description: An abbreviation of the current risk territory for an institution (FDIC Risk Territory). All periods are displayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#]
    #[serde(rename="RISKTERR")]
    pub fdic_risk_territory: Option<String>,

    #[doc = r#"## FDIC Field:: `S10T250B`
    Title: ASSETS 10B TO 250B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="S10T250B")]
    pub assets_10b_to_250b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SASSER`
    Title: SASSER FLAG
    Description: A flag used to indicate whether an institution was a former savings association that has converted to a bank charter and is still a SAIF insured institution. 0=not a sasser institution. 1=is a sasser institution."#]
    #[serde(rename="SASSER")]
    pub sasser_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SB`
    Title: SAVINGS BANK FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SB")]
    pub savings_bank_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SC`
    Title: SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SC")]
    pub securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCR`
    Title: SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCR")]
    pub securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCAA`
    Title: TOTAL AVAILABLE-FOR-SALE AT AMORTIZED COST SECURITIES ON A CONSOLIDATED BASIS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCAA")]
    pub total_available_for_sale_at_amortized_cost_securities_on_a_consolidated_basis: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCHF`
    Title: TOTAL HELD-TO-MATURITY AT FAIR VALUE SECURITIES ON A CONSOLIDATED BASIS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCHF")]
    pub total_held_to_maturity_at_fair_value_securities_on_a_consolidated_basis: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCAGE`
    Title: U.S. AGENCY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCAGE")]
    pub u_s_agency: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCASPNHA`
    Title: U.S. AGENCY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCASPNHA")]
    pub u_s_agency_scaspnha: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCASPNAF`
    Title: U.S. AGENCY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCASPNAF")]
    pub u_s_agency_scaspnaf: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCASPNSUM`
    Title: NON-MORT BACKED ISSUES BY US GOVT OR SPONSORED AGENCIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCASPNSUM")]
    pub non_mort_backed_issues_by_us_govt_or_sponsored_agencies: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCASPNSUMR`
    Title: NON-MORT BACKED ISSUES BY US GOVT OR SPONSORED AGENCIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCASPNSUMR")]
    pub non_mort_backed_issues_by_us_govt_or_sponsored_agencies_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCDEQ`
    Title: DOMESTIC SEC*DEBT & EQUITY - CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCDEQ")]
    pub domestic_sec_debt_equity_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCDOMO`
    Title: OTHER DOMESTIC DEBT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCDOMO")]
    pub other_domestic_debt: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCDOMOR`
    Title: OTHER DOMESTIC DEBT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCDOMOR")]
    pub other_domestic_debt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCEQ`
    Title: EQUITY SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCEQ")]
    pub equity_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCFDEQ`
    Title: FOREIGN DEBT & EQUITY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCFDEQ")]
    pub foreign_debt_equity: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCFORD`
    Title: FOREIGN DEBT SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCFORD")]
    pub foreign_debt_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCFORDR`
    Title: FOREIGN DEBT SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCFORDR")]
    pub foreign_debt_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMTGBK`
    Title: MORTGAGE BACKED SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMTGBK")]
    pub mortgage_backed_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMTGBKR`
    Title: MORTGAGE BACKED SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMTGBKR")]
    pub mortgage_backed_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMUNI`
    Title: MUNICIPAL SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMUNI")]
    pub municipal_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMUNIR`
    Title: MUNICIPAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMUNIR")]
    pub municipal_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMV`
    Title: SECURITIES-MV
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMV")]
    pub securities_mv: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCODPC`
    Title: RES-OTH DOM DEBT*PRIV CERTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCODPC")]
    pub res_oth_dom_debt_priv_certs: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCODPCR`
    Title: RES-OTH DOM DEBT*PRIV CERTS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCODPCR")]
    pub res_oth_dom_debt_priv_certs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCRES`
    Title: CONTRA-ASSETS TO SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCRES")]
    pub contra_assets_to_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCUS`
    Title: U.S. TREASURY & AGENCY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCUS")]
    pub u_s_treasury_agency: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCUSR`
    Title: U.S. TREASURY & AGENCY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCUSR")]
    pub u_s_treasury_agency_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCUSA`
    Title: U.S. AGENCY ALL OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCUSA")]
    pub u_s_agency_all_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCUST`
    Title: U.S. TREASURY SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCUST")]
    pub u_s_treasury_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCUSTR`
    Title: U.S. TREASURY SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCUSTR")]
    pub u_s_treasury_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SIMS_LAT`
    Title: GEOGRAPHIC LATITUDE OF MAIN OFFICE
    Description: Geographic latitude of main office."#]
    #[serde(rename="SIMS_LAT")]
    pub geographic_latitude_of_main_office: Option<f64>,

    #[doc = r#"## FDIC Field:: `SIMS_LONG`
    Title: GEOGRAPHIC LONGITUDE OF MAIN OFFICE
    Description: Geographic longitude of main office"#]
    #[serde(rename="SIMS_LONG")]
    pub geographic_longitude_of_main_office: Option<f64>,

    #[doc = r#"## FDIC Field:: `SL`
    Title: SAVINGS AND LOAN FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SL")]
    pub savings_and_loan_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SM`
    Title: STATE MEMBER BANK FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SM")]
    pub state_member_bank_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `STALP`
    Title: FIPS STATE ALPHA CODE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="STALP")]
    pub fips_state_alpha_code: Option<String>,

    #[doc = r#"## FDIC Field:: `STCHRTR`
    Title: STATE CHARTER FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="STCHRTR")]
    pub state_charter_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `STNAME`
    Title: STATE NAME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="STNAME")]
    pub state_name: Option<String>,

    #[doc = r#"## FDIC Field:: `STNUM`
    Title: FIPS STATE NUMBER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="STNUM")]
    pub fips_state_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUBLLPF`
    Title: SUB. DEBT & L/L PREFERRED STK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SUBLLPF")]
    pub sub_debt_l_l_preferred_stk: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUBND`
    Title: SUBORDINATED NOTES & DEBENTURES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SUBND")]
    pub subordinated_notes_debentures: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ25`
    Title: ASSETS UNDER 25M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ25")]
    pub assets_under_25m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ100`
    Title: ASSETS UNDER 100M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ100")]
    pub assets_under_100m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ100MP`
    Title: ASSETS OVER 100M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ100MP")]
    pub assets_over_100m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ100T3`
    Title: ASSETS 100M TO 300M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ100T3")]
    pub assets_100m_to_300m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ100T5`
    Title: ASSETS 100M TO 500M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ100T5")]
    pub assets_100m_to_500m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ100T1B`
    Title: ASSETS 100M TO 1B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ100T1B")]
    pub assets_100m_to_1b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ10BP`
    Title: ASSETS OVER 10B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ10BP")]
    pub assets_over_10b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ1BP`
    Title: ASSETS OVER 1B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ1BP")]
    pub assets_over_1b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ1BT10B`
    Title: ASSETS 1B TO 10B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ1BT10B")]
    pub assets_1b_to_10b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ1BT3B`
    Title: ASSETS 1B TO 3B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ1BT3B")]
    pub assets_1b_to_3b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ1BT5B`
    Title: ASSETS 1B TO 5B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ1BT5B")]
    pub assets_1b_to_5b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ250BP`
    Title: ASSETS OVER 250B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ250BP")]
    pub assets_over_250b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ25T50`
    Title: ASSETS 25M TO 50M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ25T50")]
    pub assets_25m_to_50m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ300T5`
    Title: ASSETS 300M TO 500M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ300T5")]
    pub assets_300m_to_500m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ3BT10B`
    Title: ASSETS 3B TO 10B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ3BT10B")]
    pub assets_3b_to_10b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ500T1B`
    Title: ASSETS 500M TO 1B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ500T1B")]
    pub assets_500m_to_1b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ50T100`
    Title: ASSETS 50M TO 100M FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ50T100")]
    pub assets_50m_to_100m_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ5BP`
    Title: ASSETS OVER 5B FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ5BP")]
    pub assets_over_5b_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `TFRA`
    Title: TOTAL FIDUCIARY AND RELATED ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TFRA")]
    pub total_fiduciary_and_related_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRADE`
    Title: TRADING ACCOUNTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRADE")]
    pub trading_accounts: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRADEL`
    Title: TRADING LIABILITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRADEL")]
    pub trading_liabilities: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRADELR`
    Title: TRADING LIABILITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRADELR")]
    pub trading_liabilities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRADER`
    Title: TRADING ACCOUNTS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRADER")]
    pub trading_accounts_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRN`
    Title: TRANSACTION-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRN")]
    pub transaction_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNR`
    Title: TRANSACTION-TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNR")]
    pub transaction_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNIPC`
    Title: TRANSACTION-IPC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNIPC")]
    pub transaction_ipc: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNIPCOC`
    Title: TRAN-IPC-OFFICIAL CHECKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNIPCOC")]
    pub tran_ipc_official_checks: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNIPCOCR`
    Title: TRAN-IPC-OFFICIAL CHECKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNIPCOCR")]
    pub tran_ipc_official_checks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNMUNI`
    Title: TRANSACTION-MUNI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNMUNI")]
    pub transaction_muni: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNMUNIR`
    Title: TRANSACTION-MUNI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNMUNIR")]
    pub transaction_muni_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNUSGOV`
    Title: TRANSACTION-U.S. GOVERNMENT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNUSGOV")]
    pub transaction_u_s_government: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNUSGOVR`
    Title: TRANSACTION-U.S. GOVERNMENT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNUSGOVR")]
    pub transaction_u_s_government_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRUSTPWR`
    Title: TRUST POWER GRANTED CODES
    Description: Is a two digit numeric code which identifies the trust power granted categories of an institution. 00 - Trust powers not known. 10 - Full trust powers granted. 11 - Full trust powers granted, exercised. 12 - Full trust powers granted, not exercised. 20 - Limited trust powers granted. 21 - Limited trust powers granted, exercised. 30 - Trust powers not granted. 31 - Trust powers not granted but exercised. 40 - Full trust powers grandfathered."#]
    #[serde(rename="TRUSTPWR")]
    pub trust_power_granted_codes: Option<f64>,

    #[doc = r#"## FDIC Field:: `TS`
    Title: TIME & SAVINGS DEPOSITS-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TS")]
    pub time_savings_deposits_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `TSR`
    Title: TIME & SAVINGS DEPOSITS-TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TSR")]
    pub time_savings_deposits_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TTL`
    Title: TT&L NOTE OPTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TTL")]
    pub tt_l_note_option: Option<f64>,

    #[doc = r#"## FDIC Field:: `TTLOTBOR`
    Title: TT&L & OTHER BORROWINGS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TTLOTBOR")]
    pub tt_l_other_borrowings: Option<f64>,

    #[doc = r#"## FDIC Field:: `UNINC`
    Title: UNEARNED INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UNINC")]
    pub unearned_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `UNINUM`
    Title: BANK UNIQUE NUMBER
    Description: A unique identification number assigned to an institution by the FDIC."#]
    #[serde(rename="UNINUM")]
    pub bank_unique_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `USA`
    Title: USA LOCATED INSTITUTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="USA")]
    pub usa_located_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `UYAMTG`
    Title: UNAMORTIZED YIELD ADJ-MTG LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UYAMTG")]
    pub unamortized_yield_adj_mtg_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCUBK`
    Title: ASST-BCK UNUSED COMMIT - RELATED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCUBK")]
    pub asst_bck_unused_commit_related: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCUBKR`
    Title: ASST-BCK UNUSED COMMIT - RELATED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCUBKR")]
    pub asst_bck_unused_commit_related_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCUOTH`
    Title: ASSET-BACK UNUSED COMMIT - OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCUOTH")]
    pub asset_back_unused_commit_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCUOTHR`
    Title: ASSET-BACK UNUSED COMMIT - OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCUOTHR")]
    pub asset_back_unused_commit_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCXBK`
    Title: ASSET-BACK CREDIT EX-RELATED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCXBK")]
    pub asset_back_credit_ex_related: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCXBKR`
    Title: ASSET-BACK CREDIT EX-RELATED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCXBKR")]
    pub asset_back_credit_ex_related_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCXOTH`
    Title: ASSET-BACK CREDIT EX-OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCXOTH")]
    pub asset_back_credit_ex_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `ABCXOTHR`
    Title: ASSET-BACK CREDIT EX-OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ABCXOTHR")]
    pub asset_back_credit_ex_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASCEOTH`
    Title: C.E. RECOURSE NOT SECUR. - OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASCEOTH")]
    pub c_e_recourse_not_secur_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASCEOTHR`
    Title: C.E. RECOURSE NOT SECUR. - OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASCEOTHR")]
    pub c_e_recourse_not_secur_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASCERES`
    Title: C.E. RECOURSE NOT SECUR. - RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASCERES")]
    pub c_e_recourse_not_secur_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASCERESR`
    Title: C.E. RECOURSE NOT SECUR. - RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASCERESR")]
    pub c_e_recourse_not_secur_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASDROTH`
    Title: SOLD W/RECOURSE N/SECUR. - OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASDROTH")]
    pub sold_w_recourse_n_secur_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASDROTHR`
    Title: SOLD W/RECOURSE N/SECUR. - OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASDROTHR")]
    pub sold_w_recourse_n_secur_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASDRRES`
    Title: SOLD W/RECOURSE N/SECUR.- RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASDRRES")]
    pub sold_w_recourse_n_secur_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASDRRESR`
    Title: SOLD W/RECOURSE N/SECUR.- RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASDRRESR")]
    pub sold_w_recourse_n_secur_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASSET2`
    Title: TOTAL ASSETS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASSET2")]
    pub total_assets_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASSET5`
    Title: TOTAL ASSETS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASSET5")]
    pub total_assets_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASSETFOR`
    Title: TOTAL ASSETS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASSETFOR")]
    pub total_assets_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASSTLT`
    Title: LONG-TERM ASSETS (5+ YEARS)-QBP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASSTLT")]
    pub long_term_assets_5_years_qbp: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASSTLTR`
    Title: LONG-TERM ASSETS (5+ YEARS) RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASSTLTR")]
    pub long_term_assets_5_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ASTEMPM`
    Title: ASSETS PER EMPLOYEE IN MILLION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASTEMPM")]
    pub assets_per_employee_in_million: Option<f64>,

    #[doc = r#"## FDIC Field:: `AVASSETJ`
    Title: AVERAGE ASSETS-ADJUSTED-PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AVASSETJ")]
    pub average_assets_adjusted_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `AVASSETJR`
    Title: AVERAGE ASSETS-ADJUSTED-PCA RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AVASSETJR")]
    pub average_assets_adjusted_pca_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `BROINS`
    Title: BROKERED DEP-INSURED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BROINS")]
    pub brokered_dep_insured: Option<f64>,

    #[doc = r#"## FDIC Field:: `BROINSR`
    Title: BROKERED DEP-INSURED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BROINSR")]
    pub brokered_dep_insured_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CALLYMD`
    Title: REPORT DATE (CCYYMMDD)
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CALLYMD")]
    pub report_date_ccyymmdd: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHBALFOR`
    Title: CASH & DUE FROM DEP INST-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHBALFOR")]
    pub cash_due_from_dep_inst_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHBALNI`
    Title: NONINTEREST-BEARING CASH & DUE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHBALNI")]
    pub noninterest_bearing_cash_due: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHBALNIR`
    Title: NONINTEREST-BEARING CASH & DUE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHBALNIR")]
    pub noninterest_bearing_cash_due_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHCIC`
    Title: CASH ITEMS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHCIC")]
    pub cash_items: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHCICR`
    Title: CASH ITEMS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHCICR")]
    pub cash_items_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHCOIN`
    Title: CURRENCY & COIN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHCOIN")]
    pub currency_coin: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHCOINR`
    Title: CURRENCY & COIN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHCOINR")]
    pub currency_coin_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHFLA`
    Title: NET OPERATING CASH FLOW-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHFLA")]
    pub net_operating_cash_flow_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHFLQ`
    Title: NET OPERATING CASH FLOW-ANN Quarterly
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHFLQ")]
    pub net_operating_cash_flow_ann_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHFRB`
    Title: BAL DUE FROM FRB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHFRB")]
    pub bal_due_from_frb: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHFRBR`
    Title: BAL DUE FROM FRB RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHFRBR")]
    pub bal_due_from_frb_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHITEM`
    Title: CASH ITEM COLLEC IN DOMESTIC OFFICES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHITEM")]
    pub cash_item_collec_in_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHITEMR`
    Title: CASH ITEMS COLLEC IN DOMESTIC OFFICES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHITEMR")]
    pub cash_items_collec_in_domestic_offices_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHNUS`
    Title: BAL DUE FROM BK FOR COUNTRY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHNUS")]
    pub bal_due_from_bk_for_country: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHNUSR`
    Title: BAL DUE FROM BK FOR COUNTRY RATIOS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHNUSR")]
    pub bal_due_from_bk_for_country_ratios: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHNUSFBK`
    Title: BAL DUE FROM FOR BR OF OTH US BK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHNUSFBK")]
    pub bal_due_from_for_br_of_oth_us_bk: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHUS`
    Title: BAL DUE FROM DEP INST U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHUS")]
    pub bal_due_from_dep_inst_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHUSR`
    Title: BAL DUE FROM DEP INST U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHUSR")]
    pub bal_due_from_dep_inst_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHUSFBK`
    Title: BAL DUE FROM U.S. BR OF FOR BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CHUSFBK")]
    pub bal_due_from_u_s_br_of_for_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `CITY`
    Title: CITY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CITY")]
    pub city: Option<String>,

    #[doc = r#"## FDIC Field:: `COREDEP`
    Title: CORE DEPOSITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="COREDEP")]
    pub core_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `COREDEPR`
    Title: CORE DEPOSITS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="COREDEPR")]
    pub core_deposits_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAG`
    Title: AGRICULTURAL LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAG")]
    pub agricultural_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAGR`
    Title: AGRICULTURAL LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAGR")]
    pub agricultural_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAGQ`
    Title: AGRICULTURAL LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAGQ")]
    pub agricultural_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAGQR`
    Title: AGRICULTURAL LOAN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAGQR")]
    pub agricultural_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAGSM`
    Title: AG LOAN RECOVERIES*SMALL BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAGSM")]
    pub ag_loan_recoveries_small_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAGSMR`
    Title: AAG LOAN RECOVERIES*SMALL BKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAGSMR")]
    pub aag_loan_recoveries_small_bks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAGSMQ`
    Title: AG LOAN RECOVERIES*SMALL BKS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAGSMQ")]
    pub ag_loan_recoveries_small_bks_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAGSMQR`
    Title: AG LOAN RECOVERIES*SMALL BKS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAGSMQR")]
    pub ag_loan_recoveries_small_bks_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAUTO`
    Title: AUTO LOANS - RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAUTO")]
    pub auto_loans_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAUTOR`
    Title: AUTO LOANS - RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAUTOR")]
    pub auto_loans_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAUTOQ`
    Title: AUTO LOANS - RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAUTOQ")]
    pub auto_loans_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRAUTOQR`
    Title: AUTO LOANS - RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRAUTOQR")]
    pub auto_loans_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCI`
    Title: COMMERCIAL LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCI")]
    pub commercial_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCIR`
    Title: COMMERCIAL LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCIR")]
    pub commercial_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCIQ`
    Title: COMMERCIAL LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCIQ")]
    pub commercial_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCIQR`
    Title: COMMERCIAL LOAN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCIQR")]
    pub commercial_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCINUS`
    Title: COMMERCIAL LOAN RECOVERIES NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCINUS")]
    pub commercial_loan_recoveries_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCINUSR`
    Title: COMMERCIAL LOAN RECOVERIES NON-U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCINUSR")]
    pub commercial_loan_recoveries_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCINUSQ`
    Title: COMMERCIAL LOAN RECOVERIES NON-U.S. QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCINUSQ")]
    pub commercial_loan_recoveries_non_u_s_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCINUSQR`
    Title: COMMERCIAL LOAN RECOVERIES NON-U.S. QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCINUSQR")]
    pub commercial_loan_recoveries_non_u_s_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCON`
    Title: CONSUMER LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCON")]
    pub consumer_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCONR`
    Title: CONSUMER LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCONR")]
    pub consumer_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCONQ`
    Title: CONSUMER LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCONQ")]
    pub consumer_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCONQR`
    Title: CONSUMER LOAN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCONQR")]
    pub consumer_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCONOTH`
    Title: OTHER CONSUMER LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCONOTH")]
    pub other_consumer_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCONOTHR`
    Title: OTHER CONSUMER LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCONOTHR")]
    pub other_consumer_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCONOTQ`
    Title: OTHER CONSUMER LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCONOTQ")]
    pub other_consumer_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCONOTQR`
    Title: OTHER CONSUMER LOAN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCONOTQR")]
    pub other_consumer_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCRCD`
    Title: CREDIT CARD LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCRCD")]
    pub credit_card_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCRCDR`
    Title: CREDIT CARD LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCRCDR")]
    pub credit_card_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCRCDQ`
    Title: CREDIT CARD LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCRCDQ")]
    pub credit_card_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRCRCDQR`
    Title: CREDIT CARD LOAN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRCRCDQR")]
    pub credit_card_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEP`
    Title: DEPOSITORY INST LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEP")]
    pub depository_inst_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEPR`
    Title: DEPOSITORY INST LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEPR")]
    pub depository_inst_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEPQ`
    Title: DEPOSITORY INST LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEPQ")]
    pub depository_inst_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEPQR`
    Title: DEPOSITORY INST LOAN RECOVERIES Quarterly RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEPQR")]
    pub depository_inst_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEPNUS`
    Title: FOREIGN DEPS INST LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEPNUS")]
    pub foreign_deps_inst_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEPNUSR`
    Title: FOREIGN DEPS INST LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEPNUSR")]
    pub foreign_deps_inst_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEPNUQ`
    Title: FOREIGN DEPS INST LN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEPNUQ")]
    pub foreign_deps_inst_ln_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRDEPNUQR`
    Title: FOREIGN DEPS INST LN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRDEPNUQR")]
    pub foreign_deps_inst_ln_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRFORGV`
    Title: FOREIGN GOVERNMENT LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRFORGV")]
    pub foreign_government_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRFORGVR`
    Title: FOREIGN GOVERNMENT LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRFORGVR")]
    pub foreign_government_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRFORGVQ`
    Title: FOREIGN GOVERNMENT LN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRFORGVQ")]
    pub foreign_government_ln_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRFORGVQR`
    Title: FOREIGN GOVERNMENT LN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRFORGVQR")]
    pub foreign_government_ln_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLS`
    Title: LEASE RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLS")]
    pub lease_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLSR`
    Title: LEASE RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLSR")]
    pub lease_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLSQ`
    Title: LEASE RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLSQ")]
    pub lease_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRLSQR`
    Title: LEASE RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRLSQR")]
    pub lease_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CROTHER`
    Title: ALL OTHER LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CROTHER")]
    pub all_other_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CROTHERR`
    Title: ALL OTHER LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CROTHERR")]
    pub all_other_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CROTHQ`
    Title: ALL OTHER LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CROTHQ")]
    pub all_other_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CROTHQR`
    Title: ALL OTHER LOAN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CROTHQR")]
    pub all_other_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRE`
    Title: REAL ESTATE LOAN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRE")]
    pub real_estate_loan_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRER`
    Title: REAL ESTATE LOAN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRER")]
    pub real_estate_loan_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREQ`
    Title: REAL ESTATE LOAN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREQ")]
    pub real_estate_loan_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREQR`
    Title: REAL ESTATE LOAN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREQR")]
    pub real_estate_loan_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREAG`
    Title: FARMLAND RE LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREAG")]
    pub farmland_re_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREAGR`
    Title: FARMLAND RE LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREAGR")]
    pub farmland_re_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREAGQ`
    Title: FARMLAND RE LN RECOVERIES-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREAGQ")]
    pub farmland_re_ln_recoveries_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREAGQR`
    Title: FARMLAND RE LN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREAGQR")]
    pub farmland_re_ln_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRECNFM`
    Title: 1-4 FAM CONSTRUCT LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRECNFM")]
    pub _1_4_fam_construct_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRECNOT`
    Title: OTHER CONSTRUCT LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRECNOT")]
    pub other_construct_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRECONQ`
    Title: CONSTRUCTION RE LN RECOVER-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRECONQ")]
    pub construction_re_ln_recover_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRECONQR`
    Title: CONSTRUCTION RE LN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRECONQR")]
    pub construction_re_ln_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRECONS`
    Title: CONSTRUCTION RE LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRECONS")]
    pub construction_re_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRECONSR`
    Title: CONSTRUCTION RE LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRECONSR")]
    pub construction_re_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREFOR`
    Title: REAL ESTATE LN RECOVERIES - FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREFOR")]
    pub real_estate_ln_recoveries_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREFORR`
    Title: REAL ESTATE LN RECOVERIES - FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREFORR")]
    pub real_estate_ln_recoveries_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREFORQ`
    Title: REAL ESTATE LN RECOVERIES - FOR QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREFORQ")]
    pub real_estate_ln_recoveries_for_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREFORQR`
    Title: REAL ESTATE LN RECOVERIES - FOR QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREFORQR")]
    pub real_estate_ln_recoveries_for_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRELOC`
    Title: LINE OF CREDIT RE LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRELOC")]
    pub line_of_credit_re_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRELOCR`
    Title: LINE OF CREDIT RE LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRELOCR")]
    pub line_of_credit_re_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRELOCQ`
    Title: LINE OF CREDIT RE LN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRELOCQ")]
    pub line_of_credit_re_ln_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRELOCQR`
    Title: LINE OF CREDIT RE LN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRELOCQR")]
    pub line_of_credit_re_ln_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREMULQ`
    Title: MULTIFAMILY RE LN RECOVERIES-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREMULQ")]
    pub multifamily_re_ln_recoveries_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREMULQR`
    Title: MULTIFAMILY RES RE LN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREMULQR")]
    pub multifamily_res_re_ln_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREMULT`
    Title: MULTIFAMILY RES RE LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREMULT")]
    pub multifamily_res_re_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREMULTR`
    Title: MULTIFAMILY RES RE LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREMULTR")]
    pub multifamily_res_re_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENRES`
    Title: NONFARM NONRES RE LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENRES")]
    pub nonfarm_nonres_re_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENRESR`
    Title: NONFARM NONRES RE LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENRESR")]
    pub nonfarm_nonres_re_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENROT`
    Title: OTHER NONFARM NONRES RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENROT")]
    pub other_nonfarm_nonres_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENROW`
    Title: OWN-OCCUP NONFARM NONRES RECOV
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENROW")]
    pub own_occup_nonfarm_nonres_recov: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENRSQ`
    Title: NONFARM NONRES RE LN RECOVER-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENRSQ")]
    pub nonfarm_nonres_re_ln_recover_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENRSQR`
    Title: NONFARM NONRES RE LN RECOVER-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENRSQR")]
    pub nonfarm_nonres_re_ln_recover_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENUS`
    Title: NON-U.S. RE LN RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENUS")]
    pub non_u_s_re_ln_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENUSR`
    Title: NON-U.S. RE LN RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENUSR")]
    pub non_u_s_re_ln_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENUSQ`
    Title: NON-U.S. RE LN RECOVERIES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENUSQ")]
    pub non_u_s_re_ln_recoveries_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRENUSQR`
    Title: NON-U.S. RE LN RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRENUSQR")]
    pub non_u_s_re_ln_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERES`
    Title: RE LOANS 1-4 FAMILY RECOVERIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERES")]
    pub re_loans_1_4_family_recoveries: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERESR`
    Title: RE LOANS 1-4 FAMILY RECOVERIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERESR")]
    pub re_loans_1_4_family_recoveries_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERESQ`
    Title: RE LOANS 1-4 FAMILY RECOVER-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERESQ")]
    pub re_loans_1_4_family_recover_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERESQR`
    Title: RE LOANS 1-4 FAMILY RECOVERIES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERESQR")]
    pub re_loans_1_4_family_recoveries_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERSF2`
    Title: RE LOAN 1-4 FAM JR LIEN-RECOVER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERSF2")]
    pub re_loan_1_4_fam_jr_lien_recover: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERSF2R`
    Title: RE LOAN 1-4 FAM JR LIEN-RECOVER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERSF2R")]
    pub re_loan_1_4_fam_jr_lien_recover_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERS2Q`
    Title: RE LOAN 1-4 FAM JR LIEN-RECOVER QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERS2Q")]
    pub re_loan_1_4_fam_jr_lien_recover_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERS2QR`
    Title: RE LOAN 1-4 FAM JR LIEN-RECOVER QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERS2QR")]
    pub re_loan_1_4_fam_jr_lien_recover_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERSFM`
    Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERSFM")]
    pub re_loan_1_4_fam_first_lien_recov: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERSFMR`
    Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERSFMR")]
    pub re_loan_1_4_fam_first_lien_recov_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERSFQ`
    Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERSFQ")]
    pub re_loan_1_4_fam_first_lien_recov_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRRERSFQR`
    Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRRERSFQR")]
    pub re_loan_1_4_fam_first_lien_recov_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREOFFDOM`
    Title: RE LOAN RECOVERIES DOMESTIC OFFICES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREOFFDOM")]
    pub re_loan_recoveries_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREOFFDOMR`
    Title: RE LOAN RECOVERIES DOMESTIC OFFICES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREOFFDOMR")]
    pub re_loan_recoveries_domestic_offices_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREOFFDOMQ`
    Title: RE LOAN RECOVERIES DOMESTIC OFFICES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREOFFDOMQ")]
    pub re_loan_recoveries_domestic_offices_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `CRREOFFDOMQR`
    Title: RE LOAN RECOVERIES DOMESTIC OFFICES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CRREOFFDOMQR")]
    pub re_loan_recoveries_domestic_offices_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `CTDERBEN`
    Title: CR DER (NET)-PURCHASE PROTECT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CTDERBEN")]
    pub cr_der_net_purchase_protect: Option<f64>,

    #[doc = r#"## FDIC Field:: `CTDERGTY`
    Title: CR DER(NET) - SOLD PROTECTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CTDERGTY")]
    pub cr_der_net_sold_protection: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPBEFEX`
    Title: TOTAL DEPOSIT LIAB BEF EXCLUSION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPBEFEX")]
    pub total_deposit_liab_bef_exclusion: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPCSBQ`
    Title: ESTIMATED ASSESSABLE DEPOSITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPCSBQ")]
    pub estimated_assessable_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPCSBQR`
    Title: ESTIMATED ASSESSABLE DEPOSITS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPCSBQR")]
    pub estimated_assessable_deposits_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPDASTR`
    Title: TOT DOMESTIC DEPOSIT / ASSET
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPDASTR")]
    pub tot_domestic_deposit_asset: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPFBKF`
    Title: FOREIGN BANKS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPFBKF")]
    pub foreign_banks_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPFBKFR`
    Title: FOREIGN BANKS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPFBKFR")]
    pub foreign_banks_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPFGOVF`
    Title: FOREIGN GOVERNMENTS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPFGOVF")]
    pub foreign_governments_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPFGOVFR`
    Title: FOREIGN GOVERNMENTS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPFGOVFR")]
    pub foreign_governments_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIDOM`
    Title: INTEREST-BEARING DEP-DOM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIDOM")]
    pub interest_bearing_dep_dom: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIDOMR`
    Title: INTEREST-BEARING DEP-DOM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIDOMR")]
    pub interest_bearing_dep_dom_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPINS`
    Title: ESTIMATED INSURED DEPOSITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPINS")]
    pub estimated_insured_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPINSR`
    Title: ESTIMATED INSURED DEPOSITS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPINSR")]
    pub estimated_insured_deposits_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLGAMT`
    Title: AMT DEP ACC GREATER THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLGAMT")]
    pub amt_dep_acc_greater_than_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLGAMTR`
    Title: AMT DEP ACC GREATER THAN $250,000 RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLGAMTR")]
    pub amt_dep_acc_greater_than_250_000_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLGB`
    Title: NUM DEP ACC GREATER THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLGB")]
    pub num_dep_acc_greater_than_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLGRA`
    Title: AMT OF RETIREMENT DEP ACC OF MORE THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLGRA")]
    pub amt_of_retirement_dep_acc_of_more_than_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLGRAR`
    Title: AMT OF RETIREMENT DEP ACC OF MORE THAN $250,000 RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLGRAR")]
    pub amt_of_retirement_dep_acc_of_more_than_250_000_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLGRN`
    Title: NUM OF RETIREMENT DEP ACC MORE THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLGRN")]
    pub num_of_retirement_dep_acc_more_than_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLSNB`
    Title: DEP THRU LIST SVC NOT BROKERED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLSNB")]
    pub dep_thru_list_svc_not_brokered: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPLSNBR`
    Title: DEP THRU LIST SVC NOT BROKERED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPLSNBR")]
    pub dep_thru_list_svc_not_brokered_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPNIDOM`
    Title: NONINTEREST-BEARING DEP-DOM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPNIDOM")]
    pub noninterest_bearing_dep_dom: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPNIDOMR`
    Title: NONINTEREST-BEARING DEP-DOM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPNIDOMR")]
    pub noninterest_bearing_dep_dom_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSMAMT`
    Title: AMT DEP ACC AT $250,000 OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPSMAMT")]
    pub amt_dep_acc_at_250_000_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSMAMTR`
    Title: AMT DEP ACC AT $250,000 OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPSMAMTR")]
    pub amt_dep_acc_at_250_000_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSMB`
    Title: NUM DEP ACC EQUAL OR LESS THAN EQUAL TO $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPSMB")]
    pub num_dep_acc_equal_or_less_than_equal_to_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSMRA`
    Title: AMT RETIREMENT DEP ACC OF $250,000 OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPSMRA")]
    pub amt_retirement_dep_acc_of_250_000_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSMRAR`
    Title: AMT RETIREMENT DEP ACC OF $250,000 OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPSMRAR")]
    pub amt_retirement_dep_acc_of_250_000_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSMRN`
    Title: NUM RETIREMENT DEP ACC OF $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPSMRN")]
    pub num_retirement_dep_acc_of_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPALLEX`
    Title: TOTAL ALLOWABLE EXCLUSIONS (INCLUDING FOREIGN DEPOSITS)
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPALLEX")]
    pub total_allowable_exclusions_including_foreign_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPUNA`
    Title: EST UNINSURED DEP IN DOM-OFF IN INSURED BRANCHES IN US TERR AND POSSESSIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPUNA")]
    pub est_uninsured_dep_in_dom_off_in_insured_branches_in_us_terr_and_possessions: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPUNAR`
    Title: EST UNINSURED DEP IN DOM-OFF IN INSURED BRANCHES IN US TERR AND POSSESSIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPUNAR")]
    pub est_uninsured_dep_in_dom_off_in_insured_branches_in_us_terr_and_possessions_depunar: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPUNINS`
    Title: ESTIMATED UNINSURED DEPOSITS IN DOMESTIC OFFICES AND IN INSURED BRANCHES IN US TERRITORIES AND POSSESSIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPUNINS")]
    pub estimated_uninsured_deposits_in_domestic_offices_and_in_insured_branches_in_us_territories_and_possessions: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPUSBKF`
    Title: U.S. BANKS&OTH.US INST-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPUSBKF")]
    pub u_s_banks_oth_us_inst_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPUSBKFR`
    Title: U.S. BANKS&OTH.US INST-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPUSBKFR")]
    pub u_s_banks_oth_us_inst_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPUSMF`
    Title: U.S.GOVT & ST & POL SUBS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPUSMF")]
    pub u_s_govt_st_pol_subs_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPUSMFR`
    Title: U.S.GOVT & ST & POL SUBS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPUSMFR")]
    pub u_s_govt_st_pol_subs_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAG`
    Title: AGRICULTURAL LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAG")]
    pub agricultural_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAGR`
    Title: AGRICULTURAL LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAGR")]
    pub agricultural_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAGQ`
    Title: AGRICULTURAL LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAGQ")]
    pub agricultural_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAGQR`
    Title: AGRICULTURAL LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAGQR")]
    pub agricultural_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAGSM`
    Title: AG LOAN CHARGE-OFFS*SMALL BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAGSM")]
    pub ag_loan_charge_offs_small_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAGSMR`
    Title: AG LOAN CHARGE-OFFS*SMALL BKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAGSMR")]
    pub ag_loan_charge_offs_small_bks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAGSMQ`
    Title: AG LOAN CHARGE-OFFS*SMALL BKS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAGSMQ")]
    pub ag_loan_charge_offs_small_bks_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAGSMQR`
    Title: AG LOAN CHARGE-OFFS*SMALL BKS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAGSMQR")]
    pub ag_loan_charge_offs_small_bks_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAUTO`
    Title: AUTO LOANS - CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAUTO")]
    pub auto_loans_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAUTOR`
    Title: AUTO LOANS - CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAUTOR")]
    pub auto_loans_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAUTOQ`
    Title: AUTO LOANS - CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAUTOQ")]
    pub auto_loans_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRAUTOQR`
    Title: AUTO LOANS - CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRAUTOQR")]
    pub auto_loans_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCI`
    Title: COMMERCIAL LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCI")]
    pub commercial_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCIR`
    Title: COMMERCIAL LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCIR")]
    pub commercial_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCIQ`
    Title: COMMERCIAL LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCIQ")]
    pub commercial_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCIQR`
    Title: COMMERCIAL LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCIQR")]
    pub commercial_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCINUS`
    Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCINUS")]
    pub commercial_loan_charge_offs_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCINUSR`
    Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCINUSR")]
    pub commercial_loan_charge_offs_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCINUSQ`
    Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S. QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCINUSQ")]
    pub commercial_loan_charge_offs_non_u_s_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCINUSQR`
    Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S. QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCINUSQR")]
    pub commercial_loan_charge_offs_non_u_s_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCON`
    Title: CONSUMER LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCON")]
    pub consumer_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCONR`
    Title: CONSUMER LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCONR")]
    pub consumer_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCONQ`
    Title: CONSUMER LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCONQ")]
    pub consumer_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCONQR`
    Title: CONSUMER LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCONQR")]
    pub consumer_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCONOTH`
    Title: OTHER CONSUMER LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCONOTH")]
    pub other_consumer_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCONOTHR`
    Title: OTHER CONSUMER LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCONOTHR")]
    pub other_consumer_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCONOTQ`
    Title: OTHER CONSUMER LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCONOTQ")]
    pub other_consumer_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCONOTQR`
    Title: OTHER CONSUMER LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCONOTQR")]
    pub other_consumer_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCRCD`
    Title: CREDIT CARD LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCRCD")]
    pub credit_card_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCRCDR`
    Title: CREDIT CARD LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCRCDR")]
    pub credit_card_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCRCDQ`
    Title: CREDIT CARD LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCRCDQ")]
    pub credit_card_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRCRCDQR`
    Title: CREDIT CARD LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRCRCDQR")]
    pub credit_card_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEP`
    Title: DEPOSITORY INST LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEP")]
    pub depository_inst_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEPR`
    Title: DEPOSITORY INST LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEPR")]
    pub depository_inst_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEPQ`
    Title: DEPOSITORY INST LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEPQ")]
    pub depository_inst_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEPQR`
    Title: DEPOSITORY INST LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEPQR")]
    pub depository_inst_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEPNUS`
    Title: FOREIGN DEPS INST LN CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEPNUS")]
    pub foreign_deps_inst_ln_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEPNUSR`
    Title: FOREIGN DEPS INST LN CHG-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEPNUSR")]
    pub foreign_deps_inst_ln_chg_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEPNUQ`
    Title: FOREIGN DEPS INST LN CHG-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEPNUQ")]
    pub foreign_deps_inst_ln_chg_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRDEPNUQR`
    Title: FOREIGN DEPS INST LN CHG-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRDEPNUQR")]
    pub foreign_deps_inst_ln_chg_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRFORGV`
    Title: FOREIGN GOVERNMENT LN CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRFORGV")]
    pub foreign_government_ln_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRFORGVR`
    Title: FOREIGN GOVERNMENT LN CHG-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRFORGVR")]
    pub foreign_government_ln_chg_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRFORGVQ`
    Title: FOREIGN GOVERNMENT LN CHG-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRFORGVQ")]
    pub foreign_government_ln_chg_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRFORGVQR`
    Title: FOREIGN GOVERNMENT LN CHG-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRFORGVQR")]
    pub foreign_government_ln_chg_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLS`
    Title: LEASE CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLS")]
    pub lease_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLSR`
    Title: LEASE CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLSR")]
    pub lease_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLSQ`
    Title: LEASE CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLSQ")]
    pub lease_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRLSQR`
    Title: LEASE CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRLSQR")]
    pub lease_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DROTHER`
    Title: ALL OTHER LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DROTHER")]
    pub all_other_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DROTHERR`
    Title: ALL OTHER LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DROTHERR")]
    pub all_other_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DROTHQ`
    Title: ALL OTHER LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DROTHQ")]
    pub all_other_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DROTHQR`
    Title: ALL OTHER LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DROTHQR")]
    pub all_other_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRE`
    Title: REAL ESTATE LOAN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRE")]
    pub real_estate_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRER`
    Title: REAL ESTATE LOAN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRER")]
    pub real_estate_loan_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREQ`
    Title: REAL ESTATE LOAN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREQ")]
    pub real_estate_loan_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREQR`
    Title: REAL ESTATE LOAN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREQR")]
    pub real_estate_loan_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREAG`
    Title: FARMLAND RE LN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREAG")]
    pub farmland_re_ln_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREAGR`
    Title: FARMLAND RE LN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREAGR")]
    pub farmland_re_ln_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREAGQ`
    Title: FARMLAND RE LN CHG-OFFS-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREAGQ")]
    pub farmland_re_ln_chg_offs_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREAGQR`
    Title: FARMLAND RE LN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREAGQR")]
    pub farmland_re_ln_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRECNFM`
    Title: 1-4 FAM CONSTRUCT LN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRECNFM")]
    pub _1_4_fam_construct_ln_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRECNOT`
    Title: OTHER CONSTRUCT LN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRECNOT")]
    pub other_construct_ln_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRECONQ`
    Title: CONSTRUCTION RE LN CHG-OFFS-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRECONQ")]
    pub construction_re_ln_chg_offs_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRECONQR`
    Title: CONSTRUCTION RE LN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRECONQR")]
    pub construction_re_ln_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRECONS`
    Title: CONSTRUCTION RE LN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRECONS")]
    pub construction_re_ln_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRECONSR`
    Title: CONSTRUCTION RE LN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRECONSR")]
    pub construction_re_ln_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREFOR`
    Title: REAL ESTATE LOAN CHRG-OFFS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREFOR")]
    pub real_estate_loan_chrg_offs_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREFORR`
    Title: REAL ESTATE LOAN CHRG-OFFS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREFORR")]
    pub real_estate_loan_chrg_offs_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREFORQ`
    Title: REAL ESTATE LOAN CHRG-OFFS-FOR QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREFORQ")]
    pub real_estate_loan_chrg_offs_for_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREFORQR`
    Title: REAL ESTATE LOAN CHRG-OFFS-FOR QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREFORQR")]
    pub real_estate_loan_chrg_offs_for_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRELOC`
    Title: LINE OF CREDIT RE LN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRELOC")]
    pub line_of_credit_re_ln_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRELOCR`
    Title: LINE OF CREDIT RE LN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRELOCR")]
    pub line_of_credit_re_ln_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRELOCQ`
    Title: LINE OF CREDIT RE LN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRELOCQ")]
    pub line_of_credit_re_ln_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRELOCQR`
    Title: LINE OF CREDIT RE LN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRELOCQR")]
    pub line_of_credit_re_ln_charge_offs_ratio_drrelocqr: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREMULQ`
    Title: MULTIFAMILY RE LN CHG-OFFS-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREMULQ")]
    pub multifamily_re_ln_chg_offs_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREMULQR`
    Title: MULTIFAMILY RES RE LN CHARGE-OFF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREMULQR")]
    pub multifamily_res_re_ln_charge_off_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREMULT`
    Title: MULTIFAMILY RES RE LN CHARGE-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREMULT")]
    pub multifamily_res_re_ln_charge_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREMULTR`
    Title: MULTIFAMILY RES RE LN CHARGE-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREMULTR")]
    pub multifamily_res_re_ln_charge_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENRES`
    Title: NONFARM NONRES RE LN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENRES")]
    pub nonfarm_nonres_re_ln_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENRESR`
    Title: NONFARM NONRES RE LN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENRESR")]
    pub nonfarm_nonres_re_ln_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENROT`
    Title: OTHER NONFARM NONRES RE CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENROT")]
    pub other_nonfarm_nonres_re_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENROW`
    Title: OWN-OCCUP NONFARM NONRES CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENROW")]
    pub own_occup_nonfarm_nonres_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENRSQ`
    Title: NONFARM NONRES RE LN CHG-OFF-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENRSQ")]
    pub nonfarm_nonres_re_ln_chg_off_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENRSQR`
    Title: NONFARM NONRES RE LN CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENRSQR")]
    pub nonfarm_nonres_re_ln_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENUS`
    Title: NON-U.S. RE LN CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENUS")]
    pub non_u_s_re_ln_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENUSR`
    Title: NON-U.S. RE LN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENUSR")]
    pub non_u_s_re_ln_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENUSQ`
    Title: NON-U.S. RE LN CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENUSQ")]
    pub non_u_s_re_ln_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRENUSQR`
    Title: NON-U.S. RE LN CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRENUSQR")]
    pub non_u_s_re_ln_charge_offs_ratio_drrenusqr: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERES`
    Title: RE LOANS 1-4 FAMILY CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERES")]
    pub re_loans_1_4_family_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERESR`
    Title: RE LOANS 1-4 FAMILY CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERESR")]
    pub re_loans_1_4_family_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERESQ`
    Title: RE LOANS 1-4 FAMILY CHG-OFFS-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERESQ")]
    pub re_loans_1_4_family_chg_offs_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERESQR`
    Title: RE LOANS 1-4 FAMILY CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERESQR")]
    pub re_loans_1_4_family_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERSF2`
    Title: RE LN 1-4 FAM JR LIEN-CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERSF2")]
    pub re_ln_1_4_fam_jr_lien_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERSF2R`
    Title: RE LN 1-4 FAM JR LIEN-CHG-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERSF2R")]
    pub re_ln_1_4_fam_jr_lien_chg_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERS2Q`
    Title: RE LN 1-4 FAM JR LIEN-CHG-OFF QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERS2Q")]
    pub re_ln_1_4_fam_jr_lien_chg_off_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERS2QR`
    Title: RE LN 1-4 FAM JR LIEN-CHG-OFF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERS2QR")]
    pub re_ln_1_4_fam_jr_lien_chg_off_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERSFM`
    Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERSFM")]
    pub re_ln_1_4_fam_first_lien_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERSFMR`
    Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERSFMR")]
    pub re_ln_1_4_fam_first_lien_chg_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERSFQ`
    Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERSFQ")]
    pub re_ln_1_4_fam_first_lien_chg_off_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRRERSFQR`
    Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRRERSFQR")]
    pub re_ln_1_4_fam_first_lien_chg_off_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREOFFDOM`
    Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREOFFDOM")]
    pub real_estate_loan_charge_offs_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREOFFDOMR`
    Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREOFFDOMR")]
    pub real_estate_loan_charge_offs_domestic_offices_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREOFFDOMQ`
    Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREOFFDOMQ")]
    pub real_estate_loan_charge_offs_domestic_offices_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `DRREOFFDOMQR`
    Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DRREOFFDOMQR")]
    pub real_estate_loan_charge_offs_domestic_offices_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EDCM`
    Title: EQUITY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EDCM")]
    pub equity: Option<f64>,

    #[doc = r#"## FDIC Field:: `EEFF`
    Title: EFFICIENCY RATIO EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EEFF")]
    pub efficiency_ratio_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `EEFFQ`
    Title: EFFICIENCY RATIO EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EEFFQ")]
    pub efficiency_ratio_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EEFFR`
    Title: EFFICIENCY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EEFFR")]
    pub efficiency_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EEFFQR`
    Title: EFFICIENCY QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EEFFQR")]
    pub efficiency_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EFFDATE`
    Title: EFFECTIVE DATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EFFDATE")]
    pub effective_date: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTGW`
    Title: GOODWILL IMPAIRMENT LOSSES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTGW")]
    pub goodwill_impairment_losses: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTGWR`
    Title: GOODWILL IMPAIRMENT LOSSES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTGWR")]
    pub goodwill_impairment_losses_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTGWQ`
    Title: GOODWILL IMPAIRMENT LOSSES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTGWQ")]
    pub goodwill_impairment_losses_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTGWQR`
    Title: GOODWILL IMPAIRMENT LOSSES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTGWQR")]
    pub goodwill_impairment_losses_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTOTH`
    Title: AMORT & IMPAIR LOSSES OTH INTAN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTOTH")]
    pub amort_impair_losses_oth_intan: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTOTHR`
    Title: AMORT & IMPAIR LOSSES OTH INTAN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTOTHR")]
    pub amort_impair_losses_oth_intan_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTOTHQ`
    Title: AMORT & IMPAIR LOSSES OTH INTAN QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTOTHQ")]
    pub amort_impair_losses_oth_intan_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EINTOTHQR`
    Title: AMORT & IMPAIR LOSSES OTH INTAN QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EINTOTHQR")]
    pub amort_impair_losses_oth_intan_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNANTR`
    Title: LOAN LOSS PROV/NT CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNANTR")]
    pub loan_loss_prov_nt_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATRA`
    Title: ELNATRA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATRA")]
    pub elnatra: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATRY`
    Title: CREDIT LOSS PROV/AVE ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATRY")]
    pub credit_loss_prov_ave_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ELNATRYQ`
    Title: CREDIT LOSS PROV/AVE ASSETS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ELNATRYQ")]
    pub credit_loss_prov_ave_assets_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCEAUTO`
    Title: CR EXPOSURE-ENHANCEMENTS - AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCEAUTO")]
    pub cr_exposure_enhancements_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCEAUTOR`
    Title: CR EXPOSURE-ENHANCEMENTS - AUTO RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCEAUTOR")]
    pub cr_exposure_enhancements_auto_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCECI`
    Title: CR EXPOSURE - ENHANCEMENTS - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCECI")]
    pub cr_exposure_enhancements_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCECIR`
    Title: CR EXPOSURE - ENHANCEMENTS - CI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCECIR")]
    pub cr_exposure_enhancements_ci_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCECON`
    Title: CR EXPOSURE - ENHANCEMENTS - CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCECON")]
    pub cr_exposure_enhancements_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCECONR`
    Title: CR EXPOSURE - ENHANCEMENTS - CON RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCECONR")]
    pub cr_exposure_enhancements_con_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCEOTH`
    Title: CR EXPOSURE - ENHANCEMENTS - OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCEOTH")]
    pub cr_exposure_enhancements_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCEOTHR`
    Title: CR EXPOSURE - ENHANCEMENTS - OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCEOTHR")]
    pub cr_exposure_enhancements_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCERES`
    Title: CR EXPOSURE - ENHANCEMENTS - RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCERES")]
    pub cr_exposure_enhancements_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENCERESR`
    Title: CR EXPOSURE - ENHANCEMENTS - RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENCERESR")]
    pub cr_exposure_enhancements_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHINT`
    Title: OTHER INTEREST EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHINT")]
    pub other_interest_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHINTR`
    Title: OTHER INTEREST EXPENSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHINTR")]
    pub other_interest_expense_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHINTQ`
    Title: OTHER INTEREST EXPENSE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHINTQ")]
    pub other_interest_expense_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHINTQR`
    Title: OTHER INTEREST EXPENSE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHINTQR")]
    pub other_interest_expense_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQ5`
    Title: TOTAL BANK EQUITY CAPITAL-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQ5")]
    pub total_bank_equity_capital_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCBHCTR`
    Title: TRANSACTIONS WITH BHC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCBHCTR")]
    pub transactions_with_bhc: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCBHCTRR`
    Title: TRANSACTIONS WITH BHC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCBHCTRR")]
    pub transactions_with_bhc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCCOMPI`
    Title: OTHER COMPREHENSIVE INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCCOMPI")]
    pub other_comprehensive_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCCOMPIR`
    Title: OTHER COMPREHENSIVE INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCCOMPIR")]
    pub other_comprehensive_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVA`
    Title: CASH DIVIDENDS ON COMM & PFD-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVA")]
    pub cash_dividends_on_comm_pfd_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCMRG`
    Title: CHANGES DUE TO MERGERS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCMRG")]
    pub changes_due_to_mergers: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCMRGR`
    Title: CHANGES DUE TO MERGERS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCMRGR")]
    pub changes_due_to_mergers_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCPREV`
    Title: BK EQ CAP MOST RECENTLY REPORTED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCPREV")]
    pub bk_eq_cap_most_recently_reported: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCPREVR`
    Title: BK EQ CAP MOST RECENTLY REPORTED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCPREVR")]
    pub bk_eq_cap_most_recently_reported_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCREST`
    Title: ACCOUNTING CHANGES & CORRECTIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCREST")]
    pub accounting_changes_corrections: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCRESTR`
    Title: ACCOUNTING CHANGES & CORRECTIONS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCRESTR")]
    pub accounting_changes_corrections_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCSTKRX`
    Title: SALE OF CAPITAL STOCK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCSTKRX")]
    pub sale_of_capital_stock: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCSTKRXR`
    Title: SALE OF CAPITAL STOCK RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCSTKRXR")]
    pub sale_of_capital_stock_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCSXQ`
    Title: SALE OF CAPITAL STOCK QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCSXQ")]
    pub sale_of_capital_stock_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCSXQR`
    Title: SALE OF CAPITAL STOCK QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCSXQR")]
    pub sale_of_capital_stock_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCTRSTX`
    Title: TREASURY STOCK TRANSACTIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCTRSTX")]
    pub treasury_stock_transactions: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCTRSTXR`
    Title: TREASURY STOCK TRANSACTIONS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCTRSTXR")]
    pub treasury_stock_transactions_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQTOT`
    Title: TOTAL EQUITY CAPITAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQTOT")]
    pub total_equity_capital: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQTOTR`
    Title: TOTAL EQUITY CAPITAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQTOTR")]
    pub total_equity_capital_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQV`
    Title: BANK EQUITY CAPITAL/ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQV")]
    pub bank_equity_capital_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ERNAST`
    Title: TOTAL EARNING ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ERNAST")]
    pub total_earning_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ERNAST2`
    Title: TOTAL EARNING ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ERNAST2")]
    pub total_earning_assets_ernast2: Option<f64>,

    #[doc = r#"## FDIC Field:: `ERNAST5`
    Title: TOTAL EARNING ASSETS-CAVG5I
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ERNAST5")]
    pub total_earning_assets_cavg5i: Option<f64>,

    #[doc = r#"## FDIC Field:: `ERNASTR`
    Title: EARNING ASSETS / TOTAL ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ERNASTR")]
    pub earning_assets_total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESTYMD`
    Title: ESTABLISHED DATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESTYMD")]
    pub established_date: Option<f64>,

    #[doc = r#"## FDIC Field:: `ENDEFYMD`
    Title: INACTIVE DATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ENDEFYMD")]
    pub inactive_date: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORG_END_NUM_DTE`
    Title: INACTIVE DATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORG_END_NUM_DTE")]
    pub inactive_date_org_end_num_dte: Option<f64>,

    #[doc = r#"## FDIC Field:: `ETTLOTMG`
    Title: TT&L
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ETTLOTMG")]
    pub tt_l: Option<f64>,

    #[doc = r#"## FDIC Field:: `FORMTFR`
    Title: THRIFT FINANCIAL REPORT FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FORMTFR")]
    pub thrift_financial_report_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `FX`
    Title: FOREIGN EXCHANGE-TOTAL CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FX")]
    pub foreign_exchange_total_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `FXFFC`
    Title: FOR EXCH-FUTURES & FORWARD CONTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FXFFC")]
    pub for_exch_futures_forward_contr: Option<f64>,

    #[doc = r#"## FDIC Field:: `FXNVS`
    Title: FOR EXCHANGE-SWAPS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FXNVS")]
    pub for_exchange_swaps: Option<f64>,

    #[doc = r#"## FDIC Field:: `FXPOC`
    Title: FOR EXCH-PUR OPTION CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FXPOC")]
    pub for_exch_pur_option_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `FXSPOT`
    Title: SPOT FOREIGN EXCHANGE CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FXSPOT")]
    pub spot_foreign_exchange_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `FXWOC`
    Title: FOR EXCH-WRITTEN OPTION CONTRACT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FXWOC")]
    pub for_exch_written_option_contract: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBEFTXQ`
    Title: INC BEFORE INC TAXS & DISC-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBEFTXQ")]
    pub inc_before_inc_taxs_disc_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBEFXTR`
    Title: INCOME BEFORE DISC OPR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBEFXTR")]
    pub income_before_disc_opr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBEFXTRR`
    Title: INCOME BEFORE DISC OPR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBEFXTRR")]
    pub income_before_disc_opr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBEFXTRQ`
    Title: INCOME BEFORE DISC OPR QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBEFXTRQ")]
    pub income_before_disc_opr_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IEFF`
    Title: EFFICIENCY RATIO INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IEFF")]
    pub efficiency_ratio_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `IEFFQ`
    Title: EFFICIENCY RATIO INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IEFFQ")]
    pub efficiency_ratio_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBEFXTRQR`
    Title: INCOME BEFORE DISC OPR QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBEFXTRQR")]
    pub income_before_disc_opr_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFIDUC`
    Title: FIDUCIARY ACTIVITIES INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFIDUC")]
    pub fiduciary_activities_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFIDUCR`
    Title: FIDUCIARY ACTIVITIES INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFIDUCR")]
    pub fiduciary_activities_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFIDUCQ`
    Title: FIDUCIARY ACTIVITIES INCOME-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFIDUCQ")]
    pub fiduciary_activities_income_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IFIDUCQR`
    Title: FIDUCIARY ACTIVITIES INCOME-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IFIDUCQR")]
    pub fiduciary_activities_income_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCMEX`
    Title: TRADING ACCOUNT-COMMODITY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCMEX")]
    pub trading_account_commodity: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCMEXR`
    Title: TRADING ACCOUNT-COMMODITY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCMEXR")]
    pub trading_account_commodity_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCMEXQ`
    Title: TRADING ACCOUNT-COMMODITY QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCMEXQ")]
    pub trading_account_commodity_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCMEXQR`
    Title: TRADING ACCOUNT-COMMODITY RATIO QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCMEXQR")]
    pub trading_account_commodity_ratio_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCREX`
    Title: TRADING REVENUE- CREDIT EXPOSURE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCREX")]
    pub trading_revenue_credit_exposure: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCREXR`
    Title: TRADING REVENUE- CREDIT EXPOSURE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCREXR")]
    pub trading_revenue_credit_exposure_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCREXQ`
    Title: TRADING REVENUE- CREDIT EXPOSURE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCREXQ")]
    pub trading_revenue_credit_exposure_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLCREXQR`
    Title: TRADING REVENUE- CREDIT EXPOSURE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLCREXQR")]
    pub trading_revenue_credit_exposure_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLEDEX`
    Title: TRADING ACCOUNT-EQ DERIVATIVE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLEDEX")]
    pub trading_account_eq_derivative: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLEDEXR`
    Title: TRADING ACCOUNT-EQ DERIVATIVE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLEDEXR")]
    pub trading_account_eq_derivative_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLEDEXQ`
    Title: TRADING ACCOUNT-EQ DERIVATIVE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLEDEXQ")]
    pub trading_account_eq_derivative_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLEDEXQR`
    Title: TRADING ACCOUNT-EQ DERIVATIVE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLEDEXQR")]
    pub trading_account_eq_derivative_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLFXEX`
    Title: TRADING ACCOUNT-FOREIGN EXCHANGE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLFXEX")]
    pub trading_account_foreign_exchange: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLFXEXR`
    Title: RADING ACCOUNT-FOREIGN EXCHANGE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLFXEXR")]
    pub rading_account_foreign_exchange_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLFXEXQ`
    Title: TRADING ACCOUNT-FOREIGN EXCHANGE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLFXEXQ")]
    pub trading_account_foreign_exchange_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLFXEXQR`
    Title: RADING ACCOUNT-FOREIGN EXCHANGE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLFXEXQR")]
    pub rading_account_foreign_exchange_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLRTEX`
    Title: TRADING ACCOUNT-INTEREST RATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLRTEX")]
    pub trading_account_interest_rate: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLRTEXR`
    Title: TRADING ACCOUNT-INTEREST RATE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLRTEXR")]
    pub trading_account_interest_rate_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLRTEXQ`
    Title: TRADING ACCOUNT-INTEREST RATE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLRTEXQ")]
    pub trading_account_interest_rate_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLRTEXQR`
    Title: TRADING ACCOUNT-INTEREST RATE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLRTEXQR")]
    pub trading_account_interest_rate_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLSECQ`
    Title: SECURITIES GAINS AND LOSSES-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLSECQ")]
    pub securities_gains_and_losses_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLTRAD`
    Title: TRADING REVENUES-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLTRAD")]
    pub trading_revenues_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLTRADR`
    Title: TRADING REVENUES-TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLTRADR")]
    pub trading_revenues_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLTRDQ`
    Title: TRADING REVENUE-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLTRDQ")]
    pub trading_revenue_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLTRDQR`
    Title: TRADING REVENUE-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLTRDQR")]
    pub trading_revenue_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSCOM`
    Title: INSURANCE COMMISSIONS & FEES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSCOM")]
    pub insurance_commissions_fees: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSCOMR`
    Title: INSURANCE COMMISSIONS & FEES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSCOMR")]
    pub insurance_commissions_fees_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSCOMQ`
    Title: INSURANCE COMMISSIONS & FEES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSCOMQ")]
    pub insurance_commissions_fees_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSCOMQR`
    Title: INSURANCE COMMISSIONS & FEES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSCOMQR")]
    pub insurance_commissions_fees_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSOTH`
    Title: INSURANCE COM+FEES-OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSOTH")]
    pub insurance_com_fees_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSOTHR`
    Title: INSURANCE COM+FEES-OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSOTHR")]
    pub insurance_com_fees_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSOTHQ`
    Title: INSURANCE COM+FEES-OTHER QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSOTHQ")]
    pub insurance_com_fees_other_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSOTHQR`
    Title: INSURANCE COM+FEES-OTHER QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSOTHQR")]
    pub insurance_com_fees_other_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSUND`
    Title: INSURANCE UNDERWRITNG INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSUND")]
    pub insurance_underwritng_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSUNDR`
    Title: INSURANCE UNDERWRITNG INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSUNDR")]
    pub insurance_underwritng_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSUNDQ`
    Title: INSURANCE UNDERWRITNG INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSUNDQ")]
    pub insurance_underwritng_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINSUNDQR`
    Title: INSURANCE UNDERWRITNG INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINSUNDQR")]
    pub insurance_underwritng_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINVFEE`
    Title: INVEST BANK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINVFEE")]
    pub invest_bank: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINVFEER`
    Title: INVEST BANK RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINVFEER")]
    pub invest_bank_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINVFEEQ`
    Title: INVEST BANK QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINVFEEQ")]
    pub invest_bank_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IINVFEEQR`
    Title: INVEST BANK QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IINVFEEQR")]
    pub invest_bank_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSAGNT1`
    Title: PRIMARY INSURER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSAGNT1")]
    pub primary_insurer: Option<String>,

    #[doc = r#"## FDIC Field:: `INTANGCC`
    Title: PURCH CC REL & NONMTG SER ASTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANGCC")]
    pub purch_cc_rel_nonmtg_ser_asts: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTANGW`
    Title: GOODWILL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANGW")]
    pub goodwill: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTANGWR`
    Title: GOODWILL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANGWR")]
    pub goodwill_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTANMSR`
    Title: MORTGAGE SERVICING ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANMSR")]
    pub mortgage_servicing_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTANMSRR`
    Title: MORTGAGE SERVICING ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANMSRR")]
    pub mortgage_servicing_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTANOTH`
    Title: OTHER IDENTIFIABLE INTANG ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANOTH")]
    pub other_identifiable_intang_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTANOTHR`
    Title: OTHER IDENTIFIABLE INTANG ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTANOTHR")]
    pub other_identifiable_intang_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINCYQ`
    Title: INTEREST INCOME/EARNING ASSETS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINCYQ")]
    pub interest_income_earning_assets_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINCA`
    Title: TOTAL INTEREST INCOME ANNUAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINCA")]
    pub total_interest_income_annual: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTNII`
    Title: OTHER NONINTEREST INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTNII")]
    pub other_noninterest_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTNIIR`
    Title: OTHER NONINTEREST INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTNIIR")]
    pub other_noninterest_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTNIIQ`
    Title: OTHER NONINTEREST INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTNIIQ")]
    pub other_noninterest_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IOTNIIQR`
    Title: OTHER NONINTEREST INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IOTNIIQR")]
    pub other_noninterest_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISECZ`
    Title: SECURITIZATION INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISECZ")]
    pub securitization_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISECZR`
    Title: SECURITIZATION INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISECZR")]
    pub securitization_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISECZQ`
    Title: SECURITIZATION INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISECZQ")]
    pub securitization_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISECZQR`
    Title: SECURITIZATION INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISECZQR")]
    pub securitization_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERCHGQ`
    Title: SERVICE CHARGE ON DEP ACCTS-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERCHGQ")]
    pub service_charge_on_dep_accts_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERCHGQR`
    Title: SERVICE CHARGE ON DEPOSIT ACCTS-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERCHGQR")]
    pub service_charge_on_deposit_accts_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERFEE`
    Title: SERVICING FEES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERFEE")]
    pub servicing_fees: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERFEER`
    Title: SERVICING FEES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERFEER")]
    pub servicing_fees_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERFEEQ`
    Title: SERVICING FEES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERFEEQ")]
    pub servicing_fees_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERFEEQR`
    Title: SERVICING FEES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERFEEQR")]
    pub servicing_fees_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IVENCAP`
    Title: VENTURE CAPITAL REVENUE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IVENCAP")]
    pub venture_capital_revenue: Option<f64>,

    #[doc = r#"## FDIC Field:: `IVENCAPR`
    Title: VENTURE CAPITAL REVENUE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IVENCAPR")]
    pub venture_capital_revenue_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IVENCAPQ`
    Title: VENTURE CAPITAL REVENUE QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IVENCAPQ")]
    pub venture_capital_revenue_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IVENCAPQR`
    Title: VENTURE CAPITAL REVENUE QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IVENCAPQR")]
    pub venture_capital_revenue_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LAG`
    Title: AG LOANS - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LAG")]
    pub ag_loans_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `LAGR`
    Title: AG LOANS - LOSS SHARE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LAGR")]
    pub ag_loans_loss_share_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LCI`
    Title: C&I LOANS - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LCI")]
    pub c_i_loans_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `LCIR`
    Title: C&I LOANS - LOSS SHARE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LCIR")]
    pub c_i_loans_loss_share_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LCON`
    Title: CONSUMER LOANS - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LCON")]
    pub consumer_loans_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `LCONR`
    Title: CONSUMER LOANS - LOSS SHARE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LCONR")]
    pub consumer_loans_loss_share_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LIABFOR`
    Title: TOTAL LIABILITIES-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LIABFOR")]
    pub total_liabilities_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG1`
    Title: AGRICULTURAL LOANS-UNDER 100-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG1")]
    pub agricultural_loans_under_100: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG1R`
    Title: AGRICULTURAL LOANS-UNDER 100-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG1R")]
    pub agricultural_loans_under_100_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG2`
    Title: AGRICULTURAL LOANS-100-250-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG2")]
    pub agricultural_loans_100_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG2R`
    Title: AGRICULTURAL LOANS-100-250-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG2R")]
    pub agricultural_loans_100_250_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG3`
    Title: AGRICULTURAL LOANS-250-500-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG3")]
    pub agricultural_loans_250_500: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG3R`
    Title: AGRICULTURAL LOANS-250-500-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG3R")]
    pub agricultural_loans_250_500_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG4`
    Title: AGRICULTURAL LOANS-UNDER 500-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG4")]
    pub agricultural_loans_under_500: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG4R`
    Title: AGRICULTURAL LOANS-UNDER 500-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG4R")]
    pub agricultural_loans_under_500_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG5`
    Title: AG LOANS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG5")]
    pub ag_loans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG22`
    Title: AG LOANS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG22")]
    pub ag_loans_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG1N`
    Title: AGRICULTURAL LOANS-UNDER 100-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG1N")]
    pub agricultural_loans_under_100_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG1NR`
    Title: AGRICULTURAL LOANS-UNDER 100-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG1NR")]
    pub agricultural_loans_under_100_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG2N`
    Title: AGRICULTURAL LOANS-100-250-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG2N")]
    pub agricultural_loans_100_250_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG2NR`
    Title: AGRICULTURAL LOANS-100-250-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG2NR")]
    pub agricultural_loans_100_250_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG3N`
    Title: AGRICULTURAL LOANS-250-500-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG3N")]
    pub agricultural_loans_250_500_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG3NR`
    Title: AGRICULTURAL LOANS-250-500-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG3NR")]
    pub agricultural_loans_250_500_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG4N`
    Title: AGRICULTURAL LOANS-UNDER 500-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG4N")]
    pub agricultural_loans_under_500_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAG4NR`
    Title: AGRICULTURAL LOANS-UNDER 500-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAG4NR")]
    pub agricultural_loans_under_500_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAGFOR`
    Title: AGRICULTURAL LOANS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAGFOR")]
    pub agricultural_loans_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAGFORR`
    Title: AGRICULTURAL LOANS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAGFORR")]
    pub agricultural_loans_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNATRESR`
    Title: LOAN LOSS RESERVE/GROSS LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNATRESR")]
    pub loan_loss_reserve_gross_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAUTO2`
    Title: CONSUMER LOANS - AUTO - CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAUTO2")]
    pub consumer_loans_auto_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNAUTO5`
    Title: CONSUMER LOANS - AUTO - CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNAUTO5")]
    pub consumer_loans_auto_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI1`
    Title: C&I LOANS-UNDER-100-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI1")]
    pub c_i_loans_under_100: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI1R`
    Title: C&I LOANS-UNDER-100-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI1R")]
    pub c_i_loans_under_100_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI2`
    Title: C&I LOANS-100-250-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI2")]
    pub c_i_loans_100_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI2R`
    Title: C&I LOANS-100-250-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI2R")]
    pub c_i_loans_100_250_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI3`
    Title: C&I LOANS-250-1M-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI3")]
    pub c_i_loans_250_1m: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI3R`
    Title: C&I LOANS-250-1M-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI3R")]
    pub c_i_loans_250_1m_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI4`
    Title: C&I LOANS-UNDER-1M-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI4")]
    pub c_i_loans_under_1m: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI4R`
    Title: C&I LOANS-UNDER-1M-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI4R")]
    pub c_i_loans_under_1m_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI5`
    Title: C&I LOANS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI5")]
    pub c_i_loans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI22`
    Title: C&I LOANS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI22")]
    pub c_i_loans_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI1N`
    Title: C&I LOANS-UNDER-100-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI1N")]
    pub c_i_loans_under_100_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI1NR`
    Title: C&I LOANS-UNDER-100-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI1NR")]
    pub c_i_loans_under_100_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI2N`
    Title: C&I LOANS-100-250-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI2N")]
    pub c_i_loans_100_250_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI2NR`
    Title: C&I LOANS-250-1M-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI2NR")]
    pub c_i_loans_250_1m_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI3N`
    Title: C&I LOANS-250-1M-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI3N")]
    pub c_i_loans_250_1m_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI3NR`
    Title: C&I LOANS-250-1M-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI3NR")]
    pub c_i_loans_250_1m_num_ratio_lnci3nr: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI4N`
    Title: C&I LOANS-UNDER-1M-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI4N")]
    pub c_i_loans_under_1m_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCI4NR`
    Title: C&I LOANS-UNDER-1M-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCI4NR")]
    pub c_i_loans_under_1m_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCIFOR`
    Title: C&I LOANS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCIFOR")]
    pub c_i_loans_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCIFORR`
    Title: C&I LOANS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCIFORR")]
    pub c_i_loans_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCINUS`
    Title: C&I LOANS-NON-U.S. DOMICILE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCINUS")]
    pub c_i_loans_non_u_s_domicile: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCINUSF`
    Title: C&I LOANS-NON-U.S. DOMICILE-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCINUSF")]
    pub c_i_loans_non_u_s_domicile_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCINUSFR`
    Title: C&I LOANS-NON-U.S. DOMICILE-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCINUSFR")]
    pub c_i_loans_non_u_s_domicile_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCOMRE`
    Title: COMMERCIAL RE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCOMRE")]
    pub commercial_re_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCOMRER`
    Title: COMMERCIAL RE LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCOMRER")]
    pub commercial_re_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCOMRE2`
    Title: COMMERCIAL RE LOANS2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCOMRE2")]
    pub commercial_re_loans2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCOMRE5`
    Title: COMMERCIAL RE LOANS CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCOMRE5")]
    pub commercial_re_loans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCON2`
    Title: CONSUMER LOANS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCON2")]
    pub consumer_loans_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCON5`
    Title: CONSUMER LOANS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCON5")]
    pub consumer_loans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONFOR`
    Title: CONSUMER LOANS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONFOR")]
    pub consumer_loans_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONFORR`
    Title: CONSUMER LOANS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONFORR")]
    pub consumer_loans_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONORP`
    Title: OTHER CONSUMER & RELATED PLANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONORP")]
    pub other_consumer_related_plans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONOT2`
    Title: OTHER CONSUMER LOANS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONOT2")]
    pub other_consumer_loans_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONOT5`
    Title: OTHER CONSUMER LOANS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONOT5")]
    pub other_consumer_loans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONRP`
    Title: CONSUMER LNS-RELATED PLANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONRP")]
    pub consumer_lns_related_plans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONRPR`
    Title: CONSUMER LNS-RELATED PLANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONRPR")]
    pub consumer_lns_related_plans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONTRA`
    Title: OTHER CONTRA ACCOUNTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONTRA")]
    pub other_contra_accounts: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONTRAR`
    Title: OTHER CONTRA ACCOUNTS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONTRAR")]
    pub other_contra_accounts_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCRCD2`
    Title: CREDIT CARD PLANS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCRCD2")]
    pub credit_card_plans_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCRCD5`
    Title: CREDIT CARD PLANS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCRCD5")]
    pub credit_card_plans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPAC`
    Title: TOTAL DEP INST LNS & ACCEPT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPAC")]
    pub total_dep_inst_lns_accept: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPACD`
    Title: TOTAL DEP INST LNS & ACCEPT-DOM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPACD")]
    pub total_dep_inst_lns_accept_dom: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPAOBK`
    Title: LOANS TO DEPOSITORY INSTITUTIONS AND ACCEPTANCE OF OTHER BANKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPAOBK")]
    pub loans_to_depository_institutions_and_acceptance_of_other_banks: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPAOBKR`
    Title: LOANS TO DEPOSITORY INSTITUTIONS AND ACCEPTANCE OF OTHER BANKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPAOBKR")]
    pub loans_to_depository_institutions_and_acceptance_of_other_banks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPCB`
    Title: DEP INST LNS-COMMERCIAL BANKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPCB")]
    pub dep_inst_lns_commercial_banks: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPCBF`
    Title: DEP INST LNS-COMMERCIAL BK-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPCBF")]
    pub dep_inst_lns_commercial_bk_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPCBFR`
    Title: DEP INST LNS-COMMERCIAL BK-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPCBFR")]
    pub dep_inst_lns_commercial_bk_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPFC`
    Title: DEP INST LNS-FOR COUNTRY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPFC")]
    pub dep_inst_lns_for_country: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPFCF`
    Title: DEP INST LNS-FOR COUNTRY-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPFCF")]
    pub dep_inst_lns_for_country_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPFCFR`
    Title: DEP INST LNS-FOR COUNTRY-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPFCFR")]
    pub dep_inst_lns_for_country_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPFUS`
    Title: DEP INST LNS-FOR COUNTRY-U.S. BR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPFUS")]
    pub dep_inst_lns_for_country_u_s_br: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPUS`
    Title: DEP INST LNS-OTH U.S. INST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPUS")]
    pub dep_inst_lns_oth_u_s_inst: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPUSB`
    Title: DEP INST LNS-COM BKS-U.S.BRANCH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPUSB")]
    pub dep_inst_lns_com_bks_u_s_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPUSF`
    Title: DEP INST LNS-OTH U.S. INST-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPUSF")]
    pub dep_inst_lns_oth_u_s_inst_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNDEPUSFR`
    Title: DEP INST LNS-OTH U.S. INST-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNDEPUSFR")]
    pub dep_inst_lns_oth_u_s_inst_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNEXAMT`
    Title: EXECUTIVE OFFICER LOANS-AMOUNT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNEXAMT")]
    pub executive_officer_loans_amount: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNEXAMTR`
    Title: EXECUTIVE OFFICER LOANS-AMOUNT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNEXAMTR")]
    pub executive_officer_loans_amount_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNFGFOR`
    Title: FOREIGN GOVT LOANS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNFGFOR")]
    pub foreign_govt_loans_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNFGFORR`
    Title: FOREIGN GOVT LOANS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNFGFORR")]
    pub foreign_govt_loans_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSDEPR`
    Title: NET LOANS & LEASES/DEPOSITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSDEPR")]
    pub net_loans_leases_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSFOR`
    Title: LN&LS + UNEARNED INC-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSFOR")]
    pub ln_ls_unearned_inc_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSFORR`
    Title: LN&LS + UNEARNED INC-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSFORR")]
    pub ln_ls_unearned_inc_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGR5`
    Title: LOANS AND LEASES-TOTAL-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGR5")]
    pub loans_and_leases_total_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGRF`
    Title: LOANS AND LEASES-TOTAL-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGRF")]
    pub loans_and_leases_total_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGRFR`
    Title: LOANS AND LEASES-TOTAL-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGRFR")]
    pub loans_and_leases_total_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSNTV`
    Title: NET LOANS & LEASES/ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSNTV")]
    pub net_loans_leases_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSNQR`
    Title: NET LOANS & LEASES/ASSETS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSNQR")]
    pub net_loans_leases_assets_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSSALE`
    Title: LOANS & LEASES HELD FOR RESALE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSSALE")]
    pub loans_leases_held_for_resale: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSSALER`
    Title: LOANS & LEASES HELD FOR RESALE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSSALER")]
    pub loans_leases_held_for_resale_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNPLEDGE`
    Title: PLEDGED LOANS AND LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNPLEDGE")]
    pub pledged_loans_and_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNPLEDGER`
    Title: PLEDGED LOANS AND LEASES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNPLEDGER")]
    pub pledged_loans_and_leases_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNMUNIF`
    Title: MUNI LOANS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNMUNIF")]
    pub muni_loans_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNMUNIFR`
    Title: MUNI LOANS-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNMUNIFR")]
    pub muni_loans_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT1T3`
    Title: ALL OTHER LNS & LS * 1-3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT1T3")]
    pub all_other_lns_ls_1_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT1T3R`
    Title: ALL OTHER LNS & LS * 1-3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT1T3R")]
    pub all_other_lns_ls_1_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT3LES`
    Title: ALL OTHER LNS & LS*3 MO OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT3LES")]
    pub all_other_lns_ls_3_mo_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT3LESR`
    Title: ALL OTHER LNS & LS*3 MO OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT3LESR")]
    pub all_other_lns_ls_3_mo_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT3T5`
    Title: ALL OTHER LNS & LS * 3-5 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT3T5")]
    pub all_other_lns_ls_3_5_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT3T5R`
    Title: ALL OTHER LNS & LS * 3-5 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT3T5R")]
    pub all_other_lns_ls_3_5_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT3T12`
    Title: ALL OTHER LNS & LS * 3-12 MONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT3T12")]
    pub all_other_lns_ls_3_12_mons: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT3T12R`
    Title: ALL OTHER LNS & LS * 3-12 MONS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT3T12R")]
    pub all_other_lns_ls_3_12_mons_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT5T15`
    Title: ALL OTHER LNS & LS * 5-15 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT5T15")]
    pub all_other_lns_ls_5_15_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOT5T15R`
    Title: ALL OTHER LNS & LS * 5-15 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOT5T15R")]
    pub all_other_lns_ls_5_15_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTCI2`
    Title: OTHER LOANS & LEASES-QBP-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTCI2")]
    pub other_loans_leases_qbp_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTCI5`
    Title: OTHER LOANS & LEASES-QBP-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTCI5")]
    pub other_loans_leases_qbp_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTHERF`
    Title: LN TO NONDEP FIN INST & OTH-FGN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTHERF")]
    pub ln_to_nondep_fin_inst_oth_fgn: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTHERFR`
    Title: LN TO NONDEP FIN INST & OTH-FGN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTHERFR")]
    pub ln_to_nondep_fin_inst_oth_fgn_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTOV15`
    Title: ALL OTHER LNS & LS * OVER 15 YRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTOV15")]
    pub all_other_lns_ls_over_15_yrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNOTOV15R`
    Title: ALL OTHER LNS & LS * OVER 15 YRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNOTOV15R")]
    pub all_other_lns_ls_over_15_yrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG1`
    Title: RE AGRICULTURAL-UNDER 100-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG1")]
    pub re_agricultural_under_100: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG1R`
    Title: RE AGRICULTURAL-UNDER 100-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG1R")]
    pub re_agricultural_under_100_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG2`
    Title: RE AGRICULTURAL-100-250-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG2")]
    pub re_agricultural_100_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG2R`
    Title: RE AGRICULTURAL-100-250-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG2R")]
    pub re_agricultural_100_250_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG3`
    Title: RE AGRICULTURAL-250-500-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG3")]
    pub re_agricultural_250_500: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG3R`
    Title: RE AGRICULTURAL-250-500-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG3R")]
    pub re_agricultural_250_500_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG4`
    Title: RE AGRICULTURAL-UNDER 500-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG4")]
    pub re_agricultural_under_500: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG4R`
    Title: RE AGRICULTURAL-UNDER 500-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG4R")]
    pub re_agricultural_under_500_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG1N`
    Title: RE AGRICULTURAL-UNDER 100-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG1N")]
    pub re_agricultural_under_100_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG1NR`
    Title: RE AGRICULTURAL-UNDER 100-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG1NR")]
    pub re_agricultural_under_100_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG2N`
    Title: RE AGRICULTURAL-100-250-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG2N")]
    pub re_agricultural_100_250_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG2NR`
    Title: RE AGRICULTURAL-100-250-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG2NR")]
    pub re_agricultural_100_250_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG3N`
    Title: RE AGRICULTURAL-250-500-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG3N")]
    pub re_agricultural_250_500_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG3NR`
    Title: RE AGRICULTURAL-250-500-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG3NR")]
    pub re_agricultural_250_500_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG4N`
    Title: RE AGRICULTURAL-UNDER 500-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG4N")]
    pub re_agricultural_under_500_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG4NR`
    Title: RE AGRICULTURAL-UNDER 500-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG4NR")]
    pub re_agricultural_under_500_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECNFM`
    Title: 1-4 FAM RE CONSTRUCTION LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECNFM")]
    pub _1_4_fam_re_construction_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECNFMR`
    Title: 1-4 FAM RE CONSTRUCTION LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECNFMR")]
    pub _1_4_fam_re_construction_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECNOT`
    Title: OTHER RE CONSTRUCTION & LAND LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECNOT")]
    pub other_re_construction_land_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRECNOTR`
    Title: OTHER RE CONSTRUCTION & LAND LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRECNOTR")]
    pub other_re_construction_land_ln_lnrecnotr: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREOTH`
    Title: ALL OTHER RE OWNED-1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREOTH")]
    pub all_other_re_owned_1_4_family: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREOTH2`
    Title: ALL OTHER RE OWNED-1-4 FAMILY2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREOTH2")]
    pub all_other_re_owned_1_4_family2: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREOTH5`
    Title: RE 1-4 FAMILY OTHER LOANS CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREOTH5")]
    pub re_1_4_family_other_loans_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR1`
    Title: RE NONFARM NONRES-UNDER 100-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR1")]
    pub re_nonfarm_nonres_under_100: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR1R`
    Title: RE NONFARM NONRES-UNDER 100-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR1R")]
    pub re_nonfarm_nonres_under_100_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR2`
    Title: RE NONFARM NONRES-100-250-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR2")]
    pub re_nonfarm_nonres_100_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR2R`
    Title: RE NONFARM NONRES-100-250-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR2R")]
    pub re_nonfarm_nonres_100_250_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR3`
    Title: RE NONFARM NONRES-250-1M-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR3")]
    pub re_nonfarm_nonres_250_1m: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR3R`
    Title: RE NONFARM NONRES-250-1M-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR3R")]
    pub re_nonfarm_nonres_250_1m_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR4`
    Title: RE NONFARM NONRES-UNDER 1M-$
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR4")]
    pub re_nonfarm_nonres_under_1m: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR4R`
    Title: RE NONFARM NONRES-UNDER 1M-$ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR4R")]
    pub re_nonfarm_nonres_under_1m_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR1N`
    Title: RE NONFARM NONRES-UNDER 100-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR1N")]
    pub re_nonfarm_nonres_under_100_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR1NR`
    Title: RE NONFARM NONRES-UNDER 100-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR1NR")]
    pub re_nonfarm_nonres_under_100_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR2N`
    Title: RE NONFARM NONRES-100-250-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR2N")]
    pub re_nonfarm_nonres_100_250_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR2NR`
    Title: RE NONFARM NONRES-100-250-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR2NR")]
    pub re_nonfarm_nonres_100_250_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR3N`
    Title: RE NONFARM NONRES-250-1M-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR3N")]
    pub re_nonfarm_nonres_250_1m_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR3NR`
    Title: RE NONFARM NONRES-250-1M-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR3NR")]
    pub re_nonfarm_nonres_250_1m_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR4N`
    Title: RE NONFARM NONRES-UNDER 1M-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR4N")]
    pub re_nonfarm_nonres_under_1m_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENR4NR`
    Title: RE NONFARM NONRES-UNDER 1M-NUM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENR4NR")]
    pub re_nonfarm_nonres_under_1m_num_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENROT`
    Title: OTHER NONFARM NONRES RE LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENROT")]
    pub other_nonfarm_nonres_re_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENROTR`
    Title: OTHER NONFARM NONRES RE LNS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENROTR")]
    pub other_nonfarm_nonres_re_lns_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENROW`
    Title: OWNER-OCC NONFARM NONRES RE LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENROW")]
    pub owner_occ_nonfarm_nonres_re_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENROWR`
    Title: OWNER-OCC NONFARM NONRES RE LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENROWR")]
    pub owner_occ_nonfarm_nonres_re_lns_lnrenrowr: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENUS`
    Title: RE LNS-NON US ADDRESSEES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENUS")]
    pub re_lns_non_us_addressees: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRENUSR`
    Title: RE LNS-NON US ADDRESSEES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRENUSR")]
    pub re_lns_non_us_addressees_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERSF1`
    Title: RE 1-4 FAMILY-FIRST LIENS-ADJUST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERSF1")]
    pub re_1_4_family_first_liens_adjust: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERSF1R`
    Title: RE 1-4 FAMILY-FIRST LIENS-ADJUST RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERSF1R")]
    pub re_1_4_family_first_liens_adjust_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERSF2`
    Title: RE 1-4 FAMILY-SECOND LIENS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERSF2")]
    pub re_1_4_family_second_liens: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERSF2R`
    Title: RE 1-4 FAMILY-SECOND LIENS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERSF2R")]
    pub re_1_4_family_second_liens_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERSFM`
    Title: RE 1-4 FAMILY-FIRST LIENS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERSFM")]
    pub re_1_4_family_first_liens: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERSFMR`
    Title: RE 1-4 FAMILY-FIRST LIENS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERSFMR")]
    pub re_1_4_family_first_liens_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRESNCR`
    Title: LOAN LOSS RESERVE/N/C LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRESNCR")]
    pub loan_loss_reserve_n_c_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS1T3`
    Title: RE 1-4 FAMILY * 1-3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS1T3")]
    pub re_1_4_family_1_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS1T3R`
    Title: RE 1-4 FAMILY * 1-3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS1T3R")]
    pub re_1_4_family_1_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS3LES`
    Title: RE 1-4 FAMILY * 3 MONS OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS3LES")]
    pub re_1_4_family_3_mons_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS3LESR`
    Title: RE 1-4 FAMILY * 3 MONS OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS3LESR")]
    pub re_1_4_family_3_mons_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS3T5`
    Title: RE 1-4 FAMILY * 3-5 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS3T5")]
    pub re_1_4_family_3_5_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS3T5R`
    Title: RE 1-4 FAMILY * 3-5 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS3T5R")]
    pub re_1_4_family_3_5_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS3T12`
    Title: RE 1-4 FAMILY * 3-12 MONTHS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS3T12")]
    pub re_1_4_family_3_12_months: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS3T12R`
    Title: RE 1-4 FAMILY * 3-12 MONTHS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS3T12R")]
    pub re_1_4_family_3_12_months_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS5T15`
    Title: RE 1-4 FAMILY * 5-15 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS5T15")]
    pub re_1_4_family_5_15_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRS5T15R`
    Title: RE 1-4 FAMILY * 5-15 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRS5T15R")]
    pub re_1_4_family_5_15_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRSOV15`
    Title: RE 1-4 FAMILY * OVER 15 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRSOV15")]
    pub re_1_4_family_over_15_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRSOV15R`
    Title: RE 1-4 FAMILY * OVER 15 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRSOV15R")]
    pub re_1_4_family_over_15_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNSB`
    Title: SMALL BUSINESS LNS SOLD-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNSB")]
    pub small_business_lns_sold_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNSBR`
    Title: SMALL BUSINESS LNS SOLD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNSBR")]
    pub small_business_lns_sold: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNSERV`
    Title: PRIN BAL- LNS SERVICE FOR OTHERS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNSERV")]
    pub prin_bal_lns_service_for_others: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNSERVR`
    Title: PRIN BAL- LNS SERVICE FOR OTHERS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNSERVR")]
    pub prin_bal_lns_service_for_others_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCCOM`
    Title: COMMERCIAL LETTERS OF CREDIT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCCOM")]
    pub commercial_letters_of_credit: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCCOMR`
    Title: COMMERCIAL LETTERS OF CREDIT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCCOMR")]
    pub commercial_letters_of_credit_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFPSB`
    Title: FIN & PERFORM STANDBY LOC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFPSB")]
    pub fin_perform_standby_loc: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFPSBR`
    Title: FIN & PERFORM STANDBY LOC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFPSBR")]
    pub fin_perform_standby_loc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFPSBK`
    Title: FIN & PERFORM STANDBY LOC-CONVEY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFPSBK")]
    pub fin_perform_standby_loc_convey: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFPSBKR`
    Title: FIN & PERFORM STANDBY LOC-CONVEY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFPSBKR")]
    pub fin_perform_standby_loc_convey_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFSB`
    Title: FINANCIAL STANDBY LOC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFSB")]
    pub financial_standby_loc: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFSBR`
    Title: FINANCIAL STANDBY LOC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFSBR")]
    pub financial_standby_loc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFSBK`
    Title: FINANCIAL STANDBY LOC-CONVEYED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFSBK")]
    pub financial_standby_loc_conveyed: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCFSBKR`
    Title: FINANCIAL STANDBY LOC-CONVEYED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCFSBKR")]
    pub financial_standby_loc_conveyed_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCPSB`
    Title: PERFORMANCE STANDBY LOC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCPSB")]
    pub performance_standby_loc: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCPSBR`
    Title: PERFORMANCE STANDBY LOC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCPSBR")]
    pub performance_standby_loc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCPSBK`
    Title: PERFORMANCE STANDBY LOC-CONVEYED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCPSBK")]
    pub performance_standby_loc_conveyed: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOCPSBKR`
    Title: PERFORMANCE STANDBY LOC-CONVEYED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOCPSBKR")]
    pub performance_standby_loc_conveyed_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOREGTY`
    Title: ORE PROTECTED - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOREGTY")]
    pub ore_protected_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOREGTYR`
    Title: ORE PROTECTED - LOSS SHARE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOREGTYR")]
    pub ore_protected_loss_share_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOTH`
    Title: ALL OTHER LN & LS - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOTH")]
    pub all_other_ln_ls_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `LOTHR`
    Title: ALL OTHER LN & LS - LOSS SHARE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LOTHR")]
    pub all_other_ln_ls_loss_share_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LREAG`
    Title: RE FARMLAND LN - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LREAG")]
    pub re_farmland_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `LREAGR`
    Title: RE FARMLAND LN - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LREAGR")]
    pub re_farmland_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LRECONS`
    Title: RE CONSTRUCT LN - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LRECONS")]
    pub re_construct_ln_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `LRECONSR`
    Title: RE CONSTRUCT LN - LOSS SHARE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LRECONSR")]
    pub re_construct_ln_loss_share_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LREMULT`
    Title: RE MULTIFAMILY LN-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LREMULT")]
    pub re_multifamily_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `LREMULTR`
    Title: RE MULTIFAMILY LN-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LREMULTR")]
    pub re_multifamily_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LRENRES`
    Title: RE NONFARM NONRES LN - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LRENRES")]
    pub re_nonfarm_nonres_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `LRENRESR`
    Title: RE NONFARM NONRES LN - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LRENRESR")]
    pub re_nonfarm_nonres_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LRERES`
    Title: RE 1-4 FAMILY LNS - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LRERES")]
    pub re_1_4_family_lns_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `LRERESR`
    Title: RE 1-4 FAMILY LNS - LOSS SHARE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LRERESR")]
    pub re_1_4_family_lns_loss_share_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSALNLS`
    Title: CARRY AMT LOSS SHARE-LNLS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSALNLS")]
    pub carry_amt_loss_share_lnls: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSALNLSR`
    Title: CARRY AMT LOSS SHARE-LNLS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSALNLSR")]
    pub carry_amt_loss_share_lnls_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSAOA`
    Title: CARRY AMT LOSS SHARE -OTH ASSET
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSAOA")]
    pub carry_amt_loss_share_oth_asset: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSAOAR`
    Title: CARRY AMT LOSS SHARE -OTH ASSET RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSAOAR")]
    pub carry_amt_loss_share_oth_asset_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSAORE`
    Title: CARRY AMT LOSS SHARE- ORE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSAORE")]
    pub carry_amt_loss_share_ore: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSAORER`
    Title: CARRY AMT LOSS SHARE- ORE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSAORER")]
    pub carry_amt_loss_share_ore_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSASCDBT`
    Title: CARRY AMT LOSS SHARE -DEBT SEC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSASCDBT")]
    pub carry_amt_loss_share_debt_sec: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSASCDBTR`
    Title: CARRY AMT LOSS SHARE -DEBT SEC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSASCDBTR")]
    pub carry_amt_loss_share_debt_sec_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSFOR`
    Title: LEASES-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSFOR")]
    pub leases_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `LSFORR`
    Title: LEASES-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LSFORR")]
    pub leases_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSA`
    Title: FIPS MSA CODE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MSA")]
    pub fips_msa_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSRECE`
    Title: OUT PRIN BAL MORT W/ RECOURSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MSRECE")]
    pub out_prin_bal_mort_w_recourse: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSRECER`
    Title: OUT PRIN BAL MORT W/ RECOURSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MSRECER")]
    pub out_prin_bal_mort_w_recourse_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSRESFCL`
    Title: 1-4 FM SERVICED IN FORECLOSURE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MSRESFCL")]
    pub _1_4_fm_serviced_in_foreclosure: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSRESFCLR`
    Title: 1-4 FM SERVICED IN FORECLOSURE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MSRESFCLR")]
    pub _1_4_fm_serviced_in_foreclosure_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSRNRECE`
    Title: OUT PRIN BAL MORT W/ NO RECOURSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MSRNRECE")]
    pub out_prin_bal_mort_w_no_recourse: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSRNRECER`
    Title: OUT PRIN BAL MORT W/ NO RECOURSE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MSRNRECER")]
    pub out_prin_bal_mort_w_no_recourse_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAAG`
    Title: NONACCRUAL-AGRICULTURAL LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAAG")]
    pub nonaccrual_agricultural_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAAGR`
    Title: NONACCRUAL-AGRICULTURAL LNS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAAGR")]
    pub nonaccrual_agricultural_lns_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAAGSM`
    Title: NONACCRUAL-AG LNS*SMALL BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAAGSM")]
    pub nonaccrual_ag_lns_small_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAAGSMR`
    Title: NONACCRUAL-AG LNS*SMALL BKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAAGSMR")]
    pub nonaccrual_ag_lns_small_bks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAASSET`
    Title: NONACCRUAL-TOTAL ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAASSET")]
    pub nonaccrual_total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAASSETR`
    Title: NONACCRUAL-AG LNS*SMALL BKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAASSETR")]
    pub nonaccrual_ag_lns_small_bks_ratio_naassetr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAAUTO`
    Title: NONACCRUAL AUTO LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAAUTO")]
    pub nonaccrual_auto_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAAUTOR`
    Title: NONACCRUAL AUTO LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAAUTOR")]
    pub nonaccrual_auto_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACI`
    Title: NONACCRUAL-C&I LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACI")]
    pub nonaccrual_c_i_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACIR`
    Title: NONACCRUAL-C&I LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACIR")]
    pub nonaccrual_c_i_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACINUS`
    Title: NONACCRUAL-C&I*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACINUS")]
    pub nonaccrual_c_i_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACINUSR`
    Title: NONACCRUAL-C&I*NON-U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACINUSR")]
    pub nonaccrual_c_i_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACON`
    Title: NONACCRUAL-CONSUMER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACON")]
    pub nonaccrual_consumer_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACONR`
    Title: NONACCRUAL-CONSUMER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACONR")]
    pub nonaccrual_consumer_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACONOTH`
    Title: NONACCRUAL-OTHER CONSUMER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACONOTH")]
    pub nonaccrual_other_consumer: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACONOTHR`
    Title: NONACCRUAL-OTHER CONSUMER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACONOTHR")]
    pub nonaccrual_other_consumer_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACRCD`
    Title: NONACCRUAL-CREDIT CARD PLANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACRCD")]
    pub nonaccrual_credit_card_plans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACRCDR`
    Title: NONACCRUAL-CREDIT CARD PLANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACRCDR")]
    pub nonaccrual_credit_card_plans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NADEP`
    Title: NONACCRUAL-DEP INST LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NADEP")]
    pub nonaccrual_dep_inst_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NADEPR`
    Title: NONACCRUAL-DEP INST LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NADEPR")]
    pub nonaccrual_dep_inst_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NADEPNUS`
    Title: NONACCRUAL-DEP INST*NON U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NADEPNUS")]
    pub nonaccrual_dep_inst_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `NADEPNUSR`
    Title: NONACCRUAL-DEP INST*NON U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NADEPNUSR")]
    pub nonaccrual_dep_inst_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAFG`
    Title: NONACCRUAL-FOREIGN GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAFG")]
    pub nonaccrual_foreign_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAFGR`
    Title: NONACCRUAL-FOREIGN GOVT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAFGR")]
    pub nonaccrual_foreign_govt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAGTY`
    Title: NONACCRUAL-GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAGTY")]
    pub nonaccrual_gty_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAGTYR`
    Title: NONACCRUAL -GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAGTYR")]
    pub nonaccrual_gty_ln_ls_nagtyr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAGTYGNM`
    Title: NONACCRUAL REBOOKED GNMA LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAGTYGNM")]
    pub nonaccrual_rebooked_gnma_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAGTYGNMR`
    Title: NONACCRUAL REBOOKED GNMA LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAGTYGNMR")]
    pub nonaccrual_rebooked_gnma_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAGTYPAR`
    Title: NONACCRUAL-PART GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAGTYPAR")]
    pub nonaccrual_part_gty_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAGTYPARR`
    Title: NONACCRUAL-PART GTY LN&LS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAGTYPARR")]
    pub nonaccrual_part_gty_ln_ls_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALAG`
    Title: NONACCRUAL AG LOANS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALAG")]
    pub nonaccrual_ag_loans_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALAGR`
    Title: NONACCRUAL AG LOANS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALAGR")]
    pub nonaccrual_ag_loans_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALCI`
    Title: NONACCRUAL C&I LNS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALCI")]
    pub nonaccrual_c_i_lns_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALCIR`
    Title: NONACCRUAL C&I LNS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALCIR")]
    pub nonaccrual_c_i_lns_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALCON`
    Title: NONACCRUAL CONSUMER LN -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALCON")]
    pub nonaccrual_consumer_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALCONR`
    Title: NONACCRUAL CONSUMER LN -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALCONR")]
    pub nonaccrual_consumer_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALGTY`
    Title: NONACCR PROTECT (GTY)-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALGTY")]
    pub nonaccr_protect_gty_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALGTYR`
    Title: NONACCRUAL PROTECT (GTY)-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALGTYR")]
    pub nonaccrual_protect_gty_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALNSALE`
    Title: NONACCRUAL-L&L HELD FOR SALE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALNSALE")]
    pub nonaccrual_l_l_held_for_sale: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALNSALER`
    Title: NONACCRUAL-L&L HELD FOR SALE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALNSALER")]
    pub nonaccrual_l_l_held_for_sale_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALOTH`
    Title: NONACCRUAL OTHER LNS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALOTH")]
    pub nonaccrual_other_lns_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALOTHR`
    Title: NONACCRUAL OTHER LNS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALOTHR")]
    pub nonaccrual_other_lns_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALREAG`
    Title: NONACCRUAL RE FARM-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALREAG")]
    pub nonaccrual_re_farm_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALREAGR`
    Title: NONACCRUAL RE FARM LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALREAGR")]
    pub nonaccrual_re_farm_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALRECON`
    Title: NONACCRUAL CONSTR LN -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALRECON")]
    pub nonaccrual_constr_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALRECONR`
    Title: NONACCRUAL CONSTR LN -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALRECONR")]
    pub nonaccrual_constr_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALREMUL`
    Title: NONACCRUAL MULTIFAM - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALREMUL")]
    pub nonaccrual_multifam_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALREMULR`
    Title: NONACCRUAL MULTIFAM - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALREMULR")]
    pub nonaccrual_multifam_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALRENRS`
    Title: NONACCRUAL NFNR LN - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALRENRS")]
    pub nonaccrual_nfnr_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALRENRSR`
    Title: NONACCRUAL NFNR LN - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALRENRSR")]
    pub nonaccrual_nfnr_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALRERES`
    Title: NONACCRUAL 1-4 FM LN-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALRERES")]
    pub nonaccrual_1_4_fm_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALRERESR`
    Title: NONACCRUAL 1-4 FM LN-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALRERESR")]
    pub nonaccrual_1_4_fm_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALS`
    Title: NONACCRUAL-LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALS")]
    pub nonaccrual_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALSR`
    Title: NONACCRUAL-LEASES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALSR")]
    pub nonaccrual_leases_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALTOT`
    Title: NONACCRUAL TOTAL LOANS - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALTOT")]
    pub nonaccrual_total_loans_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `NALTOTR`
    Title: NONACCRUAL TOTAL LOANS - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NALTOTR")]
    pub nonaccrual_total_loans_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAME`
    Title: INSTITUTION NAME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAME")]
    pub institution_name: Option<String>,

    #[doc = r#"## FDIC Field:: `NAMEFULL`
    Title: INSTITUTION FULL NAME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAMEFULL")]
    pub institution_full_name: Option<String>,

    #[doc = r#"## FDIC Field:: `NAOTHLN`
    Title: NONACCRUAL-ALL OTHER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAOTHLN")]
    pub nonaccrual_all_other_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAOTHLNR`
    Title: NONACCRUAL-ALL OTHER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAOTHLNR")]
    pub nonaccrual_all_other_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARE`
    Title: NONACCRUAL-REAL ESTATE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARE")]
    pub nonaccrual_real_estate_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARER`
    Title: NONACCRUAL-REAL ESTATE LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARER")]
    pub nonaccrual_real_estate_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAREAG`
    Title: NONACCRUAL-RE*FARMLAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAREAG")]
    pub nonaccrual_re_farmland: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAREAGR`
    Title: NONACCRUAL-RE*FARMLAND RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAREAGR")]
    pub nonaccrual_re_farmland_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARECNFM`
    Title: NONACCRUAL 1-4 FAM CONSTRUCT LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARECNFM")]
    pub nonaccrual_1_4_fam_construct_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARECNFMR`
    Title: NONACCRUAL 1-4 FAM CONSTRUCT LN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARECNFMR")]
    pub nonaccrual_1_4_fam_construct_ln_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARECNOT`
    Title: NONACCRUAL OTHER CONSTR & LAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARECNOT")]
    pub nonaccrual_other_constr_land: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARECNOTR`
    Title: NONACCRUAL OTHER CONSTR & LAND RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARECNOTR")]
    pub nonaccrual_other_constr_land_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARECONS`
    Title: NONACCRUAL-RE*CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARECONS")]
    pub nonaccrual_re_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARECONSR`
    Title: NONACCRUAL-RE*CONSTRUCTION RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARECONSR")]
    pub nonaccrual_re_construction_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAREFOR`
    Title: NONACCRUAL-RE*FOREIGN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAREFOR")]
    pub nonaccrual_re_foreign: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAREFORR`
    Title: NONACCRUAL-RE*FOREIGN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAREFORR")]
    pub nonaccrual_re_foreign_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARELOC`
    Title: NONACCRUAL-RE*1-4 FAM LINES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARELOC")]
    pub nonaccrual_re_1_4_fam_lines: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARELOCR`
    Title: NONACCRUAL-RE*1-4 FAM LINES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARELOCR")]
    pub nonaccrual_re_1_4_fam_lines_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAREMULT`
    Title: NONACCRUAL-RE*MULTIFAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAREMULT")]
    pub nonaccrual_re_multifamily: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAREMULTR`
    Title: NONACCRUAL-RE*MULTIFAMILY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NAREMULTR")]
    pub nonaccrual_re_multifamily_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENRES`
    Title: NONACCRUAL-RE*NONFARM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENRES")]
    pub nonaccrual_re_nonfarm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENRESR`
    Title: NONACCRUAL-RE*NONFARM NONRES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENRESR")]
    pub nonaccrual_re_nonfarm_nonres_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENROT`
    Title: NONACCRUAL OTHER NONFARM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENROT")]
    pub nonaccrual_other_nonfarm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENROTR`
    Title: NONACCRUAL OTHER NONFARM NONRES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENROTR")]
    pub nonaccrual_other_nonfarm_nonres_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENROW`
    Title: NONACCRUAL 0WN-OCC NONFRM NONRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENROW")]
    pub nonaccrual_0wn_occ_nonfrm_nonrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENROWR`
    Title: NONACCRUAL OWN-OCC NONFRM NONRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENROWR")]
    pub nonaccrual_own_occ_nonfrm_nonrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENUS`
    Title: NONACCRUAL-RE*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENUS")]
    pub nonaccrual_re_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARENUSR`
    Title: NONACCRUAL-RE*NON-U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARENUSR")]
    pub nonaccrual_re_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARERES`
    Title: NONACCRUAL-RE*1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARERES")]
    pub nonaccrual_re_1_4_family: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARERESR`
    Title: NONACCRUAL-RE*1-4 FAMILY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARERESR")]
    pub nonaccrual_re_1_4_family_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARERSF2`
    Title: NONACCRUAL-RE*1-4 JUNIOR LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARERSF2")]
    pub nonaccrual_re_1_4_junior_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARERSF2R`
    Title: NONACCRUAL-RE*1-4 JN LIEN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARERSF2R")]
    pub nonaccrual_re_1_4_jn_lien_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARERSFM`
    Title: NONACCRUAL-RE*1-4 IST LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARERSFM")]
    pub nonaccrual_re_1_4_ist_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARERSFMR`
    Title: NONACCRUAL-RE*1-4 IST LIEN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARERSFMR")]
    pub nonaccrual_re_1_4_ist_lien_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSCI`
    Title: NONACCRUAL RESTRUCT C&I LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSCI")]
    pub nonaccrual_restruct_c_i_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSCONS`
    Title: NONACCR RESTRUCT CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSCONS")]
    pub nonaccr_restruct_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSLNFM`
    Title: NONACCRUAL RESTRU LN- 1-4 FAM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSLNFM")]
    pub nonaccrual_restru_ln_1_4_fam: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSLNFMR`
    Title: NONACCRUAL RESTRU LN- 1-4 FAM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSLNFMR")]
    pub nonaccrual_restru_ln_1_4_fam_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSLNLS`
    Title: NONACCRUAL RESTRU LN EXCL 1-4 FM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSLNLS")]
    pub nonaccrual_restru_ln_excl_1_4_fm: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSLNLSR`
    Title: NONACCRUAL RESTRU LN EXCL 1-4 FM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSLNLSR")]
    pub nonaccrual_restru_ln_excl_1_4_fm_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSLNLT`
    Title: NONACCRUAL RESTRUCT LN- TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSLNLT")]
    pub nonaccrual_restruct_ln_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSLNLTR`
    Title: NONACCRUAL RESTRUCT LN- TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSLNLTR")]
    pub nonaccrual_restruct_ln_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSMULT`
    Title: NONACCRUAL RESTRUCT MULTIFAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSMULT")]
    pub nonaccrual_restruct_multifamily: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSNRES`
    Title: NONACCR RESTRUCTURED NFNR LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSNRES")]
    pub nonaccr_restructured_nfnr_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARSOTH`
    Title: NONACCRUAL RESTRUCT ALL OTH LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARSOTH")]
    pub nonaccrual_restruct_all_oth_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NASCDEBT`
    Title: NONACCRUAL-DEBT SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NASCDEBT")]
    pub nonaccrual_debt_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `NASCDEBTR`
    Title: NONACCRUAL-DEBT SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NASCDEBTR")]
    pub nonaccrual_debt_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCAG`
    Title: TOTAL N/C-AGRICULTURAL LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCAG")]
    pub total_n_c_agricultural_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCAUTO`
    Title: N/C AUTO LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCAUTO")]
    pub n_c_auto_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCCI`
    Title: TOTAL N/C-C&I LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCCI")]
    pub total_n_c_c_i_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCCOMRER`
    Title: NC COMMERCIAL RE/COMMERCIAL RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCCOMRER")]
    pub nc_commercial_re_commercial_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCCOMRE`
    Title: NC COMMERCIAL RE/COMMERCIAL RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCCOMRE")]
    pub nc_commercial_re_commercial_re_nccomre: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCCON`
    Title: TOTAL N/C-CONSUMER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCCON")]
    pub total_n_c_consumer_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCCONOTH`
    Title: TOTAL N/C-OTHER CONSUMER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCCONOTH")]
    pub total_n_c_other_consumer: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCCRCD`
    Title: TOTAL N/C CREDIT CARD PLANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCCRCD")]
    pub total_n_c_credit_card_plans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCDEP`
    Title: TOTAL N/C-DEP INST LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCDEP")]
    pub total_n_c_dep_inst_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCFG`
    Title: TOTAL N/C-FOREIGN GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCFG")]
    pub total_n_c_foreign_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCGTYPAR`
    Title: TOTAL N/C-PART GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCGTYPAR")]
    pub total_n_c_part_gty_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCLNLSR`
    Title: N/C LNS & LS/GROSS LNS & LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCLNLSR")]
    pub n_c_lns_ls_gross_lns_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCLS`
    Title: TOTAL N/C-LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCLS")]
    pub total_n_c_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCOTHLN`
    Title: TOTAL N/C-ALL OTHER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCOTHLN")]
    pub total_n_c_all_other_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRE`
    Title: TOTAL N/C REAL ESTATE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRE")]
    pub total_n_c_real_estate_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRECONR`
    Title: N/C CONST REAL ESTATE/CONST RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRECONR")]
    pub n_c_const_real_estate_const_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRECONS`
    Title: TOTAL N/C CONST REAL ESTATE CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRECONS")]
    pub total_n_c_const_real_estate_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRELOC`
    Title: TOTAL N/C-RE 1-4 FAMILY LINES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRELOC")]
    pub total_n_c_re_1_4_family_lines: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRELOCR`
    Title: N/C HOME EQUITY/HOME EQUITY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRELOCR")]
    pub n_c_home_equity_home_equity: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCREMULR`
    Title: N/C MULTIFAMLY RE/MULTIFAMLY RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCREMULR")]
    pub n_c_multifamly_re_multifamly_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCREMULT`
    Title: TOTAL N/C MULTIFAMLY RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCREMULT")]
    pub total_n_c_multifamly_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRENRER`
    Title: N/C NONFARM NONRES RE/NONRES RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRENRER")]
    pub n_c_nonfarm_nonres_re_nonres_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRENRES`
    Title: TOTAL N/C NONFARM NONRES RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRENRES")]
    pub total_n_c_nonfarm_nonres_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRER`
    Title: N/C REAL ESTATE LNS/REAL ESTATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRER")]
    pub n_c_real_estate_lns_real_estate: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRERESO`
    Title: N/C 1-4 OTHER RE/1-4 OTHER RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRERESO")]
    pub n_c_1_4_other_re_1_4_other_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCREREOR`
    Title: N/C 1-4 OTHER RE/1-4 OTHER RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCREREOR")]
    pub n_c_1_4_other_re_1_4_other_re_ncrereor: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRERES`
    Title: N/C 1-4 FAMILY RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRERES")]
    pub n_c_1_4_family_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRERESR`
    Title: N/C 1-4 FAMILY RE/1-4 FAMILY RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRERESR")]
    pub n_c_1_4_family_re_1_4_family_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETGNAST`
    Title: NET G/L ON SALES OF FIX ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETGNAST")]
    pub net_g_l_on_sales_of_fix_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETGNASTR`
    Title: NET G/L ON SALES OF FIX ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETGNASTR")]
    pub net_g_l_on_sales_of_fix_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTGLFXAQ`
    Title: NET G/L ON SALES OF FIX ASSETS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTGLFXAQ")]
    pub net_g_l_on_sales_of_fix_assets_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTGLFXAQR`
    Title: NET G/L ON SALES OF FIX ASSETS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTGLFXAQR")]
    pub net_g_l_on_sales_of_fix_assets_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETGNSLN`
    Title: NET G/L ON SALES OF LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETGNSLN")]
    pub net_g_l_on_sales_of_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETGNSLNR`
    Title: NET G/L ON SALES OF LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETGNSLNR")]
    pub net_g_l_on_sales_of_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTGLLNQ`
    Title: NET G/L ON SALES OF LOANS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTGLLNQ")]
    pub net_g_l_on_sales_of_loans_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTGLLNQR`
    Title: NET G/L ON SALES OF LOANS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTGLLNQR")]
    pub net_g_l_on_sales_of_loans_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETGNSRE`
    Title: NET G/L ON OTHER RE OWNED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETGNSRE")]
    pub net_g_l_on_other_re_owned: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETGNSRER`
    Title: NET G/L ON OTHER RE OWNED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETGNSRER")]
    pub net_g_l_on_other_re_owned_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTGLREQ`
    Title: NET G/L ON OTHER RE OWNED QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTGLREQ")]
    pub net_g_l_on_other_re_owned_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTGLREQR`
    Title: NET G/L ON OTHER RE OWNED QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTGLREQR")]
    pub net_g_l_on_other_re_owned_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINCA`
    Title: NET INCOME- BANK- ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NETINCA")]
    pub net_income_bank_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIMY`
    Title: NET INTEREST MARGIN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIMY")]
    pub net_interest_margin: Option<f64>,

    #[doc = r#"## FDIC Field:: `NIMYQ`
    Title: NET INTEREST MARGIN QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NIMYQ")]
    pub net_interest_margin_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJ`
    Title: NET OPERATING INCOME-ADJ
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJ")]
    pub net_operating_income_adj: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJR`
    Title: NET OPERATING INCOME-ADJ RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJR")]
    pub net_operating_income_adj_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJY`
    Title: NET OPERATING INCOME-ADJ/ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJY")]
    pub net_operating_income_adj_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJYQ`
    Title: NET OPERATING INCOME-ADJ/ASSETS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJYQ")]
    pub net_operating_income_adj_assets_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJA`
    Title: NET OPERATING INCOME-ADJ ANNUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJA")]
    pub net_operating_income_adj_annually: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJQ`
    Title: NET OPERATING INCOME-ADJ QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJQ")]
    pub net_operating_income_adj_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJQA`
    Title: NET OPERATING INCOME-ADJ QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJQA")]
    pub net_operating_income_adj_quarterly_noijqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIJQR`
    Title: NET OPERATING INCOME-ADJ QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIJQR")]
    pub net_operating_income_adj_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIIAY`
    Title: NONINTEREST INC/AVERAGE ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIIAY")]
    pub noninterest_inc_average_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIIAYQ`
    Title: NONINTEREST INC/AVERAGE ASSETS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIIAYQ")]
    pub noninterest_inc_average_assets_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIIA`
    Title: TOTAL NONINTEREST INCOME ANNUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIIA")]
    pub total_noninterest_income_annually: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIIQ`
    Title: TOTAL NONINTEREST INCOME-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIIQ")]
    pub total_noninterest_income_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIIQA`
    Title: TOTAL NONINTEREST INCOME-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIIQA")]
    pub total_noninterest_income_qtr_noniiqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIIQR`
    Title: TOTAL NONINTEREST INCOME-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIIQR")]
    pub total_noninterest_income_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIXAY`
    Title: NONINTEREST EXP/AVERAGE ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIXAY")]
    pub noninterest_exp_average_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIXAYQ`
    Title: NONINTEREST EXP/AVERAGE ASSETS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIXAYQ")]
    pub noninterest_exp_average_assets_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NONIXA`
    Title: TOTAL NONINTEREST EXPENSE ANNUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NONIXA")]
    pub total_noninterest_expense_annually: Option<f64>,

    #[doc = r#"## FDIC Field:: `NPERF`
    Title: NONPERF ASSETS/TOTAL ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NPERF")]
    pub nonperf_assets_total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `NPERFV`
    Title: NONPERF ASSETS/TOTAL ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NPERFV")]
    pub nonperf_assets_total_assets_nperfv: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAG`
    Title: AGRICULTURAL LN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAG")]
    pub agricultural_ln_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGR`
    Title: AGRICULTURAL LN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGR")]
    pub agricultural_ln_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGA`
    Title: AGRICULTURAL LN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGA")]
    pub agricultural_ln_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGQ`
    Title: AG LOAN NET CHARGE-OFFS-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGQ")]
    pub ag_loan_net_charge_offs_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGQR`
    Title: AG LOAN NET CHARGE-OFFS-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGQR")]
    pub ag_loan_net_charge_offs_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGSM`
    Title: AG LN NET CHARGE-OFFS*SMALL BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGSM")]
    pub ag_ln_net_charge_offs_small_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGSMR`
    Title: AG LN NET CHARGE-OFFS*SMALL BKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGSMR")]
    pub ag_ln_net_charge_offs_small_bks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGSMQ`
    Title: AG LN NET CHARGE-OFFS*SMALL BKS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGSMQ")]
    pub ag_ln_net_charge_offs_small_bks_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGSMQR`
    Title: AG LN NET CHARGE-OFFS*SMALL BKS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGSMQR")]
    pub ag_ln_net_charge_offs_small_bks_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAUTO`
    Title: AUTO LOANS - NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAUTO")]
    pub auto_loans_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAUTOR`
    Title: AUTO LOANS - NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAUTOR")]
    pub auto_loans_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAUTOA`
    Title: AUTO LNS - NET CHG-OFFS - ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAUTOA")]
    pub auto_lns_net_chg_offs_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAUTOQ`
    Title: AUTO LNS - NET CHG-OFFS - QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAUTOQ")]
    pub auto_lns_net_chg_offs_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAUTOLNQR`
    Title: AUTO LNS - NET CHG-OFFS - QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAUTOLNQR")]
    pub auto_lns_net_chg_offs_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAUTOQR`
    Title: AUTO LN-CHG-OFF- QTR/AUTO LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAUTOQR")]
    pub auto_ln_chg_off_qtr_auto_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCI`
    Title: COMMERCIAL LOAN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCI")]
    pub commercial_loan_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCIR`
    Title: COMMERCIAL LOAN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCIR")]
    pub commercial_loan_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCIA`
    Title: COMMERCIAL LOAN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCIA")]
    pub commercial_loan_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCINUS`
    Title: NON-U.S.COMMERCIAL LN NET CHG-OF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCINUS")]
    pub non_u_s_commercial_ln_net_chg_of: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCINUSR`
    Title: NON-U.S.COMMERCIAL LN NET CHG-OF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCINUSR")]
    pub non_u_s_commercial_ln_net_chg_of_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCINUSQ`
    Title: NON-U.S.COMMERCIAL LN NET CHG-OF QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCINUSQ")]
    pub non_u_s_commercial_ln_net_chg_of_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCINUSQR`
    Title: NON-U.S.COMMERCIAL LN NET CHG-OF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCINUSQR")]
    pub non_u_s_commercial_ln_net_chg_of_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCIQ`
    Title: COMMERCIAL LOAN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCIQ")]
    pub commercial_loan_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCIQR`
    Title: COMMERCIAL LOAN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCIQR")]
    pub commercial_loan_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCOMRER`
    Title: COMMERCIAL RE CHG-OFF/COMM RE LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCOMRER")]
    pub commercial_re_chg_off_comm_re_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCOMREQ`
    Title: COMMERCIAL RE CHG-OFF/COMM RE LN QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCOMREQ")]
    pub commercial_re_chg_off_comm_re_ln_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCOMREA`
    Title: COMMERCIAL RE LN CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCOMREA")]
    pub commercial_re_ln_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCON`
    Title: CONSUMER LOAN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCON")]
    pub consumer_loan_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONR`
    Title: CONSUMER LOAN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONR")]
    pub consumer_loan_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONA`
    Title: CONSUMER LOAN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONA")]
    pub consumer_loan_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONOTA`
    Title: OTHER CONSUMER LOAN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONOTA")]
    pub other_consumer_loan_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONOTH`
    Title: OTHER CONSUMER LN NET CHARGE-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONOTH")]
    pub other_consumer_ln_net_charge_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONOTHR`
    Title: OTHER CONSUMER LN NET CHARGE-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONOTHR")]
    pub other_consumer_ln_net_charge_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONOTQ`
    Title: OTHER CONSUMER LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONOTQ")]
    pub other_consumer_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONOTQR`
    Title: OTHER CONSUMER LN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONOTQR")]
    pub other_consumer_ln_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONQ`
    Title: CONSUMER LOAN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONQ")]
    pub consumer_loan_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONQR`
    Title: CONSUMER LOAN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONQR")]
    pub consumer_loan_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONTQR`
    Title: OTH.CONSUMER CHGOFF-QTR/OTH.CONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONTQR")]
    pub oth_consumer_chgoff_qtr_oth_cons: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCRCD`
    Title: CREDIT CARD LOAN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCRCD")]
    pub credit_card_loan_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCRCDR`
    Title: CREDIT CARD LOAN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCRCDR")]
    pub credit_card_loan_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCRCDA`
    Title: CREDIT CARD LOAN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCRCDA")]
    pub credit_card_loan_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCRCDQ`
    Title: CREDIT CARD LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCRCDQ")]
    pub credit_card_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCRCDQR`
    Title: CREDIT CARD LN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCRCDQR")]
    pub credit_card_ln_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEP`
    Title: DEPOSITORY INST LOAN NET CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEP")]
    pub depository_inst_loan_net_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEPR`
    Title: DEPOSITORY INST LOAN NET CHG-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEPR")]
    pub depository_inst_loan_net_chg_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEPNUS`
    Title: FOREIGN DEP INST LN NET CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEPNUS")]
    pub foreign_dep_inst_ln_net_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEPNUSR`
    Title: FOREIGN DEP INST LN NET CHG-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEPNUSR")]
    pub foreign_dep_inst_ln_net_chg_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEPNUQ`
    Title: FOREIGN DEP INST LN NET CHG-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEPNUQ")]
    pub foreign_dep_inst_ln_net_chg_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEPNUQR`
    Title: FOREIGN DEP INST LN NET CHG-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEPNUQR")]
    pub foreign_dep_inst_ln_net_chg_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEPQ`
    Title: DEPOSITORY INST LOAN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEPQ")]
    pub depository_inst_loan_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTDEPQR`
    Title: DEPOSITORY INST LOAN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTDEPQR")]
    pub depository_inst_loan_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTFORGV`
    Title: FOREIGN GOVT LN NET CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTFORGV")]
    pub foreign_govt_ln_net_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTFORGVR`
    Title: FOREIGN GOVT LN NET CHG-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTFORGVR")]
    pub foreign_govt_ln_net_chg_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTFORGVQ`
    Title: FOREIGN GOV LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTFORGVQ")]
    pub foreign_gov_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTFORGVQR`
    Title: FOREIGN GOV LN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTFORGVQR")]
    pub foreign_gov_ln_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTINCHPP`
    Title: NET INCOME-BK-HIGHER-PP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTINCHPP")]
    pub net_income_bk_higher_pp: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTINCL`
    Title: NET INCOME-BANK- LOSERS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTINCL")]
    pub net_income_bank_losers: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTINCLQ`
    Title: NET INCOME-BK-LOSER-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTINCLQ")]
    pub net_income_bk_loser_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLSA`
    Title: TOTAL LN&LS NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLSA")]
    pub total_ln_ls_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTINQHPP`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTINQHPP")]
    pub missing_title_ntinqhpp: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLSR`
    Title: NET CHARGE-OFFS/LOANS & LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLSR")]
    pub net_charge_offs_loans_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLNLSQR`
    Title: NET CHARGE-OFFS/LOANS & LEASES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLNLSQR")]
    pub net_charge_offs_loans_leases_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLS`
    Title: LEASE NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLS")]
    pub lease_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLSR`
    Title: LEASE NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLSR")]
    pub lease_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLSQ`
    Title: LEASE NET CHARGE-OFFS-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLSQ")]
    pub lease_net_charge_offs_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTLSQR`
    Title: LEASE NET CHARGE-OFFS-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTLSQR")]
    pub lease_net_charge_offs_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTOTHER`
    Title: ALL OTHER LOAN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTOTHER")]
    pub all_other_loan_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTOTHERR`
    Title: ALL OTHER LOAN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTOTHERR")]
    pub all_other_loan_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTOTHQ`
    Title: ALL OTHER LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTOTHQ")]
    pub all_other_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTOTHQR`
    Title: ALL OTHER LN NET-CHG-QTRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTOTHQR")]
    pub all_other_ln_net_chg_qtrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRCDSM`
    Title: AMT TIME DEP OF $100,000 OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRCDSM")]
    pub amt_time_dep_of_100_000_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRCDSMR`
    Title: AMT TIME DEP OF $100,000 OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRCDSMR")]
    pub amt_time_dep_of_100_000_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRCOMOT`
    Title: NONTRANSACTN-COM BKS & OTH U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRCOMOT")]
    pub nontransactn_com_bks_oth_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRCOMOTR`
    Title: NONTRANSACTN-COM BKS & OTH U.S RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRCOMOTR")]
    pub nontransactn_com_bks_oth_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRE`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRE")]
    pub real_estate_loan_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMUQA`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMUQA")]
    pub missing_title_ntremuqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECOQA`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECOQA")]
    pub missing_title_ntrecoqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELNR`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELNR")]
    pub real_estate_loan_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREQ`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREQ")]
    pub real_estate_loan_net_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREQA`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREQA")]
    pub real_estate_loan_net_charge_offs_quarterly_ntreqa: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERQ`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERQ")]
    pub real_estate_loan_net_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREAG`
    Title: FARMLAND RE LN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREAG")]
    pub farmland_re_ln_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREAGR`
    Title: FARMLAND RE LN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREAGR")]
    pub farmland_re_ln_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREAGQ`
    Title: FARMLAND RE LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREAGQ")]
    pub farmland_re_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREA`
    Title: RE LN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREA")]
    pub re_ln_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREAGQR`
    Title: FARMLAND RE LN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREAGQR")]
    pub farmland_re_ln_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECNFM`
    Title: 1-4 FAM CONST LN NET-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECNFM")]
    pub _1_4_fam_const_ln_net_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECNOT`
    Title: OTHER CONSTRUCT NET CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECNOT")]
    pub other_construct_net_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECONQ`
    Title: CONSTRUCTION RE LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECONQ")]
    pub construction_re_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECONQR`
    Title: CONSTRUCTION RE LN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECONQR")]
    pub construction_re_ln_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECONS`
    Title: CONSTRUCTION RE LN NET CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECONS")]
    pub construction_re_ln_net_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECOSA`
    Title: CONST RE LOANS NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECOSA")]
    pub const_re_loans_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECONSR`
    Title: CONSTRUCTION RE LN NET CHG-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECONSR")]
    pub construction_re_ln_net_chg_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECOSR`
    Title: CONST RE CHG-OFF/CONST RE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECOSR")]
    pub const_re_chg_off_const_re_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRECOQR`
    Title: CONST RE CHG-OFF/CONST RE LOANS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRECOQR")]
    pub const_re_chg_off_const_re_loans_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREFOR`
    Title: REAL ESTATE LN NET CHG-OFF-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREFOR")]
    pub real_estate_ln_net_chg_off_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREFORR`
    Title: REAL ESTATE LN NET CHG-OFF-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREFORR")]
    pub real_estate_ln_net_chg_off_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREFORQ`
    Title: REAL ESTATE LN NET CHG-OFF-FOR QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREFORQ")]
    pub real_estate_ln_net_chg_off_for_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREFORQR`
    Title: REAL ESTATE LN NET CHG-OFF-FOR QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREFORQR")]
    pub real_estate_ln_net_chg_off_for_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELOC`
    Title: LINE OF CREDIT RE LN NET CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELOC")]
    pub line_of_credit_re_ln_net_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELOCLNR`
    Title: LINE OF CREDIT RE LN NET CHG-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELOCLNR")]
    pub line_of_credit_re_ln_net_chg_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELOCQ`
    Title: LINE OF CREDIT RE LN NET CHG-OFF QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELOCQ")]
    pub line_of_credit_re_ln_net_chg_off_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELOCA`
    Title: LINE OF CREDIT RE LN NET CHG-OFF ANNUALLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELOCA")]
    pub line_of_credit_re_ln_net_chg_off_annually: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELOCQR`
    Title: LINE OF CREDIT RE LN NET CHG-OFF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELOCQR")]
    pub line_of_credit_re_ln_net_chg_off_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELOCRQ`
    Title: HOME EQUITY CHG-OFF/HOME EQ LNS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELOCRQ")]
    pub home_equity_chg_off_home_eq_lns_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRELOCR`
    Title: HOME EQUITY CHG-OFF/HOME EQ LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRELOCR")]
    pub home_equity_chg_off_home_eq_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMULQ`
    Title: MULTIFAMILY RE LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMULQ")]
    pub multifamily_re_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMULA`
    Title: MULTIFAMILY RES RE LN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMULA")]
    pub multifamily_res_re_ln_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMULQR`
    Title: MULTIFAMILY RE LN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMULQR")]
    pub multifamily_re_ln_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMULR`
    Title: MULTIFAM RE CHG-OFF/MULTI RE LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMULR")]
    pub multifam_re_chg_off_multi_re_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMUQR`
    Title: MULTIFAM RE CHG-OFF/MULTI RE LN QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMUQR")]
    pub multifam_re_chg_off_multi_re_ln_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMULT`
    Title: MULTIFAMLY RES RE LN NET CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMULT")]
    pub multifamly_res_re_ln_net_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREMULTR`
    Title: MULTIFAMLY RES RE LN NET CHG-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREMULTR")]
    pub multifamly_res_re_ln_net_chg_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENRES`
    Title: NONFARM NONRES RE LN NET CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENRES")]
    pub nonfarm_nonres_re_ln_net_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENRESR`
    Title: NONFARM NONRES RE LN NET CHG-OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENRESR")]
    pub nonfarm_nonres_re_ln_net_chg_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENROT`
    Title: OTHER NONFARM NONRS NET CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENROT")]
    pub other_nonfarm_nonrs_net_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENROW`
    Title: OWN OCC NONFRM NONRS NET CHG-OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENROW")]
    pub own_occ_nonfrm_nonrs_net_chg_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENRSA`
    Title: NONFARM NONRES RE LN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENRSA")]
    pub nonfarm_nonres_re_ln_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENRSQ`
    Title: NONFARM NONRES RE LN NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENRSQ")]
    pub nonfarm_nonres_re_ln_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENRSQR`
    Title: NONFARM NONRES RE LN NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENRSQR")]
    pub nonfarm_nonres_re_ln_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENRSR`
    Title: NONRES CHG-OFF/NONRES LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENRSR")]
    pub nonres_chg_off_nonres_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENRQR`
    Title: NONRES CHG-OFF/NONRES LOANS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENRQR")]
    pub nonres_chg_off_nonres_loans_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENUS`
    Title: NON-U.S. RE LN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENUS")]
    pub non_u_s_re_ln_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENUSR`
    Title: NON-U.S. RE LN NET CHARGE-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENUSR")]
    pub non_u_s_re_ln_net_charge_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENUSQ`
    Title: NON-U.S. RE LN NET CHARGE-OFFS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENUSQ")]
    pub non_u_s_re_ln_net_charge_offs_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOTHA`
    Title: OTHER 1-4 FAM RE OTHER LN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOTHA")]
    pub other_1_4_fam_re_other_ln_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRENUSQR`
    Title: NON-U.S. RE LN NET CHARGE-OFFS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRENUSQR")]
    pub non_u_s_re_ln_net_charge_offs_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOTHR`
    Title: OTHER 1-4 FAM RE CHG-OFF/OTH 1-4
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOTHR")]
    pub other_1_4_fam_re_chg_off_oth_1_4: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOTHRQR`
    Title: OTHER 1-4 FAM RE CHG-OFF/OTH 1-4 QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOTHRQR")]
    pub other_1_4_fam_re_chg_off_oth_1_4_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOTQA`
    Title: OTHER 1-4 FAM RE CHG-OFF/OTH 1-4 QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOTQA")]
    pub other_1_4_fam_re_chg_off_oth_1_4_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRER`
    Title: RE CHARGE-OFF/RE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRER")]
    pub re_charge_off_re_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREQR`
    Title: RE CHARGE-OFF/RE LOANS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREQR")]
    pub re_charge_off_re_loans_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERES`
    Title: RE LOANS 1-4 FAMILY NET CHG-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERES")]
    pub re_loans_1_4_family_net_chg_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERESLNR`
    Title: RE LOANS 1-4 FAMILY NET CHG-OFFS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERESLNR")]
    pub re_loans_1_4_family_net_chg_offs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERESQ`
    Title: RE LOANS 1-4 FAMILY NET-CHG-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERESQ")]
    pub re_loans_1_4_family_net_chg_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERESA`
    Title: RE LOANS 1-4 FAMILY NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERESA")]
    pub re_loans_1_4_family_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERESQR`
    Title: RE LOANS 1-4 FAMILY NET-CHG-QTR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERESQR")]
    pub re_loans_1_4_family_net_chg_qtr_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERESR`
    Title: 1-4 FAM RE CHG-OFF/1-4 FAM LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERESR")]
    pub _1_4_fam_re_chg_off_1_4_fam_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERESRQ`
    Title: 1-4 FAM RE CHG-OFF/1-4 FAM LOANS QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERESRQ")]
    pub _1_4_fam_re_chg_off_1_4_fam_loans_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERSF2`
    Title: RE LN 1-4 FAM JR LIEN-NET C/OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERSF2")]
    pub re_ln_1_4_fam_jr_lien_net_c_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERSF2R`
    Title: RE LN 1-4 FAM JR LIEN-NET C/OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERSF2R")]
    pub re_ln_1_4_fam_jr_lien_net_c_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERS2Q`
    Title: RE LN 1-4 FAM JR LIEN-NET C/OFF QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERS2Q")]
    pub re_ln_1_4_fam_jr_lien_net_c_off_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERS2QR`
    Title: RE LN 1-4 FAM JR LIEN-NET C/OFF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERS2QR")]
    pub re_ln_1_4_fam_jr_lien_net_c_off_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERSFM`
    Title: RE LN 1-4FAM IST LIEN-NET C/OFF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERSFM")]
    pub re_ln_1_4fam_ist_lien_net_c_off: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERSFMR`
    Title: RE LN 1-4FAM IST LIEN-NET C/OFF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERSFMR")]
    pub re_ln_1_4fam_ist_lien_net_c_off_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERSFQ`
    Title: RE LN 1-4FAM IST LIEN-NET C/OFF QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERSFQ")]
    pub re_ln_1_4fam_ist_lien_net_c_off_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRERSFQR`
    Title: RE LN 1-4FAM IST LIEN-NET C/OFF QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRERSFQR")]
    pub re_ln_1_4fam_ist_lien_net_c_off_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOFFDOM`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOFFDOM")]
    pub real_estate_loan_net_charge_offs_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOFFDOMR`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOFFDOMR")]
    pub real_estate_loan_net_charge_offs_domestic_offices_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOFFDOMQ`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOFFDOMQ")]
    pub real_estate_loan_net_charge_offs_domestic_offices_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREOFFDOMQR`
    Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREOFFDOMQR")]
    pub real_estate_loan_net_charge_offs_domestic_offices_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRFC`
    Title: NONTRANSACTION-FOR COUNTRY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRFC")]
    pub nontransaction_for_country: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRFCFG`
    Title: NONTRANSACTION-FOR CNTRY & GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRFCFG")]
    pub nontransaction_for_cntry_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRFCFGR`
    Title: NONTRANSACTION-FOR CNTRY & GOVT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRFCFGR")]
    pub nontransaction_for_cntry_govt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRFG`
    Title: NONTRANSACTION-FOR GOVERNMENT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRFG")]
    pub nontransaction_for_government: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRSMMDA`
    Title: SAVINGS DEP-MMDA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRSMMDA")]
    pub savings_dep_mmda: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRSMMDAR`
    Title: SAVINGS DEP-MMDA RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRSMMDAR")]
    pub savings_dep_mmda_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRSOTH`
    Title: SAVINGS DEP-OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRSOTH")]
    pub savings_dep_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRSOTHR`
    Title: SAVINGS DEP-OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRSOTHR")]
    pub savings_dep_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OAIENC`
    Title: INCOME EARNED NOT COLLECTED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OAIENC")]
    pub income_earned_not_collected: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFGEN`
    Title: LIFE INS ASSETS - GENERAL ACC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFGEN")]
    pub life_ins_assets_general_acc: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFGENR`
    Title: LIFE INS ASSETS - GENERAL ACC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFGENR")]
    pub life_ins_assets_general_acc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFHYB`
    Title: LIFE INS ASSETS - HYBRID ACC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFHYB")]
    pub life_ins_assets_hybrid_acc: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFHYBR`
    Title: LIFE INS ASSETS - HYBRID ACC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFHYBR")]
    pub life_ins_assets_hybrid_acc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFINS`
    Title: LIFE INSURANCE ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFINS")]
    pub life_insurance_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFINSR`
    Title: LIFE INSURANCE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFINSR")]
    pub life_insurance_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFSEP`
    Title: LIFE INS ASSETS - SEPARATE ACC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFSEP")]
    pub life_ins_assets_separate_acc: Option<f64>,

    #[doc = r#"## FDIC Field:: `OALIFSEPR`
    Title: LIFE INS ASSETS - SEPARATE ACC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OALIFSEPR")]
    pub life_ins_assets_separate_acc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OBSDIR`
    Title: OFF-BALANCE SHEET DERIVATIVES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OBSDIR")]
    pub off_balance_sheet_derivatives: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREAG`
    Title: ALL OTHER RE OWNED-FARMLAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREAG")]
    pub all_other_re_owned_farmland: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREAGR`
    Title: ALL OTHER RE OWNED-FARMLAND RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREAGR")]
    pub all_other_re_owned_farmland_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORECONS`
    Title: ALL OTHER RE OWNED-CONST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORECONS")]
    pub all_other_re_owned_const: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORECONSR`
    Title: ALL OTHER RE OWNED-CONST RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORECONSR")]
    pub all_other_re_owned_const_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREGNMA`
    Title: ALL OTHER RE OWNED-GNMA LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREGNMA")]
    pub all_other_re_owned_gnma_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREINV`
    Title: DIRECT & INDIRECT INVEST IN ORE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREINV")]
    pub direct_indirect_invest_in_ore: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREINVR`
    Title: DIRECT & INDIRECT INVEST IN ORE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREINVR")]
    pub direct_indirect_invest_in_ore_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREMULT`
    Title: ALL OTHER RE OWNED-MULTI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREMULT")]
    pub all_other_re_owned_multi: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREMULTR`
    Title: ALL OTHER RE OWNED-MULTI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREMULTR")]
    pub all_other_re_owned_multi_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORENRES`
    Title: ALL OTHER RE OWNED-NONFARM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORENRES")]
    pub all_other_re_owned_nonfarm: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORENRESR`
    Title: ALL OTHER RE OWNED-NONFARM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORENRESR")]
    pub all_other_re_owned_nonfarm_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREOTH`
    Title: OTHER REAL ESTATE OWNED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREOTH")]
    pub other_real_estate_owned_oreoth: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREOTHR`
    Title: OTHER REAL ESTATE OWNED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREOTHR")]
    pub other_real_estate_owned_ratio_oreothr: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREOTHF`
    Title: OTHER REAL ESTATE OWNED-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREOTHF")]
    pub other_real_estate_owned_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `OREOTHFR`
    Title: OTHER REAL ESTATE OWNED-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OREOTHFR")]
    pub other_real_estate_owned_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORERES`
    Title: ALL OTHER RE OWNED-1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORERES")]
    pub all_other_re_owned_1_4_family_oreres: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORERESR`
    Title: ALL OTHER RE OWNED 1-4 FAMILIY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ORERESR")]
    pub all_other_re_owned_1_4_familiy_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBORF`
    Title: OTHER BORROWED MONEY-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBORF")]
    pub other_borrowed_money_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHFFC`
    Title: OTHER-FUTURES & FORWARD CONTRACT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHFFC")]
    pub other_futures_forward_contract: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHFFCR`
    Title: OTHER-FUTURES & FORWARD CONTRACT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHFFCR")]
    pub other_futures_forward_contract_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHNVS`
    Title: OTHER-NOTIONAL VALUE SWAPS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHNVS")]
    pub other_notional_value_swaps: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHOFFBS`
    Title: ALL OTH OFF-BALANCE SHEET LIAB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHOFFBS")]
    pub all_oth_off_balance_sheet_liab: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHOFFBSR`
    Title: ALL OTH OFF-BALANCE SHEET LIAB RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHOFFBSR")]
    pub all_oth_off_balance_sheet_liab_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHPOC`
    Title: OTHER-PURCHASED OPTION CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHPOC")]
    pub other_purchased_option_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHWOC`
    Title: OTHER-WRITTEN OPTION CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHWOC")]
    pub other_written_option_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTSREGNM`
    Title: OTS REGION NAME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTSREGNM")]
    pub ots_region_name: Option<String>,

    #[doc = r#"## FDIC Field:: `OWNCRCI`
    Title: REC OWN INTEREST SEC - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNCRCI")]
    pub rec_own_interest_sec_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNCRCRD`
    Title: REC OWN INTEREST SEC - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNCRCRD")]
    pub rec_own_interest_sec_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNCRHEL`
    Title: REC OWN INTEREST SEC - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNCRHEL")]
    pub rec_own_interest_sec_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNDRCI`
    Title: C/O OWN INTEREST SEC - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNDRCI")]
    pub c_o_own_interest_sec_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNDRCRD`
    Title: C/O OWN INTEREST SEC - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNDRCRD")]
    pub c_o_own_interest_sec_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNDRHEL`
    Title: C/O OWN INTEREST SEC - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNDRHEL")]
    pub c_o_own_interest_sec_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNLNCI`
    Title: LN SECURE HELD IN SEC - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNLNCI")]
    pub ln_secure_held_in_sec_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNLNCRD`
    Title: LN SECURE HELD IN SEC - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNLNCRD")]
    pub ln_secure_held_in_sec_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNLNHEL`
    Title: LN SECURE HELD IN SEC - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNLNHEL")]
    pub ln_secure_held_in_sec_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNP3CI`
    Title: PD 30-89 OWN INTEREST SEC - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNP3CI")]
    pub pd_30_89_own_interest_sec_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNP3CRD`
    Title: PD 30-89 OWN INTEREST SEC - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNP3CRD")]
    pub pd_30_89_own_interest_sec_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNP3HEL`
    Title: PD30-89 OWN INTEREST SEC - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNP3HEL")]
    pub pd30_89_own_interest_sec_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNP9CI`
    Title: PD 90 + OWN INTEREST SEC - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNP9CI")]
    pub pd_90_own_interest_sec_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNP9CRD`
    Title: PD 90 + OWN INTEREST SEC - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNP9CRD")]
    pub pd_90_own_interest_sec_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNP9HEL`
    Title: PD 90 + OWN INTEREST SEC - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNP9HEL")]
    pub pd_90_own_interest_sec_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNSCCI`
    Title: SEC. SECURE HELD IN RC-B - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNSCCI")]
    pub sec_secure_held_in_rc_b_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNSCCRD`
    Title: SEC. SECURE HELD IN RC-B - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNSCCRD")]
    pub sec_secure_held_in_rc_b_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `OWNSCHEL`
    Title: SEC. SECURE HELD IN RC-B - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OWNSCHEL")]
    pub sec_secure_held_in_rc_b_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3AG`
    Title: 30-89 DAYS P/D-AGRICULTURAL LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3AG")]
    pub _30_89_days_p_d_agricultural_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3AGR`
    Title: 30-89 DAYS P/D-AGRICULTURAL LNS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3AGR")]
    pub _30_89_days_p_d_agricultural_lns_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3AGSM`
    Title: 30-89 DAYS P/D-AG LNS*SMALL BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3AGSM")]
    pub _30_89_days_p_d_ag_lns_small_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3AGSMR`
    Title: 30-89 DAYS P/D-AG LNS*SMALL BKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3AGSMR")]
    pub _30_89_days_p_d_ag_lns_small_bks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3ASSET`
    Title: 30-89 DAYS P/D-TOTAL ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3ASSET")]
    pub _30_89_days_p_d_total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3ASSETR`
    Title: 30-89 DAYS P/D TOTAL ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3ASSETR")]
    pub _30_89_days_p_d_total_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3AUTO`
    Title: 30-89 DAYS P/D AUTO LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3AUTO")]
    pub _30_89_days_p_d_auto_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3AUTOR`
    Title: 30-89 DAYS P/D AUTO LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3AUTOR")]
    pub _30_89_days_p_d_auto_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CI`
    Title: 30-89 DAYS P/D-C&I LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CI")]
    pub _30_89_days_p_d_c_i_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CIR`
    Title: 30-89 DAYS P/D-C&I LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CIR")]
    pub _30_89_days_p_d_c_i_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CINUS`
    Title: 30-89 DAYS P/D-C&I*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CINUS")]
    pub _30_89_days_p_d_c_i_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CINUSR`
    Title: 30-89 DAYS P/D-C&I*NON-U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CINUSR")]
    pub _30_89_days_p_d_c_i_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CON`
    Title: 30-89 DAYS P/D-CONSUMER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CON")]
    pub _30_89_days_p_d_consumer_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CONR`
    Title: 30-89 DAYS P/D-CONSUMER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CONR")]
    pub _30_89_days_p_d_consumer_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CONOTH`
    Title: 30-89 DAYS P/D-OTHER CONSUMER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CONOTH")]
    pub _30_89_days_p_d_other_consumer: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CONOTHR`
    Title: 30-89 DAYS P/D-OTHER CONSUMER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CONOTHR")]
    pub _30_89_days_p_d_other_consumer_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CRCD`
    Title: 30-89 DAYS P/D-CREDIT CARD PLANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CRCD")]
    pub _30_89_days_p_d_credit_card_plans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3CRCDR`
    Title: 30-89 DAYS P/D-CREDIT CARD PLANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3CRCDR")]
    pub _30_89_days_p_d_credit_card_plans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3DEP`
    Title: 30-89 DAYS P/D-DEP INST LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3DEP")]
    pub _30_89_days_p_d_dep_inst_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3DEPR`
    Title: 30-89 DAYS P/D-DEP INST LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3DEPR")]
    pub _30_89_days_p_d_dep_inst_loans_p3depr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3DEPNUS`
    Title: 30-89 DAYS P/D-DEP INST*NON U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3DEPNUS")]
    pub _30_89_days_p_d_dep_inst_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3DEPNUSR`
    Title: 30-89 DAYS P/D-DEP INST*NON U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3DEPNUSR")]
    pub _30_89_days_p_d_dep_inst_non_u_s_p3depnusr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3FG`
    Title: 30-89 DAYS P/D-FOREIGN GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3FG")]
    pub _30_89_days_p_d_foreign_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3FGR`
    Title: 30-89 DAYS P/D-FOREIGN GOVT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3FGR")]
    pub _30_89_days_p_d_foreign_govt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3GTY`
    Title: 30-89 DAYS P/D-GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3GTY")]
    pub _30_89_days_p_d_gty_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3GTYR`
    Title: 30-89 DAYS P/D-GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3GTYR")]
    pub _30_89_days_p_d_gty_ln_ls_p3gtyr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3GTYGNM`
    Title: 30-89 DAY P/D-REBOOKED GNMA LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3GTYGNM")]
    pub _30_89_day_p_d_rebooked_gnma_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3GTYGNMR`
    Title: 30-89 DAY P/D-REBOOKED GNMA LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3GTYGNMR")]
    pub _30_89_day_p_d_rebooked_gnma_lns_p3gtygnmr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3GTYPAR`
    Title: 30-89 DAYS P/D-PART GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3GTYPAR")]
    pub _30_89_days_p_d_part_gty_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3GTYPARR`
    Title: 30-89 DAYS P/D-PART GTY LN&LS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3GTYPARR")]
    pub _30_89_days_p_d_part_gty_ln_ls_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LAG`
    Title: 30-89 DAY P/D AG LOANS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LAG")]
    pub _30_89_day_p_d_ag_loans_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LAGR`
    Title: 30-89 DAY P/D AG LOANS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LAGR")]
    pub _30_89_day_p_d_ag_loans_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LCI`
    Title: 30-89 DAYS P/D C&I LNS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LCI")]
    pub _30_89_days_p_d_c_i_lns_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LCIR`
    Title: 30-89 DAYS P/D C&I LNS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LCIR")]
    pub _30_89_days_p_d_c_i_lns_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LCON`
    Title: 30-89 D P/D CONSUMER -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LCON")]
    pub _30_89_d_p_d_consumer_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LCONR`
    Title: 30-89 D P/D CONSUMER -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LCONR")]
    pub _30_89_d_p_d_consumer_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LGTY`
    Title: 30-89 P/D PROTECT (GTY)-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LGTY")]
    pub _30_89_p_d_protect_gty_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LGTYR`
    Title: 30-89 P/D PROTECT (GTY)-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LGTYR")]
    pub _30_89_p_d_protect_gty_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LNSALE`
    Title: 30-89 DAYS P/D-L&L HELD FOR SALE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LNSALE")]
    pub _30_89_days_p_d_l_l_held_for_sale: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LNSALER`
    Title: 30-89 DAYS P/D-L&L HELD FOR SALE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LNSALER")]
    pub _30_89_days_p_d_l_l_held_for_sale_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LOTH`
    Title: 30-89 DAYS P/D OTH LNS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LOTH")]
    pub _30_89_days_p_d_oth_lns_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LOTHR`
    Title: 30-89 DAYS P/D OTH LNS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LOTHR")]
    pub _30_89_days_p_d_oth_lns_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LREAG`
    Title: 30-89 DAY P/D RE FARM-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LREAG")]
    pub _30_89_day_p_d_re_farm_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LREAGR`
    Title: 30-89 DAY P/D RE FARM-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LREAGR")]
    pub _30_89_day_p_d_re_farm_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LRECON`
    Title: 30-89 P/D CONSTRUCTION -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LRECON")]
    pub _30_89_p_d_construction_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LRECONR`
    Title: 30-89 P/D CONSTRUCTION -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LRECONR")]
    pub _30_89_p_d_construction_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LREMUL`
    Title: 30-89 DAY P/D MULTIFAM -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LREMUL")]
    pub _30_89_day_p_d_multifam_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LREMULR`
    Title: 30-89 DAY P/D MULTIFAM -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LREMULR")]
    pub _30_89_day_p_d_multifam_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LRENRS`
    Title: 30-89 P/D NONFRM NONRS -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LRENRS")]
    pub _30_89_p_d_nonfrm_nonrs_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LRENRSR`
    Title: 30-89 P/D NONFRM NONRS -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LRENRSR")]
    pub _30_89_p_d_nonfrm_nonrs_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LRERES`
    Title: 30-89 D P/D 1-4 FAMILY -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LRERES")]
    pub _30_89_d_p_d_1_4_family_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LRERESR`
    Title: 30-89 P/D 1-4 FAMILY -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LRERESR")]
    pub _30_89_p_d_1_4_family_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LS`
    Title: 30-89 DAYS P/D-LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LS")]
    pub _30_89_days_p_d_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LSR`
    Title: 30-89 DAYS P/D-LEASES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LSR")]
    pub _30_89_days_p_d_leases_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LTOT`
    Title: 30-89 D P/D TOTAL LOANS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LTOT")]
    pub _30_89_d_p_d_total_loans_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3LTOTR`
    Title: 30-89 DAYS P/D-TOTAL LOANS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3LTOTR")]
    pub _30_89_days_p_d_total_loans_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3OTHLN`
    Title: 30-89 DAYS P/D-ALL OTHER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3OTHLN")]
    pub _30_89_days_p_d_all_other_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3OTHLNR`
    Title: 30-89 DAYS P/D-ALL OTHER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3OTHLNR")]
    pub _30_89_days_p_d_all_other_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RE`
    Title: 30-89 DAYS P/D-REAL ESTATE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RE")]
    pub _30_89_days_p_d_real_estate_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RER`
    Title: 30-89 DAYS P/D-REAL ESTATE LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RER")]
    pub _30_89_days_p_d_real_estate_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3REAG`
    Title: 30-89 DAYS P/D-RE*FARMLAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3REAG")]
    pub _30_89_days_p_d_re_farmland: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3REAGR`
    Title: 30-89 DAYS P/D-RE*FARMLAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3REAGR")]
    pub _30_89_days_p_d_re_farmland_p3reagr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RECNFM`
    Title: 30-89 DAYS P/D 1-4 FAM CONSTR LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RECNFM")]
    pub _30_89_days_p_d_1_4_fam_constr_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RECNFMR`
    Title: 30-89 DAYS P/D 1-4 FAM CONSTR LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RECNFMR")]
    pub _30_89_days_p_d_1_4_fam_constr_ln_p3recnfmr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RECNOT`
    Title: 30-89 DAYS P/D OTH CONSTR & LAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RECNOT")]
    pub _30_89_days_p_d_oth_constr_land: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RECNOTR`
    Title: 30-89 DAYS P/D OTH CONSTR & LAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RECNOTR")]
    pub _30_89_days_p_d_oth_constr_land_p3recnotr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RECONS`
    Title: 30-89 DAYS P/D-RE*CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RECONS")]
    pub _30_89_days_p_d_re_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RECONSR`
    Title: 30-89 DAYS P/D-RE*CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RECONSR")]
    pub _30_89_days_p_d_re_construction_p3reconsr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3REFOR`
    Title: 30-89 DAYS P/D-RE*FOREIGN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3REFOR")]
    pub _30_89_days_p_d_re_foreign: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3REFORR`
    Title: 30-89 DAYS P/D-RE*FOREIGN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3REFORR")]
    pub _30_89_days_p_d_re_foreign_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RELOC`
    Title: 30-89 DAYS P/D-RE*1-4 FAM LINES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RELOC")]
    pub _30_89_days_p_d_re_1_4_fam_lines: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RELOCR`
    Title: 30-89 DAYS P/D-RE*1-4 FAM LINES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RELOCR")]
    pub _30_89_days_p_d_re_1_4_fam_lines_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3REMULT`
    Title: 30-89 DAYS P/D-RE*MULTIFAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3REMULT")]
    pub _30_89_days_p_d_re_multifamily: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3REMULTR`
    Title: 30-89 DAYS P/D-RE*MULTIFAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3REMULTR")]
    pub _30_89_days_p_d_re_multifamily_p3remultr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENRES`
    Title: 30-89 DAYS P/D-RE*NONFARM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENRES")]
    pub _30_89_days_p_d_re_nonfarm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENRESR`
    Title: 30-89 DAYS P/D-RE*NONFARM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENRESR")]
    pub _30_89_days_p_d_re_nonfarm_nonres_p3renresr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENROT`
    Title: 30-89 DAYS P/D OTH NONFRM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENROT")]
    pub _30_89_days_p_d_oth_nonfrm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENROTR`
    Title: 30-89 DAYS P/D OTH NONFRM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENROTR")]
    pub _30_89_days_p_d_oth_nonfrm_nonres_p3renrotr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENROW`
    Title: 30-89 DAYS P/D 0WN-OCC NONF NONRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENROW")]
    pub _30_89_days_p_d_0wn_occ_nonf_nonrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENROWR`
    Title: 30-89 DAYS P/D OWN-OCC NONF NONRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENROWR")]
    pub _30_89_days_p_d_own_occ_nonf_nonrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENUS`
    Title: 30-89 DAYS P/D-RE*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENUS")]
    pub _30_89_days_p_d_re_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RENUSR`
    Title: 30-89 DAYS P/D-RE*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RENUSR")]
    pub _30_89_days_p_d_re_non_u_s_p3renusr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RERES`
    Title: 30-89 DAYS P/D-RE*1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RERES")]
    pub _30_89_days_p_d_re_1_4_family: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RERESR`
    Title: 30-89 DAYS P/D-RE*1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RERESR")]
    pub _30_89_days_p_d_re_1_4_family_p3reresr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RERSF2`
    Title: 30-89 DAYS P/D-RE*1-4 JN LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RERSF2")]
    pub _30_89_days_p_d_re_1_4_jn_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RERSF2R`
    Title: 30-89 DAYS P/D-RE*1-4 JN LIEN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RERSF2R")]
    pub _30_89_days_p_d_re_1_4_jn_lien_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RERSFM`
    Title: 30-89 DAYS P/D-RE*1-4 IST LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RERSFM")]
    pub _30_89_days_p_d_re_1_4_ist_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RERSFMR`
    Title: 30-89 DAYS P/D-RE*1-4 IST LIEN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RERSFMR")]
    pub _30_89_days_p_d_re_1_4_ist_lien_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSCI`
    Title: 30-89 DAY P/D RESTRUCT C&I LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSCI")]
    pub _30_89_day_p_d_restruct_c_i_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSCONS`
    Title: 30-89 P/D RESTRUCT CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSCONS")]
    pub _30_89_p_d_restruct_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSLNFM`
    Title: 30-89 DAY P/D RESTR LN- 1-4 FAM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSLNFM")]
    pub _30_89_day_p_d_restr_ln_1_4_fam: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSLNFMR`
    Title: 30-89 DAY P/D RESTR LN- 1-4 FAM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSLNFMR")]
    pub _30_89_day_p_d_restr_ln_1_4_fam_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSLNLS`
    Title: 30-89 D P/D RESTR LN EXCL1-4 FM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSLNLS")]
    pub _30_89_d_p_d_restr_ln_excl1_4_fm: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSLNLSR`
    Title: 30-89 D P/D RESTR LN EXCL1-4 FM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSLNLSR")]
    pub _30_89_d_p_d_restr_ln_excl1_4_fm_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSLNLT`
    Title: 30-89 DAY P/D RESTR LN- TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSLNLT")]
    pub _30_89_day_p_d_restr_ln_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSLNLTR`
    Title: 30-89 DAY P/D RESTR LN- TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSLNLTR")]
    pub _30_89_day_p_d_restr_ln_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSMULT`
    Title: 30-89 D P/D RESTRUCT MULTIFAM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSMULT")]
    pub _30_89_d_p_d_restruct_multifam: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSNRES`
    Title: 30-89 DAY P/D RESTRUCT NFNR LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSNRES")]
    pub _30_89_day_p_d_restruct_nfnr_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RSOTH`
    Title: 30-89 D P/D RESTRUCT ALL OTH LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RSOTH")]
    pub _30_89_d_p_d_restruct_all_oth_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3SCDEBT`
    Title: 30-89 DAYS P/D-DEBT SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3SCDEBT")]
    pub _30_89_days_p_d_debt_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3SCDEBTR`
    Title: 30-89 DAYS P/D-DEBT SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3SCDEBTR")]
    pub _30_89_days_p_d_debt_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9AG`
    Title: 90+ DAYS P/D-AGRICULTURAL LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9AG")]
    pub _90_days_p_d_agricultural_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9AGR`
    Title: 90+ DAYS P/D-AGRICULTURAL LNS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9AGR")]
    pub _90_days_p_d_agricultural_lns_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9AGSM`
    Title: 90+ DAYS P/D-AG LNS*SMALL BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9AGSM")]
    pub _90_days_p_d_ag_lns_small_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9AGSMR`
    Title: 90+ DAYS P/D-AG LNS*SMALL BKS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9AGSMR")]
    pub _90_days_p_d_ag_lns_small_bks_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9ASSET`
    Title: 90+ DAYS P/D-TOTAL ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9ASSET")]
    pub _90_days_p_d_total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9ASSETR`
    Title: 90+ DAYS P/D-TOTAL ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9ASSETR")]
    pub _90_days_p_d_total_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9AUTO`
    Title: 90+ DAYS P/D AUTO LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9AUTO")]
    pub _90_days_p_d_auto_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9AUTOR`
    Title: 90+ DAYS P/D AUTO LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9AUTOR")]
    pub _90_days_p_d_auto_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CI`
    Title: 90+ DAYS P/D-C&I LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CI")]
    pub _90_days_p_d_c_i_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CIR`
    Title: 90+ DAYS P/D-C&I LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CIR")]
    pub _90_days_p_d_c_i_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CINUS`
    Title: 90+ DAYS P/D-C&I*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CINUS")]
    pub _90_days_p_d_c_i_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CINUSR`
    Title: 90+ DAYS P/D-C&I*NON-U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CINUSR")]
    pub _90_days_p_d_c_i_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CON`
    Title: 90+ DAYS P/D-CONSUMER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CON")]
    pub _90_days_p_d_consumer_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CONR`
    Title: 90+ DAYS P/D-CONSUMER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CONR")]
    pub _90_days_p_d_consumer_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CONOTH`
    Title: 90+ DAYS P/D-OTHER CONSUMER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CONOTH")]
    pub _90_days_p_d_other_consumer: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CONOTHR`
    Title: 90+ DAYS P/D-OTHER CONSUMER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CONOTHR")]
    pub _90_days_p_d_other_consumer_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CRCD`
    Title: 90+ DAYS P/D-CREDIT CARD PLANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CRCD")]
    pub _90_days_p_d_credit_card_plans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9CRCDR`
    Title: 90+ DAYS P/D-CREDIT CARD PLANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9CRCDR")]
    pub _90_days_p_d_credit_card_plans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9DEP`
    Title: 90+ DAYS P/D-DEP INST LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9DEP")]
    pub _90_days_p_d_dep_inst_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9DEPR`
    Title: 90+ DAYS P/D-DEP INST LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9DEPR")]
    pub _90_days_p_d_dep_inst_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9DEPNUS`
    Title: 90+ DAYS P/D-DEP INST*NON U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9DEPNUS")]
    pub _90_days_p_d_dep_inst_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9DEPNUSR`
    Title: 90+ DAYS P/D-DEP INST*NON U.S. RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9DEPNUSR")]
    pub _90_days_p_d_dep_inst_non_u_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9FG`
    Title: 90+ DAYS P/D-FOREIGN GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9FG")]
    pub _90_days_p_d_foreign_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9FGR`
    Title: 90+ DAYS P/D-FOREIGN GOVT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9FGR")]
    pub _90_days_p_d_foreign_govt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9GTY`
    Title: 90+ DAYS P/D-GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9GTY")]
    pub _90_days_p_d_gty_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9GTYR`
    Title: 90+ DAYS P/D-GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9GTYR")]
    pub _90_days_p_d_gty_ln_ls_p9gtyr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9GTYGNM`
    Title: 90+ DAYS P/D-REBOOKED GNMA LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9GTYGNM")]
    pub _90_days_p_d_rebooked_gnma_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9GTYGNMR`
    Title: 90+ DAY P/D-REBOOKED GNMA LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9GTYGNMR")]
    pub _90_day_p_d_rebooked_gnma_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9GTYPAR`
    Title: 90+ DAYS P/D-PART GTY LN&LS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9GTYPAR")]
    pub _90_days_p_d_part_gty_ln_ls: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9GTYPARR`
    Title: 90+ DAYS P/D-PART GTY LN&LS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9GTYPARR")]
    pub _90_days_p_d_part_gty_ln_ls_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LAG`
    Title: 90+ DAYS P/D AG LOANS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LAG")]
    pub _90_days_p_d_ag_loans_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LAGR`
    Title: 90+ DAYS P/D AG LOANS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LAGR")]
    pub _90_days_p_d_ag_loans_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LCI`
    Title: 90+DAYS P/D C&I LNS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LCI")]
    pub _90_days_p_d_c_i_lns_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LCIR`
    Title: 90+ DAYS P/D C&I LNS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LCIR")]
    pub _90_days_p_d_c_i_lns_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LCON`
    Title: 90+ D P/D CONSUMER LN - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LCON")]
    pub _90_d_p_d_consumer_ln_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LCONR`
    Title: 90+ D P/D CONSUMER LN - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LCONR")]
    pub _90_d_p_d_consumer_ln_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LGTY`
    Title: 90+ D P/D PROTECT (GTY)-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LGTY")]
    pub _90_d_p_d_protect_gty_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LGTYR`
    Title: 90+ D P/D PROTECT (GTY)-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LGTYR")]
    pub _90_d_p_d_protect_gty_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LNSALE`
    Title: 90 DAYS P/D-L&L HELD FOR SALE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LNSALE")]
    pub _90_days_p_d_l_l_held_for_sale: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LNSALER`
    Title: 90+ DAYS P/D-L&L HELD FOR SALE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LNSALER")]
    pub _90_days_p_d_l_l_held_for_sale_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LOTH`
    Title: 90+ DAYS P/D OTHER LNS-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LOTH")]
    pub _90_days_p_d_other_lns_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LOTHR`
    Title: 90+ DAYS P/D OTHER LNS-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LOTHR")]
    pub _90_days_p_d_other_lns_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LREAG`
    Title: 90+ DAY P/D RE FARM-LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LREAG")]
    pub _90_day_p_d_re_farm_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LREAGR`
    Title: 90+ DAY P/D RE FARM-LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LREAGR")]
    pub _90_day_p_d_re_farm_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LRECON`
    Title: 90+ D P/D CONSTRUCTION -LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LRECON")]
    pub _90_d_p_d_construction_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LRECONR`
    Title: 90+ D P/D CONSTRUCTION -LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LRECONR")]
    pub _90_d_p_d_construction_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LREMUL`
    Title: 90+ DAY P/D MULTIFAM - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LREMUL")]
    pub _90_day_p_d_multifam_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LREMULR`
    Title: 90+ DAY P/D MULTIFAM - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LREMULR")]
    pub _90_day_p_d_multifam_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LRENRS`
    Title: 90+ D P/D NFNR - LOSS SHARE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LRENRS")]
    pub _90_d_p_d_nfnr_loss_share: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LRENRSR`
    Title: 90+ D P/D NFNR - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LRENRSR")]
    pub _90_d_p_d_nfnr_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LRERES`
    Title: 90+ D P/D 1-4 FAMILY - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LRERES")]
    pub _90_d_p_d_1_4_family_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LRERESR`
    Title: 90+ D P/D 1-4 FAMILY - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LRERESR")]
    pub _90_d_p_d_1_4_family_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LS`
    Title: 90+ DAYS P/D-LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LS")]
    pub _90_days_p_d_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LSR`
    Title: 90+ DAYS P/D-LEASES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LSR")]
    pub _90_days_p_d_leases_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LTOT`
    Title: 90+ D P/D TOTAL LOANS - LOSS SH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LTOT")]
    pub _90_d_p_d_total_loans_loss_sh: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9LTOTR`
    Title: 90+ D P/D TOTAL LOANS - LOSS SH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9LTOTR")]
    pub _90_d_p_d_total_loans_loss_sh_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9OTHLN`
    Title: 90+ DAYS P/D-ALL OTHER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9OTHLN")]
    pub _90_days_p_d_all_other_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9OTHLNR`
    Title: 90+ DAYS P/D-ALL OTHER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9OTHLNR")]
    pub _90_days_p_d_all_other_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RE`
    Title: 90+ DAYS P/D-REAL ESTATE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RE")]
    pub _90_days_p_d_real_estate_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RER`
    Title: 90+ DAYS P/D-REAL ESTATE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RER")]
    pub _90_days_p_d_real_estate_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9REAG`
    Title: 90+ DAYS P/D-RE*FARMLAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9REAG")]
    pub _90_days_p_d_re_farmland: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9REAGR`
    Title: 90+ DAYS P/D-RE*FARMLAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9REAGR")]
    pub _90_days_p_d_re_farmland_p9reagr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RECNFM`
    Title: 90+ DAYS P/D 1-4 FAM CONSTRUC LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RECNFM")]
    pub _90_days_p_d_1_4_fam_construc_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RECNFMR`
    Title: 90+ DAYS P/D 1-4 FAM CONSTRUC LN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RECNFMR")]
    pub _90_days_p_d_1_4_fam_construc_ln_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RECNOT`
    Title: 90+ DAYS P/D OTHER CONSTR & LAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RECNOT")]
    pub _90_days_p_d_other_constr_land: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RECNOTR`
    Title: 90+ DAYS P/D OTHER CONSTR & LAND RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RECNOTR")]
    pub _90_days_p_d_other_constr_land_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RECONS`
    Title: 90+ DAYS P/D-RE*CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RECONS")]
    pub _90_days_p_d_re_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RECONSR`
    Title: 90+ DAYS P/D-RE*CONSTRUCTION RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RECONSR")]
    pub _90_days_p_d_re_construction_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9REFOR`
    Title: 90 + DAYS P/D-RE*FOREIGN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9REFOR")]
    pub _90_days_p_d_re_foreign: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9REFORR`
    Title: 90+ DAYS P/D-RE*FOREIGN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9REFORR")]
    pub _90_days_p_d_re_foreign_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RELOC`
    Title: 90+ DAYS P/D-RE*1-4 FAM LINES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RELOC")]
    pub _90_days_p_d_re_1_4_fam_lines: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RELOCR`
    Title: 90+ DAYS P/D-RE*1-4 FAM LINES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RELOCR")]
    pub _90_days_p_d_re_1_4_fam_lines_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9REMULT`
    Title: 90+ DAYS P/D-RE*MULTIFAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9REMULT")]
    pub _90_days_p_d_re_multifamily: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9REMULTR`
    Title: 90+ DAYS P/D-RE*MULTIFAMILY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9REMULTR")]
    pub _90_days_p_d_re_multifamily_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENRES`
    Title: 90+ DAYS P/D-RE*NONFARM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENRES")]
    pub _90_days_p_d_re_nonfarm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENRESR`
    Title: 90+ DAYS P/D-RE*NONFARM NONRES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENRESR")]
    pub _90_days_p_d_re_nonfarm_nonres_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENROT`
    Title: 90+ DAYS P/D OTHER NONFRM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENROT")]
    pub _90_days_p_d_other_nonfrm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENROTR`
    Title: 90+ DAYS P/D OTHER NONFRM NONRES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENROTR")]
    pub _90_days_p_d_other_nonfrm_nonres_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENROW`
    Title: 90+ DAYS P/D 0WN-OCC NONFR NONRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENROW")]
    pub _90_days_p_d_0wn_occ_nonfr_nonrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENROWR`
    Title: 90+ DAYS P/D OWN-OCC NONFR NONRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENROWR")]
    pub _90_days_p_d_own_occ_nonfr_nonrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENUS`
    Title: 90+ DAYS P/D-RE*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENUS")]
    pub _90_days_p_d_re_non_u_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RENUSR`
    Title: 90+ DAYS P/D-RE*NON-U.S.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RENUSR")]
    pub _90_days_p_d_re_non_u_s_p9renusr: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RERES`
    Title: 90+ DAYS P/D-RE*1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RERES")]
    pub _90_days_p_d_re_1_4_family: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RERESR`
    Title: 90+ DAYS P/D-RE*1-4 FAMILY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RERESR")]
    pub _90_days_p_d_re_1_4_family_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RERSF2`
    Title: 90+ DAYS P/D-RE*1-4 JN LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RERSF2")]
    pub _90_days_p_d_re_1_4_jn_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RERSF2R`
    Title: 90+ DAYS P/D-RE*1-4 JN LIEN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RERSF2R")]
    pub _90_days_p_d_re_1_4_jn_lien_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RERSFM`
    Title: 90+ DAYS P/D-RE*1-4 IST LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RERSFM")]
    pub _90_days_p_d_re_1_4_ist_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RERSFMR`
    Title: 90+ DAYS P/D-RE*1-4 IST LIEN RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RERSFMR")]
    pub _90_days_p_d_re_1_4_ist_lien_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSCI`
    Title: 90+ DAY P/D RESTRUCT C&I LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSCI")]
    pub _90_day_p_d_restruct_c_i_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSCONS`
    Title: 90+ D P/D RESTRUCT CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSCONS")]
    pub _90_d_p_d_restruct_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSLNFM`
    Title: 90+ DAYS P/D RESTR LN- 1-4 FAM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSLNFM")]
    pub _90_days_p_d_restr_ln_1_4_fam: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSLNFMR`
    Title: 90+ DAYS P/D RESTR LN- 1-4 FAM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSLNFMR")]
    pub _90_days_p_d_restr_ln_1_4_fam_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSLNLS`
    Title: 90+ DAY P/D RESTRU LN EXCL 1-4 FM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSLNLS")]
    pub _90_day_p_d_restru_ln_excl_1_4_fm: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSLNLSR`
    Title: 90+ DAY P/D RESTRU LN EXCL 1-4 FM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSLNLSR")]
    pub _90_day_p_d_restru_ln_excl_1_4_fm_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSLNLT`
    Title: 90+ DAY P/D RESTR LN- TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSLNLT")]
    pub _90_day_p_d_restr_ln_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSLNLTR`
    Title: 90+ DAY P/D RESTR LN- TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSLNLTR")]
    pub _90_day_p_d_restr_ln_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSMULT`
    Title: 90+ DAY P/D RESTRUCT MULTIFAM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSMULT")]
    pub _90_day_p_d_restruct_multifam: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSNRES`
    Title: 90+ DAY P/D RESTRUCT NFNR LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSNRES")]
    pub _90_day_p_d_restruct_nfnr_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RSOTH`
    Title: 90+ D P/D RESTRUCT ALL OTH LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RSOTH")]
    pub _90_d_p_d_restruct_all_oth_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9SCDEBT`
    Title: 90+ DAYS P/D-DEBT SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9SCDEBT")]
    pub _90_days_p_d_debt_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9SCDEBTR`
    Title: 90+ DAYS P/D-DEBT SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9SCDEBTR")]
    pub _90_days_p_d_debt_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `PARTACQU`
    Title: PARTICIPATIONS ACQUIRED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PARTACQU")]
    pub participations_acquired: Option<f64>,

    #[doc = r#"## FDIC Field:: `PARTCONV`
    Title: PARTICIPATIONS CONVEYED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PARTCONV")]
    pub participations_conveyed: Option<f64>,

    #[doc = r#"## FDIC Field:: `PARTCONVR`
    Title: PARTICIPATIONS CONVEYED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PARTCONVR")]
    pub participations_conveyed_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RB2LNRES`
    Title: ALLOWANCE FOR L&L IN TIER 2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RB2LNRES")]
    pub allowance_for_l_l_in_tier_2: Option<f64>,

    #[doc = r#"## FDIC Field:: `RB2LNRESR`
    Title: ALLOWANCE FOR L&L IN TIER 2 RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RB2LNRESR")]
    pub allowance_for_l_l_in_tier_2_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBC`
    Title: RBC-TOTAL-PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBC")]
    pub rbc_total_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT1`
    Title: TIER 1 RBC-PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT1")]
    pub tier_1_rbc_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT2`
    Title: RBC-TIER2-PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT2")]
    pub rbc_tier2_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT2R`
    Title: RBC-TIER2-PCA RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT2R")]
    pub rbc_tier2_pca_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT1C`
    Title: RC-R COMMON EQ TIER 1 CAPITAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT1C")]
    pub rc_r_common_eq_tier_1_capital: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT1CER`
    Title: COMMON EQUITY TIER 1 RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT1CER")]
    pub common_equity_tier_1_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT1J`
    Title: TIER 1 RBC ADJUSTED LLR - PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT1J")]
    pub tier_1_rbc_adjusted_llr_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT1JR`
    Title: TIER 1 RBC ADJUSTED LLR - PCA RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT1JR")]
    pub tier_1_rbc_adjusted_llr_pca_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBC1AAJ`
    Title: LEVERAGE RATIO-PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBC1AAJ")]
    pub leverage_ratio_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBC1RWAJ`
    Title: TIER 1 RBC RATIO-PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBC1RWAJ")]
    pub tier_1_rbc_ratio_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCRWAJ`
    Title: TOTAL RBC RATIO-PCA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCRWAJ")]
    pub total_rbc_ratio_pca: Option<f64>,

    #[doc = r#"## FDIC Field:: `REPOPURF`
    Title: REPURCHASE AGREEMENT-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="REPOPURF")]
    pub repurchase_agreement_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `REPOSLDF`
    Title: REVERSE REPURCHASE AGREEMENT-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="REPOSLDF")]
    pub reverse_repurchase_agreement_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROEINJR`
    Title: RETAINED EARNINGS/AVG BK EQUITY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ROEINJR")]
    pub retained_earnings_avg_bk_equity: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSCI`
    Title: RESTRUCTURED LN - C&I
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSCI")]
    pub restructured_ln_c_i: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSCONS`
    Title: RESTRUCTURED LN - CONSTRUCTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSCONS")]
    pub restructured_ln_construction: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSLNLS`
    Title: RESTRUCTURED LN EXCL 1-4 FM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSLNLS")]
    pub restructured_ln_excl_1_4_fm: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSLNLSR`
    Title: RESTRUCTURED LN EXCL 1-4 FM RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSLNLSR")]
    pub restructured_ln_excl_1_4_fm_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSLNLTOT`
    Title: RESTRUCTURED LOANS - TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSLNLTOT")]
    pub restructured_loans_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSLNLTOTR`
    Title: RESTRUCTURED LOANS - TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSLNLTOTR")]
    pub restructured_loans_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSLNREFM`
    Title: RESTRUCTURED LOANS - 1-4 FAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSLNREFM")]
    pub restructured_loans_1_4_family: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSLNREFMR`
    Title: RESTRUCTURED LOANS - 1-4 FAMILY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSLNREFMR")]
    pub restructured_loans_1_4_family_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSMULT`
    Title: RESTRUCTURED LN - MULTIFAMILY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSMULT")]
    pub restructured_ln_multifamily: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSNRES`
    Title: RESTRUCT LN - NONFARM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSNRES")]
    pub restruct_ln_nonfarm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSOTHER`
    Title: RESTRUCTURED LN - ALL OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSOTHER")]
    pub restructured_ln_all_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSSDID`
    Title: FEDERAL RESERVE ID NUMBER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RSSDID")]
    pub federal_reserve_id_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `RT`
    Title: INTEREST RATE-TOTAL CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RT")]
    pub interest_rate_total_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `RTFFC`
    Title: INT RATE-FUTURES & FORWARD CONTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RTFFC")]
    pub int_rate_futures_forward_contr: Option<f64>,

    #[doc = r#"## FDIC Field:: `RTNVS`
    Title: INT RATE-SWAPS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RTNVS")]
    pub int_rate_swaps: Option<f64>,

    #[doc = r#"## FDIC Field:: `RTPOC`
    Title: INT RATE-PUR OPTION CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RTPOC")]
    pub int_rate_pur_option_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `RTWOC`
    Title: INT RATE-WRITTEN OPTION CONTRACT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RTWOC")]
    pub int_rate_written_option_contract: Option<f64>,

    #[doc = r#"## FDIC Field:: `RWAJ`
    Title: RWA-ADJUST-PCA-T1 & CET1 RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RWAJ")]
    pub rwa_adjust_pca_t1_cet1_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `RWAJT`
    Title: RWA-ADJUSTED-PCA-TOTAL RBC RAT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RWAJT")]
    pub rwa_adjusted_pca_total_rbc_rat: Option<f64>,

    #[doc = r#"## FDIC Field:: `RWAJTR`
    Title: RWA-ADJUSTED-PCA-TOTAL RBC RAT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RWAJTR")]
    pub rwa_adjusted_pca_total_rbc_rat_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCABS`
    Title: ABS-TOTAL-B/S
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCABS")]
    pub abs_total_b_s: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCABSR`
    Title: ABS-TOTAL-B/S RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCABSR")]
    pub abs_total_b_s_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCAF`
    Title: SECURITIES-AF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCAF")]
    pub securities_af: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCAFR`
    Title: SECURITIES-AF RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCAFR")]
    pub securities_af_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCAOT`
    Title: U.S. AGENCY ALL OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCAOT")]
    pub u_s_agency_all_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMMB`
    Title: COMMERCIAL MBS - TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMMB")]
    pub commercial_mbs_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMOG`
    Title: OTHER COMMERCIAL MBS-GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMOG")]
    pub other_commercial_mbs_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMOGR`
    Title: OTHER COMMERCIAL MBS-GOVT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMOGR")]
    pub other_commercial_mbs_govt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMOT`
    Title: OTHER COMMERCIAL MBS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMOT")]
    pub other_commercial_mbs: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMOTR`
    Title: OTHER COMMERCIAL MBS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMOTR")]
    pub other_commercial_mbs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMPT`
    Title: COMMERCIAL MBS PASS-THROUGH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMPT")]
    pub commercial_mbs_pass_through: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMPTR`
    Title: COMMERCIAL MBS PASS-THROUGH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMPTR")]
    pub commercial_mbs_pass_through_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCOL`
    Title: U.S. AGENCY COLLATERAL MTG-RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCOL")]
    pub u_s_agency_collateral_mtg_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCOLR`
    Title: U.S. AGENCY COLLATERAL MTG-RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCOLR")]
    pub u_s_agency_collateral_mtg_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCPTG`
    Title: COMM MBS PASS-THRU-GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCPTG")]
    pub comm_mbs_pass_thru_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCPTGR`
    Title: COMM MBS PASS-THRU-GOVT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCPTGR")]
    pub comm_mbs_pass_thru_govt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCEQFV`
    Title: EQ SEC READILY DET FV
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCEQFV")]
    pub eq_sec_readily_det_fv: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCEQFVR`
    Title: EQ SEC READILY DET FV RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCEQFVR")]
    pub eq_sec_readily_det_fv_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCFMN`
    Title: U.S. AGENCY ISSUED*FNMA-RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCFMN")]
    pub u_s_agency_issued_fnma_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCFMNR`
    Title: U.S. AGENCY ISSUED*FNMA-RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCFMNR")]
    pub u_s_agency_issued_fnma_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCGNM`
    Title: U.S. AGENCY GTY BY GNMA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCGNM")]
    pub u_s_agency_gty_by_gnma: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCGNMR`
    Title: U.S. AGENCY GTY BY GNMA RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCGNMR")]
    pub u_s_agency_gty_by_gnma_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCGTY`
    Title: U.S. AGENCY ISSUED OR GTY-RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCGTY")]
    pub u_s_agency_issued_or_gty_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCGTYR`
    Title: U.S. AGENCY ISSUED OR GTY-RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCGTYR")]
    pub u_s_agency_issued_or_gty_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCHA`
    Title: SECURITIES-HA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCHA")]
    pub securities_ha: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCHAR`
    Title: SECURITIES-HA RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCHAR")]
    pub securities_ha_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCHTMRES`
    Title: LESS ALLOW FOR CREDIT LOSSES ON HELD TO MATURITY DEBT SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCHTMRES")]
    pub less_allow_for_credit_losses_on_held_to_maturity_debt_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCHTMRESR`
    Title: LESS ALLOW FOR CREDIT LOSSES ON HELD TO MATURITY DEBT SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCHTMRESR")]
    pub less_allow_for_credit_losses_on_held_to_maturity_debt_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCLENT`
    Title: SECURITIES LENT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCLENT")]
    pub securities_lent: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCLENTR`
    Title: SECURITIES LENT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCLENTR")]
    pub securities_lent_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM1T3`
    Title: NONMTG DEBT SEC * 1-3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM1T3")]
    pub nonmtg_debt_sec_1_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM1T3R`
    Title: NONMTG DEBT SEC * 1-3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM1T3R")]
    pub nonmtg_debt_sec_1_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM3LES`
    Title: NONMTG DEBT SEC*3 MONS OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM3LES")]
    pub nonmtg_debt_sec_3_mons_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM3LESR`
    Title: NONMTG DEBT SEC*3 MONS OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM3LESR")]
    pub nonmtg_debt_sec_3_mons_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM3T5`
    Title: NONMTG DEBT SEC * 3-5 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM3T5")]
    pub nonmtg_debt_sec_3_5_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM3T5R`
    Title: NONMTG DEBT SEC * 3-5 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM3T5R")]
    pub nonmtg_debt_sec_3_5_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM3T12`
    Title: NONMTG DEBT SEC * 3-12 MONTHS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM3T12")]
    pub nonmtg_debt_sec_3_12_months: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM3T12R`
    Title: NONMTG DEBT SEC * 3-12 MONTHS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM3T12R")]
    pub nonmtg_debt_sec_3_12_months_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM5T15`
    Title: NONMTG DEBT SEC * 5-15 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM5T15")]
    pub nonmtg_debt_sec_5_15_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNM5T15R`
    Title: NONMTG DEBT SEC * 5-15 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNM5T15R")]
    pub nonmtg_debt_sec_5_15_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNMOV15`
    Title: NONMTG DEBT SEC * OVER 15 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNMOV15")]
    pub nonmtg_debt_sec_over_15_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCNMOV15R`
    Title: NONMTG DEBT SEC * OVER 15 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCNMOV15R")]
    pub nonmtg_debt_sec_over_15_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCO3YLES`
    Title: OTH MORTGAGE SEC * 3 YR OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCO3YLES")]
    pub oth_mortgage_sec_3_yr_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCO3YLESR`
    Title: OTH MORTGAGE SEC * 3 YR OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCO3YLESR")]
    pub oth_mortgage_sec_3_yr_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SC1LES`
    Title: Fixed and floating rate debt securities (included above) with remaining maturity of one year or less
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SC1LES")]
    pub fixed_and_floating_rate_debt_securities_included_above_with_remaining_maturity_of_one_year_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `SC1LESR`
    Title: Fixed and floating rate debt securities (included above) with remaining maturity of one year or less ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SC1LESR")]
    pub fixed_and_floating_rate_debt_securities_included_above_with_remaining_maturity_of_one_year_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCODOT`
    Title: OTH DOM DEBT*ALL OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCODOT")]
    pub oth_dom_debt_all_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCODOTR`
    Title: OTH DOM DEBT*ALL OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCODOTR")]
    pub oth_dom_debt_all_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCODPI`
    Title: CMO PRIV ISSUED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCODPI")]
    pub cmo_priv_issued: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCODPIR`
    Title: CMO PRIV ISSUED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCODPIR")]
    pub cmo_priv_issued_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCOOV3Y`
    Title: OTH MORTGAGE SEC * OVER 3 YRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCOOV3Y")]
    pub oth_mortgage_sec_over_3_yrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCOOV3YR`
    Title: OTH MORTGAGE SEC * OVER 3 YRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCOOV3YR")]
    pub oth_mortgage_sec_over_3_yrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPLEDGE`
    Title: PLEDGED SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPLEDGE")]
    pub pledged_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPLEDGER`
    Title: PLEDGED SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPLEDGER")]
    pub pledged_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT1T3`
    Title: MTG PASS-THRU SEC * 1-3 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT1T3")]
    pub mtg_pass_thru_sec_1_3_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT1T3R`
    Title: MTG PASS-THRU SEC * 1-3 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT1T3R")]
    pub mtg_pass_thru_sec_1_3_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT3LES`
    Title: MTG PASS-THRU SEC*3 MON OR LESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT3LES")]
    pub mtg_pass_thru_sec_3_mon_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT3LESR`
    Title: MTG PASS-THRU SEC*3 MON OR LESS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT3LESR")]
    pub mtg_pass_thru_sec_3_mon_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT3T5`
    Title: MTG PASS-THRU SEC * 3-5 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT3T5")]
    pub mtg_pass_thru_sec_3_5_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT3T5R`
    Title: MTG PASS-THRU SEC * 3-5 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT3T5R")]
    pub mtg_pass_thru_sec_3_5_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT3T12`
    Title: MTG PASS-THRU SEC * 3-12 MONTHS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT3T12")]
    pub mtg_pass_thru_sec_3_12_months: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT3T12R`
    Title: MTG PASS-THRU SEC * 3-12 MONTHS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT3T12R")]
    pub mtg_pass_thru_sec_3_12_months_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT5T15`
    Title: MTG PASS-THRU SEC * 5-15 YEARS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT5T15")]
    pub mtg_pass_thru_sec_5_15_years: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPT5T15R`
    Title: MTG PASS-THRU SEC * 5-15 YEARS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPT5T15R")]
    pub mtg_pass_thru_sec_5_15_years_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPTOV15`
    Title: MTG PASS-THRU SEC * OVER 15 YRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPTOV15")]
    pub mtg_pass_thru_sec_over_15_yrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCPTOV15R`
    Title: MTG PASS-THRU SEC * OVER 15 YRS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCPTOV15R")]
    pub mtg_pass_thru_sec_over_15_yrs_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCRDEBT`
    Title: DEBT SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCRDEBT")]
    pub debt_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCRDEBTR`
    Title: DEBT SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCRDEBTR")]
    pub debt_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCSFP`
    Title: STRUCTURED FIN PROD - TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCSFP")]
    pub structured_fin_prod_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCSFPR`
    Title: STRUCTURED FIN PROD - TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCSFPR")]
    pub structured_fin_prod_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCSNHAA`
    Title: STRUCTURED NOTES AMORTIZED COST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCSNHAA")]
    pub structured_notes_amortized_cost: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCSNHAAR`
    Title: STRUCTURED NOTES AMORTIZED COST RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCSNHAAR")]
    pub structured_notes_amortized_cost_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCSNHAF`
    Title: STRUCTURED NOTES-FAIR VALUE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCSNHAF")]
    pub structured_notes_fair_value: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCSNHAFR`
    Title: STRUCTURED NOTES-FAIR VALUE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCSNHAFR")]
    pub structured_notes_fair_value_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCSPN`
    Title: U.S. AGENCY GOVT SPONSORED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCSPN")]
    pub u_s_agency_govt_sponsored: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30AUTO`
    Title: 30-89 PD LN-SECURITIZATION-AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30AUTO")]
    pub _30_89_pd_ln_securitization_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30AUTOR`
    Title: 30-89 PD LN-SECURITIZATION-AUTO RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30AUTOR")]
    pub _30_89_pd_ln_securitization_auto_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30CI`
    Title: 30-89 PD LN-SECURITIZATION-CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30CI")]
    pub _30_89_pd_ln_securitization_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30CIR`
    Title: 30-89 PD LN-SECURITIZATION-CI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30CIR")]
    pub _30_89_pd_ln_securitization_ci_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30CON`
    Title: 30-89 PD LN-SECURITIZATION-CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30CON")]
    pub _30_89_pd_ln_securitization_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30CONR`
    Title: 30-89 PD LN-SECURITIZATION-CON RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30CONR")]
    pub _30_89_pd_ln_securitization_con_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30CRCD`
    Title: 30-89 PD LN-SECURITIZATION-CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30CRCD")]
    pub _30_89_pd_ln_securitization_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30CRCDR`
    Title: 30-89 PD LN-SECURITIZATION-CRCD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30CRCDR")]
    pub _30_89_pd_ln_securitization_crcd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30HEL`
    Title: 30-89 PD LN-SECURITIZATION-HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30HEL")]
    pub _30_89_pd_ln_securitization_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30HELR`
    Title: 30-89 PD LN-SECURITIZATION-HEL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30HELR")]
    pub _30_89_pd_ln_securitization_hel_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30OTH`
    Title: 30-89 PD LN-SECURITIZATION-OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30OTH")]
    pub _30_89_pd_ln_securitization_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30OTHR`
    Title: 30-89 PD LN-SECURITIZATION-OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30OTHR")]
    pub _30_89_pd_ln_securitization_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30RES`
    Title: 30-89 PD LN-SECURITIZATION -RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30RES")]
    pub _30_89_pd_ln_securitization_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ30RESR`
    Title: 30-89 PD LN-SECURITIZATION -RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ30RESR")]
    pub _30_89_pd_ln_securitization_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90AUTO`
    Title: 90 + PD LN-SECURITIZATION-AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90AUTO")]
    pub _90_pd_ln_securitization_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90AUTOR`
    Title: 90 + PD LN-SECURITIZATION-AUTO RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90AUTOR")]
    pub _90_pd_ln_securitization_auto_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90CI`
    Title: 90 + PD LN-SECURITIZATION-CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90CI")]
    pub _90_pd_ln_securitization_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90CIR`
    Title: 90 + PD LN-SECURITIZATION-CI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90CIR")]
    pub _90_pd_ln_securitization_ci_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90CON`
    Title: 90 + PD LN-SECURITIZATION-CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90CON")]
    pub _90_pd_ln_securitization_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90CONR`
    Title: 90 + PD LN-SECURITIZATION-CON RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90CONR")]
    pub _90_pd_ln_securitization_con_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90CRCD`
    Title: 90 + PD LN-SECURITIZATION-CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90CRCD")]
    pub _90_pd_ln_securitization_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90CRCDR`
    Title: 90 + PD LN-SECURITIZATION-CRCD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90CRCDR")]
    pub _90_pd_ln_securitization_crcd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90HEL`
    Title: 90+ PD LN-SECURITIZATION-HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90HEL")]
    pub _90_pd_ln_securitization_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90HELR`
    Title: 90+ PD LN-SECURITIZATION-HEL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90HELR")]
    pub _90_pd_ln_securitization_hel_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90OTH`
    Title: 90 + PD LN-SECURITIZATION-OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90OTH")]
    pub _90_pd_ln_securitization_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90OTHR`
    Title: 90 + PD LN-SECURITIZATION-OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90OTHR")]
    pub _90_pd_ln_securitization_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90RES`
    Title: 90 + PD LN-SECURITIZATION-RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90RES")]
    pub _90_pd_ln_securitization_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZ90RESR`
    Title: 90 + PD LN-SECURITIZATION-RES RATION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZ90RESR")]
    pub _90_pd_ln_securitization_res_ration: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRAUTO`
    Title: REC ASSET SECURITIZATION-AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRAUTO")]
    pub rec_asset_securitization_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRAUTOR`
    Title: REC ASSET SECURITIZATION-AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRAUTOR")]
    pub rec_asset_securitization_auto_szcrautor: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCDFE`
    Title: OUTSTDG CC FEES IN SECURITZD CC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCDFE")]
    pub outstdg_cc_fees_in_securitzd_cc: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCDFER`
    Title: OUTSTDG CC FEES IN SECURITZD CC RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCDFER")]
    pub outstdg_cc_fees_in_securitzd_cc_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCI`
    Title: REC ASSET SECURITIZATION-CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCI")]
    pub rec_asset_securitization_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCIR`
    Title: REC ASSET SECURITIZATION-CI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCIR")]
    pub rec_asset_securitization_ci_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCON`
    Title: REC ASSET SECURITIZATION-CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCON")]
    pub rec_asset_securitization_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCONR`
    Title: REC ASSET SECURITIZATION-CON RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCONR")]
    pub rec_asset_securitization_con_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCRCD`
    Title: REC ASSET SECURITIZATION - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCRCD")]
    pub rec_asset_securitization_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRCRCDR`
    Title: REC ASSET SECURITIZATION - CRCD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRCRCDR")]
    pub rec_asset_securitization_crcd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRHEL`
    Title: RE PRIN SEC ASSET SOLD-HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRHEL")]
    pub re_prin_sec_asset_sold_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRHELR`
    Title: RE PRIN SEC ASSET SOLD-HEL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRHELR")]
    pub re_prin_sec_asset_sold_hel_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCROTH`
    Title: REC ASSET SECURITIZATION-
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCROTH")]
    pub rec_asset_securitization: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCROTHR`
    Title: REC ASSET SECURITIZATION- RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCROTHR")]
    pub rec_asset_securitization_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRRES`
    Title: REC ASSET SECURITIZATION-RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRRES")]
    pub rec_asset_securitization_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZCRRESR`
    Title: REC ASSET SECURITIZATION-RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZCRRESR")]
    pub rec_asset_securitization_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRAUTO`
    Title: C/O ON ASSET SECURITIZATION-AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRAUTO")]
    pub c_o_on_asset_securitization_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRAUTOR`
    Title: C/O ON ASSET SECURITIZATION-AUTO RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRAUTOR")]
    pub c_o_on_asset_securitization_auto_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRCI`
    Title: C/O ON ASSET SECURITIZATION-CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRCI")]
    pub c_o_on_asset_securitization_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRCIR`
    Title: C/O ON ASSET SECURITIZATION-CI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRCIR")]
    pub c_o_on_asset_securitization_ci_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRCON`
    Title: C/O ON ASSET SECURITIZATION-CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRCON")]
    pub c_o_on_asset_securitization_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRCONR`
    Title: C/O ON ASSET SECURITIZATION-CON RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRCONR")]
    pub c_o_on_asset_securitization_con_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRCRCD`
    Title: C/O ON ASSET SECURITIZATION-CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRCRCD")]
    pub c_o_on_asset_securitization_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRCRCDR`
    Title: C/O ON ASSET SECURITIZATION-CRCD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRCRCDR")]
    pub c_o_on_asset_securitization_crcd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRHEL`
    Title: C/O ON ASSET SECURITIZATION-HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRHEL")]
    pub c_o_on_asset_securitization_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRHELR`
    Title: C/O ON ASSET SECURITIZATION-HEL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRHELR")]
    pub c_o_on_asset_securitization_hel_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDROTH`
    Title: C/O ON ASSET SECURITIZATION-OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDROTH")]
    pub c_o_on_asset_securitization_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDROTHR`
    Title: C/O ON ASSET SECURITIZATION-OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDROTHR")]
    pub c_o_on_asset_securitization_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZDRRES`
    Title: C/O ON ASSET SECURITIZATION-RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZDRRES")]
    pub c_o_on_asset_securitization_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLAUT`
    Title: CR EXP ON SECURITIZATN - AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLAUT")]
    pub cr_exp_on_securitizatn_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLAUTR`
    Title: CR EXP ON SECURITIZATN - AUTO RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLAUTR")]
    pub cr_exp_on_securitizatn_auto_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLCCD`
    Title: CR EXP ON SECURITIZATN - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLCCD")]
    pub cr_exp_on_securitizatn_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLCCDR`
    Title: CR EXP ON SECURITIZATN - CRCD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLCCDR")]
    pub cr_exp_on_securitizatn_crcd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLCI`
    Title: CR EXP ON SECURITIZATN -CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLCI")]
    pub cr_exp_on_securitizatn_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLCIR`
    Title: CR EXP ON SECURITIZATN -CI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLCIR")]
    pub cr_exp_on_securitizatn_ci_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLCON`
    Title: CR EXP ON SECURITIZATN - CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLCON")]
    pub cr_exp_on_securitizatn_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLCONR`
    Title: CR EXP ON SECURITIZATN - CON RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLCONR")]
    pub cr_exp_on_securitizatn_con_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLHEL`
    Title: CR EXP ON SECURITIZATN - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLHEL")]
    pub cr_exp_on_securitizatn_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLHELR`
    Title: CR EXP ON SECURITIZATN - HEL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLHELR")]
    pub cr_exp_on_securitizatn_hel_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLOTH`
    Title: CR EXP ON SECURITIZATN -OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLOTH")]
    pub cr_exp_on_securitizatn_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLOTHR`
    Title: CR EXP ON SECURITIZATN -OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLOTHR")]
    pub cr_exp_on_securitizatn_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLRES`
    Title: CR EXP ON SECURITIZATION RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLRES")]
    pub cr_exp_on_securitization_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZISLRESR`
    Title: CR EXP ON SECURITIZATION RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZISLRESR")]
    pub cr_exp_on_securitization_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLAUTO`
    Title: RE PRIN SEC ASSET SOLD - AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLAUTO")]
    pub re_prin_sec_asset_sold_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLAUTOR`
    Title: RE PRIN SEC ASSET SOLD - AUTO RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLAUTOR")]
    pub re_prin_sec_asset_sold_auto_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNCI`
    Title: RE PRIN SEC ASSET SOLD - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNCI")]
    pub re_prin_sec_asset_sold_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNCIR`
    Title: RE PRIN SEC ASSET SOLD - CI RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNCIR")]
    pub re_prin_sec_asset_sold_ci_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNCON`
    Title: RE PRIN SEC ASSET SOLD - CONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNCON")]
    pub re_prin_sec_asset_sold_cons: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNCONR`
    Title: RE PRIN SEC ASSET SOLD - CONS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNCONR")]
    pub re_prin_sec_asset_sold_cons_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNCRCD`
    Title: RE PRIN SEC ASSET SOLD - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNCRCD")]
    pub re_prin_sec_asset_sold_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNCRCDR`
    Title: RE PRIN SEC ASSET SOLD - CRCD RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNCRCDR")]
    pub re_prin_sec_asset_sold_crcd_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNHEL`
    Title: RE PRIN SEC ASSET SOLD - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNHEL")]
    pub re_prin_sec_asset_sold_hel_szlnhel: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNHELR`
    Title: RE PRIN SEC ASSET SOLD - HEL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNHELR")]
    pub re_prin_sec_asset_sold_hel_ratio_szlnhelr: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNOTH`
    Title: RE PRIN SEC ASSET SOLD - OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNOTH")]
    pub re_prin_sec_asset_sold_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNOTHR`
    Title: RE PRIN SEC ASSET SOLD - OTH RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNOTHR")]
    pub re_prin_sec_asset_sold_oth_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNRES`
    Title: RE PRIN SEC ASSET SOLD-RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNRES")]
    pub re_prin_sec_asset_sold_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZLNRESR`
    Title: RE PRIN SEC ASSET SOLD-RES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZLNRESR")]
    pub re_prin_sec_asset_sold_res_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZUCAUTO`
    Title: COMMITS FOR LIQUIDITY  - AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZUCAUTO")]
    pub commits_for_liquidity_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZUCCI`
    Title: COMMITS FOR LIQUIDITY  - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZUCCI")]
    pub commits_for_liquidity_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZUCCON`
    Title: COMMITS FOR LIQUIDITY  - CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZUCCON")]
    pub commits_for_liquidity_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZUCCRCD`
    Title: COMMITS FOR LIQUIDITY  - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZUCCRCD")]
    pub commits_for_liquidity_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZUCHEL`
    Title: COMMITS FOR LIQUIDITY - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZUCHEL")]
    pub commits_for_liquidity_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZUCOTH`
    Title: COMMITS FOR LIQUIDITY  - OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZUCOTH")]
    pub commits_for_liquidity_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `SZUCRES`
    Title: COMMITS FOR LIQUIDITY  - RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SZUCRES")]
    pub commits_for_liquidity_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCAMA`
    Title: CORP TRUST-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCAMA")]
    pub corp_trust_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCAMANUM`
    Title: CORP TRUST-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCAMANUM")]
    pub corp_trust_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCANMA`
    Title: CORP TRUST-NON-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCANMA")]
    pub corp_trust_non_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCANMNUM`
    Title: CORP TRUST-NON-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCANMNUM")]
    pub corp_trust_non_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCANUM`
    Title: CORP TRUST-TRUSTEESHIPS-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCANUM")]
    pub corp_trust_trusteeships_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCANUMD`
    Title: CORP & MUNI-TRUSTEE-DEFAULT-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCANUMD")]
    pub corp_muni_trustee_default_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCAPAO`
    Title: CORP TRUST-TRUSTEESHIPS-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCAPAO")]
    pub corp_trust_trusteeships_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCAPAOD`
    Title: CORP & MUNI-TRUSTEE-DEFAULT-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCAPAOD")]
    pub corp_muni_trustee_default_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCATNUM`
    Title: CORP TRUST-TRANSFER-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCATNUM")]
    pub corp_trust_transfer_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCDEMV`
    Title: CIFS -DOM EQUITY-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCDEMV")]
    pub cifs_dom_equity_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCDENUM`
    Title: CIFS -DOM EQUITY-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCDENUM")]
    pub cifs_dom_equity_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCIEMV`
    Title: CIFS -INTL/GLOBAL-EQ-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCIEMV")]
    pub cifs_intl_global_eq_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCIENUM`
    Title: CIFS -INTL/GLOBAL-EQ-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCIENUM")]
    pub cifs_intl_global_eq_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCMBMV`
    Title: CIFS-MUNICIPAL BOND-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCMBMV")]
    pub cifs_municipal_bond_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCMBNUM`
    Title: CIFS-MUNICIPAL BOND-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCMBNUM")]
    pub cifs_municipal_bond_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSBMV`
    Title: CIFS -STOCK/BOND-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSBMV")]
    pub cifs_stock_bond_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSBNUM`
    Title: CIFS -STOCK/BOND-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSBNUM")]
    pub cifs_stock_bond_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSNMA`
    Title: CUST AND SAFE ACCT-NON-MAN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSNMA")]
    pub cust_and_safe_acct_non_man_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSNMNUM`
    Title: CUST AND SAFE ACCT-NON-MAN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSNMNUM")]
    pub cust_and_safe_acct_non_man_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSOMV`
    Title: CIFS-SPECIALTY/OTHER-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSOMV")]
    pub cifs_specialty_other_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSONUM`
    Title: CIFS-SPECIALTY/OTHER-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSONUM")]
    pub cifs_specialty_other_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSTMV`
    Title: CIFS-SHORT TERM INV-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSTMV")]
    pub cifs_short_term_inv_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCSTNUM`
    Title: CIFS-SHORT TERM INV-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCSTNUM")]
    pub cifs_short_term_inv_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCTBMV`
    Title: CIFS - TAXABLE BOND-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCTBMV")]
    pub cifs_taxable_bond_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCTBNUM`
    Title: CIFS - TAXABLE BOND-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCTBNUM")]
    pub cifs_taxable_bond_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCTOTMV`
    Title: CIFS-TOTAL-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCTOTMV")]
    pub cifs_total_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TCTOTNUM`
    Title: CIFS-TOTAL-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TCTOTNUM")]
    pub cifs_total_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEBMA`
    Title: EMP BENE-DEF BENE-MANAGE-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEBMA")]
    pub emp_bene_def_bene_manage_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEBMANUM`
    Title: EMP BENE-DEF BENE-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEBMANUM")]
    pub emp_bene_def_bene_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEBNMA`
    Title: EMP BENE-DEF BENE-NON-MAN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEBNMA")]
    pub emp_bene_def_bene_non_man_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEBNMNUM`
    Title: EMP BENE-DEF BENE-NON-MAN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEBNMNUM")]
    pub emp_bene_def_bene_non_man_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TECMA`
    Title: EMP BENE-CONTRIB-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TECMA")]
    pub emp_bene_contrib_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TECMANUM`
    Title: EMP BENE-CONTRI-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TECMANUM")]
    pub emp_bene_contri_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TECNMA`
    Title: EMP BENE-CONTRI-NON-MAN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TECNMA")]
    pub emp_bene_contri_non_man_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TECNMNUM`
    Title: EMP BENE-CONTRI-NON-MANAGE-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TECNMNUM")]
    pub emp_bene_contri_non_manage_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TECPS`
    Title: EMP BEN & RET TR - COM & PF STK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TECPS")]
    pub emp_ben_ret_tr_com_pf_stk: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEEQF`
    Title: EMP BEN & RET TR - EQ MUT FUND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEEQF")]
    pub emp_ben_ret_tr_eq_mut_fund: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEI`
    Title: EMP BEN & RET TR - INT BEARING
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEI")]
    pub emp_ben_ret_tr_int_bearing: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEMATOT`
    Title: EMP BEN & RET TR-TOT MANAGE AST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEMATOT")]
    pub emp_ben_ret_tr_tot_manage_ast: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEMISC`
    Title: EMP BEN & RET TR - MISC ASSET
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEMISC")]
    pub emp_ben_ret_tr_misc_asset: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEMMF`
    Title: EMP BEN & RET TR - MONEY MKT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEMMF")]
    pub emp_ben_ret_tr_money_mkt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TENI`
    Title: EMP BEN & RET TR - NONINT BEAR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TENI")]
    pub emp_ben_ret_tr_nonint_bear: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEOTHB`
    Title: EMP BEN & RET TR-OTH NOTE & BND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEOTHB")]
    pub emp_ben_ret_tr_oth_note_bnd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEOTHF`
    Title: EMP BEN & RET TR - OTH MUT FUND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEOTHF")]
    pub emp_ben_ret_tr_oth_mut_fund: Option<f64>,

    #[doc = r#"## FDIC Field:: `TERE`
    Title: EMP BEN & RET TR - REAL ESTATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TERE")]
    pub emp_ben_ret_tr_real_estate: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEREMTG`
    Title: EMP BEN & RET TR - RE MTG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEREMTG")]
    pub emp_ben_ret_tr_re_mtg: Option<f64>,

    #[doc = r#"## FDIC Field:: `TESCMUN`
    Title: EMP BEN & RET TR - MUNI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TESCMUN")]
    pub emp_ben_ret_tr_muni: Option<f64>,

    #[doc = r#"## FDIC Field:: `TESCUS`
    Title: EMP BEN & RET TR -U.S TREAS & OB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TESCUS")]
    pub emp_ben_ret_tr_u_s_treas_ob: Option<f64>,

    #[doc = r#"## FDIC Field:: `TESTO`
    Title: EMP BEN & RET TR - SHRT TERM OB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TESTO")]
    pub emp_ben_ret_tr_shrt_term_ob: Option<f64>,

    #[doc = r#"## FDIC Field:: `TETOT`
    Title: EXPENSE FIDUCIARY - YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TETOT")]
    pub expense_fiduciary_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TETRF`
    Title: EMP BEN & RET TR - TRUST FUND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TETRF")]
    pub emp_ben_ret_tr_trust_fund: Option<f64>,

    #[doc = r#"## FDIC Field:: `TEUF`
    Title: EMP BEN & RET TR - UNREG FUNDS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TEUF")]
    pub emp_ben_ret_tr_unreg_funds: Option<f64>,

    #[doc = r#"## FDIC Field:: `TFEMA`
    Title: FOUNDATION & ENDOW-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TFEMA")]
    pub foundation_endow_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TFEMANUM`
    Title: FOUNDATION & ENDOW-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TFEMANUM")]
    pub foundation_endow_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TFENMA`
    Title: FOUNDATION & END-NON-MANAGE-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TFENMA")]
    pub foundation_end_non_manage_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TFENMNUM`
    Title: FOUNDATION & END-NON-MANAGE-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TFENMNUM")]
    pub foundation_end_non_manage_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TICA`
    Title: GR.INC-CORP TRUST & AGENCY-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TICA")]
    pub gr_inc_corp_trust_agency_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TICS`
    Title: GR.INC-CUSTODY-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TICS")]
    pub gr_inc_custody_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIEB`
    Title: GR.INC-EMP. BENEFIT-BENEFIT-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIEB")]
    pub gr_inc_emp_benefit_benefit_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIEC`
    Title: GR.INC-EMP. BENEFIT- CONTRI-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIEC")]
    pub gr_inc_emp_benefit_contri_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIFE`
    Title: GR. INC- FOUNDATION & ENDOW-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIFE")]
    pub gr_inc_foundation_endow_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIMA`
    Title: GR.INC - INVESTMENT AGCY - YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIMA")]
    pub gr_inc_investment_agcy_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIMMA`
    Title: INVESTMENT AGENCY-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIMMA")]
    pub investment_agency_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIMMANUM`
    Title: INVESTMENT AGENCY-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIMMANUM")]
    pub investment_agency_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIMNMA`
    Title: INVESTMENT AGCY NON-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIMNMA")]
    pub investment_agcy_non_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIMNMNUM`
    Title: INVESTMENT AGCY NON-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIMNMNUM")]
    pub investment_agcy_non_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TINTRA`
    Title: INTRACOMPANY INC FIDUCIARY-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TINTRA")]
    pub intracompany_inc_fiduciary_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIOF`
    Title: GR.INC-OTHER FIDUCIARY-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIOF")]
    pub gr_inc_other_fiduciary_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIOR`
    Title: GR.INC-OTHER RETIREMENT -YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIOR")]
    pub gr_inc_other_retirement_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIP`
    Title: GR.INC-PERSONAL & AG ACCTS-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIP")]
    pub gr_inc_personal_ag_accts_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TIR`
    Title: GR.INC-RELATED SERV-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TIR")]
    pub gr_inc_related_serv_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TITOTF`
    Title: TOT FOREIGN OFF GROSS FIDUC-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TITOTF")]
    pub tot_foreign_off_gross_fiduc_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TMAF`
    Title: FIDUCIARY FGN OFF-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TMAF")]
    pub fiduciary_fgn_off_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TMAFNUM`
    Title: FIDUCIARY FGN OFF-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TMAFNUM")]
    pub fiduciary_fgn_off_managed_amt_tmafnum: Option<f64>,

    #[doc = r#"## FDIC Field:: `TMASMF`
    Title: ADVISED/SPONSORED MUT FND -AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TMASMF")]
    pub advised_sponsored_mut_fnd_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TMASMFN`
    Title: ADVISED/SPONSORED MUTAL FND-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TMASMFN")]
    pub advised_sponsored_mutal_fnd_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TNI`
    Title: NET FIDUCIARY INCOME -YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TNI")]
    pub net_fiduciary_income_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TNL`
    Title: NET LOSS FROM FIDUCIARY-YTD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TNL")]
    pub net_loss_from_fiduciary_ytd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TNMAF`
    Title: FIDUCIARY FGN OFF-NON-MAN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TNMAF")]
    pub fiduciary_fgn_off_non_man_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TNMNUMF`
    Title: FIDUCIARY FGN OFF-NON-MAN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TNMNUMF")]
    pub fiduciary_fgn_off_non_man_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOCPS`
    Title: ALL OTH MAN ASSET-COM & PFD STK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOCPS")]
    pub all_oth_man_asset_com_pfd_stk: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOEQF`
    Title: ALL OTH MANAGE AST - EQ MUT FND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOEQF")]
    pub all_oth_manage_ast_eq_mut_fnd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOFMA`
    Title: OTH FIDUCIARY-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOFMA")]
    pub oth_fiduciary_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOFMANUM`
    Title: OTH FIDUCIARY-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOFMANUM")]
    pub oth_fiduciary_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOFNMA`
    Title: OTH FIDUCIARY NON-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOFNMA")]
    pub oth_fiduciary_non_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOFNMNUM`
    Title: OTH FIDUCIARY-NON-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOFNMNUM")]
    pub oth_fiduciary_non_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOI`
    Title: ALL OTH MANAGE ASSET - INT BEAR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOI")]
    pub all_oth_manage_asset_int_bear: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOMATOT`
    Title: ALL OTHER MANAGED ASSET- TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOMATOT")]
    pub all_other_managed_asset_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOMISC`
    Title: ALL OTH MAN ASSET - MISC ASSET
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOMISC")]
    pub all_oth_man_asset_misc_asset: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOMMF`
    Title: ALL OTH MANAGE AST - MONEY MKT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOMMF")]
    pub all_oth_manage_ast_money_mkt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TONI`
    Title: ALL OTH MAN ASSET - NONINT BEAR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TONI")]
    pub all_oth_man_asset_nonint_bear: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOOTHB`
    Title: ALL OTH MAN AST -OTH NOTE & BND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOOTHB")]
    pub all_oth_man_ast_oth_note_bnd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOOTHF`
    Title: ALL OTH MAN ASSET - OTH MUT FND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOOTHF")]
    pub all_oth_man_asset_oth_mut_fnd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TORE`
    Title: ALL OTH MAN ASSET - REAL ESTATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TORE")]
    pub all_oth_man_asset_real_estate: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOREMTG`
    Title: ALL OTHER MANAGE ASSET - RE MTG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOREMTG")]
    pub all_other_manage_asset_re_mtg: Option<f64>,

    #[doc = r#"## FDIC Field:: `TORMA`
    Title: OTH RETIREMENT-MANAGED-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TORMA")]
    pub oth_retirement_managed_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TORMANUM`
    Title: OTH RETIREMENT-MANAGED-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TORMANUM")]
    pub oth_retirement_managed_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TORNMA`
    Title: OTH RETIREMENT-NON-MAN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TORNMA")]
    pub oth_retirement_non_man_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TORNMNUM`
    Title: OTH RETIREMENT-NON-MAN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TORNMNUM")]
    pub oth_retirement_non_man_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOSCMUN`
    Title: ALL OTHER MANAGED ASSET - MUNI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOSCMUN")]
    pub all_other_managed_asset_muni: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOSCUS`
    Title: ALL OTH MAN AST-U.S. TREAS & OB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOSCUS")]
    pub all_oth_man_ast_u_s_treas_ob: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOSTO`
    Title: ALL OTH MAN AST - SHRT TERM OBL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOSTO")]
    pub all_oth_man_ast_shrt_term_obl: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOTRF`
    Title: ALL OTH MAN ASSET - TRUST FUND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOTRF")]
    pub all_oth_man_asset_trust_fund: Option<f64>,

    #[doc = r#"## FDIC Field:: `TOUF`
    Title: ALL OTH MAN ASSET - UNREG FUNDS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TOUF")]
    pub all_oth_man_asset_unreg_funds: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPICPS`
    Title: PER TR & INV AGY- COM & PRF STK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPICPS")]
    pub per_tr_inv_agy_com_prf_stk: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIEQF`
    Title: PER TR & INV AGY - EQ MUT FUND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIEQF")]
    pub per_tr_inv_agy_eq_mut_fund: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPII`
    Title: PER TR & INV AGY - INT BEARING
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPII")]
    pub per_tr_inv_agy_int_bearing: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIMATOT`
    Title: PER TR & INV AGY-TOT MANAGE AST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIMATOT")]
    pub per_tr_inv_agy_tot_manage_ast: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIMISC`
    Title: PER TR & INV AGY - MISC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIMISC")]
    pub per_tr_inv_agy_misc: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIMMF`
    Title: PER TR & INV AGY - MONEY MKT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIMMF")]
    pub per_tr_inv_agy_money_mkt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPINI`
    Title: PER TR & INV AGY-NONINT BEARING
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPINI")]
    pub per_tr_inv_agy_nonint_bearing: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIOTHB`
    Title: PER TR & INV AGY-OTH NOTE & BND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIOTHB")]
    pub per_tr_inv_agy_oth_note_bnd: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIOTHF`
    Title: PER TR & INV AGY - OTH MUT FUND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIOTHF")]
    pub per_tr_inv_agy_oth_mut_fund: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIRE`
    Title: PER TR & INV AGY - REAL ESTATE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIRE")]
    pub per_tr_inv_agy_real_estate: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIREMTG`
    Title: PER TR & INV AGY - RE MTG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIREMTG")]
    pub per_tr_inv_agy_re_mtg: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPISCMUN`
    Title: PER TR & INV AGY - MUNI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPISCMUN")]
    pub per_tr_inv_agy_muni: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPISCUS`
    Title: PER TR & INV AGY-U.S TREAS & OB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPISCUS")]
    pub per_tr_inv_agy_u_s_treas_ob: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPISTO`
    Title: PER TR & INV AGY - SHRT TERM OB
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPISTO")]
    pub per_tr_inv_agy_shrt_term_ob: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPITRF`
    Title: PER TR & INV AGY - TRUST FUND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPITRF")]
    pub per_tr_inv_agy_trust_fund: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPIUF`
    Title: PER TR & INV AGY- UNREG FUNDS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPIUF")]
    pub per_tr_inv_agy_unreg_funds: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPMA`
    Title: MANAGED ASSET-PER & AGEN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPMA")]
    pub managed_asset_per_agen_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPMANUM`
    Title: MANAGED ASSET - PER&AGEN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPMANUM")]
    pub managed_asset_per_agen_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPNMA`
    Title: NON-MANAGED - PER&AGEN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPNMA")]
    pub non_managed_per_agen_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TPNMNUM`
    Title: NON-MANAGED ASSET-PER&AGEN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TPNMNUM")]
    pub non_managed_asset_per_agen_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TREXER`
    Title: TRUST POWERS EXERCISED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TREXER")]
    pub trust_powers_exercised: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRFOR`
    Title: TRADING ACCOUNTS-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRFOR")]
    pub trading_accounts_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRHMA`
    Title: IRA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRHMA")]
    pub ira: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRHMANUM`
    Title: IRA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRHMANUM")]
    pub ira_trhmanum: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRHNMA`
    Title: IRA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRHNMA")]
    pub ira_trhnma: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRHNMNUM`
    Title: IRA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRHNMNUM")]
    pub ira_trhnmnum: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRLREVAL`
    Title: TRADE-DERIVATIVES NEG VAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRLREVAL")]
    pub trade_derivatives_neg_val: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRLREVALR`
    Title: TRADE-DERIVATED NEG VAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRLREVALR")]
    pub trade_derivated_neg_val_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNCBO`
    Title: TRANSACTION-COM BKS& OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNCBO")]
    pub transaction_com_bks_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNCBOR`
    Title: TRANSACTION-COM BKS& OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNCBOR")]
    pub transaction_com_bks_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNFC`
    Title: TRANSACTION-FOR COUNTRY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNFC")]
    pub transaction_for_country: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNFCFG`
    Title: TRANSACTION-FOR COUNTRY & GOVT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNFCFG")]
    pub transaction_for_country_govt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNFCFGR`
    Title: TRANSACTION-FOR COUNTRY & GOVT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNFCFGR")]
    pub transaction_for_country_govt_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNFG`
    Title: TRANSACTION-FOREIGN GOVERNMENT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNFG")]
    pub transaction_foreign_government: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNNIA`
    Title: AMT NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNNIA")]
    pub amt_non_interest_bearing_transaction_acc_more_than_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNNIAR`
    Title: AMT NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNNIAR")]
    pub amt_non_interest_bearing_transaction_acc_more_than_250_000_trnniar: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRNNIN`
    Title: NUM NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRNNIN")]
    pub num_non_interest_bearing_transaction_acc_more_than_250_000: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRPOWER`
    Title: INSTITUTION HAS TRUST POWER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRPOWER")]
    pub institution_has_trust_power: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRREVALD`
    Title: TRADE-DERIV POS VAL-DOM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRREVALD")]
    pub trade_deriv_pos_val_dom: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRREVALF`
    Title: TRADE-DERIV POS VALUE-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRREVALF")]
    pub trade_deriv_pos_value_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRREVALSUM`
    Title: REVALUATION GAINS ON OFF-BALANCE SHEET CONTRACTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRREVALSUM")]
    pub revaluation_gains_on_off_balance_sheet_contracts: Option<f64>,

    #[doc = r#"## FDIC Field:: `TRREVALSUMR`
    Title: REVALUATION GAINS ON OFF-BALANCE SHEET CONTRACTS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TRREVALSUMR")]
    pub revaluation_gains_on_off_balance_sheet_contracts_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `TTMA`
    Title: TOT FIDUCIARY ACCTS-MAN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TTMA")]
    pub tot_fiduciary_accts_man_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TTNANUM`
    Title: TOT FIDUCIARY ACCTS-MAN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TTNANUM")]
    pub tot_fiduciary_accts_man_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `TTNMA`
    Title: TOT FIDUCIARY ACCTS-NON-MAN-AMT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TTNMA")]
    pub tot_fiduciary_accts_non_man_amt: Option<f64>,

    #[doc = r#"## FDIC Field:: `TTNMNUM`
    Title: TOT FIDUCIARY ACCTS-NON-MAN-NUM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="TTNMNUM")]
    pub tot_fiduciary_accts_non_man_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `UC`
    Title: UNUSED COMMIT-TOTAL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UC")]
    pub unused_commit_total: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCR`
    Title: UNUSED COMMIT-TOTAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCR")]
    pub unused_commit_total_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCOMRE`
    Title: UNUSED COMMIT-COM RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCOMRE")]
    pub unused_commit_com_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCOMRER`
    Title: UNUSED COMMIT-COM RE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCOMRER")]
    pub unused_commit_com_re_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCOMRES`
    Title: UNUSED COMMIT-SECURED COM RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCOMRES")]
    pub unused_commit_secured_com_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCOMRESR`
    Title: UNUSED COMMIT-SECURED COM RE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCOMRESR")]
    pub unused_commit_secured_com_re_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCOMREU`
    Title: UNUSED COMMIT-UNSECURED COM RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCOMREU")]
    pub unused_commit_unsecured_com_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCOMREUR`
    Title: UNUSED COMMIT-UNSECURED COM RE RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCOMREUR")]
    pub unused_commit_unsecured_com_re_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCRCD`
    Title: UNUSED COMMIT-CREDIT CARD LINES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCRCD")]
    pub unused_commit_credit_card_lines: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCCRCDR`
    Title: UNUSED COMMIT-CREDIT CARD LINES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCCRCDR")]
    pub unused_commit_credit_card_lines_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCLN`
    Title: UNUSED COMMIT-TOTAL LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCLN")]
    pub unused_commit_total_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCLOC`
    Title: UNUSED COMMIT-HOME EQUITY LINES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCLOC")]
    pub unused_commit_home_equity_lines: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCLOCR`
    Title: UNUSED COMMIT-HOME EQUITY LINES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCLOCR")]
    pub unused_commit_home_equity_lines_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCOTHER`
    Title: UNUSED COMMIT-ALL OTHER
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCOTHER")]
    pub unused_commit_all_other: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCOTHERR`
    Title: UNUSED COMMIT-ALL OTHER RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCOTHERR")]
    pub unused_commit_all_other_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCOVER1`
    Title: UNUSED COM-OVER 1 YR-RC-R COL A
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCOVER1")]
    pub unused_com_over_1_yr_rc_r_col_a: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCOVER1R`
    Title: UNUSED COM-OVER 1 YR-RC-R COL A RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCOVER1R")]
    pub unused_com_over_1_yr_rc_r_col_a_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSC`
    Title: UNUSED COMMIT-SEC UNDERWRITING
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSC")]
    pub unused_commit_sec_underwriting: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSCR`
    Title: UNUSED COMMIT-SEC UNDERWRITING RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSCR")]
    pub unused_commit_sec_underwriting_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSZAUTO`
    Title: UNUSED COMMIT FOR SECUR. - AUTO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSZAUTO")]
    pub unused_commit_for_secur_auto: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSZCI`
    Title: UNUSED COMMIT FOR SECUR. - CI
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSZCI")]
    pub unused_commit_for_secur_ci: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSZCON`
    Title: UNUSED COMMIT FOR SECUR. - CON
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSZCON")]
    pub unused_commit_for_secur_con: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSZCRCD`
    Title: UNUSED COMMIT FOR SECUR. - CRCD
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSZCRCD")]
    pub unused_commit_for_secur_crcd: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSZHEL`
    Title: UNUSED COMMIT FOR SECUR. - HEL
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSZHEL")]
    pub unused_commit_for_secur_hel: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSZOTH`
    Title: UNUSED COMMIT FOR SECUR. - OTH
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSZOTH")]
    pub unused_commit_for_secur_oth: Option<f64>,

    #[doc = r#"## FDIC Field:: `UCSZRES`
    Title: UNUSED COMMIT FOR SECUR. - RES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UCSZRES")]
    pub unused_commit_for_secur_res: Option<f64>,

    #[doc = r#"## FDIC Field:: `UNINCFOR`
    Title: UNEARNED INCOME-FOR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UNINCFOR")]
    pub unearned_income_for: Option<f64>,

    #[doc = r#"## FDIC Field:: `UNINCFORR`
    Title: UNEARNED INCOME-FOR RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UNINCFORR")]
    pub unearned_income_for_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `VOLIAB`
    Title: VOLATILE LIABILITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="VOLIAB")]
    pub volatile_liabilities: Option<f64>,

    #[doc = r#"## FDIC Field:: `VOLIABR`
    Title: VOLATILE LIABILITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="VOLIABR")]
    pub volatile_liabilities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ZIP`
    Title: ZIP CODE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ZIP")]
    pub zip_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `LIPNMTG`
    Title: NONMORTGAGE LOANS IN PROCESS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LIPNMTG")]
    pub nonmortgage_loans_in_process: Option<f64>,

    #[doc = r#"## FDIC Field:: `UYANMTG`
    Title: UNAMORTIZED YIELD ADJ-NONMTG LNS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UYANMTG")]
    pub unamortized_yield_adj_nonmtg_lns: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNLS`
    Title: LOAN & LEASE INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNLS")]
    pub loan_lease_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `UNIT`
    Title: BANKS UNIT
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UNIT")]
    pub banks_unit: Option<f64>,

    #[doc = r#"## FDIC Field:: `PTAXNETINC`
    Title: PRE-TAX NET INCOME OPERATING INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PTAXNETINC")]
    pub pre_tax_net_income_operating_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `PTAXNETINCR`
    Title: PRE-TAX NET INCOME OPERATING INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PTAXNETINCR")]
    pub pre_tax_net_income_operating_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `PTAXNETINCQ`
    Title: PRE-TAX NET INCOME OPERATING INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PTAXNETINCQ")]
    pub pre_tax_net_income_operating_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `PTAXNETINCQR`
    Title: PRE-TAX NET INCOME OPERATING INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PTAXNETINCQR")]
    pub pre_tax_net_income_operating_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONII`
    Title: ADDITIONAL NONINTEREST INCOME
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONII")]
    pub additional_noninterest_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONIIR`
    Title: ADDITIONAL NONINTEREST INCOME RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONIIR")]
    pub additional_noninterest_income_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONIIQ`
    Title: ADDITIONAL NONINTEREST INCOME QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONIIQ")]
    pub additional_noninterest_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `ADDNONIIQR`
    Title: ADDITIONAL NONINTEREST INCOME QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ADDNONIIQR")]
    pub additional_noninterest_income_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `AVMMLF`
    Title: Quarterly average amount of assets purchased under the MMLF and excluded from “Total assets for the leverage ratio.”
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AVMMLF")]
    pub quarterly_average_amount_of_assets_purchased_under_the_mmlf_and_excluded_from_total_assets_for_the_leverage_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `AVMMLFR`
    Title: Quarterly average amount of assets purchased under the MMLF and excluded from “Total assets for the leverage ratio.” ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AVMMLFR")]
    pub quarterly_average_amount_of_assets_purchased_under_the_mmlf_and_excluded_from_total_assets_for_the_leverage_ratio_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `AVPPPPLG`
    Title: Quarterly average amount of PPP loans pledged to the PPPLF and excluded from “Total assets for the leverage ratio.”
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AVPPPPLG")]
    pub quarterly_average_amount_of_ppp_loans_pledged_to_the_ppplf_and_excluded_from_total_assets_for_the_leverage_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `AVPPPPLGR`
    Title: Quarterly average amount of PPP loans pledged to the PPPLF and excluded from “Total assets for the leverage ratio.” ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AVPPPPLGR")]
    pub quarterly_average_amount_of_ppp_loans_pledged_to_the_ppplf_and_excluded_from_total_assets_for_the_leverage_ratio_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `MMLFBAL`
    Title: Outstanding balance of assets purchased under the Money Market Mutual Fund Liquidity Facility (MMLF).
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MMLFBAL")]
    pub outstanding_balance_of_assets_purchased_under_the_money_market_mutual_fund_liquidity_facility_mmlf: Option<f64>,

    #[doc = r#"## FDIC Field:: `MMLFBALR`
    Title: Outstanding balance of assets purchased under the Money Market Mutual Fund Liquidity Facility (MMLF) ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MMLFBALR")]
    pub outstanding_balance_of_assets_purchased_under_the_money_market_mutual_fund_liquidity_facility_mmlf_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLFOV1`
    Title: Outstanding balance under the PPPLF with a remaining maturity of more than one year
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLFOV1")]
    pub outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_more_than_one_year: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLFOV1R`
    Title: Outstanding balance under the PPPLF with a remaining maturity of more than one year ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLFOV1R")]
    pub outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_more_than_one_year_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLNBAL`
    Title: Outstanding balance of PPP loans
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLNBAL")]
    pub outstanding_balance_of_ppp_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLNBALR`
    Title: Outstanding balance of PPP loans ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLNBALR")]
    pub outstanding_balance_of_ppp_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLNNUM`
    Title: Number of PPP loans outstanding
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLNNUM")]
    pub number_of_ppp_loans_outstanding: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLNNUMR`
    Title: Number of PPP loans outstanding ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLNNUMR")]
    pub number_of_ppp_loans_outstanding_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLNPLG`
    Title: Outstanding balance of PPP loans pledged to the PPPLF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLNPLG")]
    pub outstanding_balance_of_ppp_loans_pledged_to_the_ppplf: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLNPLGR`
    Title: Outstanding balance of PPP loans pledged to the PPPLF ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLNPLGR")]
    pub outstanding_balance_of_ppp_loans_pledged_to_the_ppplf_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLF1LS`
    Title: Outstanding balance under the PPPLF with a remaining maturity of one year or less
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLF1LS")]
    pub outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_one_year_or_less: Option<f64>,

    #[doc = r#"## FDIC Field:: `PPPLF1LSR`
    Title: Outstanding balance under the PPPLF with a remaining maturity of one year or less ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="PPPLF1LSR")]
    pub outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_one_year_or_less_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCIR`
    Title: COMMERCIAL & INDUSTRIAL LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCIR")]
    pub commercial_industrial_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCIQR`
    Title: COMMERCIAL & INDUSTRIAL LOANS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCIQR")]
    pub commercial_industrial_loans_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCONR`
    Title: LOANS TO INDIVIDUALS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCONR")]
    pub loans_to_individuals: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCRDR`
    Title: CREDIT CARDS & RELATED PLANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCRDR")]
    pub credit_cards_related_plans: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCOOR`
    Title: OTHER LOANS TO INDIVIDUALS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCOOR")]
    pub other_loans_to_individuals: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCOOQR`
    Title: OTHER LOANS TO INDIVIDUALS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCOOQR")]
    pub other_loans_to_individuals_idntcooqr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCRDQR`
    Title: CREDIT CARDS & RELATED PLANS QUARTERLY
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCRDQR")]
    pub credit_cards_related_plans_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSTCNT`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSTCNT")]
    pub missing_title_instcnt: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTILR`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTILR")]
    pub missing_title_idntilr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDOTHNII`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDOTHNII")]
    pub missing_title_idothnii: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAUTOPR`
    Title: AUTOMOBILE LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAUTOPR")]
    pub automobile_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONOTR`
    Title: OTHER CONSUMER LOANS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONOTR")]
    pub other_consumer_loans: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDERNCVR`
    Title: EARNINGS COVERAGE OF NET LOAN CHARGE-OFFS (X)
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDERNCVR")]
    pub earnings_coverage_of_net_loan_charge_offs_x: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDERNCVQR`
    Title: Earnings coverage of net loan charge-offs
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDERNCVQR")]
    pub earnings_coverage_of_net_loan_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQCDIVNTINC`
    Title: CASH DIVIDENDS TO NET INCOME (YTD ONLY)
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQCDIVNTINC")]
    pub cash_dividends_to_net_income_ytd_only: Option<f64>,

    #[doc = r#"## FDIC Field:: `NACDIR`
    Title: NOTIONAL AMOUNT OF CREDIT DERIVATIVES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NACDIR")]
    pub notional_amount_of_credit_derivatives: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCOMREQR`
    Title: COMMERCIAL RE CHG-OFF/COMM RE LN QUARTERLY RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCOMREQR")]
    pub commercial_re_chg_off_comm_re_ln_quarterly_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTALLOTHNUM`
    Title: Net Charge-offs All other loans & leases (including farm) Numerator
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTALLOTHNUM")]
    pub net_charge_offs_all_other_loans_leases_including_farm_numerator: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTALLOTHDEN`
    Title: Net Charge-offs All other loans & leases (including farm) denominator
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTALLOTHDEN")]
    pub net_charge_offs_all_other_loans_leases_including_farm_denominator: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTALLOTHR`
    Title: ALL OTHER LOANS & LEASES (INCLUDING FARM)
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTALLOTHR")]
    pub all_other_loans_leases_including_farm: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTALLOTHQR`
    Title: Net Charge-offs All other loans & leases (including farm)
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTALLOTHQR")]
    pub net_charge_offs_all_other_loans_leases_including_farm: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCCOOR`
    Title: Other loans to individuals
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCCOOR")]
    pub other_loans_to_individuals_idnccoor: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCOTHR`
    Title: All other loans & leases (including farm )
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCOTHR")]
    pub all_other_loans_leases_including_farm_idncothr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCCIR`
    Title: COMMERCIAL & INDUSTRIAL LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCCIR")]
    pub commercial_industrial_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCCONR`
    Title: LOANS TO INDIVIDUALS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCCONR")]
    pub loans_to_individuals_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCCRDR`
    Title: CREDIT CARDS & RELATED PLANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCCRDR")]
    pub credit_cards_related_plans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCATOR`
    Title: AUTOMOBILE LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCATOR")]
    pub automobile_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTATOR`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTATOR")]
    pub missing_title_idntator: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNTCOTR`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNTCOTR")]
    pub missing_title_idntcotr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDDEPINR`
    Title: IDDEPINR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDDEPINR")]
    pub iddepinr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDDIVNIR`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDDIVNIR")]
    pub missing_title_iddivnir: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCCOTR`
    Title: OTHER CONSUMER LOANS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCCOTR")]
    pub other_consumer_loans_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `INTINCY`
    Title: INTEREST INCOME TO EARNING ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INTINCY")]
    pub interest_income_to_earning_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDNCGTPR`
    Title: NONCURRENT LOANS WHICH ARE WHOLLY OR PARTIALLY GUARANTEED BY THE U.S. GOVERNMENT RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDNCGTPR")]
    pub noncurrent_loans_which_are_wholly_or_partially_guaranteed_by_the_u_s_government_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDLNCORR`
    Title: NET LOANS AND LEASES TO CORE DEPOSITS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDLNCORR")]
    pub net_loans_and_leases_to_core_deposits_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDT1CNOCB`
    Title: ID NO CB FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDT1CNOCB")]
    pub id_no_cb_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDT1JNOCB`
    Title: ID NO J CB FLAG
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDT1JNOCB")]
    pub id_no_j_cb_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDT1CER`
    Title: COMMON EQUITY TIER 1 CAPITAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDT1CER")]
    pub common_equity_tier_1_capital_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `IDT1RWAJR`
    Title: TIER 1 RISK-BASED CAPITAL RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IDT1RWAJR")]
    pub tier_1_risk_based_capital_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCEQNFT`
    Title: EQUITY SECURITIES NOT HELD FOR TRADING
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCEQNFT")]
    pub equity_securities_not_held_for_trading: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCRMBPI`
    Title: PRIV ISSUED RES MORTGAGE-BACKED SECURITIES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCRMBPI")]
    pub priv_issued_res_mortgage_backed_securities: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCRMBPIR`
    Title: PRIV ISSUED RES MORTGAGE-BACKED SECURITIES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCRMBPIR")]
    pub priv_issued_res_mortgage_backed_securities_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCUSO`
    Title: U.S GOVERNMENT OBLIGATIONS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCUSO")]
    pub u_s_government_obligations: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCUSOR`
    Title: U.S GOVERNMENT OBLIGATIONS RATIOS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCUSOR")]
    pub u_s_government_obligations_ratios: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMOS`
    Title: OTHER COMM MORTGAGE-BACKED SEC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMOS")]
    pub other_comm_mortgage_backed_sec: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCCMOSR`
    Title: OTHER COMM MORTGAGE-BACKED SEC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCCMOSR")]
    pub other_comm_mortgage_backed_sec_sccmosr: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCTATFR`
    Title: ASSETS HELD IN TRADING ACCOUNTS FOR TFR REPORTERS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCTATFR")]
    pub assets_held_in_trading_accounts_for_tfr_reporters: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGRS`
    Title: LOANS AND LEASES, GROSS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGRS")]
    pub loans_and_leases_gross: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSGRSR`
    Title: LOANS AND LEASES, GROSS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSGRSR")]
    pub loans_and_leases_gross_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `AOA`
    Title: ALL OTH ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AOA")]
    pub all_oth_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `AOAR`
    Title: ALL OTH ASSETS RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AOAR")]
    pub all_oth_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESTINS`
    Title: PERCENTAGE INSURED ESTIMATED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESTINS")]
    pub percentage_insured_estimated: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESTINSR`
    Title: PERCENTAGE INSURED ESTIMATED RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESTINSR")]
    pub percentage_insured_estimated_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RELNDO`
    Title: P/D 30-89 REAL ESTATE LOANS IN DOMESTIC OFFICES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RELNDO")]
    pub p_d_30_89_real_estate_loans_in_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RELNDOR`
    Title: P/D 30-89 REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RELNDOR")]
    pub p_d_30_89_real_estate_loans_in_domestic_offices_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RELNDO`
    Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RELNDO")]
    pub _90_real_estate_loans_in_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `P9RELNDOR`
    Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P9RELNDOR")]
    pub _90_real_estate_loans_in_domestic_offices_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARELNDO`
    Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARELNDO")]
    pub _90_real_estate_loans_in_domestic_offices_narelndo: Option<f64>,

    #[doc = r#"## FDIC Field:: `NARELNDOR`
    Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NARELNDOR")]
    pub _90_real_estate_loans_in_domestic_offices_ratio_narelndor: Option<f64>,

    #[doc = r#"## FDIC Field:: `STCNTY`
    Title: State and County Nunber
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="STCNTY")]
    pub state_and_county_nunber: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA`
    Title: Metropolitan Statistical Area
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CBSA")]
    pub metropolitan_statistical_area: Option<String>,

    #[doc = r#"## FDIC Field:: `INSDATE`
    Title: Date of Deposit Insurance
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSDATE")]
    pub date_of_deposit_insurance: Option<String>,

    #[doc = r#"## FDIC Field:: `UPDDATE`
    Title: Last Structure Change Process Date
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="UPDDATE")]
    pub last_structure_change_process_date: Option<String>,

    #[doc = r#"## FDIC Field:: `ASSETR`
    Title: Total Assets Ratio
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ASSETR")]
    pub total_assets_ratio: Option<f64>,

    #[doc = r#"## FDIC Field:: `AVASSET`
    Title: AVG TOTAL ASSETS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="AVASSET")]
    pub avg_total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `BROINSLG`
    Title: BROKERED DEP-INSURED-LARGE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BROINSLG")]
    pub brokered_dep_insured_large: Option<f64>,

    #[doc = r#"## FDIC Field:: `CT1AJTOT`
    Title: RC-R TOTAL ADJ & DED COM EQ T1
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CT1AJTOT")]
    pub rc_r_total_adj_ded_com_eq_t1: Option<f64>,

    #[doc = r#"## FDIC Field:: `CT1BADJ`
    Title: RC-R COM EQUITY T1 BEFORE ADJ
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CT1BADJ")]
    pub rc_r_com_equity_t1_before_adj: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEP2`
    Title: TOTAL DEPOSITS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEP2")]
    pub total_deposits_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEP5`
    Title: TOTAL DEPOSITS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEP5")]
    pub total_deposits_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPIY1`
    Title: INTEREST-BEARING-DEP-Y1
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DEPIY1")]
    pub interest_bearing_dep_y1: Option<f64>,

    #[doc = r#"## FDIC Field:: `ECD100`
    Title: INT EXPENSE TIME CD GT $250
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ECD100")]
    pub int_expense_time_cd_gt_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `ECD100A`
    Title: INT EXP TIME CD GT $250
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ECD100A")]
    pub int_exp_time_cd_gt_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `ECD100Q`
    Title: INT EXP TIME CD GT $250
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ECD100Q")]
    pub int_exp_time_cd_gt_250_ecd100q: Option<f64>,

    #[doc = r#"## FDIC Field:: `EFREPPA`
    Title: FED FUNDS & REPO INT EXPENSE-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EFREPPA")]
    pub fed_funds_repo_int_expense_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHTIMA`
    Title: INT EXP TIME CD LE $250
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHTIMA")]
    pub int_exp_time_cd_le_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHTIME`
    Title: INT EXPENSE TIME CD LE $250
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHTIME")]
    pub int_expense_time_cd_le_250: Option<f64>,

    #[doc = r#"## FDIC Field:: `EOTHTIMQ`
    Title: INT EXP TIME CD LE $250
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EOTHTIMQ")]
    pub int_exp_time_cd_le_250_eothtimq: Option<f64>,

    #[doc = r#"## FDIC Field:: `EQUPGR`
    Title: UNDIVIDED PROFITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="EQUPGR")]
    pub undivided_profits: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESAVDP`
    Title: NONTRANSACTION SAV ACCTS INT EXP
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESAVDP")]
    pub nontransaction_sav_accts_int_exp: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESAVDPA`
    Title: NONTRANSACT SAV ACCT INT EXT-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESAVDPA")]
    pub nontransact_sav_acct_int_ext_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESAVDPQ`
    Title: NONTRANSACT SAV ACCT INT EXP-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESAVDPQ")]
    pub nontransact_sav_acct_int_exp_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESUBNDA`
    Title: SUBORDINATED NOTES INT EXP-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ESUBNDA")]
    pub subordinated_notes_int_exp_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ETRANDEP`
    Title: TRANSACTION ACCOUNTS INT EXPENSE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ETRANDEP")]
    pub transaction_accounts_int_expense: Option<f64>,

    #[doc = r#"## FDIC Field:: `ETRANDPA`
    Title: TRANSACTION ACCOUNTS INT EXP-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ETRANDPA")]
    pub transaction_accounts_int_exp_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ETRANDPQ`
    Title: TRANSACTION ACCOUNTS INT EXP-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ETRANDPQ")]
    pub transaction_accounts_int_exp_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `ETTLOTBA`
    Title: TT&L & OTHER BORROW INT EXP-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ETTLOTBA")]
    pub tt_l_other_borrow_int_exp_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ETTLOTBQ`
    Title: TT&L & OTHER BORROW INT EXP-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ETTLOTBQ")]
    pub tt_l_other_borrow_int_exp_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `FFPUR`
    Title: FEDERAL FUNDS PURCHASED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FFPUR")]
    pub federal_funds_purchased: Option<f64>,

    #[doc = r#"## FDIC Field:: `IBEFTXA`
    Title: INC BEFORE INC TAXS & DISC-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IBEFTXA")]
    pub inc_before_inc_taxs_disc_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLSCA`
    Title: AVAILABLE-FOR-SALE SECS G/L
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLSCA")]
    pub available_for_sale_secs_g_l: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLSCAQ`
    Title: AVAILABLE-FOR-SALE SEC G/L-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLSCAQ")]
    pub available_for_sale_sec_g_l_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `IGLSCH`
    Title: HELD-TO-MATURITY SECS G/L
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="IGLSCH")]
    pub held_to_maturity_secs_g_l: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNA`
    Title: LOAN INCOME-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNA")]
    pub loan_income_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNLSA`
    Title: LOAN & LEASE INCOME-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNLSA")]
    pub loan_lease_income_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNLSQ`
    Title: LOAN & LEASE INCOME-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNLSQ")]
    pub loan_lease_income_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNLSXA`
    Title: TAX-EXEMPT LN & LS INT INC-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNLSXA")]
    pub tax_exempt_ln_ls_int_inc_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNLSXQ`
    Title: TAX-EXEMPT LN & LS INT INC-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNLSXQ")]
    pub tax_exempt_ln_ls_int_inc_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNMUNIQ`
    Title: MUNICIPAL LOAN INCOME-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNMUNIQ")]
    pub municipal_loan_income_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `ILNQ`
    Title: LOAN INCOME-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ILNQ")]
    pub loan_income_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISCA`
    Title: TOTAL SECURITY INCOME-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISCA")]
    pub total_security_income_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ISERCHGA`
    Title: SERVICE CHARGE ON DEP ACCTS-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ISERCHGA")]
    pub service_charge_on_dep_accts_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITAXA`
    Title: APPLICABLE INCOME TAXES-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITAXA")]
    pub applicable_income_taxes_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `ITAXQA`
    Title: APPLICABLE INCOME TAXES-QTR-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ITAXQA")]
    pub applicable_income_taxes_qtr_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCDT1R`
    Title: CONSTR & LAND DEV LNS/TIER 1
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCDT1R")]
    pub constr_land_dev_lns_tier_1: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCIT1R`
    Title: C&I LOANS/TIER 1
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCIT1R")]
    pub c_i_loans_tier_1: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNCONT1R`
    Title: CONSUMER LOANS/TIER 1
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNCONT1R")]
    pub consumer_loans_tier_1: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNLSRES`
    Title: ALLOWANCE FOR LOAN AND LEASES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNLSRES")]
    pub allowance_for_loan_and_leases: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNREAG5`
    Title: RE AGRICULTURAL-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNREAG5")]
    pub re_agricultural_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `LNRERT1R`
    Title: RE LOANS/TIER 1
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="LNRERT1R")]
    pub re_loans_tier_1: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCREAG`
    Title: TOTAL N/C-RE*FARMLAND
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCREAG")]
    pub total_n_c_re_farmland: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRECNFM`
    Title: N/C 1-4 FAMILY CONSTRUCTION LOAN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRECNFM")]
    pub n_c_1_4_family_construction_loan: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRECNOT`
    Title: N/C OTHER CONSTRUCT & LAND DEV
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRECNOT")]
    pub n_c_other_construct_land_dev: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRENROT`
    Title: N/C OTHER NONFARM NONRES RE LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRENROT")]
    pub n_c_other_nonfarm_nonres_re_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRENROW`
    Title: N/C OWN-OCCUPIED NONFARM NONRES
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRENROW")]
    pub n_c_own_occupied_nonfarm_nonres: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRERS2R`
    Title: N/C 1-4 FAM JR LN/1-4 FAM JR LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRERS2R")]
    pub n_c_1_4_fam_jr_ln_1_4_fam_jr_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRERSF2`
    Title: N/C RE 1-4 FAM JUNIOR LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRERSF2")]
    pub n_c_re_1_4_fam_junior_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRERSFM`
    Title: N/C RE 1-4 FAM FIRST LIEN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRERSFM")]
    pub n_c_re_1_4_fam_first_lien: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRERSFR`
    Title: N/C 1-4 FAM 1STLN/1-4 FAM IST LN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRERSFR")]
    pub n_c_1_4_fam_1stln_1_4_fam_ist_ln: Option<f64>,

    #[doc = r#"## FDIC Field:: `NCRSLNLS`
    Title: NC RESTRUCT LOANS EXCL 1-4 FM
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NCRSLNLS")]
    pub nc_restruct_loans_excl_1_4_fm: Option<f64>,

    #[doc = r#"## FDIC Field:: `NOIQ`
    Title: NET OPERATING INCOME-QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NOIQ")]
    pub net_operating_income_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGQA`
    Title: AG LOAN NET CHARGE-OFFS-QTR-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGQA")]
    pub ag_loan_net_charge_offs_qtr_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGSMA`
    Title: AG LN NET CHARGE-OFFS ANN*SM BKS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGSMA")]
    pub ag_ln_net_charge_offs_ann_sm_bks: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTAGSMQA`
    Title: AG LOAN NET-CHG-QTR-ANN*SMALL BK
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTAGSMQA")]
    pub ag_loan_net_chg_qtr_ann_small_bk: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCIQA`
    Title: COMMERCIAL LOAN NET-CHG-QTR-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCIQA")]
    pub commercial_loan_net_chg_qtr_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCOMRE`
    Title: COMMERCIAL RE LN NET CHARGE-OFFS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCOMRE")]
    pub commercial_re_ln_net_charge_offs: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCOMRQA`
    Title: COMML RE NET-CHARGE-OFF-QTR-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCOMRQA")]
    pub comml_re_net_charge_off_qtr_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCONQA`
    Title: CONSUMER LN NET-CHG-QTR-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCONQA")]
    pub consumer_ln_net_chg_qtr_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTCRCDQA`
    Title: CREDIT CARD LN NET-CHG-QTR-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTCRCDQA")]
    pub credit_card_ln_net_chg_qtr_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTIRTQ`
    Title: RETAINED EARNINGS- BANK- QTR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTIRTQ")]
    pub retained_earnings_bank_qtr: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTRCDSMJ`
    Title: Time Deposits Less Than Or Equal To insurance Limit
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTRCDSMJ")]
    pub time_deposits_less_than_or_equal_to_insurance_limit: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREAGA`
    Title: FARMLAND RE LN NET-CHG-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREAGA")]
    pub farmland_re_ln_net_chg_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `NTREAGQA`
    Title: FARM RE LN NET CHRG-OFF-QTR-ANN
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="NTREAGQA")]
    pub farm_re_ln_net_chrg_off_qtr_ann: Option<f64>,

    #[doc = r#"## FDIC Field:: `OBOR`
    Title: OTHER BORROWED FUNDS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OBOR")]
    pub other_borrowed_funds: Option<f64>,

    #[doc = r#"## FDIC Field:: `OBOR2`
    Title: OTHER BORROWED FUNDS-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OBOR2")]
    pub other_borrowed_funds_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `OBOR5`
    Title: OTHER BORROWED FUNDS-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OBOR5")]
    pub other_borrowed_funds_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBFH03`
    Title: OTH BOR FHLB-OVER 3 YRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBFH03")]
    pub oth_bor_fhlb_over_3_yrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTHBFH13`
    Title: OTH BOR. FHLB-1 TO 3 YRS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTHBFH13")]
    pub oth_bor_fhlb_1_to_3_yrs: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3COMRE`
    Title: 30-89 DAYS P/D-COMMERCIAL RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3COMRE")]
    pub _30_89_days_p_d_commercial_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RECONR`
    Title: 30-89 PAST DUE CONST RE/CONST RE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RECONR")]
    pub _30_89_past_due_const_re_const_re: Option<f64>,

    #[doc = r#"## FDIC Field:: `P3RERS2R`
    Title: 30-89 P/D 1-4FAM JR/1-4 FAM JR
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="P3RERS2R")]
    pub _30_89_p_d_1_4fam_jr_1_4_fam_jr: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCEQUP`
    Title: RETAINED EARNINGS - RBC
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCEQUP")]
    pub retained_earnings_rbc: Option<f64>,

    #[doc = r#"## FDIC Field:: `RBCT1W`
    Title: TIER 1 CAPITAL - REPORTED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RBCT1W")]
    pub tier_1_capital_reported: Option<f64>,

    #[doc = r#"## FDIC Field:: `REPOPUR`
    Title: REPURCHASE AGREEMENTS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="REPOPUR")]
    pub repurchase_agreements: Option<f64>,

    #[doc = r#"## FDIC Field:: `SC2`
    Title: SECURITIES-CAVG2
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SC2")]
    pub securities_cavg2: Option<f64>,

    #[doc = r#"## FDIC Field:: `SC5`
    Title: SECURITIES-CAVG5
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SC5")]
    pub securities_cavg5: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMUNIAA`
    Title: MUNICIPAL SECURITIES -AA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMUNIAA")]
    pub municipal_securities_aa: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMUNIAF`
    Title: MUNICIPAL SECURITIES -AF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMUNIAF")]
    pub municipal_securities_af: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMUNIHA`
    Title: MUNICIPAL SECURITIES -HA
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMUNIHA")]
    pub municipal_securities_ha: Option<f64>,

    #[doc = r#"## FDIC Field:: `SCMUNIHF`
    Title: MUNICIPAL SECURITIES -HF
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SCMUNIHF")]
    pub municipal_securities_hf: Option<f64>,

}

/// FDIC BankFind API `/financials` endpoint handler
/// Get Financial Information for FDIC Insured Institutions
/// Returns financial information for financial institutions
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter criteria that refines the records included in the result. All values must be entered in UPPERCASE.
Examples:  
* Filter data by the numeric range  
`ASSET:&#91;1000 TO 9999&#93;`
    Example: CERT:14
 - `fields` (String, optional): Comma delimited list of fields with quarterly financial data to return. All values must be entered in UPPERCASE.
    Example: CERT,REPDTE,ASSET,DEP
 - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
    Example: REPDTE
 - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
    Example: DESC
 - `limit` (u32, optional): The number of records to return. Default is 10 and maximum is 10,000. However, if the fields request is for more than 250 fields (variables), the maximum limit is 500 to ensure the request is successful.
    Example: 10
 - `offset` (u32, optional): The offset of page to return.
 - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE.
    Example: CERT
 - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE.
    Example: REPDTE
 - `agg_sum_fields` (String, optional): The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE.
    Example: ASSET
 - `agg_limit` (u32, optional): The limit on how many aggregated results will be displayed
    Example: 1
 - `format` (String, optional): The format of the data to return.
    Example: json
 - `download` (bool, optional): Whether the data should be downloaded as a file.
 - `filename` (String, optional): The filename to use when downloading data.
    Example: data_file
"#]
#[utoipa::path(
    get,
    path = "/financials",
    params(FinancialsParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<FinancialsProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "Financials"
)]
pub async fn financials_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<FinancialsParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "financials",
    )
    .await
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
            
            activity_event_code: None,
            total_assets: None,
            branching: None,
            call_form_number: None,
            community_bank: None,
            core_based_statistical_division_number: None,
            core_based_statistical_division_name: None,
            multi_state_offices_flag: None,
            address: None,
            community_bank_ratio: None,
            time_dep_250_000_or_more_remaining_maturity_repricing_of_1_3_years: None,
            time_dep_250_000_or_more_remaining_maturity_repricing_of_1_3_years_ratio: None,
            time_dep_250_000_or_more_remaining_maturity_repricing_of_3_month_or_less: None,
            time_dep_250_000_or_more_remaining_maturity_repricing_of_3_month_or_less_ratio: None,
            time_dep_250_000_or_less_remaining_maturity_repricing_of_3_month_or_less: None,
            time_dep_250_000_or_less_remaining_maturity_repricing_of_3_month_or_less_ratio: None,
            time_dep_250_000_or_more_remaining_maturity_or_repricing_over_3_years: None,
            time_dep_250_000_or_more_remaining_maturity_or_repricing_over_3_years_ratio: None,
            time_dep_250_000_or_less_remaining_maturity_or_repricing_over_3_years: None,
            time_dep_250_000_or_less_remaining_maturity_or_repricing_over_3_years_ratio: None,
            time_dep_250_000_or_more_remaining_maturity_or_repricing_3_12_months: None,
            time_dep_250_000_or_more_remaining_maturity_or_repricing_3_12_months_ratio: None,
            time_dep_250_000_or_less_remaining_maturity_or_repricing_3_12_months: None,
            time_dep_250_000_or_less_remaining_maturity_or_repricing_3_12_months_ratio: None,
            time_dep_250_000_or_less_remaining_maturity_or_repricing_1_3_years: None,
            time_dep_250_000_or_less_remaining_maturity_or_repricing_1_3_years_ratio: None,
            fdic_certificate: None,
            directly_owned_by_another_bank_cert: None,
            city_of_high_holder: None,
            classcode: None,
            closed_institution_flag: None,
            fips_cmsa_code: None,
            fips_country_code: None,
            fips_country_number: None,
            fips_county_number: None,
            combined_statistical_area: None,
            denovo_institution: None,
            total_deposits: None,
            total_deposits_ratio: None,
            deposits_held_in_domestic_offices: None,
            deposits_held_in_dom_off_ratio: None,
            division_flag: None,
            docket_number: None,
            international_activity_flag: None,
            entity_type: None,
            equity_capital: None,
            equity_capital_eq2: None,
            equity_capital_ratio: None,
            failed_institution_flag: None,
            fdic_compliance_area: None,
            fdic_compliance_territory: None,
            dca_field_office: None,
            ffiec_call_report_31_filer: None,
            bank_holding_company_type: None,
            bank_not_member_of_hold_company: None,
            secondary_insurer: None,
            tbd: None,
            deposit_insurance_fund_member: None,
            agricultural_lending_institution_indicator: None,
            credit_card_institutions: None,
            saif_insured: None,
            minority_owned_institutions: None,
            ownership_type: None,
            bank_holding_company_regulatory_top_holder: None,
            net_income: None,
            net_income_ratio: None,
            net_income_quarterly: None,
            net_income_quarterly_netincqa: None,
            net_income_quarterly_ratio: None,
            number_of_domestic_offices: None,
            number_of_foreign_offices: None,
            number_of_us_offices: None,
            directly_owned_by_another_bank_cert_parcert: None,
            report_date: None,
            report_date_repdte_raw: None,
            report_date_repdte: None,
            report_year: None,
            report_date_risdate: None,
            return_on_assets_roa: None,
            pretax_return_on_assets: None,
            quarterly_pretax_return_on_assets: None,
            quarterly_return_on_assets: None,
            return_on_equity_roe: None,
            quarterly_return_on_equity: None,
            rssdid_high_regulatory_holder: None,
            asset_concentration_hierarchy: None,
            asset_concentration_hierarchy_description: None,
            regulatory_holding_company_state_location: None,
            subchapter_s_corporations: None,
            tract: None,
            trust_powers: None,
            banks_liability_on_acceptances: None,
            active_institution_flag: None,
            institution_class: None,
            premises_and_fixed_assets: None,
            premises_and_fixed_assets_ratio: None,
            brokered_dep: None,
            brokered_ratio: None,
            report_date_ccyymm: None,
            cash_due_from_depository_inst: None,
            cash_due_from_depository_inst_ratio: None,
            interest_bearing_cash_due: None,
            interest_bearing_cash_due_ratio: None,
            charter_agent: None,
            rtc_conservatorship_flag: None,
            total_ln_ls_recoveries: None,
            total_ln_ls_recoveries_ratio: None,
            total_ln_ls_recoveries_quarterly: None,
            total_ln_ls_recoveries_quarterly_ratio: None,
            customers_acceptances: None,
            dda_trans_total: None,
            dda_trans_total_ratio: None,
            total_deposits_for: None,
            total_deposits_for_ratio: None,
            interest_bearing_dep: None,
            interest_bearing_dep_for: None,
            interest_bearing_dep_for_ratio: None,
            ipc_official_checks_for: None,
            ipc_official_checks_for_ratio: None,
            ipc_for: None,
            noninterest_bearing_dep: None,
            noninterest_bearing_dep_for: None,
            noninterest_bearing_dep_for_ratio: None,
            total_ln_ls_charge_offs: None,
            total_ln_ls_charge_offs_ratio: None,
            total_ln_ls_charge_offs_quarterly: None,
            total_ln_ls_charge_offs_quarterly_ratio: None,
            amort_impair_loss_ast: None,
            amort_impair_loss_ast_ratio: None,
            amort_impair_loss_ast_quarterly: None,
            amort_impair_loss_ast_quarterly_ratio: None,
            deposit_interest_expense: None,
            deposit_interest_expense_dom: None,
            deposit_interest_expense_dom_ratio: None,
            deposit_interest_expense_dom_quarterly: None,
            deposit_interest_expense_dom_quarterly_ratio: None,
            deposit_interest_expense_for: None,
            deposit_interest_expense_for_ratio: None,
            deposit_interest_expense_for_quarterly: None,
            deposit_interest_expense_for_quarterly_ratio: None,
            advances_from_fhlbank_int_exp: None,
            fed_funds_repos_int_expense: None,
            fed_funds_repos_int_expense_ratio: None,
            fed_funds_repos_int_expense_quarterly: None,
            fed_funds_repos_int_expense_quarterly_ratio: None,
            total_interest_expense: None,
            total_interest_expense_ratio: None,
            total_interest_expense_quarterly: None,
            total_interest_expense_quarterly_eintxqa: None,
            total_interest_expense_annually: None,
            total_interest_expense_quarterly_ratio: None,
            provisions_for_credit_losses: None,
            provisions_for_credit_losses_ratio: None,
            provisions_for_credit_losses_quarterly: None,
            provisions_for_credit_losses_quarterly_elnatqa: None,
            provisions_for_credit_losses_quarterly_ratio: None,
            provisions_for_credit_losses_quarterly_ratio_elnlosq: None,
            provisions_for_credit_losses_quarterly_ratio_nttotq: None,
            provisions_for_ln_lease_losses: None,
            mortgage_debt_interest_expense: None,
            additional_noninterest_expense: None,
            additional_noninterest_expense_ratio: None,
            additional_noninterest_expense_quarterly: None,
            additional_noninterest_expense_quarterly_ratio: None,
            all_other_noninterest_expense: None,
            all_other_noninterest_expense_ratio: None,
            all_other_noninterest_expense_quarterly: None,
            all_other_noninterest_expense_quarterly_ratio: None,
            premises_fixed_assets_expense: None,
            premises_equipment_expense_ratio: None,
            premises_fixed_assets_expense_quarterly: None,
            premises_equipment_expense_quarterly_ratio: None,
            cash_dividends_on_comm_pref: None,
            cash_dividends_on_comm_pref_ratio: None,
            cash_dividends_on_comm_stock: None,
            cash_dividends_on_comm_stock_ratio: None,
            cash_dividends_on_pref_stock: None,
            cash_dividends_on_pref_stock_ratio: None,
            cash_dividends_on_comm_pref_quarterly: None,
            cash_dividends_on_comm_pref_quarterly_ratio: None,
            eqcfcta: None,
            minor_int_in_consol_subs_eq: None,
            common_stock: None,
            common_stock_ratio: None,
            net_worth_certificates: None,
            other_equity_capital_components: None,
            perpetual_preferred_stock: None,
            perpetual_preferred_stock_ratio: None,
            surplus: None,
            surplus_ratio: None,
            equp: None,
            up_net_other_capital_comp: None,
            up_net_other_capital_ratio: None,
            salaries_and_employee_benefits: None,
            salaries_and_employee_benefits_ratio: None,
            salaries_and_employee_benefits_quarterly: None,
            salaries_and_employee_benefits_quarterly_ratio: None,
            subordinated_notes_int_expense: None,
            tt_l_other_borrowings_int_exp: None,
            net_discontinued_operations: None,
            net_discontinued_ratio: None,
            net_discontinued_operations_quarterly: None,
            net_discontinued_operations_quarterly_ratio: None,
            fdic_region: None,
            fdic_region_desc: None,
            fdic_region_supervisory: None,
            fdic_region_supervisory_desc: None,
            fed_district: None,
            fed_district_desc: None,
            federal_charter_flag: None,
            fdic_risk_management_field_office: None,
            foreign_charter_flag: None,
            commercial_financial_report_flag: None,
            fed_funds_repos_sold: None,
            fed_funds_repos_sold_frepor: None,
            fed_funds_repos_purchased: None,
            fed_funds_repos_purchased_ratio: None,
            frs_member_flag: None,
            member_of_a_one_bank_holding_co: None,
            intl_banking_act_entity_flag: None,
            income_before_inc_taxes_disc: None,
            depository_institutions_int_inc: None,
            balances_from_depository_institutions_ytd_ratio: None,
            depository_institutions_int_inc_quarterly: None,
            depository_institutions_int_inc_quarterly_ratio: None,
            fed_funds_repo_interest_income: None,
            federal_funds_sold_ytd_ratio: None,
            fed_funds_repo_interest_income_quarterly: None,
            fed_funds_repo_interest_income_quarterly_ratio: None,
            securities_gains_and_losses: None,
            securities_gains_and_losses_ratio: None,
            securities_gains_and_losses_quarterly_ratio: None,
            loan_income_dom: None,
            domestic_office_loans_ytd_ratio: None,
            loan_income_dom_quarterly: None,
            loan_income_dom_quarterly_ratio: None,
            loan_income_for: None,
            foreign_office_loans_ytd_ratio: None,
            loan_income_for_quarterly: None,
            loan_income_for_quarterly_ratio: None,
            lease_income: None,
            lease_financing_receivables_ytd_ratio: None,
            lease_income_quarterly: None,
            lease_income_quarterly_ratio: None,
            insured_institution_flag: None,
            insured_commercial_flag: None,
            fdic_insured_flag: None,
            not_federally_insured_flag: None,
            insured_savings_institution_flag: None,
            commercial_institution_flag: None,
            saving_s_l_institution_flag: None,
            institution_type: None,
            intangible_assets: None,
            intangible_assets_ratio: None,
            interest_expense_to_earning_assets_ratio: None,
            cost_of_funding_earning_assets_quarterly: None,
            total_interest_income: None,
            total_interest_income_ytd_ratio: None,
            total_interest_income_quarterly: None,
            total_interest_income_quarterly_ratio: None,
            missing_title: None,
            invest_in_unconsolidated_subs: None,
            investments_in_re: None,
            other_fee_income: None,
            other_interest_income: None,
            other_interest_income_ytd_ratio: None,
            other_interest_income_quarterly: None,
            other_interest_income_quarterly_ratio: None,
            iras_and_keogh_plans_deposits: None,
            iras_and_keogh_plans_deposits_ratio: None,
            total_security_income: None,
            securities_ytd_ratio: None,
            total_security_income_quarterly: None,
            total_security_income_quarterly_ratio: None,
            service_charge_on_deposit_accts: None,
            service_charge_on_deposit_accts_ratio: None,
            applicable_income_taxes: None,
            applicable_income_taxes_ratio: None,
            applicable_income_taxes_quarterly: None,
            applicable_income_taxes_quarterly_ratio: None,
            interest_income_on_trading_accts: None,
            trading_accounts_ytd_ratio: None,
            interest_income_on_trading_accts_quarterly: None,
            interest_income_on_trading_accts_quarterly_ratio: None,
            total_liabilities: None,
            total_liabilities_ratio: None,
            total_liabilities_capital: None,
            total_liabilities_capital_ratio: None,
            mortgage_loans_in_process: None,
            limited_life_preferred_stock: None,
            acceptances_of_other_banks: None,
            agricultural_loans: None,
            agricultural_loans_ratio: None,
            allow_for_loans_loss_adjusted: None,
            allow_for_loans_alloc_trn_risk: None,
            allow_for_loans_alloc_trn_risk_ratio: None,
            consumer_loans_auto: None,
            consumer_loans_auto_ratio: None,
            c_i_loans: None,
            c_i_loans_ratio: None,
            consumer_loans: None,
            consumer_loans_ratio: None,
            consumer_loans_home_improvement: None,
            consumer_loans_other: None,
            consumer_loans_other_ratio: None,
            consumer_loans_credit_card_plan: None,
            consumer_loans_credit_card_plan_ratio: None,
            lns_credit_cd_related_plan: None,
            dep_institution_loans: None,
            foreign_govt_loans: None,
            foreign_govt_loans_ratio: None,
            ln_ls_unearned_inc: None,
            loans_and_leases_total: None,
            loans_and_leases_total_lnlsgr2: None,
            loans_and_leases_total_adjusted: None,
            loans_and_leases_total_ratio: None,
            loans_and_leases_net: None,
            loans_and_leases_net_ratio: None,
            muni_loans: None,
            muni_loans_ratio: None,
            other_lns_ls_comm_qbp: None,
            other_lns_ls_comm_qbp_ratio: None,
            ln_to_nondep_fin_inst_oth_ln: None,
            other_loans: None,
            other_loans_lnsotherr: None,
            re_loans: None,
            re_loans_lnre2: None,
            missing_title_lnrecon2: None,
            missing_title_lnremul2: None,
            re_loans_adjusted: None,
            re_loans_cavg5: None,
            re_loans_ratio: None,
            re_agricultural: None,
            re_construction_land_dev_cav5: None,
            re_agricultural_ratio: None,
            re_construction_land_develop: None,
            re_construction_land_develop_ratio: None,
            re_loans_dom: None,
            re_loans_dom_ratio: None,
            re_loans_for: None,
            re_loans_for_ratio: None,
            re_1_4_family_line: None,
            re_1_4_family_line_ratio: None,
            re_1_4_family_line2: None,
            re_1_4_family_line_cavg5: None,
            re_multifamily: None,
            re_multifamily_cavg5: None,
            re_multifamily_ratio: None,
            re_nonfarm_nonresidential_prop: None,
            re_nonfarm_nonresidential_cavg5: None,
            re_nonfarm_nonresidential_cavg5_lnrenre2: None,
            re_nonfarm_nonresidential_prop_ratio: None,
            prepaid_taxes_ins_on_mtg_lns: None,
            re_1_4_family: None,
            re_1_4_family_ratio: None,
            re_1_4_family2: None,
            re_1_4_family_cavg5: None,
            allowance_for_re_loan: None,
            leases: None,
            leases_ratio: None,
            metropolitan_flag: None,
            insured_savings_bank_flag: None,
            micropolitan_flag: None,
            minority_code: None,
            effective_dte_of_minority_status: None,
            mortgage_indebtedness_cap_ls: None,
            national_bank_flag: None,
            nonaccrual_loans_leases: None,
            noninsured_commercial_inst_flag: None,
            total_n_c_loans_leases: None,
            net_inc_attrib_to_minority_int: None,
            net_inc_attrib_to_minority_int_ratio: None,
            net_inc_attrib_to_minority_int_quarterly: None,
            net_inc_attrib_to_minority_int_quarterly_ratio: None,
            net_inc_bank_minority_int: None,
            net_inc_bank_minority_int_ratio: None,
            net_inc_bank_minority_int_quarterly: None,
            net_income_before_taxes_annually: None,
            missing_title_netibxqa: None,
            net_inc_bank_minority_int_quarterly_ratio: None,
            new_institution_flag: None,
            number_of_fiduciary_accounts_and_related_asset_accounts: None,
            net_interest_income: None,
            net_interest_income_ratio: None,
            net_interest_income_quarterly: None,
            net_interest_income_quarterly_nimqa: None,
            net_interest_income_annually: None,
            net_interest_income_quarterly_ratio: None,
            nonmember_insured_inst_flag: None,
            total_noninterest_income: None,
            total_noninterest_income_ratio: None,
            total_noninterest_expense: None,
            total_noninterest_expense_ratio: None,
            total_noninterest_expense_quarterly: None,
            total_noninterest_expense_quarterly_nonixqa: None,
            total_noninterest_expense_quarterly_ratio: None,
            noninsured_savings_inst_flag: None,
            total_ln_ls_net_charge_offs: None,
            total_ln_ls_net_charge_offs_ratio: None,
            total_ln_ls_net_charge_offs_quarterly: None,
            total_ln_ls_net_charge_offs_quarterly_ntlnlsqa: None,
            total_ln_ls_net_charge_offs_quarterly_ratio: None,
            nontransaction_total: None,
            nontransaction_total_ratio: None,
            nontransaction_ipc: None,
            nontransaction_ipc_ratio: None,
            nontransaction_muni: None,
            nontransaction_muni_ratio: None,
            time_deposits_total: None,
            time_deposits_over_100m: None,
            amt_total_time_dep_more_than_250_000: None,
            amt_total_time_dep_more_than_250_000_ratio: None,
            amt_time_dep_of_250_000_or_less: None,
            amt_time_dep_of_250_000_or_less_ratio: None,
            nontransaction_u_s_government: None,
            nontransaction_u_s_government_ratio: None,
            retained_earnings_anually: None,
            total_ln_ls_loss_net_chg_offs: None,
            number_of_full_time_employees: None,
            other_assets: None,
            oakar_flag: None,
            occ_district: None,
            occ_district_desc: None,
            domestic_multi_service_offices: None,
            nondomestic_offices: None,
            domestic_other_offices: None,
            sod_offices: None,
            number_of_states_with_offices: None,
            total_offices: None,
            u_s_and_other_area_offices: None,
            insured_iba_office_flag: None,
            ots_district: None,
            ots_region_number: None,
            other_liab_minor_in_subs: None,
            other_real_estate_owned: None,
            other_real_estate_owned_ratio: None,
            other_liabilities_fhlb: None,
            other_liabilities_fhlb_ratio: None,
            other_borrowed_money: None,
            oth_borrowed_funds: None,
            oth_borrowed_funds_ratio: None,
            fhlb_adv_mat_rep_one_yr_or_less: None,
            fhlb_adv_mat_rep_one_yr_or_less_ratio: None,
            fhlb_adv_mat_rep_one_yr_through_three: None,
            fhlb_adv_mat_rep_one_yr_through_three_otbfh1t3r: None,
            fhlb_adv_mat_rep_three_through_five: None,
            fhlb_adv_mat_rep_three_through_five_ratio: None,
            fhlb_adv_mat_rep_over_five_years: None,
            fhlb_adv_mat_rep_over_five_years_ratio: None,
            fhlb_adv_with_remaining_mat_one_yr_or_less: None,
            fhlb_adv_with_remaining_mat_one_yr_or_less_ratio: None,
            fhlb_structured_adv: None,
            fhlb_structured_adv_otbfhstar: None,
            oth_borr_mat_or_next_repricing_one_yr_or_less: None,
            oth_borr_mat_or_next_repricing_one_yr_or_less_ratio: None,
            oth_borr_mat_or_next_repricing_one_yr_through_three: None,
            oth_borr_mat_or_next_repricing_one_yr_through_three_ratio: None,
            oth_borr_mat_or_next_repricing_three_yr_through_five: None,
            oth_borr_mat_or_next_repricing_three_yr_through_five_ratio: None,
            oth_borr_mat_or_next_repricing_over_five_yrs: None,
            oth_borr_mat_or_next_repricing_over_five_yrs_ratio: None,
            oth_borr_mat_remaining_mat_of_one_yr_or_less: None,
            oth_borr_mat_remaning_mat_of_one_yr_or_less_ratio: None,
            all_other_liabilities: None,
            all_other_liabilities_ratio: None,
            _30_89_days_p_d_loans_leases: None,
            _90_days_p_d_loans_leases: None,
            qbp_commercial_bank_region: None,
            qbp_commercial_bank_region_desc: None,
            qbp_bif_fund_savings_region: None,
            qbp_saving_saif_fund_region: None,
            quarter_number: None,
            primary_regulating_agency: None,
            fdic_risk_territory: None,
            assets_10b_to_250b_flag: None,
            sasser_flag: None,
            savings_bank_flag: None,
            securities: None,
            securities_ratio: None,
            total_available_for_sale_at_amortized_cost_securities_on_a_consolidated_basis: None,
            total_held_to_maturity_at_fair_value_securities_on_a_consolidated_basis: None,
            u_s_agency: None,
            u_s_agency_scaspnha: None,
            u_s_agency_scaspnaf: None,
            non_mort_backed_issues_by_us_govt_or_sponsored_agencies: None,
            non_mort_backed_issues_by_us_govt_or_sponsored_agencies_ratio: None,
            domestic_sec_debt_equity_con: None,
            other_domestic_debt: None,
            other_domestic_debt_ratio: None,
            equity_securities: None,
            foreign_debt_equity: None,
            foreign_debt_securities: None,
            foreign_debt_securities_ratio: None,
            mortgage_backed_securities: None,
            mortgage_backed_securities_ratio: None,
            municipal_securities: None,
            municipal_ratio: None,
            securities_mv: None,
            res_oth_dom_debt_priv_certs: None,
            res_oth_dom_debt_priv_certs_ratio: None,
            contra_assets_to_securities: None,
            u_s_treasury_agency: None,
            u_s_treasury_agency_ratio: None,
            u_s_agency_all_other: None,
            u_s_treasury_securities: None,
            u_s_treasury_securities_ratio: None,
            geographic_latitude_of_main_office: None,
            geographic_longitude_of_main_office: None,
            savings_and_loan_flag: None,
            state_member_bank_flag: None,
            fips_state_alpha_code: None,
            state_charter_flag: None,
            state_name: None,
            fips_state_number: None,
            sub_debt_l_l_preferred_stk: None,
            subordinated_notes_debentures: None,
            assets_under_25m_flag: None,
            assets_under_100m_flag: None,
            assets_over_100m_flag: None,
            assets_100m_to_300m_flag: None,
            assets_100m_to_500m_flag: None,
            assets_100m_to_1b_flag: None,
            assets_over_10b_flag: None,
            assets_over_1b_flag: None,
            assets_1b_to_10b_flag: None,
            assets_1b_to_3b_flag: None,
            assets_1b_to_5b_flag: None,
            assets_over_250b_flag: None,
            assets_25m_to_50m_flag: None,
            assets_300m_to_500m_flag: None,
            assets_3b_to_10b_flag: None,
            assets_500m_to_1b_flag: None,
            assets_50m_to_100m_flag: None,
            assets_over_5b_flag: None,
            total_fiduciary_and_related_assets: None,
            trading_accounts: None,
            trading_liabilities: None,
            trading_liabilities_ratio: None,
            trading_accounts_ratio: None,
            transaction_total: None,
            transaction_total_ratio: None,
            transaction_ipc: None,
            tran_ipc_official_checks: None,
            tran_ipc_official_checks_ratio: None,
            transaction_muni: None,
            transaction_muni_ratio: None,
            transaction_u_s_government: None,
            transaction_u_s_government_ratio: None,
            trust_power_granted_codes: None,
            time_savings_deposits_total: None,
            time_savings_deposits_total_ratio: None,
            tt_l_note_option: None,
            tt_l_other_borrowings: None,
            unearned_income: None,
            bank_unique_number: None,
            usa_located_institution: None,
            unamortized_yield_adj_mtg_loans: None,
            asst_bck_unused_commit_related: None,
            asst_bck_unused_commit_related_ratio: None,
            asset_back_unused_commit_other: None,
            asset_back_unused_commit_other_ratio: None,
            asset_back_credit_ex_related: None,
            asset_back_credit_ex_related_ratio: None,
            asset_back_credit_ex_other: None,
            asset_back_credit_ex_other_ratio: None,
            c_e_recourse_not_secur_oth: None,
            c_e_recourse_not_secur_oth_ratio: None,
            c_e_recourse_not_secur_res: None,
            c_e_recourse_not_secur_res_ratio: None,
            sold_w_recourse_n_secur_oth: None,
            sold_w_recourse_n_secur_oth_ratio: None,
            sold_w_recourse_n_secur_res: None,
            sold_w_recourse_n_secur_res_ratio: None,
            total_assets_cavg2: None,
            total_assets_cavg5: None,
            total_assets_for: None,
            long_term_assets_5_years_qbp: None,
            long_term_assets_5_years_ratio: None,
            assets_per_employee_in_million: None,
            average_assets_adjusted_pca: None,
            average_assets_adjusted_pca_ratio: None,
            brokered_dep_insured: None,
            brokered_dep_insured_ratio: None,
            report_date_ccyymmdd: None,
            cash_due_from_dep_inst_for: None,
            noninterest_bearing_cash_due: None,
            noninterest_bearing_cash_due_ratio: None,
            cash_items: None,
            cash_items_ratio: None,
            currency_coin: None,
            currency_coin_ratio: None,
            net_operating_cash_flow_ann: None,
            net_operating_cash_flow_ann_quarterly: None,
            bal_due_from_frb: None,
            bal_due_from_frb_ratio: None,
            cash_item_collec_in_domestic_offices: None,
            cash_items_collec_in_domestic_offices_ratio: None,
            bal_due_from_bk_for_country: None,
            bal_due_from_bk_for_country_ratios: None,
            bal_due_from_for_br_of_oth_us_bk: None,
            bal_due_from_dep_inst_u_s: None,
            bal_due_from_dep_inst_u_s_ratio: None,
            bal_due_from_u_s_br_of_for_bks: None,
            city: None,
            core_deposits: None,
            core_deposits_ratio: None,
            agricultural_loan_recoveries: None,
            agricultural_loan_recoveries_ratio: None,
            agricultural_loan_recoveries_quarterly: None,
            agricultural_loan_recoveries_quarterly_ratio: None,
            ag_loan_recoveries_small_bks: None,
            aag_loan_recoveries_small_bks_ratio: None,
            ag_loan_recoveries_small_bks_quarterly: None,
            ag_loan_recoveries_small_bks_quarterly_ratio: None,
            auto_loans_recoveries: None,
            auto_loans_recoveries_ratio: None,
            auto_loans_recoveries_quarterly: None,
            auto_loans_recoveries_quarterly_ratio: None,
            commercial_loan_recoveries: None,
            commercial_loan_recoveries_ratio: None,
            commercial_loan_recoveries_quarterly: None,
            commercial_loan_recoveries_quarterly_ratio: None,
            commercial_loan_recoveries_non_u_s: None,
            commercial_loan_recoveries_non_u_s_ratio: None,
            commercial_loan_recoveries_non_u_s_quarterly: None,
            commercial_loan_recoveries_non_u_s_quarterly_ratio: None,
            consumer_loan_recoveries: None,
            consumer_loan_recoveries_ratio: None,
            consumer_loan_recoveries_quarterly: None,
            consumer_loan_recoveries_quarterly_ratio: None,
            other_consumer_loan_recoveries: None,
            other_consumer_loan_recoveries_ratio: None,
            other_consumer_loan_recoveries_quarterly: None,
            other_consumer_loan_recoveries_quarterly_ratio: None,
            credit_card_loan_recoveries: None,
            credit_card_loan_recoveries_ratio: None,
            credit_card_loan_recoveries_quarterly: None,
            credit_card_loan_recoveries_quarterly_ratio: None,
            depository_inst_loan_recoveries: None,
            depository_inst_loan_recoveries_ratio: None,
            depository_inst_loan_recoveries_quarterly: None,
            depository_inst_loan_recoveries_quarterly_ratio: None,
            foreign_deps_inst_ln_recoveries: None,
            foreign_deps_inst_ln_recoveries_ratio: None,
            foreign_deps_inst_ln_recoveries_quarterly: None,
            foreign_deps_inst_ln_recoveries_quarterly_ratio: None,
            foreign_government_ln_recoveries: None,
            foreign_government_ln_recoveries_ratio: None,
            foreign_government_ln_recoveries_quarterly: None,
            foreign_government_ln_recoveries_quarterly_ratio: None,
            lease_recoveries: None,
            lease_recoveries_ratio: None,
            lease_recoveries_quarterly: None,
            lease_recoveries_quarterly_ratio: None,
            all_other_loan_recoveries: None,
            all_other_loan_recoveries_ratio: None,
            all_other_loan_recoveries_quarterly: None,
            all_other_loan_recoveries_quarterly_ratio: None,
            real_estate_loan_recoveries: None,
            real_estate_loan_recoveries_ratio: None,
            real_estate_loan_recoveries_quarterly: None,
            real_estate_loan_recoveries_quarterly_ratio: None,
            farmland_re_ln_recoveries: None,
            farmland_re_ln_recoveries_ratio: None,
            farmland_re_ln_recoveries_qtr: None,
            farmland_re_ln_recoveries_quarterly_ratio: None,
            _1_4_fam_construct_ln_recoveries: None,
            other_construct_ln_recoveries: None,
            construction_re_ln_recover_qtr: None,
            construction_re_ln_recoveries_quarterly_ratio: None,
            construction_re_ln_recoveries: None,
            construction_re_ln_recoveries_ratio: None,
            real_estate_ln_recoveries_for: None,
            real_estate_ln_recoveries_for_ratio: None,
            real_estate_ln_recoveries_for_quarterly: None,
            real_estate_ln_recoveries_for_quarterly_ratio: None,
            line_of_credit_re_ln_recoveries: None,
            line_of_credit_re_ln_recoveries_ratio: None,
            line_of_credit_re_ln_recoveries_quarterly: None,
            line_of_credit_re_ln_recoveries_quarterly_ratio: None,
            multifamily_re_ln_recoveries_qtr: None,
            multifamily_res_re_ln_recoveries_quarterly_ratio: None,
            multifamily_res_re_ln_recoveries: None,
            multifamily_res_re_ln_recoveries_ratio: None,
            nonfarm_nonres_re_ln_recoveries: None,
            nonfarm_nonres_re_ln_recoveries_ratio: None,
            other_nonfarm_nonres_recoveries: None,
            own_occup_nonfarm_nonres_recov: None,
            nonfarm_nonres_re_ln_recover_qtr: None,
            nonfarm_nonres_re_ln_recover_qtr_ratio: None,
            non_u_s_re_ln_recoveries: None,
            non_u_s_re_ln_recoveries_ratio: None,
            non_u_s_re_ln_recoveries_quarterly: None,
            non_u_s_re_ln_recoveries_quarterly_ratio: None,
            re_loans_1_4_family_recoveries: None,
            re_loans_1_4_family_recoveries_ratio: None,
            re_loans_1_4_family_recover_qtr: None,
            re_loans_1_4_family_recoveries_quarterly_ratio: None,
            re_loan_1_4_fam_jr_lien_recover: None,
            re_loan_1_4_fam_jr_lien_recover_ratio: None,
            re_loan_1_4_fam_jr_lien_recover_quarterly: None,
            re_loan_1_4_fam_jr_lien_recover_quarterly_ratio: None,
            re_loan_1_4_fam_first_lien_recov: None,
            re_loan_1_4_fam_first_lien_recov_ratio: None,
            re_loan_1_4_fam_first_lien_recov_quarterly: None,
            re_loan_1_4_fam_first_lien_recov_quarterly_ratio: None,
            re_loan_recoveries_domestic_offices: None,
            re_loan_recoveries_domestic_offices_ratio: None,
            re_loan_recoveries_domestic_offices_quarterly: None,
            re_loan_recoveries_domestic_offices_quarterly_ratio: None,
            cr_der_net_purchase_protect: None,
            cr_der_net_sold_protection: None,
            total_deposit_liab_bef_exclusion: None,
            estimated_assessable_deposits: None,
            estimated_assessable_deposits_ratio: None,
            tot_domestic_deposit_asset: None,
            foreign_banks_for: None,
            foreign_banks_for_ratio: None,
            foreign_governments_for: None,
            foreign_governments_for_ratio: None,
            interest_bearing_dep_dom: None,
            interest_bearing_dep_dom_ratio: None,
            estimated_insured_deposits: None,
            estimated_insured_deposits_ratio: None,
            amt_dep_acc_greater_than_250_000: None,
            amt_dep_acc_greater_than_250_000_ratio: None,
            num_dep_acc_greater_than_250_000: None,
            amt_of_retirement_dep_acc_of_more_than_250_000: None,
            amt_of_retirement_dep_acc_of_more_than_250_000_ratio: None,
            num_of_retirement_dep_acc_more_than_250_000: None,
            dep_thru_list_svc_not_brokered: None,
            dep_thru_list_svc_not_brokered_ratio: None,
            noninterest_bearing_dep_dom: None,
            noninterest_bearing_dep_dom_ratio: None,
            amt_dep_acc_at_250_000_or_less: None,
            amt_dep_acc_at_250_000_or_less_ratio: None,
            num_dep_acc_equal_or_less_than_equal_to_250_000: None,
            amt_retirement_dep_acc_of_250_000_or_less: None,
            amt_retirement_dep_acc_of_250_000_or_less_ratio: None,
            num_retirement_dep_acc_of_250_000: None,
            total_allowable_exclusions_including_foreign_deposits: None,
            est_uninsured_dep_in_dom_off_in_insured_branches_in_us_terr_and_possessions: None,
            est_uninsured_dep_in_dom_off_in_insured_branches_in_us_terr_and_possessions_depunar: None,
            estimated_uninsured_deposits_in_domestic_offices_and_in_insured_branches_in_us_territories_and_possessions: None,
            u_s_banks_oth_us_inst_for: None,
            u_s_banks_oth_us_inst_for_ratio: None,
            u_s_govt_st_pol_subs_for: None,
            u_s_govt_st_pol_subs_for_ratio: None,
            agricultural_loan_charge_offs: None,
            agricultural_loan_charge_offs_ratio: None,
            agricultural_loan_charge_offs_quarterly: None,
            agricultural_loan_charge_offs_quarterly_ratio: None,
            ag_loan_charge_offs_small_bks: None,
            ag_loan_charge_offs_small_bks_ratio: None,
            ag_loan_charge_offs_small_bks_quarterly: None,
            ag_loan_charge_offs_small_bks_quarterly_ratio: None,
            auto_loans_charge_offs: None,
            auto_loans_charge_offs_ratio: None,
            auto_loans_charge_offs_quarterly: None,
            auto_loans_charge_offs_quarterly_ratio: None,
            commercial_loan_charge_offs: None,
            commercial_loan_charge_offs_ratio: None,
            commercial_loan_charge_offs_quarterly: None,
            commercial_loan_charge_offs_quarterly_ratio: None,
            commercial_loan_charge_offs_non_u_s: None,
            commercial_loan_charge_offs_non_u_s_ratio: None,
            commercial_loan_charge_offs_non_u_s_quarterly: None,
            commercial_loan_charge_offs_non_u_s_quarterly_ratio: None,
            consumer_loan_charge_offs: None,
            consumer_loan_charge_offs_ratio: None,
            consumer_loan_charge_offs_quarterly: None,
            consumer_loan_charge_offs_quarterly_ratio: None,
            other_consumer_loan_charge_offs: None,
            other_consumer_loan_charge_offs_ratio: None,
            other_consumer_loan_charge_offs_quarterly: None,
            other_consumer_loan_charge_offs_quarterly_ratio: None,
            credit_card_loan_charge_offs: None,
            credit_card_loan_charge_offs_ratio: None,
            credit_card_loan_charge_offs_quarterly: None,
            credit_card_loan_charge_offs_quarterly_ratio: None,
            depository_inst_loan_charge_offs: None,
            depository_inst_loan_charge_offs_ratio: None,
            depository_inst_loan_charge_offs_quarterly: None,
            depository_inst_loan_charge_offs_quarterly_ratio: None,
            foreign_deps_inst_ln_chg_offs: None,
            foreign_deps_inst_ln_chg_offs_ratio: None,
            foreign_deps_inst_ln_chg_offs_quarterly: None,
            foreign_deps_inst_ln_chg_offs_quarterly_ratio: None,
            foreign_government_ln_chg_offs: None,
            foreign_government_ln_chg_offs_ratio: None,
            foreign_government_ln_chg_offs_quarterly: None,
            foreign_government_ln_chg_offs_quarterly_ratio: None,
            lease_charge_offs: None,
            lease_charge_offs_ratio: None,
            lease_charge_offs_quarterly: None,
            lease_charge_offs_quarterly_ratio: None,
            all_other_loan_charge_offs: None,
            all_other_loan_charge_offs_ratio: None,
            all_other_loan_charge_offs_quarterly: None,
            all_other_loan_charge_offs_quarterly_ratio: None,
            real_estate_loan_charge_offs: None,
            real_estate_loan_charge_offs_ratio: None,
            real_estate_loan_charge_offs_quarterly: None,
            real_estate_loan_charge_offs_quarterly_ratio: None,
            farmland_re_ln_charge_offs: None,
            farmland_re_ln_charge_offs_ratio: None,
            farmland_re_ln_chg_offs_qtr: None,
            farmland_re_ln_charge_offs_quarterly_ratio: None,
            _1_4_fam_construct_ln_charge_offs: None,
            other_construct_ln_charge_offs: None,
            construction_re_ln_chg_offs_qtr: None,
            construction_re_ln_charge_offs_quarterly_ratio: None,
            construction_re_ln_charge_offs: None,
            construction_re_ln_charge_offs_ratio: None,
            real_estate_loan_chrg_offs_for: None,
            real_estate_loan_chrg_offs_for_ratio: None,
            real_estate_loan_chrg_offs_for_quarterly: None,
            real_estate_loan_chrg_offs_for_quarterly_ratio: None,
            line_of_credit_re_ln_charge_offs: None,
            line_of_credit_re_ln_charge_offs_ratio: None,
            line_of_credit_re_ln_charge_offs_quarterly: None,
            line_of_credit_re_ln_charge_offs_ratio_drrelocqr: None,
            multifamily_re_ln_chg_offs_qtr: None,
            multifamily_res_re_ln_charge_off_quarterly_ratio: None,
            multifamily_res_re_ln_charge_off: None,
            multifamily_res_re_ln_charge_off_ratio: None,
            nonfarm_nonres_re_ln_charge_offs: None,
            nonfarm_nonres_re_ln_charge_offs_ratio: None,
            other_nonfarm_nonres_re_chg_off: None,
            own_occup_nonfarm_nonres_chg_off: None,
            nonfarm_nonres_re_ln_chg_off_qtr: None,
            nonfarm_nonres_re_ln_charge_offs_quarterly_ratio: None,
            non_u_s_re_ln_charge_offs: None,
            non_u_s_re_ln_charge_offs_ratio: None,
            non_u_s_re_ln_charge_offs_quarterly: None,
            non_u_s_re_ln_charge_offs_ratio_drrenusqr: None,
            re_loans_1_4_family_charge_offs: None,
            re_loans_1_4_family_charge_offs_ratio: None,
            re_loans_1_4_family_chg_offs_qtr: None,
            re_loans_1_4_family_charge_offs_quarterly_ratio: None,
            re_ln_1_4_fam_jr_lien_chg_off: None,
            re_ln_1_4_fam_jr_lien_chg_off_ratio: None,
            re_ln_1_4_fam_jr_lien_chg_off_quarterly: None,
            re_ln_1_4_fam_jr_lien_chg_off_quarterly_ratio: None,
            re_ln_1_4_fam_first_lien_chg_off: None,
            re_ln_1_4_fam_first_lien_chg_off_ratio: None,
            re_ln_1_4_fam_first_lien_chg_off_quarterly: None,
            re_ln_1_4_fam_first_lien_chg_off_quarterly_ratio: None,
            real_estate_loan_charge_offs_domestic_offices: None,
            real_estate_loan_charge_offs_domestic_offices_ratio: None,
            real_estate_loan_charge_offs_domestic_offices_quarterly: None,
            real_estate_loan_charge_offs_domestic_offices_quarterly_ratio: None,
            equity: None,
            efficiency_ratio_expense: None,
            efficiency_ratio_expense_quarterly: None,
            efficiency_ratio: None,
            efficiency_quarterly_ratio: None,
            effective_date: None,
            goodwill_impairment_losses: None,
            goodwill_impairment_losses_ratio: None,
            goodwill_impairment_losses_quarterly: None,
            goodwill_impairment_losses_quarterly_ratio: None,
            amort_impair_losses_oth_intan: None,
            amort_impair_losses_oth_intan_ratio: None,
            amort_impair_losses_oth_intan_quarterly: None,
            amort_impair_losses_oth_intan_quarterly_ratio: None,
            loan_loss_prov_nt_chg_offs: None,
            elnatra: None,
            credit_loss_prov_ave_assets: None,
            credit_loss_prov_ave_assets_quarterly: None,
            cr_exposure_enhancements_auto: None,
            cr_exposure_enhancements_auto_ratio: None,
            cr_exposure_enhancements_ci: None,
            cr_exposure_enhancements_ci_ratio: None,
            cr_exposure_enhancements_con: None,
            cr_exposure_enhancements_con_ratio: None,
            cr_exposure_enhancements_oth: None,
            cr_exposure_enhancements_oth_ratio: None,
            cr_exposure_enhancements_res: None,
            cr_exposure_enhancements_res_ratio: None,
            other_interest_expense: None,
            other_interest_expense_ratio: None,
            other_interest_expense_quarterly: None,
            other_interest_expense_quarterly_ratio: None,
            total_bank_equity_capital_cavg5: None,
            transactions_with_bhc: None,
            transactions_with_bhc_ratio: None,
            other_comprehensive_income: None,
            other_comprehensive_income_ratio: None,
            cash_dividends_on_comm_pfd_ann: None,
            changes_due_to_mergers: None,
            changes_due_to_mergers_ratio: None,
            bk_eq_cap_most_recently_reported: None,
            bk_eq_cap_most_recently_reported_ratio: None,
            accounting_changes_corrections: None,
            accounting_changes_corrections_ratio: None,
            sale_of_capital_stock: None,
            sale_of_capital_stock_ratio: None,
            sale_of_capital_stock_quarterly: None,
            sale_of_capital_stock_quarterly_ratio: None,
            treasury_stock_transactions: None,
            treasury_stock_transactions_ratio: None,
            total_equity_capital: None,
            total_equity_capital_ratio: None,
            bank_equity_capital_assets: None,
            total_earning_assets: None,
            total_earning_assets_ernast2: None,
            total_earning_assets_cavg5i: None,
            earning_assets_total_assets: None,
            established_date: None,
            inactive_date: None,
            inactive_date_org_end_num_dte: None,
            tt_l: None,
            thrift_financial_report_flag: None,
            foreign_exchange_total_contracts: None,
            for_exch_futures_forward_contr: None,
            for_exchange_swaps: None,
            for_exch_pur_option_contracts: None,
            spot_foreign_exchange_contracts: None,
            for_exch_written_option_contract: None,
            inc_before_inc_taxs_disc_qtr: None,
            income_before_disc_opr: None,
            income_before_disc_opr_ratio: None,
            income_before_disc_opr_quarterly: None,
            efficiency_ratio_income: None,
            efficiency_ratio_income_quarterly: None,
            income_before_disc_opr_quarterly_ratio: None,
            fiduciary_activities_income: None,
            fiduciary_activities_income_ratio: None,
            fiduciary_activities_income_qtr: None,
            fiduciary_activities_income_qtr_ratio: None,
            trading_account_commodity: None,
            trading_account_commodity_ratio: None,
            trading_account_commodity_quarterly: None,
            trading_account_commodity_ratio_quarterly: None,
            trading_revenue_credit_exposure: None,
            trading_revenue_credit_exposure_ratio: None,
            trading_revenue_credit_exposure_quarterly: None,
            trading_revenue_credit_exposure_quarterly_ratio: None,
            trading_account_eq_derivative: None,
            trading_account_eq_derivative_ratio: None,
            trading_account_eq_derivative_quarterly: None,
            trading_account_eq_derivative_quarterly_ratio: None,
            trading_account_foreign_exchange: None,
            rading_account_foreign_exchange_ratio: None,
            trading_account_foreign_exchange_quarterly: None,
            rading_account_foreign_exchange_quarterly_ratio: None,
            trading_account_interest_rate: None,
            trading_account_interest_rate_ratio: None,
            trading_account_interest_rate_quarterly: None,
            trading_account_interest_rate_quarterly_ratio: None,
            securities_gains_and_losses_qtr: None,
            trading_revenues_total: None,
            trading_revenues_total_ratio: None,
            trading_revenue_qtr: None,
            trading_revenue_qtr_ratio: None,
            insurance_commissions_fees: None,
            insurance_commissions_fees_ratio: None,
            insurance_commissions_fees_quarterly: None,
            insurance_commissions_fees_quarterly_ratio: None,
            insurance_com_fees_other: None,
            insurance_com_fees_other_ratio: None,
            insurance_com_fees_other_quarterly: None,
            insurance_com_fees_other_quarterly_ratio: None,
            insurance_underwritng_income: None,
            insurance_underwritng_income_ratio: None,
            insurance_underwritng_income_quarterly: None,
            insurance_underwritng_income_quarterly_ratio: None,
            invest_bank: None,
            invest_bank_ratio: None,
            invest_bank_quarterly: None,
            invest_bank_quarterly_ratio: None,
            primary_insurer: None,
            purch_cc_rel_nonmtg_ser_asts: None,
            goodwill: None,
            goodwill_ratio: None,
            mortgage_servicing_assets: None,
            mortgage_servicing_assets_ratio: None,
            other_identifiable_intang_assets: None,
            other_identifiable_intang_assets_ratio: None,
            interest_income_earning_assets_quarterly: None,
            total_interest_income_annual: None,
            other_noninterest_income: None,
            other_noninterest_income_ratio: None,
            other_noninterest_income_quarterly: None,
            other_noninterest_income_quarterly_ratio: None,
            securitization_income: None,
            securitization_income_ratio: None,
            securitization_income_quarterly: None,
            securitization_income_quarterly_ratio: None,
            service_charge_on_dep_accts_qtr: None,
            service_charge_on_deposit_accts_qtr_ratio: None,
            servicing_fees: None,
            servicing_fees_ratio: None,
            servicing_fees_quarterly: None,
            servicing_fees_quarterly_ratio: None,
            venture_capital_revenue: None,
            venture_capital_revenue_ratio: None,
            venture_capital_revenue_quarterly: None,
            venture_capital_revenue_quarterly_ratio: None,
            ag_loans_loss_share: None,
            ag_loans_loss_share_ratio: None,
            c_i_loans_loss_share: None,
            c_i_loans_loss_share_ratio: None,
            consumer_loans_loss_share: None,
            consumer_loans_loss_share_ratio: None,
            total_liabilities_for: None,
            agricultural_loans_under_100: None,
            agricultural_loans_under_100_ratio: None,
            agricultural_loans_100_250: None,
            agricultural_loans_100_250_ratio: None,
            agricultural_loans_250_500: None,
            agricultural_loans_250_500_ratio: None,
            agricultural_loans_under_500: None,
            agricultural_loans_under_500_ratio: None,
            ag_loans_cavg5: None,
            ag_loans_cavg2: None,
            agricultural_loans_under_100_num: None,
            agricultural_loans_under_100_num_ratio: None,
            agricultural_loans_100_250_num: None,
            agricultural_loans_100_250_num_ratio: None,
            agricultural_loans_250_500_num: None,
            agricultural_loans_250_500_num_ratio: None,
            agricultural_loans_under_500_num: None,
            agricultural_loans_under_500_num_ratio: None,
            agricultural_loans_for: None,
            agricultural_loans_for_ratio: None,
            loan_loss_reserve_gross_ln_ls: None,
            consumer_loans_auto_cavg2: None,
            consumer_loans_auto_cavg5: None,
            c_i_loans_under_100: None,
            c_i_loans_under_100_ratio: None,
            c_i_loans_100_250: None,
            c_i_loans_100_250_ratio: None,
            c_i_loans_250_1m: None,
            c_i_loans_250_1m_ratio: None,
            c_i_loans_under_1m: None,
            c_i_loans_under_1m_ratio: None,
            c_i_loans_cavg5: None,
            c_i_loans_cavg2: None,
            c_i_loans_under_100_num: None,
            c_i_loans_under_100_num_ratio: None,
            c_i_loans_100_250_num: None,
            c_i_loans_250_1m_num_ratio: None,
            c_i_loans_250_1m_num: None,
            c_i_loans_250_1m_num_ratio_lnci3nr: None,
            c_i_loans_under_1m_num: None,
            c_i_loans_under_1m_num_ratio: None,
            c_i_loans_for: None,
            c_i_loans_for_ratio: None,
            c_i_loans_non_u_s_domicile: None,
            c_i_loans_non_u_s_domicile_for: None,
            c_i_loans_non_u_s_domicile_for_ratio: None,
            commercial_re_loans: None,
            commercial_re_loans_ratio: None,
            commercial_re_loans2: None,
            commercial_re_loans_cavg5: None,
            consumer_loans_cavg2: None,
            consumer_loans_cavg5: None,
            consumer_loans_for: None,
            consumer_loans_for_ratio: None,
            other_consumer_related_plans: None,
            other_consumer_loans_cavg2: None,
            other_consumer_loans_cavg5: None,
            consumer_lns_related_plans: None,
            consumer_lns_related_plans_ratio: None,
            other_contra_accounts: None,
            other_contra_accounts_ratio: None,
            credit_card_plans_cavg2: None,
            credit_card_plans_cavg5: None,
            total_dep_inst_lns_accept: None,
            total_dep_inst_lns_accept_dom: None,
            loans_to_depository_institutions_and_acceptance_of_other_banks: None,
            loans_to_depository_institutions_and_acceptance_of_other_banks_ratio: None,
            dep_inst_lns_commercial_banks: None,
            dep_inst_lns_commercial_bk_for: None,
            dep_inst_lns_commercial_bk_for_ratio: None,
            dep_inst_lns_for_country: None,
            dep_inst_lns_for_country_for: None,
            dep_inst_lns_for_country_for_ratio: None,
            dep_inst_lns_for_country_u_s_br: None,
            dep_inst_lns_oth_u_s_inst: None,
            dep_inst_lns_com_bks_u_s_branch: None,
            dep_inst_lns_oth_u_s_inst_for: None,
            dep_inst_lns_oth_u_s_inst_for_ratio: None,
            executive_officer_loans_amount: None,
            executive_officer_loans_amount_ratio: None,
            foreign_govt_loans_for: None,
            foreign_govt_loans_for_ratio: None,
            net_loans_leases_deposits: None,
            ln_ls_unearned_inc_for: None,
            ln_ls_unearned_inc_for_ratio: None,
            loans_and_leases_total_cavg5: None,
            loans_and_leases_total_for: None,
            loans_and_leases_total_for_ratio: None,
            net_loans_leases_assets: None,
            net_loans_leases_assets_quarterly_ratio: None,
            loans_leases_held_for_resale: None,
            loans_leases_held_for_resale_ratio: None,
            pledged_loans_and_leases: None,
            pledged_loans_and_leases_ratio: None,
            muni_loans_for: None,
            muni_loans_for_ratio: None,
            all_other_lns_ls_1_3_years: None,
            all_other_lns_ls_1_3_years_ratio: None,
            all_other_lns_ls_3_mo_or_less: None,
            all_other_lns_ls_3_mo_or_less_ratio: None,
            all_other_lns_ls_3_5_years: None,
            all_other_lns_ls_3_5_years_ratio: None,
            all_other_lns_ls_3_12_mons: None,
            all_other_lns_ls_3_12_mons_ratio: None,
            all_other_lns_ls_5_15_years: None,
            all_other_lns_ls_5_15_years_ratio: None,
            other_loans_leases_qbp_cavg2: None,
            other_loans_leases_qbp_cavg5: None,
            ln_to_nondep_fin_inst_oth_fgn: None,
            ln_to_nondep_fin_inst_oth_fgn_ratio: None,
            all_other_lns_ls_over_15_yrs: None,
            all_other_lns_ls_over_15_yrs_ratio: None,
            re_agricultural_under_100: None,
            re_agricultural_under_100_ratio: None,
            re_agricultural_100_250: None,
            re_agricultural_100_250_ratio: None,
            re_agricultural_250_500: None,
            re_agricultural_250_500_ratio: None,
            re_agricultural_under_500: None,
            re_agricultural_under_500_ratio: None,
            re_agricultural_under_100_num: None,
            re_agricultural_under_100_num_ratio: None,
            re_agricultural_100_250_num: None,
            re_agricultural_100_250_num_ratio: None,
            re_agricultural_250_500_num: None,
            re_agricultural_250_500_num_ratio: None,
            re_agricultural_under_500_num: None,
            re_agricultural_under_500_num_ratio: None,
            _1_4_fam_re_construction_loans: None,
            _1_4_fam_re_construction_loans_ratio: None,
            other_re_construction_land_ln: None,
            other_re_construction_land_ln_lnrecnotr: None,
            all_other_re_owned_1_4_family: None,
            all_other_re_owned_1_4_family2: None,
            re_1_4_family_other_loans_cavg5: None,
            re_nonfarm_nonres_under_100: None,
            re_nonfarm_nonres_under_100_ratio: None,
            re_nonfarm_nonres_100_250: None,
            re_nonfarm_nonres_100_250_ratio: None,
            re_nonfarm_nonres_250_1m: None,
            re_nonfarm_nonres_250_1m_ratio: None,
            re_nonfarm_nonres_under_1m: None,
            re_nonfarm_nonres_under_1m_ratio: None,
            re_nonfarm_nonres_under_100_num: None,
            re_nonfarm_nonres_under_100_num_ratio: None,
            re_nonfarm_nonres_100_250_num: None,
            re_nonfarm_nonres_100_250_num_ratio: None,
            re_nonfarm_nonres_250_1m_num: None,
            re_nonfarm_nonres_250_1m_num_ratio: None,
            re_nonfarm_nonres_under_1m_num: None,
            re_nonfarm_nonres_under_1m_num_ratio: None,
            other_nonfarm_nonres_re_lns: None,
            other_nonfarm_nonres_re_lns_ratio: None,
            owner_occ_nonfarm_nonres_re_lns: None,
            owner_occ_nonfarm_nonres_re_lns_lnrenrowr: None,
            re_lns_non_us_addressees: None,
            re_lns_non_us_addressees_ratio: None,
            re_1_4_family_first_liens_adjust: None,
            re_1_4_family_first_liens_adjust_ratio: None,
            re_1_4_family_second_liens: None,
            re_1_4_family_second_liens_ratio: None,
            re_1_4_family_first_liens: None,
            re_1_4_family_first_liens_ratio: None,
            loan_loss_reserve_n_c_loans: None,
            re_1_4_family_1_3_years: None,
            re_1_4_family_1_3_years_ratio: None,
            re_1_4_family_3_mons_or_less: None,
            re_1_4_family_3_mons_or_less_ratio: None,
            re_1_4_family_3_5_years: None,
            re_1_4_family_3_5_years_ratio: None,
            re_1_4_family_3_12_months: None,
            re_1_4_family_3_12_months_ratio: None,
            re_1_4_family_5_15_years: None,
            re_1_4_family_5_15_years_ratio: None,
            re_1_4_family_over_15_years: None,
            re_1_4_family_over_15_years_ratio: None,
            small_business_lns_sold_amt: None,
            small_business_lns_sold: None,
            prin_bal_lns_service_for_others: None,
            prin_bal_lns_service_for_others_ratio: None,
            commercial_letters_of_credit: None,
            commercial_letters_of_credit_ratio: None,
            fin_perform_standby_loc: None,
            fin_perform_standby_loc_ratio: None,
            fin_perform_standby_loc_convey: None,
            fin_perform_standby_loc_convey_ratio: None,
            financial_standby_loc: None,
            financial_standby_loc_ratio: None,
            financial_standby_loc_conveyed: None,
            financial_standby_loc_conveyed_ratio: None,
            performance_standby_loc: None,
            performance_standby_loc_ratio: None,
            performance_standby_loc_conveyed: None,
            performance_standby_loc_conveyed_ratio: None,
            ore_protected_loss_share: None,
            ore_protected_loss_share_ratio: None,
            all_other_ln_ls_loss_share: None,
            all_other_ln_ls_loss_share_ratio: None,
            re_farmland_ln_loss_sh: None,
            re_farmland_ln_loss_sh_ratio: None,
            re_construct_ln_loss_share: None,
            re_construct_ln_loss_share_ratio: None,
            re_multifamily_ln_loss_sh: None,
            re_multifamily_ln_loss_sh_ratio: None,
            re_nonfarm_nonres_ln_loss_sh: None,
            re_nonfarm_nonres_ln_loss_sh_ratio: None,
            re_1_4_family_lns_loss_share: None,
            re_1_4_family_lns_loss_share_ratio: None,
            carry_amt_loss_share_lnls: None,
            carry_amt_loss_share_lnls_ratio: None,
            carry_amt_loss_share_oth_asset: None,
            carry_amt_loss_share_oth_asset_ratio: None,
            carry_amt_loss_share_ore: None,
            carry_amt_loss_share_ore_ratio: None,
            carry_amt_loss_share_debt_sec: None,
            carry_amt_loss_share_debt_sec_ratio: None,
            leases_for: None,
            leases_for_ratio: None,
            fips_msa_code: None,
            out_prin_bal_mort_w_recourse: None,
            out_prin_bal_mort_w_recourse_ratio: None,
            _1_4_fm_serviced_in_foreclosure: None,
            _1_4_fm_serviced_in_foreclosure_ratio: None,
            out_prin_bal_mort_w_no_recourse: None,
            out_prin_bal_mort_w_no_recourse_ratio: None,
            nonaccrual_agricultural_lns: None,
            nonaccrual_agricultural_lns_ratio: None,
            nonaccrual_ag_lns_small_bks: None,
            nonaccrual_ag_lns_small_bks_ratio: None,
            nonaccrual_total_assets: None,
            nonaccrual_ag_lns_small_bks_ratio_naassetr: None,
            nonaccrual_auto_loans: None,
            nonaccrual_auto_loans_ratio: None,
            nonaccrual_c_i_loans: None,
            nonaccrual_c_i_loans_ratio: None,
            nonaccrual_c_i_non_u_s: None,
            nonaccrual_c_i_non_u_s_ratio: None,
            nonaccrual_consumer_loans: None,
            nonaccrual_consumer_loans_ratio: None,
            nonaccrual_other_consumer: None,
            nonaccrual_other_consumer_ratio: None,
            nonaccrual_credit_card_plans: None,
            nonaccrual_credit_card_plans_ratio: None,
            nonaccrual_dep_inst_loans: None,
            nonaccrual_dep_inst_loans_ratio: None,
            nonaccrual_dep_inst_non_u_s: None,
            nonaccrual_dep_inst_non_u_s_ratio: None,
            nonaccrual_foreign_govt: None,
            nonaccrual_foreign_govt_ratio: None,
            nonaccrual_gty_ln_ls: None,
            nonaccrual_gty_ln_ls_nagtyr: None,
            nonaccrual_rebooked_gnma_loans: None,
            nonaccrual_rebooked_gnma_lns: None,
            nonaccrual_part_gty_ln_ls: None,
            nonaccrual_part_gty_ln_ls_ratio: None,
            nonaccrual_ag_loans_loss_sh: None,
            nonaccrual_ag_loans_loss_sh_ratio: None,
            nonaccrual_c_i_lns_loss_sh: None,
            nonaccrual_c_i_lns_loss_sh_ratio: None,
            nonaccrual_consumer_ln_loss_sh: None,
            nonaccrual_consumer_ln_loss_sh_ratio: None,
            nonaccr_protect_gty_loss_sh: None,
            nonaccrual_protect_gty_loss_sh_ratio: None,
            nonaccrual_l_l_held_for_sale: None,
            nonaccrual_l_l_held_for_sale_ratio: None,
            nonaccrual_other_lns_loss_sh: None,
            nonaccrual_other_lns_loss_sh_ratio: None,
            nonaccrual_re_farm_loss_sh: None,
            nonaccrual_re_farm_loss_sh_ratio: None,
            nonaccrual_constr_ln_loss_sh: None,
            nonaccrual_constr_ln_loss_sh_ratio: None,
            nonaccrual_multifam_loss_sh: None,
            nonaccrual_multifam_loss_sh_ratio: None,
            nonaccrual_nfnr_ln_loss_sh: None,
            nonaccrual_nfnr_ln_loss_sh_ratio: None,
            nonaccrual_1_4_fm_ln_loss_sh: None,
            nonaccrual_1_4_fm_ln_loss_sh_ratio: None,
            nonaccrual_leases: None,
            nonaccrual_leases_ratio: None,
            nonaccrual_total_loans_loss_sh: None,
            nonaccrual_total_loans_loss_sh_ratio: None,
            institution_name: None,
            institution_full_name: None,
            nonaccrual_all_other_loans: None,
            nonaccrual_all_other_loans_ratio: None,
            nonaccrual_real_estate_loans: None,
            nonaccrual_real_estate_loans_ratio: None,
            nonaccrual_re_farmland: None,
            nonaccrual_re_farmland_ratio: None,
            nonaccrual_1_4_fam_construct_ln: None,
            nonaccrual_1_4_fam_construct_ln_ratio: None,
            nonaccrual_other_constr_land: None,
            nonaccrual_other_constr_land_ratio: None,
            nonaccrual_re_construction: None,
            nonaccrual_re_construction_ratio: None,
            nonaccrual_re_foreign: None,
            nonaccrual_re_foreign_ratio: None,
            nonaccrual_re_1_4_fam_lines: None,
            nonaccrual_re_1_4_fam_lines_ratio: None,
            nonaccrual_re_multifamily: None,
            nonaccrual_re_multifamily_ratio: None,
            nonaccrual_re_nonfarm_nonres: None,
            nonaccrual_re_nonfarm_nonres_ratio: None,
            nonaccrual_other_nonfarm_nonres: None,
            nonaccrual_other_nonfarm_nonres_ratio: None,
            nonaccrual_0wn_occ_nonfrm_nonrs: None,
            nonaccrual_own_occ_nonfrm_nonrs_ratio: None,
            nonaccrual_re_non_u_s: None,
            nonaccrual_re_non_u_s_ratio: None,
            nonaccrual_re_1_4_family: None,
            nonaccrual_re_1_4_family_ratio: None,
            nonaccrual_re_1_4_junior_lien: None,
            nonaccrual_re_1_4_jn_lien_ratio: None,
            nonaccrual_re_1_4_ist_lien: None,
            nonaccrual_re_1_4_ist_lien_ratio: None,
            nonaccrual_restruct_c_i_ln: None,
            nonaccr_restruct_construction: None,
            nonaccrual_restru_ln_1_4_fam: None,
            nonaccrual_restru_ln_1_4_fam_ratio: None,
            nonaccrual_restru_ln_excl_1_4_fm: None,
            nonaccrual_restru_ln_excl_1_4_fm_ratio: None,
            nonaccrual_restruct_ln_total: None,
            nonaccrual_restruct_ln_total_ratio: None,
            nonaccrual_restruct_multifamily: None,
            nonaccr_restructured_nfnr_ln: None,
            nonaccrual_restruct_all_oth_ln: None,
            nonaccrual_debt_securities: None,
            nonaccrual_debt_securities_ratio: None,
            total_n_c_agricultural_lns: None,
            n_c_auto_loans: None,
            total_n_c_c_i_loans: None,
            nc_commercial_re_commercial_re: None,
            nc_commercial_re_commercial_re_nccomre: None,
            total_n_c_consumer_loans: None,
            total_n_c_other_consumer: None,
            total_n_c_credit_card_plans: None,
            total_n_c_dep_inst_loans: None,
            total_n_c_foreign_govt: None,
            total_n_c_part_gty_ln_ls: None,
            n_c_lns_ls_gross_lns_ls: None,
            total_n_c_leases: None,
            total_n_c_all_other_loans: None,
            total_n_c_real_estate_loans: None,
            n_c_const_real_estate_const_re: None,
            total_n_c_const_real_estate_construction: None,
            total_n_c_re_1_4_family_lines: None,
            n_c_home_equity_home_equity: None,
            n_c_multifamly_re_multifamly_re: None,
            total_n_c_multifamly_re: None,
            n_c_nonfarm_nonres_re_nonres_re: None,
            total_n_c_nonfarm_nonres_re: None,
            n_c_real_estate_lns_real_estate: None,
            n_c_1_4_other_re_1_4_other_re: None,
            n_c_1_4_other_re_1_4_other_re_ncrereor: None,
            n_c_1_4_family_re: None,
            n_c_1_4_family_re_1_4_family_re: None,
            net_g_l_on_sales_of_fix_assets: None,
            net_g_l_on_sales_of_fix_assets_ratio: None,
            net_g_l_on_sales_of_fix_assets_quarterly: None,
            net_g_l_on_sales_of_fix_assets_quarterly_ratio: None,
            net_g_l_on_sales_of_loans: None,
            net_g_l_on_sales_of_loans_ratio: None,
            net_g_l_on_sales_of_loans_quarterly: None,
            net_g_l_on_sales_of_loans_quarterly_ratio: None,
            net_g_l_on_other_re_owned: None,
            net_g_l_on_other_re_owned_ratio: None,
            net_g_l_on_other_re_owned_quarterly: None,
            net_g_l_on_other_re_owned_quarterly_ratio: None,
            net_income_bank_ann: None,
            net_interest_margin: None,
            net_interest_margin_quarterly: None,
            net_operating_income_adj: None,
            net_operating_income_adj_ratio: None,
            net_operating_income_adj_assets: None,
            net_operating_income_adj_assets_quarterly: None,
            net_operating_income_adj_annually: None,
            net_operating_income_adj_quarterly: None,
            net_operating_income_adj_quarterly_noijqa: None,
            net_operating_income_adj_quarterly_ratio: None,
            noninterest_inc_average_assets: None,
            noninterest_inc_average_assets_quarterly: None,
            total_noninterest_income_annually: None,
            total_noninterest_income_qtr: None,
            total_noninterest_income_qtr_noniiqa: None,
            total_noninterest_income_qtr_ratio: None,
            noninterest_exp_average_assets: None,
            noninterest_exp_average_assets_quarterly: None,
            total_noninterest_expense_annually: None,
            nonperf_assets_total_assets: None,
            nonperf_assets_total_assets_nperfv: None,
            agricultural_ln_net_charge_offs: None,
            agricultural_ln_net_charge_offs_ratio: None,
            agricultural_ln_net_chg_ann: None,
            ag_loan_net_charge_offs_qtr: None,
            ag_loan_net_charge_offs_qtr_ratio: None,
            ag_ln_net_charge_offs_small_bks: None,
            ag_ln_net_charge_offs_small_bks_ratio: None,
            ag_ln_net_charge_offs_small_bks_quarterly: None,
            ag_ln_net_charge_offs_small_bks_quarterly_ratio: None,
            auto_loans_net_charge_offs: None,
            auto_loans_net_charge_offs_ratio: None,
            auto_lns_net_chg_offs_ann: None,
            auto_lns_net_chg_offs_qtr: None,
            auto_lns_net_chg_offs_qtr_ratio: None,
            auto_ln_chg_off_qtr_auto_ln: None,
            commercial_loan_net_charge_offs: None,
            commercial_loan_net_charge_offs_ratio: None,
            commercial_loan_net_chg_ann: None,
            non_u_s_commercial_ln_net_chg_of: None,
            non_u_s_commercial_ln_net_chg_of_ratio: None,
            non_u_s_commercial_ln_net_chg_of_quarterly: None,
            non_u_s_commercial_ln_net_chg_of_quarterly_ratio: None,
            commercial_loan_net_chg_qtr: None,
            commercial_loan_net_chg_qtr_ratio: None,
            commercial_re_chg_off_comm_re_ln: None,
            commercial_re_chg_off_comm_re_ln_quarterly: None,
            commercial_re_ln_chg_ann: None,
            consumer_loan_net_charge_offs: None,
            consumer_loan_net_charge_offs_ratio: None,
            consumer_loan_net_chg_ann: None,
            other_consumer_loan_net_chg_ann: None,
            other_consumer_ln_net_charge_off: None,
            other_consumer_ln_net_charge_off_ratio: None,
            other_consumer_ln_net_chg_qtr: None,
            other_consumer_ln_net_chg_qtr_ratio: None,
            consumer_loan_net_chg_qtr: None,
            consumer_loan_net_chg_qtr_ratio: None,
            oth_consumer_chgoff_qtr_oth_cons: None,
            credit_card_loan_net_charge_offs: None,
            credit_card_loan_net_charge_offs_ratio: None,
            credit_card_loan_net_chg_ann: None,
            credit_card_ln_net_chg_qtr: None,
            credit_card_ln_net_chg_qtr_ratio: None,
            depository_inst_loan_net_chg_off: None,
            depository_inst_loan_net_chg_off_ratio: None,
            foreign_dep_inst_ln_net_chg_offs: None,
            foreign_dep_inst_ln_net_chg_offs_ratio: None,
            foreign_dep_inst_ln_net_chg_offs_quarterly: None,
            foreign_dep_inst_ln_net_chg_offs_quarterly_ratio: None,
            depository_inst_loan_net_chg_qtr: None,
            depository_inst_loan_net_chg_qtr_ratio: None,
            foreign_govt_ln_net_chg_offs: None,
            foreign_govt_ln_net_chg_offs_ratio: None,
            foreign_gov_ln_net_chg_qtr: None,
            foreign_gov_ln_net_chg_qtr_ratio: None,
            net_income_bk_higher_pp: None,
            net_income_bank_losers: None,
            net_income_bk_loser_qtr: None,
            total_ln_ls_net_chg_ann: None,
            missing_title_ntinqhpp: None,
            net_charge_offs_loans_leases: None,
            net_charge_offs_loans_leases_quarterly: None,
            lease_net_charge_offs: None,
            lease_net_charge_offs_ratio: None,
            lease_net_charge_offs_qtr: None,
            lease_net_charge_offs_qtr_ratio: None,
            all_other_loan_net_charge_offs: None,
            all_other_loan_net_charge_offs_ratio: None,
            all_other_ln_net_chg_qtr: None,
            all_other_ln_net_chg_qtrs_ratio: None,
            amt_time_dep_of_100_000_or_less: None,
            amt_time_dep_of_100_000_or_less_ratio: None,
            nontransactn_com_bks_oth_u_s: None,
            nontransactn_com_bks_oth_u_s_ratio: None,
            real_estate_loan_net_charge_offs: None,
            missing_title_ntremuqa: None,
            missing_title_ntrecoqa: None,
            real_estate_loan_net_charge_offs_ratio: None,
            real_estate_loan_net_charge_offs_quarterly: None,
            real_estate_loan_net_charge_offs_quarterly_ntreqa: None,
            real_estate_loan_net_charge_offs_quarterly_ratio: None,
            farmland_re_ln_net_charge_offs: None,
            farmland_re_ln_net_charge_offs_ratio: None,
            farmland_re_ln_net_chg_qtr: None,
            re_ln_net_chg_ann: None,
            farmland_re_ln_net_chg_qtr_ratio: None,
            _1_4_fam_const_ln_net_off: None,
            other_construct_net_chg_off: None,
            construction_re_ln_net_chg_qtr: None,
            construction_re_ln_net_chg_qtr_ratio: None,
            construction_re_ln_net_chg_offs: None,
            const_re_loans_net_chg_ann: None,
            construction_re_ln_net_chg_offs_ratio: None,
            const_re_chg_off_const_re_loans: None,
            const_re_chg_off_const_re_loans_quarterly: None,
            real_estate_ln_net_chg_off_for: None,
            real_estate_ln_net_chg_off_for_ratio: None,
            real_estate_ln_net_chg_off_for_quarterly: None,
            real_estate_ln_net_chg_off_for_quarterly_ratio: None,
            line_of_credit_re_ln_net_chg_off: None,
            line_of_credit_re_ln_net_chg_off_ratio: None,
            line_of_credit_re_ln_net_chg_off_quarterly: None,
            line_of_credit_re_ln_net_chg_off_annually: None,
            line_of_credit_re_ln_net_chg_off_quarterly_ratio: None,
            home_equity_chg_off_home_eq_lns_quarterly_ratio: None,
            home_equity_chg_off_home_eq_lns: None,
            multifamily_re_ln_net_chg_qtr: None,
            multifamily_res_re_ln_net_chg_ann: None,
            multifamily_re_ln_net_chg_qtr_ratio: None,
            multifam_re_chg_off_multi_re_ln: None,
            multifam_re_chg_off_multi_re_ln_quarterly: None,
            multifamly_res_re_ln_net_chg_off: None,
            multifamly_res_re_ln_net_chg_off_ratio: None,
            nonfarm_nonres_re_ln_net_chg_off: None,
            nonfarm_nonres_re_ln_net_chg_off_ratio: None,
            other_nonfarm_nonrs_net_chg_off: None,
            own_occ_nonfrm_nonrs_net_chg_off: None,
            nonfarm_nonres_re_ln_net_chg_ann: None,
            nonfarm_nonres_re_ln_net_chg_qtr: None,
            nonfarm_nonres_re_ln_net_chg_qtr_ratio: None,
            nonres_chg_off_nonres_loans: None,
            nonres_chg_off_nonres_loans_quarterly: None,
            non_u_s_re_ln_net_charge_offs: None,
            non_u_s_re_ln_net_charge_offs_ratio: None,
            non_u_s_re_ln_net_charge_offs_quarterly: None,
            other_1_4_fam_re_other_ln_net_chg_ann: None,
            non_u_s_re_ln_net_charge_offs_quarterly_ratio: None,
            other_1_4_fam_re_chg_off_oth_1_4: None,
            other_1_4_fam_re_chg_off_oth_1_4_quarterly_ratio: None,
            other_1_4_fam_re_chg_off_oth_1_4_quarterly: None,
            re_charge_off_re_loans: None,
            re_charge_off_re_loans_quarterly: None,
            re_loans_1_4_family_net_chg_offs: None,
            re_loans_1_4_family_net_chg_offs_ratio: None,
            re_loans_1_4_family_net_chg_qtr: None,
            re_loans_1_4_family_net_chg_ann: None,
            re_loans_1_4_family_net_chg_qtr_ratio: None,
            _1_4_fam_re_chg_off_1_4_fam_loans: None,
            _1_4_fam_re_chg_off_1_4_fam_loans_quarterly_ratio: None,
            re_ln_1_4_fam_jr_lien_net_c_off: None,
            re_ln_1_4_fam_jr_lien_net_c_off_ratio: None,
            re_ln_1_4_fam_jr_lien_net_c_off_quarterly: None,
            re_ln_1_4_fam_jr_lien_net_c_off_quarterly_ratio: None,
            re_ln_1_4fam_ist_lien_net_c_off: None,
            re_ln_1_4fam_ist_lien_net_c_off_ratio: None,
            re_ln_1_4fam_ist_lien_net_c_off_quarterly: None,
            re_ln_1_4fam_ist_lien_net_c_off_quarterly_ratio: None,
            real_estate_loan_net_charge_offs_domestic_offices: None,
            real_estate_loan_net_charge_offs_domestic_offices_ratio: None,
            real_estate_loan_net_charge_offs_domestic_offices_quarterly: None,
            real_estate_loan_net_charge_offs_domestic_offices_quarterly_ratio: None,
            nontransaction_for_country: None,
            nontransaction_for_cntry_govt: None,
            nontransaction_for_cntry_govt_ratio: None,
            nontransaction_for_government: None,
            savings_dep_mmda: None,
            savings_dep_mmda_ratio: None,
            savings_dep_other: None,
            savings_dep_other_ratio: None,
            income_earned_not_collected: None,
            life_ins_assets_general_acc: None,
            life_ins_assets_general_acc_ratio: None,
            life_ins_assets_hybrid_acc: None,
            life_ins_assets_hybrid_acc_ratio: None,
            life_insurance_assets: None,
            life_insurance_ratio: None,
            life_ins_assets_separate_acc: None,
            life_ins_assets_separate_acc_ratio: None,
            off_balance_sheet_derivatives: None,
            all_other_re_owned_farmland: None,
            all_other_re_owned_farmland_ratio: None,
            all_other_re_owned_const: None,
            all_other_re_owned_const_ratio: None,
            all_other_re_owned_gnma_loans: None,
            direct_indirect_invest_in_ore: None,
            direct_indirect_invest_in_ore_ratio: None,
            all_other_re_owned_multi: None,
            all_other_re_owned_multi_ratio: None,
            all_other_re_owned_nonfarm: None,
            all_other_re_owned_nonfarm_ratio: None,
            other_real_estate_owned_oreoth: None,
            other_real_estate_owned_ratio_oreothr: None,
            other_real_estate_owned_for: None,
            other_real_estate_owned_for_ratio: None,
            all_other_re_owned_1_4_family_oreres: None,
            all_other_re_owned_1_4_familiy_ratio: None,
            other_borrowed_money_for: None,
            other_futures_forward_contract: None,
            other_futures_forward_contract_ratio: None,
            other_notional_value_swaps: None,
            all_oth_off_balance_sheet_liab: None,
            all_oth_off_balance_sheet_liab_ratio: None,
            other_purchased_option_contracts: None,
            other_written_option_contracts: None,
            ots_region_name: None,
            rec_own_interest_sec_ci: None,
            rec_own_interest_sec_crcd: None,
            rec_own_interest_sec_hel: None,
            c_o_own_interest_sec_ci: None,
            c_o_own_interest_sec_crcd: None,
            c_o_own_interest_sec_hel: None,
            ln_secure_held_in_sec_ci: None,
            ln_secure_held_in_sec_crcd: None,
            ln_secure_held_in_sec_hel: None,
            pd_30_89_own_interest_sec_ci: None,
            pd_30_89_own_interest_sec_crcd: None,
            pd30_89_own_interest_sec_hel: None,
            pd_90_own_interest_sec_ci: None,
            pd_90_own_interest_sec_crcd: None,
            pd_90_own_interest_sec_hel: None,
            sec_secure_held_in_rc_b_ci: None,
            sec_secure_held_in_rc_b_crcd: None,
            sec_secure_held_in_rc_b_hel: None,
            _30_89_days_p_d_agricultural_lns: None,
            _30_89_days_p_d_agricultural_lns_ratio: None,
            _30_89_days_p_d_ag_lns_small_bks: None,
            _30_89_days_p_d_ag_lns_small_bks_ratio: None,
            _30_89_days_p_d_total_assets: None,
            _30_89_days_p_d_total_assets_ratio: None,
            _30_89_days_p_d_auto_loans: None,
            _30_89_days_p_d_auto_loans_ratio: None,
            _30_89_days_p_d_c_i_loans: None,
            _30_89_days_p_d_c_i_loans_ratio: None,
            _30_89_days_p_d_c_i_non_u_s: None,
            _30_89_days_p_d_c_i_non_u_s_ratio: None,
            _30_89_days_p_d_consumer_loans: None,
            _30_89_days_p_d_consumer_loans_ratio: None,
            _30_89_days_p_d_other_consumer: None,
            _30_89_days_p_d_other_consumer_ratio: None,
            _30_89_days_p_d_credit_card_plans: None,
            _30_89_days_p_d_credit_card_plans_ratio: None,
            _30_89_days_p_d_dep_inst_loans: None,
            _30_89_days_p_d_dep_inst_loans_p3depr: None,
            _30_89_days_p_d_dep_inst_non_u_s: None,
            _30_89_days_p_d_dep_inst_non_u_s_p3depnusr: None,
            _30_89_days_p_d_foreign_govt: None,
            _30_89_days_p_d_foreign_govt_ratio: None,
            _30_89_days_p_d_gty_ln_ls: None,
            _30_89_days_p_d_gty_ln_ls_p3gtyr: None,
            _30_89_day_p_d_rebooked_gnma_lns: None,
            _30_89_day_p_d_rebooked_gnma_lns_p3gtygnmr: None,
            _30_89_days_p_d_part_gty_ln_ls: None,
            _30_89_days_p_d_part_gty_ln_ls_ratio: None,
            _30_89_day_p_d_ag_loans_loss_sh: None,
            _30_89_day_p_d_ag_loans_loss_sh_ratio: None,
            _30_89_days_p_d_c_i_lns_loss_sh: None,
            _30_89_days_p_d_c_i_lns_loss_sh_ratio: None,
            _30_89_d_p_d_consumer_loss_sh: None,
            _30_89_d_p_d_consumer_loss_sh_ratio: None,
            _30_89_p_d_protect_gty_loss_sh: None,
            _30_89_p_d_protect_gty_loss_sh_ratio: None,
            _30_89_days_p_d_l_l_held_for_sale: None,
            _30_89_days_p_d_l_l_held_for_sale_ratio: None,
            _30_89_days_p_d_oth_lns_loss_sh: None,
            _30_89_days_p_d_oth_lns_loss_sh_ratio: None,
            _30_89_day_p_d_re_farm_loss_sh: None,
            _30_89_day_p_d_re_farm_loss_sh_ratio: None,
            _30_89_p_d_construction_loss_sh: None,
            _30_89_p_d_construction_loss_sh_ratio: None,
            _30_89_day_p_d_multifam_loss_sh: None,
            _30_89_day_p_d_multifam_loss_sh_ratio: None,
            _30_89_p_d_nonfrm_nonrs_loss_sh: None,
            _30_89_p_d_nonfrm_nonrs_loss_sh_ratio: None,
            _30_89_d_p_d_1_4_family_loss_sh: None,
            _30_89_p_d_1_4_family_loss_sh_ratio: None,
            _30_89_days_p_d_leases: None,
            _30_89_days_p_d_leases_ratio: None,
            _30_89_d_p_d_total_loans_loss_sh: None,
            _30_89_days_p_d_total_loans_loss_sh_ratio: None,
            _30_89_days_p_d_all_other_loans: None,
            _30_89_days_p_d_all_other_loans_ratio: None,
            _30_89_days_p_d_real_estate_loans: None,
            _30_89_days_p_d_real_estate_loans_ratio: None,
            _30_89_days_p_d_re_farmland: None,
            _30_89_days_p_d_re_farmland_p3reagr: None,
            _30_89_days_p_d_1_4_fam_constr_ln: None,
            _30_89_days_p_d_1_4_fam_constr_ln_p3recnfmr: None,
            _30_89_days_p_d_oth_constr_land: None,
            _30_89_days_p_d_oth_constr_land_p3recnotr: None,
            _30_89_days_p_d_re_construction: None,
            _30_89_days_p_d_re_construction_p3reconsr: None,
            _30_89_days_p_d_re_foreign: None,
            _30_89_days_p_d_re_foreign_ratio: None,
            _30_89_days_p_d_re_1_4_fam_lines: None,
            _30_89_days_p_d_re_1_4_fam_lines_ratio: None,
            _30_89_days_p_d_re_multifamily: None,
            _30_89_days_p_d_re_multifamily_p3remultr: None,
            _30_89_days_p_d_re_nonfarm_nonres: None,
            _30_89_days_p_d_re_nonfarm_nonres_p3renresr: None,
            _30_89_days_p_d_oth_nonfrm_nonres: None,
            _30_89_days_p_d_oth_nonfrm_nonres_p3renrotr: None,
            _30_89_days_p_d_0wn_occ_nonf_nonrs: None,
            _30_89_days_p_d_own_occ_nonf_nonrs_ratio: None,
            _30_89_days_p_d_re_non_u_s: None,
            _30_89_days_p_d_re_non_u_s_p3renusr: None,
            _30_89_days_p_d_re_1_4_family: None,
            _30_89_days_p_d_re_1_4_family_p3reresr: None,
            _30_89_days_p_d_re_1_4_jn_lien: None,
            _30_89_days_p_d_re_1_4_jn_lien_ratio: None,
            _30_89_days_p_d_re_1_4_ist_lien: None,
            _30_89_days_p_d_re_1_4_ist_lien_ratio: None,
            _30_89_day_p_d_restruct_c_i_ln: None,
            _30_89_p_d_restruct_construction: None,
            _30_89_day_p_d_restr_ln_1_4_fam: None,
            _30_89_day_p_d_restr_ln_1_4_fam_ratio: None,
            _30_89_d_p_d_restr_ln_excl1_4_fm: None,
            _30_89_d_p_d_restr_ln_excl1_4_fm_ratio: None,
            _30_89_day_p_d_restr_ln_total: None,
            _30_89_day_p_d_restr_ln_total_ratio: None,
            _30_89_d_p_d_restruct_multifam: None,
            _30_89_day_p_d_restruct_nfnr_ln: None,
            _30_89_d_p_d_restruct_all_oth_ln: None,
            _30_89_days_p_d_debt_securities: None,
            _30_89_days_p_d_debt_securities_ratio: None,
            _90_days_p_d_agricultural_lns: None,
            _90_days_p_d_agricultural_lns_ratio: None,
            _90_days_p_d_ag_lns_small_bks: None,
            _90_days_p_d_ag_lns_small_bks_ratio: None,
            _90_days_p_d_total_assets: None,
            _90_days_p_d_total_assets_ratio: None,
            _90_days_p_d_auto_loans: None,
            _90_days_p_d_auto_loans_ratio: None,
            _90_days_p_d_c_i_loans: None,
            _90_days_p_d_c_i_loans_ratio: None,
            _90_days_p_d_c_i_non_u_s: None,
            _90_days_p_d_c_i_non_u_s_ratio: None,
            _90_days_p_d_consumer_loans: None,
            _90_days_p_d_consumer_loans_ratio: None,
            _90_days_p_d_other_consumer: None,
            _90_days_p_d_other_consumer_ratio: None,
            _90_days_p_d_credit_card_plans: None,
            _90_days_p_d_credit_card_plans_ratio: None,
            _90_days_p_d_dep_inst_loans: None,
            _90_days_p_d_dep_inst_loans_ratio: None,
            _90_days_p_d_dep_inst_non_u_s: None,
            _90_days_p_d_dep_inst_non_u_s_ratio: None,
            _90_days_p_d_foreign_govt: None,
            _90_days_p_d_foreign_govt_ratio: None,
            _90_days_p_d_gty_ln_ls: None,
            _90_days_p_d_gty_ln_ls_p9gtyr: None,
            _90_days_p_d_rebooked_gnma_lns: None,
            _90_day_p_d_rebooked_gnma_lns: None,
            _90_days_p_d_part_gty_ln_ls: None,
            _90_days_p_d_part_gty_ln_ls_ratio: None,
            _90_days_p_d_ag_loans_loss_sh: None,
            _90_days_p_d_ag_loans_loss_sh_ratio: None,
            _90_days_p_d_c_i_lns_loss_sh: None,
            _90_days_p_d_c_i_lns_loss_sh_ratio: None,
            _90_d_p_d_consumer_ln_loss_sh: None,
            _90_d_p_d_consumer_ln_loss_sh_ratio: None,
            _90_d_p_d_protect_gty_loss_sh: None,
            _90_d_p_d_protect_gty_loss_sh_ratio: None,
            _90_days_p_d_l_l_held_for_sale: None,
            _90_days_p_d_l_l_held_for_sale_ratio: None,
            _90_days_p_d_other_lns_loss_sh: None,
            _90_days_p_d_other_lns_loss_sh_ratio: None,
            _90_day_p_d_re_farm_loss_sh: None,
            _90_day_p_d_re_farm_loss_sh_ratio: None,
            _90_d_p_d_construction_loss_sh: None,
            _90_d_p_d_construction_loss_sh_ratio: None,
            _90_day_p_d_multifam_loss_sh: None,
            _90_day_p_d_multifam_loss_sh_ratio: None,
            _90_d_p_d_nfnr_loss_share: None,
            _90_d_p_d_nfnr_loss_sh_ratio: None,
            _90_d_p_d_1_4_family_loss_sh: None,
            _90_d_p_d_1_4_family_loss_sh_ratio: None,
            _90_days_p_d_leases: None,
            _90_days_p_d_leases_ratio: None,
            _90_d_p_d_total_loans_loss_sh: None,
            _90_d_p_d_total_loans_loss_sh_ratio: None,
            _90_days_p_d_all_other_loans: None,
            _90_days_p_d_all_other_loans_ratio: None,
            _90_days_p_d_real_estate_loans: None,
            _90_days_p_d_real_estate_ratio: None,
            _90_days_p_d_re_farmland: None,
            _90_days_p_d_re_farmland_p9reagr: None,
            _90_days_p_d_1_4_fam_construc_ln: None,
            _90_days_p_d_1_4_fam_construc_ln_ratio: None,
            _90_days_p_d_other_constr_land: None,
            _90_days_p_d_other_constr_land_ratio: None,
            _90_days_p_d_re_construction: None,
            _90_days_p_d_re_construction_ratio: None,
            _90_days_p_d_re_foreign: None,
            _90_days_p_d_re_foreign_ratio: None,
            _90_days_p_d_re_1_4_fam_lines: None,
            _90_days_p_d_re_1_4_fam_lines_ratio: None,
            _90_days_p_d_re_multifamily: None,
            _90_days_p_d_re_multifamily_ratio: None,
            _90_days_p_d_re_nonfarm_nonres: None,
            _90_days_p_d_re_nonfarm_nonres_ratio: None,
            _90_days_p_d_other_nonfrm_nonres: None,
            _90_days_p_d_other_nonfrm_nonres_ratio: None,
            _90_days_p_d_0wn_occ_nonfr_nonrs: None,
            _90_days_p_d_own_occ_nonfr_nonrs_ratio: None,
            _90_days_p_d_re_non_u_s: None,
            _90_days_p_d_re_non_u_s_p9renusr: None,
            _90_days_p_d_re_1_4_family: None,
            _90_days_p_d_re_1_4_family_ratio: None,
            _90_days_p_d_re_1_4_jn_lien: None,
            _90_days_p_d_re_1_4_jn_lien_ratio: None,
            _90_days_p_d_re_1_4_ist_lien: None,
            _90_days_p_d_re_1_4_ist_lien_ratio: None,
            _90_day_p_d_restruct_c_i_ln: None,
            _90_d_p_d_restruct_construction: None,
            _90_days_p_d_restr_ln_1_4_fam: None,
            _90_days_p_d_restr_ln_1_4_fam_ratio: None,
            _90_day_p_d_restru_ln_excl_1_4_fm: None,
            _90_day_p_d_restru_ln_excl_1_4_fm_ratio: None,
            _90_day_p_d_restr_ln_total: None,
            _90_day_p_d_restr_ln_total_ratio: None,
            _90_day_p_d_restruct_multifam: None,
            _90_day_p_d_restruct_nfnr_ln: None,
            _90_d_p_d_restruct_all_oth_ln: None,
            _90_days_p_d_debt_securities: None,
            _90_days_p_d_debt_securities_ratio: None,
            participations_acquired: None,
            participations_conveyed: None,
            participations_conveyed_ratio: None,
            allowance_for_l_l_in_tier_2: None,
            allowance_for_l_l_in_tier_2_ratio: None,
            rbc_total_pca: None,
            tier_1_rbc_pca: None,
            rbc_tier2_pca: None,
            rbc_tier2_pca_ratio: None,
            rc_r_common_eq_tier_1_capital: None,
            common_equity_tier_1_ratio: None,
            tier_1_rbc_adjusted_llr_pca: None,
            tier_1_rbc_adjusted_llr_pca_ratio: None,
            leverage_ratio_pca: None,
            tier_1_rbc_ratio_pca: None,
            total_rbc_ratio_pca: None,
            repurchase_agreement_for: None,
            reverse_repurchase_agreement_for: None,
            retained_earnings_avg_bk_equity: None,
            restructured_ln_c_i: None,
            restructured_ln_construction: None,
            restructured_ln_excl_1_4_fm: None,
            restructured_ln_excl_1_4_fm_ratio: None,
            restructured_loans_total: None,
            restructured_loans_total_ratio: None,
            restructured_loans_1_4_family: None,
            restructured_loans_1_4_family_ratio: None,
            restructured_ln_multifamily: None,
            restruct_ln_nonfarm_nonres: None,
            restructured_ln_all_other: None,
            federal_reserve_id_number: None,
            interest_rate_total_contracts: None,
            int_rate_futures_forward_contr: None,
            int_rate_swaps: None,
            int_rate_pur_option_contracts: None,
            int_rate_written_option_contract: None,
            rwa_adjust_pca_t1_cet1_ratio: None,
            rwa_adjusted_pca_total_rbc_rat: None,
            rwa_adjusted_pca_total_rbc_rat_ratio: None,
            abs_total_b_s: None,
            abs_total_b_s_ratio: None,
            securities_af: None,
            securities_af_ratio: None,
            u_s_agency_all_oth: None,
            commercial_mbs_total: None,
            other_commercial_mbs_govt: None,
            other_commercial_mbs_govt_ratio: None,
            other_commercial_mbs: None,
            other_commercial_mbs_ratio: None,
            commercial_mbs_pass_through: None,
            commercial_mbs_pass_through_ratio: None,
            u_s_agency_collateral_mtg_res: None,
            u_s_agency_collateral_mtg_res_ratio: None,
            comm_mbs_pass_thru_govt: None,
            comm_mbs_pass_thru_govt_ratio: None,
            eq_sec_readily_det_fv: None,
            eq_sec_readily_det_fv_ratio: None,
            u_s_agency_issued_fnma_res: None,
            u_s_agency_issued_fnma_res_ratio: None,
            u_s_agency_gty_by_gnma: None,
            u_s_agency_gty_by_gnma_ratio: None,
            u_s_agency_issued_or_gty_res: None,
            u_s_agency_issued_or_gty_res_ratio: None,
            securities_ha: None,
            securities_ha_ratio: None,
            less_allow_for_credit_losses_on_held_to_maturity_debt_securities: None,
            less_allow_for_credit_losses_on_held_to_maturity_debt_securities_ratio: None,
            securities_lent: None,
            securities_lent_ratio: None,
            nonmtg_debt_sec_1_3_years: None,
            nonmtg_debt_sec_1_3_years_ratio: None,
            nonmtg_debt_sec_3_mons_or_less: None,
            nonmtg_debt_sec_3_mons_or_less_ratio: None,
            nonmtg_debt_sec_3_5_years: None,
            nonmtg_debt_sec_3_5_years_ratio: None,
            nonmtg_debt_sec_3_12_months: None,
            nonmtg_debt_sec_3_12_months_ratio: None,
            nonmtg_debt_sec_5_15_years: None,
            nonmtg_debt_sec_5_15_years_ratio: None,
            nonmtg_debt_sec_over_15_years: None,
            nonmtg_debt_sec_over_15_years_ratio: None,
            oth_mortgage_sec_3_yr_or_less: None,
            oth_mortgage_sec_3_yr_or_less_ratio: None,
            fixed_and_floating_rate_debt_securities_included_above_with_remaining_maturity_of_one_year_or_less: None,
            fixed_and_floating_rate_debt_securities_included_above_with_remaining_maturity_of_one_year_or_less_ratio: None,
            oth_dom_debt_all_other: None,
            oth_dom_debt_all_other_ratio: None,
            cmo_priv_issued: None,
            cmo_priv_issued_ratio: None,
            oth_mortgage_sec_over_3_yrs: None,
            oth_mortgage_sec_over_3_yrs_ratio: None,
            pledged_securities: None,
            pledged_securities_ratio: None,
            mtg_pass_thru_sec_1_3_years: None,
            mtg_pass_thru_sec_1_3_years_ratio: None,
            mtg_pass_thru_sec_3_mon_or_less: None,
            mtg_pass_thru_sec_3_mon_or_less_ratio: None,
            mtg_pass_thru_sec_3_5_years: None,
            mtg_pass_thru_sec_3_5_years_ratio: None,
            mtg_pass_thru_sec_3_12_months: None,
            mtg_pass_thru_sec_3_12_months_ratio: None,
            mtg_pass_thru_sec_5_15_years: None,
            mtg_pass_thru_sec_5_15_years_ratio: None,
            mtg_pass_thru_sec_over_15_yrs: None,
            mtg_pass_thru_sec_over_15_yrs_ratio: None,
            debt_securities: None,
            debt_securities_ratio: None,
            structured_fin_prod_total: None,
            structured_fin_prod_total_ratio: None,
            structured_notes_amortized_cost: None,
            structured_notes_amortized_cost_ratio: None,
            structured_notes_fair_value: None,
            structured_notes_fair_value_ratio: None,
            u_s_agency_govt_sponsored: None,
            _30_89_pd_ln_securitization_auto: None,
            _30_89_pd_ln_securitization_auto_ratio: None,
            _30_89_pd_ln_securitization_ci: None,
            _30_89_pd_ln_securitization_ci_ratio: None,
            _30_89_pd_ln_securitization_con: None,
            _30_89_pd_ln_securitization_con_ratio: None,
            _30_89_pd_ln_securitization_crcd: None,
            _30_89_pd_ln_securitization_crcd_ratio: None,
            _30_89_pd_ln_securitization_hel: None,
            _30_89_pd_ln_securitization_hel_ratio: None,
            _30_89_pd_ln_securitization_oth: None,
            _30_89_pd_ln_securitization_oth_ratio: None,
            _30_89_pd_ln_securitization_res: None,
            _30_89_pd_ln_securitization_res_ratio: None,
            _90_pd_ln_securitization_auto: None,
            _90_pd_ln_securitization_auto_ratio: None,
            _90_pd_ln_securitization_ci: None,
            _90_pd_ln_securitization_ci_ratio: None,
            _90_pd_ln_securitization_con: None,
            _90_pd_ln_securitization_con_ratio: None,
            _90_pd_ln_securitization_crcd: None,
            _90_pd_ln_securitization_crcd_ratio: None,
            _90_pd_ln_securitization_hel: None,
            _90_pd_ln_securitization_hel_ratio: None,
            _90_pd_ln_securitization_oth: None,
            _90_pd_ln_securitization_oth_ratio: None,
            _90_pd_ln_securitization_res: None,
            _90_pd_ln_securitization_res_ration: None,
            rec_asset_securitization_auto: None,
            rec_asset_securitization_auto_szcrautor: None,
            outstdg_cc_fees_in_securitzd_cc: None,
            outstdg_cc_fees_in_securitzd_cc_ratio: None,
            rec_asset_securitization_ci: None,
            rec_asset_securitization_ci_ratio: None,
            rec_asset_securitization_con: None,
            rec_asset_securitization_con_ratio: None,
            rec_asset_securitization_crcd: None,
            rec_asset_securitization_crcd_ratio: None,
            re_prin_sec_asset_sold_hel: None,
            re_prin_sec_asset_sold_hel_ratio: None,
            rec_asset_securitization: None,
            rec_asset_securitization_ratio: None,
            rec_asset_securitization_res: None,
            rec_asset_securitization_res_ratio: None,
            c_o_on_asset_securitization_auto: None,
            c_o_on_asset_securitization_auto_ratio: None,
            c_o_on_asset_securitization_ci: None,
            c_o_on_asset_securitization_ci_ratio: None,
            c_o_on_asset_securitization_con: None,
            c_o_on_asset_securitization_con_ratio: None,
            c_o_on_asset_securitization_crcd: None,
            c_o_on_asset_securitization_crcd_ratio: None,
            c_o_on_asset_securitization_hel: None,
            c_o_on_asset_securitization_hel_ratio: None,
            c_o_on_asset_securitization_oth: None,
            c_o_on_asset_securitization_oth_ratio: None,
            c_o_on_asset_securitization_res: None,
            cr_exp_on_securitizatn_auto: None,
            cr_exp_on_securitizatn_auto_ratio: None,
            cr_exp_on_securitizatn_crcd: None,
            cr_exp_on_securitizatn_crcd_ratio: None,
            cr_exp_on_securitizatn_ci: None,
            cr_exp_on_securitizatn_ci_ratio: None,
            cr_exp_on_securitizatn_con: None,
            cr_exp_on_securitizatn_con_ratio: None,
            cr_exp_on_securitizatn_hel: None,
            cr_exp_on_securitizatn_hel_ratio: None,
            cr_exp_on_securitizatn_oth: None,
            cr_exp_on_securitizatn_oth_ratio: None,
            cr_exp_on_securitization_res: None,
            cr_exp_on_securitization_res_ratio: None,
            re_prin_sec_asset_sold_auto: None,
            re_prin_sec_asset_sold_auto_ratio: None,
            re_prin_sec_asset_sold_ci: None,
            re_prin_sec_asset_sold_ci_ratio: None,
            re_prin_sec_asset_sold_cons: None,
            re_prin_sec_asset_sold_cons_ratio: None,
            re_prin_sec_asset_sold_crcd: None,
            re_prin_sec_asset_sold_crcd_ratio: None,
            re_prin_sec_asset_sold_hel_szlnhel: None,
            re_prin_sec_asset_sold_hel_ratio_szlnhelr: None,
            re_prin_sec_asset_sold_oth: None,
            re_prin_sec_asset_sold_oth_ratio: None,
            re_prin_sec_asset_sold_res: None,
            re_prin_sec_asset_sold_res_ratio: None,
            commits_for_liquidity_auto: None,
            commits_for_liquidity_ci: None,
            commits_for_liquidity_con: None,
            commits_for_liquidity_crcd: None,
            commits_for_liquidity_hel: None,
            commits_for_liquidity_oth: None,
            commits_for_liquidity_res: None,
            corp_trust_managed_amt: None,
            corp_trust_managed_num: None,
            corp_trust_non_managed_amt: None,
            corp_trust_non_managed_num: None,
            corp_trust_trusteeships_num: None,
            corp_muni_trustee_default_num: None,
            corp_trust_trusteeships_amt: None,
            corp_muni_trustee_default_amt: None,
            corp_trust_transfer_num: None,
            cifs_dom_equity_amt: None,
            cifs_dom_equity_num: None,
            cifs_intl_global_eq_amt: None,
            cifs_intl_global_eq_num: None,
            cifs_municipal_bond_amt: None,
            cifs_municipal_bond_num: None,
            cifs_stock_bond_amt: None,
            cifs_stock_bond_num: None,
            cust_and_safe_acct_non_man_amt: None,
            cust_and_safe_acct_non_man_num: None,
            cifs_specialty_other_amt: None,
            cifs_specialty_other_num: None,
            cifs_short_term_inv_amt: None,
            cifs_short_term_inv_num: None,
            cifs_taxable_bond_amt: None,
            cifs_taxable_bond_num: None,
            cifs_total_amt: None,
            cifs_total_num: None,
            emp_bene_def_bene_manage_amt: None,
            emp_bene_def_bene_managed_num: None,
            emp_bene_def_bene_non_man_amt: None,
            emp_bene_def_bene_non_man_num: None,
            emp_bene_contrib_managed_amt: None,
            emp_bene_contri_managed_num: None,
            emp_bene_contri_non_man_amt: None,
            emp_bene_contri_non_manage_num: None,
            emp_ben_ret_tr_com_pf_stk: None,
            emp_ben_ret_tr_eq_mut_fund: None,
            emp_ben_ret_tr_int_bearing: None,
            emp_ben_ret_tr_tot_manage_ast: None,
            emp_ben_ret_tr_misc_asset: None,
            emp_ben_ret_tr_money_mkt: None,
            emp_ben_ret_tr_nonint_bear: None,
            emp_ben_ret_tr_oth_note_bnd: None,
            emp_ben_ret_tr_oth_mut_fund: None,
            emp_ben_ret_tr_real_estate: None,
            emp_ben_ret_tr_re_mtg: None,
            emp_ben_ret_tr_muni: None,
            emp_ben_ret_tr_u_s_treas_ob: None,
            emp_ben_ret_tr_shrt_term_ob: None,
            expense_fiduciary_ytd: None,
            emp_ben_ret_tr_trust_fund: None,
            emp_ben_ret_tr_unreg_funds: None,
            foundation_endow_managed_amt: None,
            foundation_endow_managed_num: None,
            foundation_end_non_manage_amt: None,
            foundation_end_non_manage_num: None,
            gr_inc_corp_trust_agency_ytd: None,
            gr_inc_custody_ytd: None,
            gr_inc_emp_benefit_benefit_ytd: None,
            gr_inc_emp_benefit_contri_ytd: None,
            gr_inc_foundation_endow_ytd: None,
            gr_inc_investment_agcy_ytd: None,
            investment_agency_managed_amt: None,
            investment_agency_managed_num: None,
            investment_agcy_non_managed_amt: None,
            investment_agcy_non_managed_num: None,
            intracompany_inc_fiduciary_ytd: None,
            gr_inc_other_fiduciary_ytd: None,
            gr_inc_other_retirement_ytd: None,
            gr_inc_personal_ag_accts_ytd: None,
            gr_inc_related_serv_ytd: None,
            tot_foreign_off_gross_fiduc_ytd: None,
            fiduciary_fgn_off_managed_amt: None,
            fiduciary_fgn_off_managed_amt_tmafnum: None,
            advised_sponsored_mut_fnd_amt: None,
            advised_sponsored_mutal_fnd_num: None,
            net_fiduciary_income_ytd: None,
            net_loss_from_fiduciary_ytd: None,
            fiduciary_fgn_off_non_man_amt: None,
            fiduciary_fgn_off_non_man_num: None,
            all_oth_man_asset_com_pfd_stk: None,
            all_oth_manage_ast_eq_mut_fnd: None,
            oth_fiduciary_managed_amt: None,
            oth_fiduciary_managed_num: None,
            oth_fiduciary_non_managed_amt: None,
            oth_fiduciary_non_managed_num: None,
            all_oth_manage_asset_int_bear: None,
            all_other_managed_asset_total: None,
            all_oth_man_asset_misc_asset: None,
            all_oth_manage_ast_money_mkt: None,
            all_oth_man_asset_nonint_bear: None,
            all_oth_man_ast_oth_note_bnd: None,
            all_oth_man_asset_oth_mut_fnd: None,
            all_oth_man_asset_real_estate: None,
            all_other_manage_asset_re_mtg: None,
            oth_retirement_managed_amt: None,
            oth_retirement_managed_num: None,
            oth_retirement_non_man_amt: None,
            oth_retirement_non_man_num: None,
            all_other_managed_asset_muni: None,
            all_oth_man_ast_u_s_treas_ob: None,
            all_oth_man_ast_shrt_term_obl: None,
            all_oth_man_asset_trust_fund: None,
            all_oth_man_asset_unreg_funds: None,
            per_tr_inv_agy_com_prf_stk: None,
            per_tr_inv_agy_eq_mut_fund: None,
            per_tr_inv_agy_int_bearing: None,
            per_tr_inv_agy_tot_manage_ast: None,
            per_tr_inv_agy_misc: None,
            per_tr_inv_agy_money_mkt: None,
            per_tr_inv_agy_nonint_bearing: None,
            per_tr_inv_agy_oth_note_bnd: None,
            per_tr_inv_agy_oth_mut_fund: None,
            per_tr_inv_agy_real_estate: None,
            per_tr_inv_agy_re_mtg: None,
            per_tr_inv_agy_muni: None,
            per_tr_inv_agy_u_s_treas_ob: None,
            per_tr_inv_agy_shrt_term_ob: None,
            per_tr_inv_agy_trust_fund: None,
            per_tr_inv_agy_unreg_funds: None,
            managed_asset_per_agen_amt: None,
            managed_asset_per_agen_num: None,
            non_managed_per_agen_amt: None,
            non_managed_asset_per_agen_num: None,
            trust_powers_exercised: None,
            trading_accounts_for: None,
            ira: None,
            ira_trhmanum: None,
            ira_trhnma: None,
            ira_trhnmnum: None,
            trade_derivatives_neg_val: None,
            trade_derivated_neg_val_ratio: None,
            transaction_com_bks_other: None,
            transaction_com_bks_other_ratio: None,
            transaction_for_country: None,
            transaction_for_country_govt: None,
            transaction_for_country_govt_ratio: None,
            transaction_foreign_government: None,
            amt_non_interest_bearing_transaction_acc_more_than_250_000: None,
            amt_non_interest_bearing_transaction_acc_more_than_250_000_trnniar: None,
            num_non_interest_bearing_transaction_acc_more_than_250_000: None,
            institution_has_trust_power: None,
            trade_deriv_pos_val_dom: None,
            trade_deriv_pos_value_for: None,
            revaluation_gains_on_off_balance_sheet_contracts: None,
            revaluation_gains_on_off_balance_sheet_contracts_ratio: None,
            tot_fiduciary_accts_man_amt: None,
            tot_fiduciary_accts_man_num: None,
            tot_fiduciary_accts_non_man_amt: None,
            tot_fiduciary_accts_non_man_num: None,
            unused_commit_total: None,
            unused_commit_total_ratio: None,
            unused_commit_com_re: None,
            unused_commit_com_re_ratio: None,
            unused_commit_secured_com_re: None,
            unused_commit_secured_com_re_ratio: None,
            unused_commit_unsecured_com_re: None,
            unused_commit_unsecured_com_re_ratio: None,
            unused_commit_credit_card_lines: None,
            unused_commit_credit_card_lines_ratio: None,
            unused_commit_total_loans: None,
            unused_commit_home_equity_lines: None,
            unused_commit_home_equity_lines_ratio: None,
            unused_commit_all_other: None,
            unused_commit_all_other_ratio: None,
            unused_com_over_1_yr_rc_r_col_a: None,
            unused_com_over_1_yr_rc_r_col_a_ratio: None,
            unused_commit_sec_underwriting: None,
            unused_commit_sec_underwriting_ratio: None,
            unused_commit_for_secur_auto: None,
            unused_commit_for_secur_ci: None,
            unused_commit_for_secur_con: None,
            unused_commit_for_secur_crcd: None,
            unused_commit_for_secur_hel: None,
            unused_commit_for_secur_oth: None,
            unused_commit_for_secur_res: None,
            unearned_income_for: None,
            unearned_income_for_ratio: None,
            volatile_liabilities: None,
            volatile_liabilities_ratio: None,
            zip_code: None,
            nonmortgage_loans_in_process: None,
            unamortized_yield_adj_nonmtg_lns: None,
            loan_lease_income: None,
            banks_unit: None,
            pre_tax_net_income_operating_income: None,
            pre_tax_net_income_operating_income_ratio: None,
            pre_tax_net_income_operating_income_quarterly: None,
            pre_tax_net_income_operating_income_quarterly_ratio: None,
            additional_noninterest_income: None,
            additional_noninterest_income_ratio: None,
            additional_noninterest_income_quarterly: None,
            additional_noninterest_income_quarterly_ratio: None,
            quarterly_average_amount_of_assets_purchased_under_the_mmlf_and_excluded_from_total_assets_for_the_leverage_ratio: None,
            quarterly_average_amount_of_assets_purchased_under_the_mmlf_and_excluded_from_total_assets_for_the_leverage_ratio_ratio: None,
            quarterly_average_amount_of_ppp_loans_pledged_to_the_ppplf_and_excluded_from_total_assets_for_the_leverage_ratio: None,
            quarterly_average_amount_of_ppp_loans_pledged_to_the_ppplf_and_excluded_from_total_assets_for_the_leverage_ratio_ratio: None,
            outstanding_balance_of_assets_purchased_under_the_money_market_mutual_fund_liquidity_facility_mmlf: None,
            outstanding_balance_of_assets_purchased_under_the_money_market_mutual_fund_liquidity_facility_mmlf_ratio: None,
            outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_more_than_one_year: None,
            outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_more_than_one_year_ratio: None,
            outstanding_balance_of_ppp_loans: None,
            outstanding_balance_of_ppp_loans_ratio: None,
            number_of_ppp_loans_outstanding: None,
            number_of_ppp_loans_outstanding_ratio: None,
            outstanding_balance_of_ppp_loans_pledged_to_the_ppplf: None,
            outstanding_balance_of_ppp_loans_pledged_to_the_ppplf_ratio: None,
            outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_one_year_or_less: None,
            outstanding_balance_under_the_ppplf_with_a_remaining_maturity_of_one_year_or_less_ratio: None,
            commercial_industrial_loans: None,
            commercial_industrial_loans_quarterly: None,
            loans_to_individuals: None,
            credit_cards_related_plans: None,
            other_loans_to_individuals: None,
            other_loans_to_individuals_idntcooqr: None,
            credit_cards_related_plans_quarterly: None,
            missing_title_instcnt: None,
            missing_title_idntilr: None,
            missing_title_idothnii: None,
            automobile_loans: None,
            other_consumer_loans: None,
            earnings_coverage_of_net_loan_charge_offs_x: None,
            earnings_coverage_of_net_loan_charge_offs: None,
            cash_dividends_to_net_income_ytd_only: None,
            notional_amount_of_credit_derivatives: None,
            commercial_re_chg_off_comm_re_ln_quarterly_ratio: None,
            net_charge_offs_all_other_loans_leases_including_farm_numerator: None,
            net_charge_offs_all_other_loans_leases_including_farm_denominator: None,
            all_other_loans_leases_including_farm: None,
            net_charge_offs_all_other_loans_leases_including_farm: None,
            other_loans_to_individuals_idnccoor: None,
            all_other_loans_leases_including_farm_idncothr: None,
            commercial_industrial_loans_ratio: None,
            loans_to_individuals_ratio: None,
            credit_cards_related_plans_ratio: None,
            automobile_loans_ratio: None,
            missing_title_idntator: None,
            missing_title_idntcotr: None,
            iddepinr: None,
            missing_title_iddivnir: None,
            other_consumer_loans_ratio: None,
            interest_income_to_earning_assets_ratio: None,
            noncurrent_loans_which_are_wholly_or_partially_guaranteed_by_the_u_s_government_ratio: None,
            net_loans_and_leases_to_core_deposits_ratio: None,
            id_no_cb_flag: None,
            id_no_j_cb_flag: None,
            common_equity_tier_1_capital_ratio: None,
            tier_1_risk_based_capital_ratio: None,
            equity_securities_not_held_for_trading: None,
            priv_issued_res_mortgage_backed_securities: None,
            priv_issued_res_mortgage_backed_securities_ratio: None,
            u_s_government_obligations: None,
            u_s_government_obligations_ratios: None,
            other_comm_mortgage_backed_sec: None,
            other_comm_mortgage_backed_sec_sccmosr: None,
            assets_held_in_trading_accounts_for_tfr_reporters: None,
            loans_and_leases_gross: None,
            loans_and_leases_gross_ratio: None,
            all_oth_assets: None,
            all_oth_assets_ratio: None,
            percentage_insured_estimated: None,
            percentage_insured_estimated_ratio: None,
            p_d_30_89_real_estate_loans_in_domestic_offices: None,
            p_d_30_89_real_estate_loans_in_domestic_offices_ratio: None,
            _90_real_estate_loans_in_domestic_offices: None,
            _90_real_estate_loans_in_domestic_offices_ratio: None,
            _90_real_estate_loans_in_domestic_offices_narelndo: None,
            _90_real_estate_loans_in_domestic_offices_ratio_narelndor: None,
            state_and_county_nunber: None,
            metropolitan_statistical_area: None,
            date_of_deposit_insurance: None,
            last_structure_change_process_date: None,
            total_assets_ratio: None,
            avg_total_assets: None,
            brokered_dep_insured_large: None,
            rc_r_total_adj_ded_com_eq_t1: None,
            rc_r_com_equity_t1_before_adj: None,
            total_deposits_cavg2: None,
            total_deposits_cavg5: None,
            interest_bearing_dep_y1: None,
            int_expense_time_cd_gt_250: None,
            int_exp_time_cd_gt_250: None,
            int_exp_time_cd_gt_250_ecd100q: None,
            fed_funds_repo_int_expense_ann: None,
            int_exp_time_cd_le_250: None,
            int_expense_time_cd_le_250: None,
            int_exp_time_cd_le_250_eothtimq: None,
            undivided_profits: None,
            nontransaction_sav_accts_int_exp: None,
            nontransact_sav_acct_int_ext_ann: None,
            nontransact_sav_acct_int_exp_qtr: None,
            subordinated_notes_int_exp_ann: None,
            transaction_accounts_int_expense: None,
            transaction_accounts_int_exp_ann: None,
            transaction_accounts_int_exp_qtr: None,
            tt_l_other_borrow_int_exp_ann: None,
            tt_l_other_borrow_int_exp_qtr: None,
            federal_funds_purchased: None,
            inc_before_inc_taxs_disc_ann: None,
            available_for_sale_secs_g_l: None,
            available_for_sale_sec_g_l_qtr: None,
            held_to_maturity_secs_g_l: None,
            loan_income_ann: None,
            loan_lease_income_ann: None,
            loan_lease_income_qtr: None,
            tax_exempt_ln_ls_int_inc_ann: None,
            tax_exempt_ln_ls_int_inc_qtr: None,
            municipal_loan_income_qtr: None,
            loan_income_qtr: None,
            total_security_income_ann: None,
            service_charge_on_dep_accts_ann: None,
            applicable_income_taxes_ann: None,
            applicable_income_taxes_qtr_ann: None,
            constr_land_dev_lns_tier_1: None,
            c_i_loans_tier_1: None,
            consumer_loans_tier_1: None,
            allowance_for_loan_and_leases: None,
            re_agricultural_cavg5: None,
            re_loans_tier_1: None,
            total_n_c_re_farmland: None,
            n_c_1_4_family_construction_loan: None,
            n_c_other_construct_land_dev: None,
            n_c_other_nonfarm_nonres_re_ln: None,
            n_c_own_occupied_nonfarm_nonres: None,
            n_c_1_4_fam_jr_ln_1_4_fam_jr_ln: None,
            n_c_re_1_4_fam_junior_lien: None,
            n_c_re_1_4_fam_first_lien: None,
            n_c_1_4_fam_1stln_1_4_fam_ist_ln: None,
            nc_restruct_loans_excl_1_4_fm: None,
            net_operating_income_qtr: None,
            ag_loan_net_charge_offs_qtr_ann: None,
            ag_ln_net_charge_offs_ann_sm_bks: None,
            ag_loan_net_chg_qtr_ann_small_bk: None,
            commercial_loan_net_chg_qtr_ann: None,
            commercial_re_ln_net_charge_offs: None,
            comml_re_net_charge_off_qtr_ann: None,
            consumer_ln_net_chg_qtr_ann: None,
            credit_card_ln_net_chg_qtr_ann: None,
            retained_earnings_bank_qtr: None,
            time_deposits_less_than_or_equal_to_insurance_limit: None,
            farmland_re_ln_net_chg_ann: None,
            farm_re_ln_net_chrg_off_qtr_ann: None,
            other_borrowed_funds: None,
            other_borrowed_funds_cavg2: None,
            other_borrowed_funds_cavg5: None,
            oth_bor_fhlb_over_3_yrs: None,
            oth_bor_fhlb_1_to_3_yrs: None,
            _30_89_days_p_d_commercial_re: None,
            _30_89_past_due_const_re_const_re: None,
            _30_89_p_d_1_4fam_jr_1_4_fam_jr: None,
            retained_earnings_rbc: None,
            tier_1_capital_reported: None,
            repurchase_agreements: None,
            securities_cavg2: None,
            securities_cavg5: None,
            municipal_securities_aa: None,
            municipal_securities_af: None,
            municipal_securities_ha: None,
            municipal_securities_hf: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
