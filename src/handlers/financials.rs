//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/financials` endpoint.

// Internal imports (std, crate)
use crate::common::{CommonParameters, FdicEndpoint, QueryParameters, get_fdic_bank_find_mcp_response};
use crate::config::FdicApiConfig;

// External imports (alphabetized)
use rmcp::model::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Auto-generated parameters struct for `/financials` endpoint.
/// Spec: risview_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct FinancialsProperties {
    #[doc = r#"Title: Activity Event Code"#]
    #[doc = r#"Description: Structure activity event code. Merger or closing codes only."#]
    pub actevt: Option<String>,

    #[doc = r#"Title: Total assets"#]
    #[doc = r#"Description: The sum of all assets owned by the institution including cash, loans, securities, bank premises and other assets. This total does not include off-balance-sheet accounts."#]
    pub asset: Option<f32>,

    #[doc = r#"Title: BRANCHING"#]
    #[doc = r#"Description: A flag used to indicate whether an institution has branches. 0 means no branches and 1 means it has branches."#]
    pub branch: Option<f32>,

    #[doc = r#"Title: Call Form Number"#]
    #[doc = r#"Description: TBD"#]
    pub callform: Option<f32>,

    #[doc = r#"Title: Community Bank"#]
    #[doc = r#"Description: FDIC community banks are identified based on criteria defined in the FDIC Community Banking Study. Using detailed balance sheet and geographic data, the study defines communtiy banks in terms of their traditional relationship banking and limited geographic scope of operations"#]
    pub cb: Option<String>,

    #[doc = r#"Title: Core Based Statistical Division Number"#]
    #[doc = r#"Description: A metropolitan division is a county or group of counties within a core based statistical area that contains a population of at least 2.5 million. A metropolitan division consists of one or more main/secondary countues that represent an employment center or centers, plus adjacent conuties associated withe the main county or counties through commuting ties."#]
    pub cbsadiv: Option<f32>,

    #[doc = r#"Title: Core Based Statistical Division Name"#]
    #[doc = r#"Description: A statistical geographic entity consisting of the county or counties associated with at least one core (urbanized area or urban cluster) of at least 10,000 population, plus adjacent counties having a high degree of social and economic integration with the core as measured through commuting ties with the counties containing the core."#]
    pub cbsaname: Option<String>,

    #[doc = r#"Title: Multi State Offices Flag"#]
    #[doc = r#"Description: Multi State Offices Flag"#]
    pub stmult: Option<String>,

    #[doc = r#"Title: ADDRESS"#]
    #[doc = r#"Description: ADDRESS"#]
    pub address: Option<String>,

    #[doc = r#"Title: Community Bank Ratio"#]
    #[doc = r#"Description: "#]
    pub cblrind: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 1-3 YEARS"#]
    #[doc = r#"Description: "#]
    pub cd1t3: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 1-3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub cd1t3r: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 3 MONTH OR LESS"#]
    #[doc = r#"Description: "#]
    pub cd3les: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY REPRICING OF 3 MONTH OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub cd3lesr: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY REPRICING OF 3 MONTH OR LESS"#]
    #[doc = r#"Description: "#]
    pub cd3less: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY REPRICING OF 3 MONTH OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub cd3lessr: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING OVER 3 YEARS"#]
    #[doc = r#"Description: "#]
    pub cdov3: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING OVER 3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub cdov3r: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING OVER 3 YEARS"#]
    #[doc = r#"Description: "#]
    pub cdov3s: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING OVER 3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub cdov3sr: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING 3-12 MONTHS"#]
    #[doc = r#"Description: "#]
    pub cd3t12: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR MORE REMAINING MATURITY OR REPRICING 3-12 MONTHS RATIO"#]
    #[doc = r#"Description: "#]
    pub cd3t12r: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 3-12 MONTHS"#]
    #[doc = r#"Description: "#]
    pub cd3t12s: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 3-12 MONTHS RATIO"#]
    #[doc = r#"Description: "#]
    pub cd3t12sr: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 1-3 YEARS"#]
    #[doc = r#"Description: "#]
    pub cd1t3s: Option<f32>,

    #[doc = r#"Title: TIME DEP $250,000 OR LESS REMAINING MATURITY OR REPRICING 1-3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub cd1t3sr: Option<f32>,

    #[doc = r#"Title: FDIC Certificate #"#]
    #[doc = r#"Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    pub cert: Option<f32>,

    #[doc = r#"Title: Directly owned by another bank (CERT)"#]
    #[doc = r#"Description: FDIC certificate number of the parent bank or savings institution with which the reported institution’s financial data has been consolidated. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports.  Click on the certificate number to identify the parent bank or thrift."#]
    pub certcons: Option<String>,

    #[doc = r#"Title: City of High Holder (Search-Eligible)"#]
    #[doc = r#"Description: City in which the headquarters of the institution's regulatory high holder are physically located. This field can be used for search and filtering."#]
    pub cityhcr: Option<String>,

    #[doc = r#"Title: Classcode"#]
    #[doc = r#"Description: A number that sub-categorizes a major class of institutions. 3 = National bank, Federal Reserve System (FRS) member; 13 = State commercial bank, FRS member; 15 = State savings, co-op, or insdustrial bank, FRS member; 21 = State commercial bank, not FRS member; 23 = State savings, co-op, or industrial bank, not FRS member; 25 = State mutual commercial bank, not FRS member; 33 =  Federal chartered stock savings bank; 34 = Federal chartered mutual savings bank; 35 = State chartered stock savings and loan association; 36 = State chartered mutual savings and loan association; 37 = Federal chartered stock savings and loan association; 38 = Federal chartered mutual savings and loan association; 41 = State chartered stock savings bank; 42 = State chartered mutual savings bank; 43 = Federal chartered stock savings bank (historical); 44 = Federal chartered mutual savings bank (historical); 50 = OCC chartered nondeposit and/or noninsured trust companies; 51 = Noninsured commercial bank; 52 = Noninsured domestic offices of foreign bank (International Banking Act); 53 = Noninsured industrial bank; 54 = State chartered nondeposit and/or noninsured trust company, not FRS member; 55 = State chartered domestic branches of foreign banks; 56 = OCC chartered domestic branches of foreign banks; 57 = New York investment company; 58 = State chartered nondeposit and/or noninsured trust company, FRS member; 59 = OTS chartered nondeposit and/or noninsured trust company, 61 = Noninsured private bank; 62 = Noninsured loan workout bank, OCC chartered; 63 = Noninsured loan workout bank, state chartered, FRS member; 64 = Noninsured loan workout bank, state chartered, not FRS member; 65 = Other holding company; 71 = Transfer agent; 81 = Noninsured stock savings bank; 82 = Noninsured mutual savings bank; 85 = Noninsured stock savings and loan association; 86 = Noninsured mutual savings and loan association; 89 = Noninsured insurance company; 91 = State chartered credit unions; 92 = Federal chartered credit unions; 93 = Privately insured state credit union."#]
    pub clcode: Option<f32>,

    #[doc = r#"Title: Closed Institution Flag"#]
    #[doc = r#"Description: A flag used to indicate whether an institution has been closed. 0 is institution not closed. 1 is institution closed."#]
    pub closed: Option<f32>,

    #[doc = r#"Title: FIPS CMSA Code"#]
    #[doc = r#"Description: The Federal Information Processing Standards Consolidated Metropolitan Statistical Area Code is a number representing the institution location. A CMSA consists of two or more contiguous MSAs with a combined population of over 1 million."#]
    pub cmsa: Option<f32>,

    #[doc = r#"Title: FIPS Country Code"#]
    #[doc = r#"Description: The Federal Information Processing Standards Alphabetic Code of the country in which the institution is physically located."#]
    pub cntryalp: Option<String>,

    #[doc = r#"Title: FIPS Country Number"#]
    #[doc = r#"Description: The Federal Information Processing Standards Numeric Code of the country in which the institution is physically located."#]
    pub cntrynum: Option<f32>,

    #[doc = r#"Title: FIPS County Number"#]
    #[doc = r#"Description: The Federal Information Processing Standards Numeric Code of the county in which the institution is physically located."#]
    pub cntynum: Option<f32>,

    #[doc = r#"Title: Combined Statistical Area"#]
    #[doc = r#"Description: U.S. CENSUS BUREAU OFFICE OF MANANGEMENT AND BUDGET DEFINES                                   THE COMBINED STATISTICAL AREA (CSA) AS A GEOGRAPHIC ENTITY                                         CONSISTING OF TWO OR MORE ADJACENT CORE BASED STATISTICAL AREAS                                  (CBSAS) WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 15.                                     PAIRS OF CBSAS WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST                                  25 COMBINE AUTOMATICALLY.  PAIRS OF CBSAS WITH EMPLOYMENT                                        INTERCHANGE MEASURES OF AT LEAST 15, BUT LESS THAN 25, MAY                                        COMBINE IF LOCAL OPTION IN BOTH AREAS FAVOR COMBINATION. "#]
    pub csa: Option<String>,

    #[doc = r#"Title: Denovo Institution"#]
    #[doc = r#"Description: A flag used to indicate whether an institution is a new institution (not a recharter). This flag is set quarterly. For instance, if REPDTE is 3/31/98 and DENOVO equals 1, the institution was a denovo during the first quarter of 1998."#]
    pub denovo: Option<String>,

    #[doc = r#"Title: Total deposits"#]
    #[doc = r#"Description: The sum of all deposits including demand deposits, money market deposits, other savings deposits, time deposits and deposits in foreign offices."#]
    pub dep: Option<f32>,

    #[doc = r#"Title: TOTAL DEPOSITS RATIO"#]
    #[doc = r#"Description: "#]
    pub depr: Option<f32>,

    #[doc = r#"Title: Deposits held in domestic offices"#]
    #[doc = r#"Description: The sum of all domestic office deposits, including demand deposits, money market deposits, other savings deposits and time deposits."#]
    pub depdom: Option<f32>,

    #[doc = r#"Title: DEPOSITS HELD IN DOM OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub depdomr: Option<f32>,

    #[doc = r#"Title: Division Flag"#]
    #[doc = r#"Description: A flag used to indicate whether an institution is in a CBSA division. 0 is institution is not in a CBSA division. 1 is institution is in a CBSA division."#]
    pub division: Option<f32>,

    #[doc = r#"Title: Docket Number"#]
    #[doc = r#"Description: A unique identification number assigned to institutions chartered by the office of thrift supervision or that become members of the federal home loan system."#]
    pub docket: Option<f32>,

    #[doc = r#"Title: International Activity Flag"#]
    #[doc = r#"Description: A FLAG USED TO INDICATE WHETHER AN INSTITUTION OPERATES ONE OR                                   MORE EDGE ACT OR AGREEMENT CORPORATIONS.  AN EDGE ACT CORPORATION                                 IS A FEDERALLY CHARTERED DOMESTIC ORGANIZATION THAT IS ALLOWED TO                                ENGAGE ONLY IN INTERNATIONAL BANKING OR OTHER FINANCIAL                                          TRANSACTIONS RELATED TO INTERNATIONAL BUSINESS.  AN AGREEMENT CORPORATION IS RESTRICTED, IN GENERAL, TO INTERNATIONAL BANKING OPERATIONS. 0 = NO AFFILIATIONS WITH EDGE ACT CORPORATIONS.                                                                                    1 = AFFILIATED WITH EDGE ACT CORPORATIONS."#]
    pub edgecode: Option<f32>,

    #[doc = r#"Title: Entity Type"#]
    #[doc = r#"Description: A three digit number indicating the major type or category of an  institution. The entity code is used to categorize an institution by type of financial organization."#]
    pub enttype: Option<f32>,

    #[doc = r#"Title: Equity capital"#]
    #[doc = r#"Description: Total equity capital (includes preferred and common stock, surplus and undivided profits)."#]
    pub eq: Option<f32>,

    #[doc = r#"Title: Equity capital"#]
    #[doc = r#"Description: "#]
    pub eq2: Option<f32>,

    #[doc = r#"Title: EQUITY CAPITAL RATIO"#]
    #[doc = r#"Description: "#]
    pub eqr: Option<f32>,

    #[doc = r#"Title: Failed Institution Flag"#]
    #[doc = r#"Description: A flag used to indicate whether an institution has failed. Failures include assisted mergers and payoffs."#]
    pub failed: Option<f32>,

    #[doc = r#"Title: FDIC Compliance Area"#]
    #[doc = r#"Description: A number used to identify the compliance area in which an institution is located."#]
    pub fdicarea: Option<f32>,

    #[doc = r#"Title: FDIC Compliance Territory"#]
    #[doc = r#"Description: An abbreviation of the current compliance territory where an institution is located (FDIC Compliance Territory). All periods are displayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#]
    pub fdicterr: Option<String>,

    #[doc = r#"Title: DCA Field Office"#]
    #[doc = r#"Description: The name of the compliance field office to which an institution is assigned. All periods are diplayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#]
    pub fldofdca: Option<String>,

    #[doc = r#"Title: FFIEC Call Report 31 Filer"#]
    #[doc = r#"Description: A flag (1=yes,0=no) that indicates whether and institution filed an FFIEC 031 Call Report. Commercial banks with domestic and foreign offices are required to file such a report."#]
    pub form31: Option<String>,

    #[doc = r#"Title: Bank Holding Company Type"#]
    #[doc = r#"Description: A flag used to indicate whether an institution is a member of a multibank holding company 1=yes, 0=no"#]
    pub hctmult: Option<String>,

    #[doc = r#"Title: Bank Not Member of Hold Company"#]
    #[doc = r#"Description: A flag used to indicated whether an institution is an independent bank. 0 is member of a bank hold company. 1 is not a member of a bank holding company."#]
    pub hctnone: Option<f32>,

    #[doc = r#"Title: Secondary Insurer"#]
    #[doc = r#"Description: The secondary insurer, insurance agent, or insurance status of an institution."#]
    pub insagnt2: Option<String>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub insbif: Option<f32>,

    #[doc = r#"Title: Deposit Insurance Fund member"#]
    #[doc = r#"Description: A flag used to indicate whether an institution is insured under the Deposit Insurance Fund (DIF).  As of April 1, 2006 the Bank Insurance Fund (BIF) was merged together with the Savings Institution Insurance Fund (SAIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured BIF and SAIF member institutions that are still active or open are now insured members of DIF.    0 = No, not DIF insured and 1 = Yes, DIF insured.  Note that institutions that became inactive prior to April 1006 will also have zero value."#]
    pub insdif: Option<String>,

    #[doc = r#"Title: Agricultural lending institution indicator"#]
    #[doc = r#"Description: An indicator specifying whether an institution is primarily an agricultural lending institution."#]
    pub instag: Option<String>,

    #[doc = r#"Title: Credit Card Institutions"#]
    #[doc = r#"Description: Institutions with total loans greater than 50% of total assets and credit card loans greater than 50% of total loans, including loans that have been securitized and sold."#]
    pub instcrcd: Option<String>,

    #[doc = r#"Title: SAIF Insured"#]
    #[doc = r#"Description: Institutions who are members of the Savings Association Insurance Fund. As of April 1, 2006 SAIF was merged together with the Bank Insurance Fund (BIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured SAIF member institutions, that are still active or open, are now insured members of DIF."#]
    pub inssaif: Option<f32>,

    #[doc = r#"Title: MINORITY OWNED INSTITUTIONS"#]
    #[doc = r#"Description: "#]
    pub minority: Option<f32>,

    #[doc = r#"Title: Ownership Type"#]
    #[doc = r#"Description: Banking institutions fall into one of two ownership types, stock or non-stock. An institution which sells stock to raise capital is called a stock institution. It is owned by the shareholders who benefit from profits earned by the institution. A non-stock institution, or mutual institution, is owned and controlled solely by its depositors. A mutual does not issue capital stock."#]
    pub mutual: Option<f32>,

    #[doc = r#"Title: Bank Holding Company (Regulatory Top Holder) (Search-Eligible)"#]
    #[doc = r#"Description: Regulatory top holder is assigned by the Federal Reserve Board based on ownership and control percentages. Note: Information on bank holding companies is only as of quarter-end. Regulatory top holder is any company that directly or indirectly owns, controls or has power to vote 25 percent or more of a bank's or direct holding company's shares or  controls in any manner the election of a majority of the directors or trustees of a bank or direct holding company or  exercises a controlling influence over the management or policies of a bank or direct holding company.   Information on Thrift Holding Companies that own Savings Associations but do not own banks is not currently available in the ID System.  Source: Federal Reserve Board National Information Center data base. This field can be used for search and filtering."#]
    pub namehcr: Option<String>,

    #[doc = r#"Title: Net income"#]
    #[doc = r#"Description: Net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#]
    pub netinc: Option<f32>,

    #[doc = r#"Title: NET INCOME - RATIO"#]
    #[doc = r#"Description: "#]
    pub netincr: Option<f32>,

    #[doc = r#"Title: Net income - quarterly"#]
    #[doc = r#"Description: Quarterly net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#]
    pub netincq: Option<f32>,

    #[doc = r#"Title: Net income - quarterly"#]
    #[doc = r#"Description: "#]
    pub netincqa: Option<f32>,

    #[doc = r#"Title: NET INCOME - QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub netincqr: Option<f32>,

    #[doc = r#"Title: Number of Domestic Offices"#]
    #[doc = r#"Description: The number of domestic offices (including headquarters) operated by active institutions in the 50 states of the U.S.A."#]
    pub offdom: Option<f32>,

    #[doc = r#"Title: Number of Foreign Offices"#]
    #[doc = r#"Description: The number of foreign offices (outside the U.S.) operated by the institution."#]
    pub offfor: Option<f32>,

    #[doc = r#"Title: Number of US Offices"#]
    #[doc = r#"Description: The number of offices operated by an FDIC-insured institution in all commonwealths and terrirtories of the US, along with those in freely associated states under the Compact of Free Association"#]
    pub offoa: Option<f32>,

    #[doc = r#"Title: Directly owned by another bank (CERT)"#]
    #[doc = r#"Description: The PARCERT number identifies the subsidiary institutions parent certificate number. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports."#]
    pub parcert: Option<String>,

    #[doc = r#"Title: Report Date (Search-Eligible)"#]
    #[doc = r#"Description: The last day of the financial reporting period selected. This field can be used for search and filtering."#]
    pub l_repdte: Option<String>,

    #[doc = r#"Title: Report Date (Search-Eligible)"#]
    #[doc = r#"Description: The last day of the financial reporting period selected. This field can be used for search and filtering."#]
    pub repdte_raw: Option<String>,

    #[doc = r#"Title: Report Date (Search-Eligible)"#]
    #[doc = r#"Description: The last day of the financial reporting period selected. This field can be used for search and filtering."#]
    pub repdte: Option<String>,

    #[doc = r#"Title: REPORT YEAR (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub repyear: Option<String>,

    #[doc = r#"Title: Report Date"#]
    #[doc = r#"Description: The financial reporting period selected in CCYYMM format."#]
    pub risdate: Option<String>,

    #[doc = r#"Title: Return on assets (ROA)"#]
    #[doc = r#"Description: Net income after taxes and extraordinary items (annualized) as a percent of average total assets."#]
    pub roa: Option<f32>,

    #[doc = r#"Title: Pretax return on assets"#]
    #[doc = r#"Description: Annualized pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#]
    pub roaptx: Option<f32>,

    #[doc = r#"Title: Quarterly Pretax return on assets"#]
    #[doc = r#"Description: Quarterly pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#]
    pub roaptxq: Option<f32>,

    #[doc = r#"Title: Quarterly return on assets"#]
    #[doc = r#"Description: Quarterly net income after taxes and extraordinary items as a percent of average total assets."#]
    pub roaq: Option<f32>,

    #[doc = r#"Title: Return on Equity (ROE)"#]
    #[doc = r#"Description: Annualized net income as a percent of average equity on a consolidated basis.     Note: If retained earnings are  negative, the ratio is shown as NA."#]
    pub roe: Option<f32>,

    #[doc = r#"Title: Quarterly return on equity"#]
    #[doc = r#"Description: Quarterly net income (including gains or losses on securities and extraordinary items) as a percentage of average total equity capital."#]
    pub roeq: Option<f32>,

    #[doc = r#"Title: RSSDID - High Regulatory Holder (Search-Eligible)"#]
    #[doc = r#"Description: The unique number assigned by the Federal Reserve Board to the regulatory high holding company of the institution. This field can be used for search and filtering."#]
    pub rssdhcr: Option<String>,

    #[doc = r#"Title: Asset Concentration Hierarchy"#]
    #[doc = r#"Description: An indicator of an institution's primary specialization in terms of asset concentration"#]
    pub specgrp: Option<f32>,

    #[doc = r#"Title: Asset Concentration Hierarchy Description"#]
    #[doc = r#"Description: An indicator of an institution's primary specialization in terms of asset concentration Description"#]
    pub specgrpdesc: Option<String>,

    #[doc = r#"Title: Regulatory holding company state location (Search-Eligible)"#]
    #[doc = r#"Description: State location of the regulatory high holding company (either direct or indirect owner). This field can be used for search and filtering."#]
    pub stalphcr: Option<String>,

    #[doc = r#"Title: Subchapter S Corporations"#]
    #[doc = r#"Description: The Small Business Job Protection Act of 1996 changed the Internal Revenue Code to allow financial institutions to elect Subchapter S corporation status, beginning in 1997. Banks are required to indicate on the Call Report whether there is currently in effect an election to file under Subchapter S. Thrifts have a similar requirement as of March 1998.  The most important IRS requirements to elect and maintain Subchapter S status are: There can be no more than 75 eligible shareholders and no more than one class of stock. (In general, shareholders can only be individuals, estates, and certain types of trusts. Certain retirement plans and charitable organizations will be eligible in 1998.) All shareholders must consent.  Banks and thrifts converting to Subchapter S status must use the specific charge-off method for tax purposes rather than the reserve method of accounting for bad debts and recapture tax bad debt reserves over a period of six years, if the reserve method had been used prior to conversion. (Note: even though the specific charge-off method is required for tax purposes, an adequate allowance for loan and lease losses must still be maintained on the financial statements and Call Reports.) Banks and thrifts are subject to a built-in gains (BIG) tax, if the aggregate fair market value of assets is greater than their aggregate adjusted bases on the date of conversion to Subchapter S status.     [Banks are required to indicate separately on the Call Report in December of each year, the deferred portion of income taxes reported in net income. For Subchapter S banks, some or all of their deferred tax assets and liabilities may be eliminated upon conversion to Subchapter S status; however, deferred taxes related to the BIG tax and the recapture of bad debt reserves must be recognized.].   A Subchapter S corporation is treated as a pass-through entity, similar to a partnership, for federal income tax purposes. It is generally not subject to any federal income taxes at the corporate level. Its taxable income flows through to its shareholders in proportion to their stock ownership, and the shareholders generally pay federal income taxes on their share of this taxable income. This can have the effect of reducing institutions' reported income tax expense and increasing their after-tax earnings..   The election of Subchapter S status may result in an increase in shareholders' personal tax liabilities. Therefore, S corporations typically increase the amount of earnings distributed as dividends to compensate for higher personal taxes."#]
    pub subchaps: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: Beyond having trust powers granted and exercised, institutions with fiduciary assets accounts, income, or other reportable fiduciary related service"#]
    pub tract: Option<f32>,

    #[doc = r#"Title: Trust Powers"#]
    #[doc = r#"Description: A flag used to indicate an institution's Trust Powers Granted status. 0 = No Trust Power Granted 1 = Trust Power Granted Where Trust Power has been granted specific codes are: 00 - Trust powers not know 10 - Full trust powers granted 11 - Full trust powers granted, exercised 12 - Full trust powers granted, not exercised 20 - Limited trust powers granted 21 - Limited trust powers granted, exercised 22 - Limited trust powers granted, not exercised 30 - Trust powers not granted 31 - Trust powers not granted, but exercised"#]
    pub trust: Option<String>,

    #[doc = r#"Title: BANKS LIABILITY ON ACCEPTANCES"#]
    #[doc = r#"Description: "#]
    pub acept: Option<f32>,

    #[doc = r#"Title: ACTIVE INSTITUTION FLAG"#]
    #[doc = r#"Description: "#]
    pub active: Option<f32>,

    #[doc = r#"Title: INSTITUTION CLASS (Search-Eligible)"#]
    #[doc = r#"Description: A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA). This field can be used for search and filtering."#]
    pub bkclass: Option<String>,

    #[doc = r#"Title: PREMISES AND FIXED ASSETS"#]
    #[doc = r#"Description: "#]
    pub bkprem: Option<f32>,

    #[doc = r#"Title: PREMISES AND FIXED ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub bkpremr: Option<f32>,

    #[doc = r#"Title: BROKERED DEP"#]
    #[doc = r#"Description: "#]
    pub bro: Option<f32>,

    #[doc = r#"Title: BROKERED RATIO"#]
    #[doc = r#"Description: "#]
    pub bror: Option<f32>,

    #[doc = r#"Title: REPORT DATE (CCYYMM)"#]
    #[doc = r#"Description: "#]
    pub callym: Option<f32>,

    #[doc = r#"Title: CASH & DUE FROM DEPOSITORY INST"#]
    #[doc = r#"Description: "#]
    pub chbal: Option<f32>,

    #[doc = r#"Title: CASH & DUE FROM DEPOSITORY INST RATIO"#]
    #[doc = r#"Description: "#]
    pub chbalr: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING CASH & DUE"#]
    #[doc = r#"Description: "#]
    pub chbali: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING CASH & DUE RATIO"#]
    #[doc = r#"Description: "#]
    pub chbalir: Option<f32>,

    #[doc = r#"Title: CHARTER AGENT"#]
    #[doc = r#"Description: "#]
    pub chrtagnt: Option<String>,

    #[doc = r#"Title: RTC CONSERVATORSHIP FLAG"#]
    #[doc = r#"Description: "#]
    pub conserve: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crlnls: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crlnlsr: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crlnlsq: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crlnlsqr: Option<f32>,

    #[doc = r#"Title: CUSTOMERS ACCEPTANCES"#]
    #[doc = r#"Description: "#]
    pub cusli: Option<f32>,

    #[doc = r#"Title: DDA TRANS-TOTAL"#]
    #[doc = r#"Description: "#]
    pub ddt: Option<f32>,

    #[doc = r#"Title: DDA TRANS-TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub ddtr: Option<f32>,

    #[doc = r#"Title: TOTAL DEPOSITS-FOR"#]
    #[doc = r#"Description: "#]
    pub depfor: Option<f32>,

    #[doc = r#"Title: TOTAL DEPOSITS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depforr: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING DEP"#]
    #[doc = r#"Description: "#]
    pub depi: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING DEP-FOR"#]
    #[doc = r#"Description: "#]
    pub depifor: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING DEP-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depiforr: Option<f32>,

    #[doc = r#"Title: IPC & OFFICIAL CHECKS-FOR"#]
    #[doc = r#"Description: "#]
    pub depipccf: Option<f32>,

    #[doc = r#"Title: IPC & OFFICIAL CHECKS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depipccfr: Option<f32>,

    #[doc = r#"Title: IPC-FOR"#]
    #[doc = r#"Description: "#]
    pub depipcf: Option<f32>,

    #[doc = r#"Title: NONINTEREST-BEARING DEP"#]
    #[doc = r#"Description: "#]
    pub depni: Option<f32>,

    #[doc = r#"Title: NONINTEREST-BEARING DEP-FOR"#]
    #[doc = r#"Description: "#]
    pub depnifor: Option<f32>,

    #[doc = r#"Title: NONINTEREST-BEARING DEP-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depniforr: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drlnls: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drlnlsr: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drlnlsq: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drlnlsqr: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSS AST"#]
    #[doc = r#"Description: "#]
    pub eamintan: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSS AST RATIO"#]
    #[doc = r#"Description: "#]
    pub eamintanr: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSS AST QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eamintq: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSS AST QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eamintqr: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE"#]
    #[doc = r#"Description: "#]
    pub edep: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-DOM"#]
    #[doc = r#"Description: "#]
    pub edepdom: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-DOM RATIO"#]
    #[doc = r#"Description: "#]
    pub edepdomr: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-DOM QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub edepdomq: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-DOM QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub edepdomqr: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-FOR"#]
    #[doc = r#"Description: "#]
    pub edepfor: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub edepforr: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-FOR QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub edepforq: Option<f32>,

    #[doc = r#"Title: DEPOSIT INTEREST EXPENSE-FOR QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub edepforqr: Option<f32>,

    #[doc = r#"Title: ADVANCES FROM FHLBANK INT EXP"#]
    #[doc = r#"Description: "#]
    pub efhlbadv: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS INT EXPENSE"#]
    #[doc = r#"Description: "#]
    pub efrepp: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS INT EXPENSE RATIO"#]
    #[doc = r#"Description: "#]
    pub efreppr: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS INT EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub efreppq: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS INT EXPENSE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub efreppqr: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST EXPENSE"#]
    #[doc = r#"Description: "#]
    pub eintexp: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST EXPENSE RATIO"#]
    #[doc = r#"Description: "#]
    pub eintexpr: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eintxq: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eintxqa: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST EXPENSE ANNUALLY"#]
    #[doc = r#"Description: "#]
    pub eintexpa: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST EXPENSE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eintxqr: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR CREDIT LOSSES"#]
    #[doc = r#"Description: "#]
    pub elnatr: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR CREDIT LOSSES RATIO"#]
    #[doc = r#"Description: "#]
    pub elnatrr: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub elnatq: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub elnatqa: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub elnatqr: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub elnlosq: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR CREDIT LOSSES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub nttotq: Option<f32>,

    #[doc = r#"Title: PROVISIONS FOR LN & LEASE LOSSES"#]
    #[doc = r#"Description: "#]
    pub elnlos: Option<f32>,

    #[doc = r#"Title: MORTGAGE DEBT INTEREST EXPENSE"#]
    #[doc = r#"Description: "#]
    pub emtgls: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST EXPENSE"#]
    #[doc = r#"Description: "#]
    pub addnonintexp: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST EXPENSE RATIO"#]
    #[doc = r#"Description: "#]
    pub addnonintexpr: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub addnonintexpq: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST EXPENSE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub addnonintexpqr: Option<f32>,

    #[doc = r#"Title: ALL OTHER NONINTEREST EXPENSE"#]
    #[doc = r#"Description: "#]
    pub eothnint: Option<f32>,

    #[doc = r#"Title: ALL OTHER NONINTEREST EXPENSE RATIO"#]
    #[doc = r#"Description: "#]
    pub eothnintr: Option<f32>,

    #[doc = r#"Title: ALL OTHER NONINTEREST EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eothninq: Option<f32>,

    #[doc = r#"Title: ALL OTHER NONINTEREST EXPENSE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eothninqr: Option<f32>,

    #[doc = r#"Title: PREMISES & FIXED ASSETS EXPENSE"#]
    #[doc = r#"Description: "#]
    pub epremagg: Option<f32>,

    #[doc = r#"Title: PREMISES & EQUIPMENT EXPENSE RATIO"#]
    #[doc = r#"Description: "#]
    pub epremaggr: Option<f32>,

    #[doc = r#"Title: PREMISES & FIXED ASSETS EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub epremagq: Option<f32>,

    #[doc = r#"Title: PREMISES & EQUIPMENT EXPENSE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub epremagqr: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON COMM & PREF"#]
    #[doc = r#"Description: "#]
    pub eqcdiv: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON COMM & PREF RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcdivr: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON COMM STOCK"#]
    #[doc = r#"Description: "#]
    pub eqcdivc: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON COMM STOCK RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcdivcr: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON PREF STOCK"#]
    #[doc = r#"Description: "#]
    pub eqcdivp: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON PREF STOCK RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcdivpr: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON COMM & PREF QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eqcdivq: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON COMM & PREF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcdivqr: Option<f32>,

    #[doc = r#"Title: EQCFCTA"#]
    #[doc = r#"Description: "#]
    pub eqcfcta: Option<f32>,

    #[doc = r#"Title: MINOR INT IN CONSOL SUBS-EQ"#]
    #[doc = r#"Description: "#]
    pub eqconsub: Option<f32>,

    #[doc = r#"Title: COMMON STOCK"#]
    #[doc = r#"Description: "#]
    pub eqcs: Option<f32>,

    #[doc = r#"Title: COMMON STOCK RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcsr: Option<f32>,

    #[doc = r#"Title: NET WORTH CERTIFICATES"#]
    #[doc = r#"Description: "#]
    pub eqnwcert: Option<f32>,

    #[doc = r#"Title: OTHER EQUITY CAPITAL COMPONENTS"#]
    #[doc = r#"Description: "#]
    pub eqothcc: Option<f32>,

    #[doc = r#"Title: PERPETUAL PREFERRED STOCK"#]
    #[doc = r#"Description: "#]
    pub eqpp: Option<f32>,

    #[doc = r#"Title: PERPETUAL PREFERRED STOCK RATIO"#]
    #[doc = r#"Description: "#]
    pub eqppr: Option<f32>,

    #[doc = r#"Title: SURPLUS"#]
    #[doc = r#"Description: "#]
    pub eqsur: Option<f32>,

    #[doc = r#"Title: SURPLUS RATIO"#]
    #[doc = r#"Description: "#]
    pub eqsurr: Option<f32>,

    #[doc = r#"Title: EQUP"#]
    #[doc = r#"Description: "#]
    pub equp: Option<f32>,

    #[doc = r#"Title: UP-NET & OTHER CAPITAL COMP"#]
    #[doc = r#"Description: "#]
    pub equptot: Option<f32>,

    #[doc = r#"Title: UP-NET & OTHER CAPITAL RATIO"#]
    #[doc = r#"Description: "#]
    pub equptotr: Option<f32>,

    #[doc = r#"Title: SALARIES AND EMPLOYEE BENEFITS"#]
    #[doc = r#"Description: "#]
    pub esal: Option<f32>,

    #[doc = r#"Title: SALARIES AND EMPLOYEE BENEFITS RATIO"#]
    #[doc = r#"Description: "#]
    pub esalr: Option<f32>,

    #[doc = r#"Title: SALARIES AND EMPLOYEE BENEFITS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub esalq: Option<f32>,

    #[doc = r#"Title: SALARIES AND EMPLOYEE BENEFITS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub esalqr: Option<f32>,

    #[doc = r#"Title: SUBORDINATED NOTES INT EXPENSE"#]
    #[doc = r#"Description: "#]
    pub esubnd: Option<f32>,

    #[doc = r#"Title: TT&L & OTHER BORROWINGS INT EXP"#]
    #[doc = r#"Description: "#]
    pub ettlotbo: Option<f32>,

    #[doc = r#"Title: NET DISCONTINUED OPERATIONS"#]
    #[doc = r#"Description: "#]
    pub extra: Option<f32>,

    #[doc = r#"Title: NET DISCONTINUED RATIO"#]
    #[doc = r#"Description: "#]
    pub extrar: Option<f32>,

    #[doc = r#"Title: NET DISCONTINUED OPERATIONS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub extraq: Option<f32>,

    #[doc = r#"Title: NET DISCONTINUED OPERATIONS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub extraqr: Option<f32>,

    #[doc = r#"Title: FDIC REGION"#]
    #[doc = r#"Description: "#]
    pub fdicdbs: Option<f32>,

    #[doc = r#"Title: FDIC REGION DESC"#]
    #[doc = r#"Description: "#]
    pub fdicdbsdesc: Option<String>,

    #[doc = r#"Title: FDIC REGION - SUPERVISORY"#]
    #[doc = r#"Description: "#]
    pub fdicsupv: Option<f32>,

    #[doc = r#"Title: FDIC REGION - SUPERVISORY DESC"#]
    #[doc = r#"Description: "#]
    pub fdicsupvdesc: Option<String>,

    #[doc = r#"Title: FED DISTRICT"#]
    #[doc = r#"Description: "#]
    pub fed: Option<f32>,

    #[doc = r#"Title: FED DISTRICT DESC"#]
    #[doc = r#"Description: "#]
    pub feddesc: Option<String>,

    #[doc = r#"Title: FEDERAL CHARTER FLAG"#]
    #[doc = r#"Description: "#]
    pub fedchrtr: Option<f32>,

    #[doc = r#"Title: FDIC RISK MANAGEMENT FIELD OFFICE (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub fldoff: Option<String>,

    #[doc = r#"Title: FOREIGN CHARTER FLAG"#]
    #[doc = r#"Description: "#]
    pub forchrtr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL FINANCIAL REPORT FLAG"#]
    #[doc = r#"Description: "#]
    pub formcfr: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS SOLD"#]
    #[doc = r#"Description: "#]
    pub frepo: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS SOLD"#]
    #[doc = r#"Description: "#]
    pub frepor: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS PURCHASED"#]
    #[doc = r#"Description: "#]
    pub frepp: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPOS PURCHASED RATIO"#]
    #[doc = r#"Description: "#]
    pub freppr: Option<f32>,

    #[doc = r#"Title: FRS MEMBER FLAG"#]
    #[doc = r#"Description: "#]
    pub frsmem: Option<f32>,

    #[doc = r#"Title: MEMBER OF A ONE BANK HOLDING CO"#]
    #[doc = r#"Description: "#]
    pub hctone: Option<f32>,

    #[doc = r#"Title: INTL BANKING ACT ENTITY FLAG"#]
    #[doc = r#"Description: "#]
    pub iba: Option<f32>,

    #[doc = r#"Title: INCOME BEFORE INC TAXES & DISC"#]
    #[doc = r#"Description: "#]
    pub ibeftax: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INSTITUTIONS INT INC"#]
    #[doc = r#"Description: "#]
    pub ichbal: Option<f32>,

    #[doc = r#"Title: BALANCES FROM DEPOSITORY INSTITUTIONS YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub ichbalr: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INSTITUTIONS INT INC QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ichbalq: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INSTITUTIONS INT INC QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ichbalqr: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPO INTEREST INCOME"#]
    #[doc = r#"Description: "#]
    pub ifrepo: Option<f32>,

    #[doc = r#"Title: FEDERAL FUNDS SOLD YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub ifrepor: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPO INTEREST INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ifrepoq: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPO INTEREST INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ifrepoqr: Option<f32>,

    #[doc = r#"Title: SECURITIES GAINS AND LOSSES"#]
    #[doc = r#"Description: "#]
    pub iglsec: Option<f32>,

    #[doc = r#"Title: SECURITIES GAINS AND LOSSES RATIO"#]
    #[doc = r#"Description: "#]
    pub iglsecr: Option<f32>,

    #[doc = r#"Title: SECURITIES GAINS AND LOSSES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iglsecqr: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-DOM"#]
    #[doc = r#"Description: "#]
    pub ilndom: Option<f32>,

    #[doc = r#"Title: DOMESTIC OFFICE LOANS YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub ilndomr: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-DOM QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ilndomq: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-DOM QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ilndomqr: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-FOR"#]
    #[doc = r#"Description: "#]
    pub ilnfor: Option<f32>,

    #[doc = r#"Title: FOREIGN OFFICE LOANS YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub ilnforr: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-FOR QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ilnforq: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-FOR QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ilnforqr: Option<f32>,

    #[doc = r#"Title: LEASE INCOME"#]
    #[doc = r#"Description: "#]
    pub ils: Option<f32>,

    #[doc = r#"Title: LEASE FINANCING RECEIVABLES YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub ilsr: Option<f32>,

    #[doc = r#"Title: LEASE INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ilsq: Option<f32>,

    #[doc = r#"Title: LEASE INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ilsqr: Option<f32>,

    #[doc = r#"Title: INSURED INSTITUTION FLAG"#]
    #[doc = r#"Description: "#]
    pub insall: Option<f32>,

    #[doc = r#"Title: INSURED COMMERCIAL FLAG"#]
    #[doc = r#"Description: "#]
    pub inscoml: Option<f32>,

    #[doc = r#"Title: FDIC INSURED FLAG"#]
    #[doc = r#"Description: "#]
    pub insfdic: Option<f32>,

    #[doc = r#"Title: NOT FEDERALLY INSURED FLAG"#]
    #[doc = r#"Description: "#]
    pub insnone: Option<f32>,

    #[doc = r#"Title: INSURED SAVINGS INSTITUTION FLAG"#]
    #[doc = r#"Description: "#]
    pub inssave: Option<f32>,

    #[doc = r#"Title: COMMERCIAL INSTITUTION FLAG"#]
    #[doc = r#"Description: "#]
    pub instcoml: Option<f32>,

    #[doc = r#"Title: SAVING & S&L INSTITUTION FLAG"#]
    #[doc = r#"Description: "#]
    pub instsave: Option<f32>,

    #[doc = r#"Title: INSTITUTION TYPE"#]
    #[doc = r#"Description: "#]
    pub insttype: Option<String>,

    #[doc = r#"Title: INTANGIBLE ASSETS"#]
    #[doc = r#"Description: "#]
    pub intan: Option<f32>,

    #[doc = r#"Title: INTANGIBLE ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub intanr: Option<f32>,

    #[doc = r#"Title: INTEREST EXPENSE TO EARNING ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub intexpy: Option<f32>,

    #[doc = r#"Title: COST OF FUNDING EARNING ASSETS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub intexpyq: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST INCOME"#]
    #[doc = r#"Description: "#]
    pub intinc: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST INCOME YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub intincr: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub intinq: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub intinqr: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub intinqa: Option<f32>,

    #[doc = r#"Title: INVEST IN UNCONSOLIDATED SUBS"#]
    #[doc = r#"Description: "#]
    pub invsub: Option<f32>,

    #[doc = r#"Title: INVESTMENTS IN RE"#]
    #[doc = r#"Description: "#]
    pub invsuore: Option<f32>,

    #[doc = r#"Title: OTHER FEE INCOME"#]
    #[doc = r#"Description: "#]
    pub iothfee: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST INCOME"#]
    #[doc = r#"Description: "#]
    pub iothii: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST INCOME YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub iothiir: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iothiiq: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iothiiqr: Option<f32>,

    #[doc = r#"Title: IRAS AND KEOGH PLANS-DEPOSITS"#]
    #[doc = r#"Description: "#]
    pub irakeogh: Option<f32>,

    #[doc = r#"Title: IRAS AND KEOGH PLANS-DEPOSITS RATIO"#]
    #[doc = r#"Description: "#]
    pub irakeoghr: Option<f32>,

    #[doc = r#"Title: TOTAL SECURITY INCOME"#]
    #[doc = r#"Description: "#]
    pub isc: Option<f32>,

    #[doc = r#"Title: SECURITIES YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub iscr: Option<f32>,

    #[doc = r#"Title: TOTAL SECURITY INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iscq: Option<f32>,

    #[doc = r#"Title: TOTAL SECURITY INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iscqr: Option<f32>,

    #[doc = r#"Title: SERVICE CHARGE ON DEPOSIT ACCTS"#]
    #[doc = r#"Description: "#]
    pub iserchg: Option<f32>,

    #[doc = r#"Title: SERVICE CHARGE ON DEPOSIT ACCTS RATIO"#]
    #[doc = r#"Description: "#]
    pub iserchgr: Option<f32>,

    #[doc = r#"Title: APPLICABLE INCOME TAXES"#]
    #[doc = r#"Description: "#]
    pub itax: Option<f32>,

    #[doc = r#"Title: APPLICABLE INCOME TAXES RATIO"#]
    #[doc = r#"Description: "#]
    pub itaxr: Option<f32>,

    #[doc = r#"Title: APPLICABLE INCOME TAXES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub itaxq: Option<f32>,

    #[doc = r#"Title: APPLICABLE INCOME TAXES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub itaxqr: Option<f32>,

    #[doc = r#"Title: INTEREST INCOME ON TRADING ACCTS"#]
    #[doc = r#"Description: "#]
    pub itrade: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNTS YTD RATIO"#]
    #[doc = r#"Description: "#]
    pub itrader: Option<f32>,

    #[doc = r#"Title: INTEREST INCOME ON TRADING ACCTS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub itradeq: Option<f32>,

    #[doc = r#"Title: INTEREST INCOME ON TRADING ACCTS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub itradeqr: Option<f32>,

    #[doc = r#"Title: TOTAL LIABILITIES"#]
    #[doc = r#"Description: "#]
    pub liab: Option<f32>,

    #[doc = r#"Title: TOTAL LIABILITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub liabr: Option<f32>,

    #[doc = r#"Title: TOTAL LIABILITIES & CAPITAL"#]
    #[doc = r#"Description: "#]
    pub liabeq: Option<f32>,

    #[doc = r#"Title: TOTAL LIABILITIES & CAPITAL RATIO"#]
    #[doc = r#"Description: "#]
    pub liabeqr: Option<f32>,

    #[doc = r#"Title: MORTGAGE LOANS IN PROCESS"#]
    #[doc = r#"Description: "#]
    pub lipmtg: Option<f32>,

    #[doc = r#"Title: LIMITED-LIFE PREFERRED STOCK"#]
    #[doc = r#"Description: "#]
    pub llpfdstk: Option<f32>,

    #[doc = r#"Title: ACCEPTANCES OF OTHER BANKS"#]
    #[doc = r#"Description: "#]
    pub lnacoth: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS"#]
    #[doc = r#"Description: "#]
    pub lnag: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnagr: Option<f32>,

    #[doc = r#"Title: ALLOW FOR LOANS LOSS ADJUSTED"#]
    #[doc = r#"Description: "#]
    pub lnatres: Option<f32>,

    #[doc = r#"Title: ALLOW FOR LOANS + ALLOC TRN RISK"#]
    #[doc = r#"Description: "#]
    pub lnatresj: Option<f32>,

    #[doc = r#"Title: ALLOW FOR LOANS + ALLOC TRN RISK RATIO"#]
    #[doc = r#"Description: "#]
    pub lnatresrr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS - AUTO"#]
    #[doc = r#"Description: "#]
    pub lnauto: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-AUTO RATIO"#]
    #[doc = r#"Description: "#]
    pub lnautor: Option<f32>,

    #[doc = r#"Title: C&I LOANS"#]
    #[doc = r#"Description: "#]
    pub lnci: Option<f32>,

    #[doc = r#"Title: C&I LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lncir: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS"#]
    #[doc = r#"Description: "#]
    pub lncon: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnconr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-HOME IMPROVEMENT"#]
    #[doc = r#"Description: "#]
    pub lnconot1: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-OTHER"#]
    #[doc = r#"Description: "#]
    pub lnconoth: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub lnconothr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-CREDIT CARD PLAN"#]
    #[doc = r#"Description: "#]
    pub lncrcd: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-CREDIT CARD PLAN RATIO"#]
    #[doc = r#"Description: "#]
    pub lncrcdr: Option<f32>,

    #[doc = r#"Title: LNS-CREDIT CD & RELATED PLAN"#]
    #[doc = r#"Description: "#]
    pub lncrcdrp: Option<f32>,

    #[doc = r#"Title: DEP INSTITUTION LOANS"#]
    #[doc = r#"Description: "#]
    pub lndep: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVT LOANS"#]
    #[doc = r#"Description: "#]
    pub lnfg: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVT LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnfgr: Option<f32>,

    #[doc = r#"Title: LN&LS + UNEARNED INC"#]
    #[doc = r#"Description: "#]
    pub lnls: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-TOTAL"#]
    #[doc = r#"Description: "#]
    pub lnlsgr: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-TOTAL"#]
    #[doc = r#"Description: "#]
    pub lnlsgr2: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-TOTAL ADJUSTED"#]
    #[doc = r#"Description: "#]
    pub lnlsgrj: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub lnlsgrr: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-NET"#]
    #[doc = r#"Description: "#]
    pub lnlsnet: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-NET RATIO"#]
    #[doc = r#"Description: "#]
    pub lnlsnetr: Option<f32>,

    #[doc = r#"Title: MUNI LOANS"#]
    #[doc = r#"Description: "#]
    pub lnmuni: Option<f32>,

    #[doc = r#"Title: MUNI LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnmunir: Option<f32>,

    #[doc = r#"Title: OTHER LNS & LS-COMM-QBP"#]
    #[doc = r#"Description: "#]
    pub lnotci: Option<f32>,

    #[doc = r#"Title: OTHER LNS & LS-COMM-QBP RATIO"#]
    #[doc = r#"Description: "#]
    pub lnotcir: Option<f32>,

    #[doc = r#"Title: LN TO NONDEP FIN INST & OTH LN"#]
    #[doc = r#"Description: "#]
    pub lnother: Option<f32>,

    #[doc = r#"Title: OTHER LOANS"#]
    #[doc = r#"Description: "#]
    pub lnsother: Option<f32>,

    #[doc = r#"Title: OTHER LOANS"#]
    #[doc = r#"Description: "#]
    pub lnsotherr: Option<f32>,

    #[doc = r#"Title: RE LOANS"#]
    #[doc = r#"Description: "#]
    pub lnre: Option<f32>,

    #[doc = r#"Title: RE LOANS"#]
    #[doc = r#"Description: "#]
    pub lnre2: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub lnrecon2: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub lnremul2: Option<f32>,

    #[doc = r#"Title: RE LOANS ADJUSTED"#]
    #[doc = r#"Description: "#]
    pub lnrej: Option<f32>,

    #[doc = r#"Title: RE LOANS CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnre5: Option<f32>,

    #[doc = r#"Title: RE LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrer: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL"#]
    #[doc = r#"Description: "#]
    pub lnreag: Option<f32>,

    #[doc = r#"Title: RE CONSTRUCTION & LAND DEV-CAV5"#]
    #[doc = r#"Description: "#]
    pub lnrecon5: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreagr: Option<f32>,

    #[doc = r#"Title: RE CONSTRUCTION & LAND DEVELOP"#]
    #[doc = r#"Description: "#]
    pub lnrecons: Option<f32>,

    #[doc = r#"Title: RE CONSTRUCTION & LAND DEVELOP RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreconsr: Option<f32>,

    #[doc = r#"Title: RE LOANS-DOM"#]
    #[doc = r#"Description: "#]
    pub lnredom: Option<f32>,

    #[doc = r#"Title: RE LOANS-DOM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnredomr: Option<f32>,

    #[doc = r#"Title: RE LOANS-FOR"#]
    #[doc = r#"Description: "#]
    pub lnrefor: Option<f32>,

    #[doc = r#"Title: RE LOANS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreforr: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-LINE"#]
    #[doc = r#"Description: "#]
    pub lnreloc: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-LINE RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrelocr: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-LINE2"#]
    #[doc = r#"Description: "#]
    pub lnreloc2: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-LINE-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnreloc5: Option<f32>,

    #[doc = r#"Title: RE MULTIFAMILY"#]
    #[doc = r#"Description: "#]
    pub lnremult: Option<f32>,

    #[doc = r#"Title: RE MULTIFAMILY-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnremul5: Option<f32>,

    #[doc = r#"Title: RE MULTIFAMILY RATIO"#]
    #[doc = r#"Description: "#]
    pub lnremultr: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRESIDENTIAL PROP"#]
    #[doc = r#"Description: "#]
    pub lnrenres: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRESIDENTIAL CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnrenre5: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRESIDENTIAL CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnrenre2: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRESIDENTIAL PROP RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenresr: Option<f32>,

    #[doc = r#"Title: PREPAID TAXES & INS ON MTG LNS"#]
    #[doc = r#"Description: "#]
    pub lnrepp: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub lnreres: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreresr: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY2"#]
    #[doc = r#"Description: "#]
    pub lnreres2: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnreres5: Option<f32>,

    #[doc = r#"Title: ALLOWANCE FOR RE LOAN"#]
    #[doc = r#"Description: "#]
    pub lnresre: Option<f32>,

    #[doc = r#"Title: LEASES"#]
    #[doc = r#"Description: "#]
    pub ls: Option<f32>,

    #[doc = r#"Title: LEASES RATIO"#]
    #[doc = r#"Description: "#]
    pub lsr: Option<f32>,

    #[doc = r#"Title: METROPOLITAN FLAG"#]
    #[doc = r#"Description: A flag used to indicate whether an institution is in a metropolitan statistical area. The U.S census bureau office of management and budget defines the metropolitan statistical area. A core based statistical area associated with at least one urbanized area that has a population of at least 50,000. The metropolitan statistical area comprises the central county or counties containing the core, plus adjacent outlying counties having a high degree of social and economic integration with the central county as measured through commuting. 0=institution is not in a metropolitan statistical area. 1=institution is in a metropolitan statistical area."#]
    pub metro: Option<f32>,

    #[doc = r#"Title: INSURED SAVINGS BANK FLAG"#]
    #[doc = r#"Description: "#]
    pub mi: Option<f32>,

    #[doc = r#"Title: MICROPOLITAN FLAG"#]
    #[doc = r#"Description: A flag used to indicate whether an institution is in a micropolitan statistical area. The U.S census bureau office of management and budget defines the micropolitan statistical area. A core based statistical area associated with at least one urban cluster that has a population of at least 10,000 but less than 50,000. The micropolitan statistical area comprises the central county or counties containing the core, plus adjacent outlying counties having a high degree of social and economic integration with the central county as measured through commuting. 0=institution is not in a micropolitan statistical area. 1=institution is in a micropolitan statistical area."#]
    pub micro: Option<f32>,

    #[doc = r#"Title: MINORITY CODE"#]
    #[doc = r#"Description: A character field on the institution file corresponding to a type of minority ownership. .=NONE. 01=African American. 02=Hispanic American. 03=Asian or Pacific Islander Americans. 04=Native American or Native Alaskan American. 05=Multi-Racial American. 06=Minority Board and serving African American Community. 08=Minority Board and serving Asian/Pacific Islander Americans. 10=Minority Board and serving Multi-Racial community."#]
    pub mnrtycde: Option<f32>,

    #[doc = r#"Title: EFFECTIVE DTE OF MINORITY STATUS"#]
    #[doc = r#"Description: Represent the effective date on which an institution is assigned a minority status, transaction in dates. Format(DDMONCCYY) day, month abbrev, century, and year."#]
    pub mnrtydte: Option<f32>,

    #[doc = r#"Title: MORTGAGE INDEBTEDNESS & CAP LS"#]
    #[doc = r#"Description: "#]
    pub mtgls: Option<f32>,

    #[doc = r#"Title: NATIONAL BANK FLAG"#]
    #[doc = r#"Description: "#]
    pub n: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-LOANS & LEASES"#]
    #[doc = r#"Description: "#]
    pub nalnls: Option<f32>,

    #[doc = r#"Title: NONINSURED COMMERCIAL INST FLAG"#]
    #[doc = r#"Description: "#]
    pub nc: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-LOANS & LEASES"#]
    #[doc = r#"Description: "#]
    pub nclnls: Option<f32>,

    #[doc = r#"Title: NET INC - ATTRIB TO MINORITY INT"#]
    #[doc = r#"Description: "#]
    pub netimin: Option<f32>,

    #[doc = r#"Title: NET INC - ATTRIB TO MINORITY INT RATIO"#]
    #[doc = r#"Description: "#]
    pub netiminr: Option<f32>,

    #[doc = r#"Title: NET INC - ATTRIB TO MINORITY INT QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub netiminq: Option<f32>,

    #[doc = r#"Title: NET INC - ATTRIB TO MINORITY INT QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub netiminqr: Option<f32>,

    #[doc = r#"Title: NET INC - BANK & MINORITY INT"#]
    #[doc = r#"Description: "#]
    pub netinbm: Option<f32>,

    #[doc = r#"Title: NET INC - BANK & MINORITY INT RATIO"#]
    #[doc = r#"Description: "#]
    pub netinbmr: Option<f32>,

    #[doc = r#"Title: NET INC - BANK & MINORITY INT QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub netinbmq: Option<f32>,

    #[doc = r#"Title: NET INCOME BEFORE TAXES ANNUALLY"#]
    #[doc = r#"Description: "#]
    pub netinbxa: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub netibxqa: Option<f32>,

    #[doc = r#"Title: NET INC - BANK & MINORITY INT QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub netinbmqr: Option<f32>,

    #[doc = r#"Title: NEW INSTITUTION FLAG"#]
    #[doc = r#"Description: "#]
    pub newinst: Option<f32>,

    #[doc = r#"Title: NUMBER OF FIDUCIARY ACCOUNTS AND RELATED ASSET ACCOUNTS"#]
    #[doc = r#"Description: "#]
    pub nfaa: Option<f32>,

    #[doc = r#"Title: NET INTEREST INCOME"#]
    #[doc = r#"Description: "#]
    pub nim: Option<f32>,

    #[doc = r#"Title: NET INTEREST INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub nimr: Option<f32>,

    #[doc = r#"Title: NET INTEREST INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub nimq: Option<f32>,

    #[doc = r#"Title: NET INTEREST INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub nimqa: Option<f32>,

    #[doc = r#"Title: NET INTEREST INCOME ANNUALLY"#]
    #[doc = r#"Description: "#]
    pub nima: Option<f32>,

    #[doc = r#"Title: NET INTEREST INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub nimqr: Option<f32>,

    #[doc = r#"Title: NONMEMBER INSURED INST FLAG"#]
    #[doc = r#"Description: "#]
    pub nm: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST INCOME"#]
    #[doc = r#"Description: "#]
    pub nonii: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub noniir: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST EXPENSE"#]
    #[doc = r#"Description: "#]
    pub nonix: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST EXPENSE RATIO"#]
    #[doc = r#"Description: "#]
    pub nonixr: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub nonixq: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub nonixqa: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST EXPENSE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub nonixqr: Option<f32>,

    #[doc = r#"Title: NONINSURED SAVINGS INST FLAG"#]
    #[doc = r#"Description: "#]
    pub ns: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntlnls: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntlnlscor: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS NET CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntlnlsq: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS NET CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntlnlsqa: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS NET CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntlnlscoqr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-TOTAL"#]
    #[doc = r#"Description: "#]
    pub ntr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-IPC"#]
    #[doc = r#"Description: "#]
    pub ntripc: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-IPC RATIO"#]
    #[doc = r#"Description: "#]
    pub ntripcr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-MUNI"#]
    #[doc = r#"Description: "#]
    pub ntrmuni: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-MUNI RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrmunir: Option<f32>,

    #[doc = r#"Title: TIME DEPOSITS-TOTAL"#]
    #[doc = r#"Description: "#]
    pub ntrtime: Option<f32>,

    #[doc = r#"Title: TIME DEPOSITS OVER $100M"#]
    #[doc = r#"Description: "#]
    pub ntrtmlg: Option<f32>,

    #[doc = r#"Title: AMT TOTAL TIME DEP MORE THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub ntrtmlgj: Option<f32>,

    #[doc = r#"Title: AMT TOTAL TIME DEP MORE THAN $250,000 RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrtmlgjr: Option<f32>,

    #[doc = r#"Title: AMT TIME DEP OF $250,000 OR LESS"#]
    #[doc = r#"Description: "#]
    pub ntrtmmed: Option<f32>,

    #[doc = r#"Title: AMT TIME DEP OF $250,000 OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrtmmedr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-U.S. GOVERNMENT"#]
    #[doc = r#"Description: "#]
    pub ntrusgov: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-U.S. GOVERNMENT RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrusgovr: Option<f32>,

    #[doc = r#"Title: RETAINED EARNINGS ANUALLY"#]
    #[doc = r#"Description: "#]
    pub ntirta: Option<f32>,

    #[doc = r#"Title: TOTAL LN & LS LOSS NET CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub nttot: Option<f32>,

    #[doc = r#"Title: NUMBER OF FULL TIME EMPLOYEES"#]
    #[doc = r#"Description: "#]
    pub numemp: Option<f32>,

    #[doc = r#"Title: OTHER ASSETS"#]
    #[doc = r#"Description: "#]
    pub oa: Option<f32>,

    #[doc = r#"Title: OAKAR FLAG"#]
    #[doc = r#"Description: A flag used to indicate whether an institution acquired deposits that were previously insured under a different insurance fund. 0=has no oakar deposits. 1=has oakar deposits."#]
    pub oakar: Option<f32>,

    #[doc = r#"Title: OCC DISTRICT"#]
    #[doc = r#"Description: "#]
    pub occdist: Option<f32>,

    #[doc = r#"Title: OCC DISTRICT DESC"#]
    #[doc = r#"Description: "#]
    pub occdistdesc: Option<String>,

    #[doc = r#"Title: DOMESTIC MULTI-SERVICE OFFICES"#]
    #[doc = r#"Description: The number of multiple service domestic offices operated by an institution."#]
    pub offdmult: Option<f32>,

    #[doc = r#"Title: NONDOMESTIC OFFICES"#]
    #[doc = r#"Description: The number of nondomestic offices operated by an institution."#]
    pub offndom: Option<f32>,

    #[doc = r#"Title: DOMESTIC OTHER OFFICES"#]
    #[doc = r#"Description: The number of domestic non-multiple service offices operated by an institution."#]
    pub offoth: Option<f32>,

    #[doc = r#"Title: SOD OFFICES"#]
    #[doc = r#"Description: The number of offices operated by an institution based on the summary of deposits definition of offices."#]
    pub offsod: Option<f32>,

    #[doc = r#"Title: NUMBER OF STATES WITH OFFICES"#]
    #[doc = r#"Description: The number of states with offices (including its main office)."#]
    pub offstate: Option<f32>,

    #[doc = r#"Title: TOTAL OFFICES"#]
    #[doc = r#"Description: The total number of offices operated by an institution."#]
    pub offtot: Option<f32>,

    #[doc = r#"Title: U.S. AND OTHER AREA OFFICES"#]
    #[doc = r#"Description: The number of domestic and U.S terrirtories offices operated by an institution."#]
    pub offusoa: Option<f32>,

    #[doc = r#"Title: INSURED IBA OFFICE FLAG"#]
    #[doc = r#"Description: "#]
    pub oi: Option<f32>,

    #[doc = r#"Title: OTS DISTRICT"#]
    #[doc = r#"Description: A number used to identify the office of thrift supervision district in which the institution is located. 01=Northeast. 02=Southeast. 03=Midwest. 04=West."#]
    pub otsdist: Option<f32>,

    #[doc = r#"Title: OTS REGION NUMBER"#]
    #[doc = r#"Description: A number used to identify the office of thrift supervision region in which the institution is located. 01=Northeast. 02=Southeast. 03=Midwest. 04=West."#]
    pub otsregno: Option<f32>,

    #[doc = r#"Title: OTHER LIAB & MINOR IN SUBS"#]
    #[doc = r#"Description: "#]
    pub olmin: Option<f32>,

    #[doc = r#"Title: OTHER REAL ESTATE OWNED"#]
    #[doc = r#"Description: "#]
    pub ore: Option<f32>,

    #[doc = r#"Title: OTHER REAL ESTATE OWNED RATIO"#]
    #[doc = r#"Description: "#]
    pub orer: Option<f32>,

    #[doc = r#"Title: OTHER LIABILITIES-FHLB"#]
    #[doc = r#"Description: "#]
    pub othbfhlb: Option<f32>,

    #[doc = r#"Title: OTHER LIABILITIES-FHLB RATIO"#]
    #[doc = r#"Description: "#]
    pub othbfhlbr: Option<f32>,

    #[doc = r#"Title: OTHER BORROWED MONEY"#]
    #[doc = r#"Description: "#]
    pub othbor: Option<f32>,

    #[doc = r#"Title: OTH BORROWED FUNDS"#]
    #[doc = r#"Description: "#]
    pub othbrf: Option<f32>,

    #[doc = r#"Title: OTH BORROWED FUNDS RATIO"#]
    #[doc = r#"Description: "#]
    pub othbrfr: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP ONE YR OR LESS"#]
    #[doc = r#"Description: "#]
    pub otbfh1l: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP ONE YR OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub otbfh1lr: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP ONE YR THROUGH THREE"#]
    #[doc = r#"Description: "#]
    pub otbfh1t3: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP ONE YR THROUGH THREE"#]
    #[doc = r#"Description: "#]
    pub otbfh1t3r: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP THREE THROUGH FIVE"#]
    #[doc = r#"Description: "#]
    pub otbfh3t5: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP THREE THROUGH FIVE RATIO"#]
    #[doc = r#"Description: "#]
    pub otbfh3t5r: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP OVER FIVE YEARS"#]
    #[doc = r#"Description: "#]
    pub otbfhov5: Option<f32>,

    #[doc = r#"Title: FHLB ADV MAT REP OVER FIVE YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub otbfhov5r: Option<f32>,

    #[doc = r#"Title: FHLB ADV WITH REMAINING MAT ONE YR OR LESS"#]
    #[doc = r#"Description: "#]
    pub othbfh1l: Option<f32>,

    #[doc = r#"Title: FHLB ADV WITH REMAINING MAT ONE YR OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub othbfh1lr: Option<f32>,

    #[doc = r#"Title: FHLB STRUCTURED ADV"#]
    #[doc = r#"Description: "#]
    pub otbfhsta: Option<f32>,

    #[doc = r#"Title: FHLB STRUCTURED ADV"#]
    #[doc = r#"Description: "#]
    pub otbfhstar: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING ONE YR OR LESS"#]
    #[doc = r#"Description: "#]
    pub otbot1l: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING ONE YR OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub otbot1lr: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING ONE YR THROUGH THREE"#]
    #[doc = r#"Description: "#]
    pub otbot1t3: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING ONE YR THROUGH THREE RATIO"#]
    #[doc = r#"Description: "#]
    pub otbot1t3r: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING THREE YR THROUGH FIVE"#]
    #[doc = r#"Description: "#]
    pub otbot3t5: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING THREE YR THROUGH FIVE RATIO"#]
    #[doc = r#"Description: "#]
    pub otbot3t5r: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING OVER FIVE YRS"#]
    #[doc = r#"Description: "#]
    pub otbotov5: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT OR NEXT REPRICING OVER FIVE YRS RATIO"#]
    #[doc = r#"Description: "#]
    pub otbotov5r: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT REMAINING MAT OF ONE YR OR LESS"#]
    #[doc = r#"Description: "#]
    pub othbot1l: Option<f32>,

    #[doc = r#"Title: OTH BORR MAT REMANING MAT OF ONE YR OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub othbot1lr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LIABILITIES"#]
    #[doc = r#"Description: "#]
    pub allothl: Option<f32>,

    #[doc = r#"Title: ALL OTHER LIABILITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub allothlr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-LOANS & LEASES"#]
    #[doc = r#"Description: "#]
    pub p3lnls: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-LOANS & LEASES"#]
    #[doc = r#"Description: "#]
    pub p9lnls: Option<f32>,

    #[doc = r#"Title: QBP COMMERCIAL BANK REGION"#]
    #[doc = r#"Description: "#]
    pub qbprcoml: Option<f32>,

    #[doc = r#"Title: QBP COMMERCIAL BANK REGION DESC"#]
    #[doc = r#"Description: "#]
    pub qbprcomldesc: Option<String>,

    #[doc = r#"Title: QBP BIF FUND SAVINGS REGION"#]
    #[doc = r#"Description: "#]
    pub qbprsavb: Option<f32>,

    #[doc = r#"Title: QBP SAVING SAIF FUND REGION"#]
    #[doc = r#"Description: "#]
    pub qbprsavs: Option<f32>,

    #[doc = r#"Title: QUARTER NUMBER"#]
    #[doc = r#"Description: Identifies the calendar quarter. 1=March. 2=June. 3=September. 4=December."#]
    pub qtrno: Option<f32>,

    #[doc = r#"Title: PRIMARY REGULATING AGENCY"#]
    #[doc = r#"Description: "#]
    pub regagnt: Option<String>,

    #[doc = r#"Title: FDIC RISK TERRITORY"#]
    #[doc = r#"Description: An abbreviation of the current risk territory for an institution (FDIC Risk Territory). All periods are displayed in the current perspective (exceptions can exist depending on when a quarter is updated)."#]
    pub riskterr: Option<String>,

    #[doc = r#"Title: ASSETS 10B TO 250B FLAG"#]
    #[doc = r#"Description: "#]
    pub s10t250b: Option<f32>,

    #[doc = r#"Title: SASSER FLAG"#]
    #[doc = r#"Description: A flag used to indicate whether an institution was a former savings association that has converted to a bank charter and is still a SAIF insured institution. 0=not a sasser institution. 1=is a sasser institution."#]
    pub sasser: Option<f32>,

    #[doc = r#"Title: SAVINGS BANK FLAG"#]
    #[doc = r#"Description: "#]
    pub sb: Option<f32>,

    #[doc = r#"Title: SECURITIES"#]
    #[doc = r#"Description: "#]
    pub sc: Option<f32>,

    #[doc = r#"Title: SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scr: Option<f32>,

    #[doc = r#"Title: TOTAL AVAILABLE-FOR-SALE AT AMORTIZED COST SECURITIES ON A CONSOLIDATED BASIS"#]
    #[doc = r#"Description: "#]
    pub scaa: Option<f32>,

    #[doc = r#"Title: TOTAL HELD-TO-MATURITY AT FAIR VALUE SECURITIES ON A CONSOLIDATED BASIS"#]
    #[doc = r#"Description: "#]
    pub schf: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY"#]
    #[doc = r#"Description: "#]
    pub scage: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY"#]
    #[doc = r#"Description: "#]
    pub scaspnha: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY"#]
    #[doc = r#"Description: "#]
    pub scaspnaf: Option<f32>,

    #[doc = r#"Title: NON-MORT BACKED ISSUES BY US GOVT OR SPONSORED AGENCIES"#]
    #[doc = r#"Description: "#]
    pub scaspnsum: Option<f32>,

    #[doc = r#"Title: NON-MORT BACKED ISSUES BY US GOVT OR SPONSORED AGENCIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scaspnsumr: Option<f32>,

    #[doc = r#"Title: DOMESTIC SEC*DEBT & EQUITY - CON"#]
    #[doc = r#"Description: "#]
    pub scdeq: Option<f32>,

    #[doc = r#"Title: OTHER DOMESTIC DEBT"#]
    #[doc = r#"Description: "#]
    pub scdomo: Option<f32>,

    #[doc = r#"Title: OTHER DOMESTIC DEBT RATIO"#]
    #[doc = r#"Description: "#]
    pub scdomor: Option<f32>,

    #[doc = r#"Title: EQUITY SECURITIES"#]
    #[doc = r#"Description: "#]
    pub sceq: Option<f32>,

    #[doc = r#"Title: FOREIGN DEBT & EQUITY"#]
    #[doc = r#"Description: "#]
    pub scfdeq: Option<f32>,

    #[doc = r#"Title: FOREIGN DEBT SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scford: Option<f32>,

    #[doc = r#"Title: FOREIGN DEBT SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scfordr: Option<f32>,

    #[doc = r#"Title: MORTGAGE BACKED SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scmtgbk: Option<f32>,

    #[doc = r#"Title: MORTGAGE BACKED SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scmtgbkr: Option<f32>,

    #[doc = r#"Title: MUNICIPAL SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scmuni: Option<f32>,

    #[doc = r#"Title: MUNICIPAL RATIO"#]
    #[doc = r#"Description: "#]
    pub scmunir: Option<f32>,

    #[doc = r#"Title: SECURITIES-MV"#]
    #[doc = r#"Description: "#]
    pub scmv: Option<f32>,

    #[doc = r#"Title: RES-OTH DOM DEBT*PRIV CERTS"#]
    #[doc = r#"Description: "#]
    pub scodpc: Option<f32>,

    #[doc = r#"Title: RES-OTH DOM DEBT*PRIV CERTS RATIO"#]
    #[doc = r#"Description: "#]
    pub scodpcr: Option<f32>,

    #[doc = r#"Title: CONTRA-ASSETS TO SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scres: Option<f32>,

    #[doc = r#"Title: U.S. TREASURY & AGENCY"#]
    #[doc = r#"Description: "#]
    pub scus: Option<f32>,

    #[doc = r#"Title: U.S. TREASURY & AGENCY RATIO"#]
    #[doc = r#"Description: "#]
    pub scusr: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY ALL OTHER"#]
    #[doc = r#"Description: "#]
    pub scusa: Option<f32>,

    #[doc = r#"Title: U.S. TREASURY SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scust: Option<f32>,

    #[doc = r#"Title: U.S. TREASURY SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scustr: Option<f32>,

    #[doc = r#"Title: GEOGRAPHIC LATITUDE OF MAIN OFFICE"#]
    #[doc = r#"Description: Geographic latitude of main office."#]
    pub sims_lat: Option<f32>,

    #[doc = r#"Title: GEOGRAPHIC LONGITUDE OF MAIN OFFICE"#]
    #[doc = r#"Description: Geographic longitude of main office"#]
    pub sims_long: Option<f32>,

    #[doc = r#"Title: SAVINGS AND LOAN FLAG"#]
    #[doc = r#"Description: "#]
    pub sl: Option<f32>,

    #[doc = r#"Title: STATE MEMBER BANK FLAG"#]
    #[doc = r#"Description: "#]
    pub sm: Option<f32>,

    #[doc = r#"Title: FIPS STATE ALPHA CODE (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub stalp: Option<String>,

    #[doc = r#"Title: STATE CHARTER FLAG"#]
    #[doc = r#"Description: "#]
    pub stchrtr: Option<f32>,

    #[doc = r#"Title: STATE NAME (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub stname: Option<String>,

    #[doc = r#"Title: FIPS STATE NUMBER"#]
    #[doc = r#"Description: "#]
    pub stnum: Option<f32>,

    #[doc = r#"Title: SUB. DEBT & L/L PREFERRED STK"#]
    #[doc = r#"Description: "#]
    pub subllpf: Option<f32>,

    #[doc = r#"Title: SUBORDINATED NOTES & DEBENTURES"#]
    #[doc = r#"Description: "#]
    pub subnd: Option<f32>,

    #[doc = r#"Title: ASSETS UNDER 25M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz25: Option<f32>,

    #[doc = r#"Title: ASSETS UNDER 100M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz100: Option<f32>,

    #[doc = r#"Title: ASSETS OVER 100M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz100mp: Option<f32>,

    #[doc = r#"Title: ASSETS 100M TO 300M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz100t3: Option<f32>,

    #[doc = r#"Title: ASSETS 100M TO 500M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz100t5: Option<f32>,

    #[doc = r#"Title: ASSETS 100M TO 1B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz100t1b: Option<f32>,

    #[doc = r#"Title: ASSETS OVER 10B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz10bp: Option<f32>,

    #[doc = r#"Title: ASSETS OVER 1B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz1bp: Option<f32>,

    #[doc = r#"Title: ASSETS 1B TO 10B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz1bt10b: Option<f32>,

    #[doc = r#"Title: ASSETS 1B TO 3B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz1bt3b: Option<f32>,

    #[doc = r#"Title: ASSETS 1B TO 5B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz1bt5b: Option<f32>,

    #[doc = r#"Title: ASSETS OVER 250B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz250bp: Option<f32>,

    #[doc = r#"Title: ASSETS 25M TO 50M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz25t50: Option<f32>,

    #[doc = r#"Title: ASSETS 300M TO 500M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz300t5: Option<f32>,

    #[doc = r#"Title: ASSETS 3B TO 10B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz3bt10b: Option<f32>,

    #[doc = r#"Title: ASSETS 500M TO 1B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz500t1b: Option<f32>,

    #[doc = r#"Title: ASSETS 50M TO 100M FLAG"#]
    #[doc = r#"Description: "#]
    pub sz50t100: Option<f32>,

    #[doc = r#"Title: ASSETS OVER 5B FLAG"#]
    #[doc = r#"Description: "#]
    pub sz5bp: Option<f32>,

    #[doc = r#"Title: TOTAL FIDUCIARY AND RELATED ASSETS"#]
    #[doc = r#"Description: "#]
    pub tfra: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNTS"#]
    #[doc = r#"Description: "#]
    pub trade: Option<f32>,

    #[doc = r#"Title: TRADING LIABILITIES"#]
    #[doc = r#"Description: "#]
    pub tradel: Option<f32>,

    #[doc = r#"Title: TRADING LIABILITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub tradelr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNTS RATIO"#]
    #[doc = r#"Description: "#]
    pub trader: Option<f32>,

    #[doc = r#"Title: TRANSACTION-TOTAL"#]
    #[doc = r#"Description: "#]
    pub trn: Option<f32>,

    #[doc = r#"Title: TRANSACTION-TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub trnr: Option<f32>,

    #[doc = r#"Title: TRANSACTION-IPC"#]
    #[doc = r#"Description: "#]
    pub trnipc: Option<f32>,

    #[doc = r#"Title: TRAN-IPC-OFFICIAL CHECKS"#]
    #[doc = r#"Description: "#]
    pub trnipcoc: Option<f32>,

    #[doc = r#"Title: TRAN-IPC-OFFICIAL CHECKS RATIO"#]
    #[doc = r#"Description: "#]
    pub trnipcocr: Option<f32>,

    #[doc = r#"Title: TRANSACTION-MUNI"#]
    #[doc = r#"Description: "#]
    pub trnmuni: Option<f32>,

    #[doc = r#"Title: TRANSACTION-MUNI RATIO"#]
    #[doc = r#"Description: "#]
    pub trnmunir: Option<f32>,

    #[doc = r#"Title: TRANSACTION-U.S. GOVERNMENT"#]
    #[doc = r#"Description: "#]
    pub trnusgov: Option<f32>,

    #[doc = r#"Title: TRANSACTION-U.S. GOVERNMENT RATIO"#]
    #[doc = r#"Description: "#]
    pub trnusgovr: Option<f32>,

    #[doc = r#"Title: TRUST POWER GRANTED CODES"#]
    #[doc = r#"Description: Is a two digit numeric code which identifies the trust power granted categories of an institution. 00 - Trust powers not known. 10 - Full trust powers granted. 11 - Full trust powers granted, exercised. 12 - Full trust powers granted, not exercised. 20 - Limited trust powers granted. 21 - Limited trust powers granted, exercised. 30 - Trust powers not granted. 31 - Trust powers not granted but exercised. 40 - Full trust powers grandfathered."#]
    pub trustpwr: Option<f32>,

    #[doc = r#"Title: TIME & SAVINGS DEPOSITS-TOTAL"#]
    #[doc = r#"Description: "#]
    pub ts: Option<f32>,

    #[doc = r#"Title: TIME & SAVINGS DEPOSITS-TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub tsr: Option<f32>,

    #[doc = r#"Title: TT&L NOTE OPTION"#]
    #[doc = r#"Description: "#]
    pub ttl: Option<f32>,

    #[doc = r#"Title: TT&L & OTHER BORROWINGS"#]
    #[doc = r#"Description: "#]
    pub ttlotbor: Option<f32>,

    #[doc = r#"Title: UNEARNED INCOME"#]
    #[doc = r#"Description: "#]
    pub uninc: Option<f32>,

    #[doc = r#"Title: BANK UNIQUE NUMBER"#]
    #[doc = r#"Description: A unique identification number assigned to an institution by the FDIC."#]
    pub uninum: Option<f32>,

    #[doc = r#"Title: USA LOCATED INSTITUTION"#]
    #[doc = r#"Description: "#]
    pub usa: Option<f32>,

    #[doc = r#"Title: UNAMORTIZED YIELD ADJ-MTG LOANS"#]
    #[doc = r#"Description: "#]
    pub uyamtg: Option<f32>,

    #[doc = r#"Title: ASST-BCK UNUSED COMMIT - RELATED"#]
    #[doc = r#"Description: "#]
    pub abcubk: Option<f32>,

    #[doc = r#"Title: ASST-BCK UNUSED COMMIT - RELATED RATIO"#]
    #[doc = r#"Description: "#]
    pub abcubkr: Option<f32>,

    #[doc = r#"Title: ASSET-BACK UNUSED COMMIT - OTHER"#]
    #[doc = r#"Description: "#]
    pub abcuoth: Option<f32>,

    #[doc = r#"Title: ASSET-BACK UNUSED COMMIT - OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub abcuothr: Option<f32>,

    #[doc = r#"Title: ASSET-BACK CREDIT EX-RELATED"#]
    #[doc = r#"Description: "#]
    pub abcxbk: Option<f32>,

    #[doc = r#"Title: ASSET-BACK CREDIT EX-RELATED RATIO"#]
    #[doc = r#"Description: "#]
    pub abcxbkr: Option<f32>,

    #[doc = r#"Title: ASSET-BACK CREDIT EX-OTHER"#]
    #[doc = r#"Description: "#]
    pub abcxoth: Option<f32>,

    #[doc = r#"Title: ASSET-BACK CREDIT EX-OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub abcxothr: Option<f32>,

    #[doc = r#"Title: C.E. RECOURSE NOT SECUR. - OTH"#]
    #[doc = r#"Description: "#]
    pub asceoth: Option<f32>,

    #[doc = r#"Title: C.E. RECOURSE NOT SECUR. - OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub asceothr: Option<f32>,

    #[doc = r#"Title: C.E. RECOURSE NOT SECUR. - RES"#]
    #[doc = r#"Description: "#]
    pub asceres: Option<f32>,

    #[doc = r#"Title: C.E. RECOURSE NOT SECUR. - RES RATIO"#]
    #[doc = r#"Description: "#]
    pub asceresr: Option<f32>,

    #[doc = r#"Title: SOLD W/RECOURSE N/SECUR. - OTH"#]
    #[doc = r#"Description: "#]
    pub asdroth: Option<f32>,

    #[doc = r#"Title: SOLD W/RECOURSE N/SECUR. - OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub asdrothr: Option<f32>,

    #[doc = r#"Title: SOLD W/RECOURSE N/SECUR.- RES"#]
    #[doc = r#"Description: "#]
    pub asdrres: Option<f32>,

    #[doc = r#"Title: SOLD W/RECOURSE N/SECUR.- RES RATIO"#]
    #[doc = r#"Description: "#]
    pub asdrresr: Option<f32>,

    #[doc = r#"Title: TOTAL ASSETS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub asset2: Option<f32>,

    #[doc = r#"Title: TOTAL ASSETS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub asset5: Option<f32>,

    #[doc = r#"Title: TOTAL ASSETS-FOR"#]
    #[doc = r#"Description: "#]
    pub assetfor: Option<f32>,

    #[doc = r#"Title: LONG-TERM ASSETS (5+ YEARS)-QBP"#]
    #[doc = r#"Description: "#]
    pub asstlt: Option<f32>,

    #[doc = r#"Title: LONG-TERM ASSETS (5+ YEARS) RATIO"#]
    #[doc = r#"Description: "#]
    pub asstltr: Option<f32>,

    #[doc = r#"Title: ASSETS PER EMPLOYEE IN MILLION"#]
    #[doc = r#"Description: "#]
    pub astempm: Option<f32>,

    #[doc = r#"Title: AVERAGE ASSETS-ADJUSTED-PCA"#]
    #[doc = r#"Description: "#]
    pub avassetj: Option<f32>,

    #[doc = r#"Title: AVERAGE ASSETS-ADJUSTED-PCA RATIO"#]
    #[doc = r#"Description: "#]
    pub avassetjr: Option<f32>,

    #[doc = r#"Title: BROKERED DEP-INSURED"#]
    #[doc = r#"Description: "#]
    pub broins: Option<f32>,

    #[doc = r#"Title: BROKERED DEP-INSURED RATIO"#]
    #[doc = r#"Description: "#]
    pub broinsr: Option<f32>,

    #[doc = r#"Title: REPORT DATE (CCYYMMDD)"#]
    #[doc = r#"Description: "#]
    pub callymd: Option<f32>,

    #[doc = r#"Title: CASH & DUE FROM DEP INST-FOR"#]
    #[doc = r#"Description: "#]
    pub chbalfor: Option<f32>,

    #[doc = r#"Title: NONINTEREST-BEARING CASH & DUE"#]
    #[doc = r#"Description: "#]
    pub chbalni: Option<f32>,

    #[doc = r#"Title: NONINTEREST-BEARING CASH & DUE RATIO"#]
    #[doc = r#"Description: "#]
    pub chbalnir: Option<f32>,

    #[doc = r#"Title: CASH ITEMS"#]
    #[doc = r#"Description: "#]
    pub chcic: Option<f32>,

    #[doc = r#"Title: CASH ITEMS RATIO"#]
    #[doc = r#"Description: "#]
    pub chcicr: Option<f32>,

    #[doc = r#"Title: CURRENCY & COIN"#]
    #[doc = r#"Description: "#]
    pub chcoin: Option<f32>,

    #[doc = r#"Title: CURRENCY & COIN RATIO"#]
    #[doc = r#"Description: "#]
    pub chcoinr: Option<f32>,

    #[doc = r#"Title: NET OPERATING CASH FLOW-ANN"#]
    #[doc = r#"Description: "#]
    pub chfla: Option<f32>,

    #[doc = r#"Title: NET OPERATING CASH FLOW-ANN Quarterly"#]
    #[doc = r#"Description: "#]
    pub chflq: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM FRB"#]
    #[doc = r#"Description: "#]
    pub chfrb: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM FRB RATIO"#]
    #[doc = r#"Description: "#]
    pub chfrbr: Option<f32>,

    #[doc = r#"Title: CASH ITEM COLLEC IN DOMESTIC OFFICES"#]
    #[doc = r#"Description: "#]
    pub chitem: Option<f32>,

    #[doc = r#"Title: CASH ITEMS COLLEC IN DOMESTIC OFFICES RATIO"#]
    #[doc = r#"Description: "#]
    pub chitemr: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM BK FOR COUNTRY"#]
    #[doc = r#"Description: "#]
    pub chnus: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM BK FOR COUNTRY RATIOS"#]
    #[doc = r#"Description: "#]
    pub chnusr: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM FOR BR OF OTH US BK"#]
    #[doc = r#"Description: "#]
    pub chnusfbk: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM DEP INST U.S."#]
    #[doc = r#"Description: "#]
    pub chus: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM DEP INST U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub chusr: Option<f32>,

    #[doc = r#"Title: BAL DUE FROM U.S. BR OF FOR BKS"#]
    #[doc = r#"Description: "#]
    pub chusfbk: Option<f32>,

    #[doc = r#"Title: CITY (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub city: Option<String>,

    #[doc = r#"Title: CORE DEPOSITS"#]
    #[doc = r#"Description: "#]
    pub coredep: Option<f32>,

    #[doc = r#"Title: CORE DEPOSITS RATIO"#]
    #[doc = r#"Description: "#]
    pub coredepr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crag: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub cragr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub cragq: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub cragqr: Option<f32>,

    #[doc = r#"Title: AG LOAN RECOVERIES*SMALL BKS"#]
    #[doc = r#"Description: "#]
    pub cragsm: Option<f32>,

    #[doc = r#"Title: AAG LOAN RECOVERIES*SMALL BKS RATIO"#]
    #[doc = r#"Description: "#]
    pub cragsmr: Option<f32>,

    #[doc = r#"Title: AG LOAN RECOVERIES*SMALL BKS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub cragsmq: Option<f32>,

    #[doc = r#"Title: AG LOAN RECOVERIES*SMALL BKS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub cragsmqr: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crauto: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crautor: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crautoq: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crautoqr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crci: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crcir: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crciq: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crciqr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES NON-U.S."#]
    #[doc = r#"Description: "#]
    pub crcinus: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES NON-U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub crcinusr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES NON-U.S. QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crcinusq: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN RECOVERIES NON-U.S. QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crcinusqr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crcon: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crconr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crconq: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crconqr: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crconoth: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crconothr: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crconotq: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crconotqr: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crcrcd: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crcrcdr: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crcrcdq: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crcrcdqr: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crdep: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crdepr: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crdepq: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN RECOVERIES Quarterly RATIO"#]
    #[doc = r#"Description: "#]
    pub crdepqr: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crdepnus: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crdepnusr: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crdepnuq: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crdepnuqr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crforgv: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crforgvr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crforgvq: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crforgvqr: Option<f32>,

    #[doc = r#"Title: LEASE RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crls: Option<f32>,

    #[doc = r#"Title: LEASE RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crlsr: Option<f32>,

    #[doc = r#"Title: LEASE RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crlsq: Option<f32>,

    #[doc = r#"Title: LEASE RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crlsqr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crother: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crotherr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crothq: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crothqr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crre: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crrer: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crreq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crreqr: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crreag: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crreagr: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN RECOVERIES-QTR"#]
    #[doc = r#"Description: "#]
    pub crreagq: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crreagqr: Option<f32>,

    #[doc = r#"Title: 1-4 FAM CONSTRUCT LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crrecnfm: Option<f32>,

    #[doc = r#"Title: OTHER CONSTRUCT LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crrecnot: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN RECOVER-QTR"#]
    #[doc = r#"Description: "#]
    pub crreconq: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crreconqr: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crrecons: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crreconsr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN RECOVERIES - FOR"#]
    #[doc = r#"Description: "#]
    pub crrefor: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN RECOVERIES - FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub crreforr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN RECOVERIES - FOR QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crreforq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN RECOVERIES - FOR QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crreforqr: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crreloc: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crrelocr: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crrelocq: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crrelocqr: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RE LN RECOVERIES-QTR"#]
    #[doc = r#"Description: "#]
    pub crremulq: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RES RE LN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crremulqr: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RES RE LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crremult: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RES RE LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crremultr: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crrenres: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crrenresr: Option<f32>,

    #[doc = r#"Title: OTHER NONFARM NONRES RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crrenrot: Option<f32>,

    #[doc = r#"Title: OWN-OCCUP NONFARM NONRES RECOV"#]
    #[doc = r#"Description: "#]
    pub crrenrow: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN RECOVER-QTR"#]
    #[doc = r#"Description: "#]
    pub crrenrsq: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN RECOVER-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub crrenrsqr: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crrenus: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crrenusr: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN RECOVERIES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crrenusq: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crrenusqr: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY RECOVERIES"#]
    #[doc = r#"Description: "#]
    pub crreres: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY RECOVERIES RATIO"#]
    #[doc = r#"Description: "#]
    pub crreresr: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY RECOVER-QTR"#]
    #[doc = r#"Description: "#]
    pub crreresq: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY RECOVERIES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crreresqr: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM JR LIEN-RECOVER"#]
    #[doc = r#"Description: "#]
    pub crrersf2: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM JR LIEN-RECOVER RATIO"#]
    #[doc = r#"Description: "#]
    pub crrersf2r: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM JR LIEN-RECOVER QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crrers2q: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM JR LIEN-RECOVER QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crrers2qr: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV"#]
    #[doc = r#"Description: "#]
    pub crrersfm: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV RATIO"#]
    #[doc = r#"Description: "#]
    pub crrersfmr: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crrersfq: Option<f32>,

    #[doc = r#"Title: RE LOAN 1-4 FAM FIRST LIEN-RECOV QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crrersfqr: Option<f32>,

    #[doc = r#"Title: RE LOAN RECOVERIES DOMESTIC OFFICES"#]
    #[doc = r#"Description: "#]
    pub crreoffdom: Option<f32>,

    #[doc = r#"Title: RE LOAN RECOVERIES DOMESTIC OFFICES RATIO"#]
    #[doc = r#"Description: "#]
    pub crreoffdomr: Option<f32>,

    #[doc = r#"Title: RE LOAN RECOVERIES DOMESTIC OFFICES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub crreoffdomq: Option<f32>,

    #[doc = r#"Title: RE LOAN RECOVERIES DOMESTIC OFFICES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub crreoffdomqr: Option<f32>,

    #[doc = r#"Title: CR DER (NET)-PURCHASE PROTECT"#]
    #[doc = r#"Description: "#]
    pub ctderben: Option<f32>,

    #[doc = r#"Title: CR DER(NET) - SOLD PROTECTION"#]
    #[doc = r#"Description: "#]
    pub ctdergty: Option<f32>,

    #[doc = r#"Title: TOTAL DEPOSIT LIAB BEF EXCLUSION"#]
    #[doc = r#"Description: "#]
    pub depbefex: Option<f32>,

    #[doc = r#"Title: ESTIMATED ASSESSABLE DEPOSITS"#]
    #[doc = r#"Description: "#]
    pub depcsbq: Option<f32>,

    #[doc = r#"Title: ESTIMATED ASSESSABLE DEPOSITS RATIO"#]
    #[doc = r#"Description: "#]
    pub depcsbqr: Option<f32>,

    #[doc = r#"Title: TOT DOMESTIC DEPOSIT / ASSET"#]
    #[doc = r#"Description: "#]
    pub depdastr: Option<f32>,

    #[doc = r#"Title: FOREIGN BANKS-FOR"#]
    #[doc = r#"Description: "#]
    pub depfbkf: Option<f32>,

    #[doc = r#"Title: FOREIGN BANKS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depfbkfr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENTS-FOR"#]
    #[doc = r#"Description: "#]
    pub depfgovf: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENTS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depfgovfr: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING DEP-DOM"#]
    #[doc = r#"Description: "#]
    pub depidom: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING DEP-DOM RATIO"#]
    #[doc = r#"Description: "#]
    pub depidomr: Option<f32>,

    #[doc = r#"Title: ESTIMATED INSURED DEPOSITS"#]
    #[doc = r#"Description: "#]
    pub depins: Option<f32>,

    #[doc = r#"Title: ESTIMATED INSURED DEPOSITS RATIO"#]
    #[doc = r#"Description: "#]
    pub depinsr: Option<f32>,

    #[doc = r#"Title: AMT DEP ACC GREATER THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub deplgamt: Option<f32>,

    #[doc = r#"Title: AMT DEP ACC GREATER THAN $250,000 RATIO"#]
    #[doc = r#"Description: "#]
    pub deplgamtr: Option<f32>,

    #[doc = r#"Title: NUM DEP ACC GREATER THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub deplgb: Option<f32>,

    #[doc = r#"Title: AMT OF RETIREMENT DEP ACC OF MORE THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub deplgra: Option<f32>,

    #[doc = r#"Title: AMT OF RETIREMENT DEP ACC OF MORE THAN $250,000 RATIO"#]
    #[doc = r#"Description: "#]
    pub deplgrar: Option<f32>,

    #[doc = r#"Title: NUM OF RETIREMENT DEP ACC MORE THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub deplgrn: Option<f32>,

    #[doc = r#"Title: DEP THRU LIST SVC NOT BROKERED"#]
    #[doc = r#"Description: "#]
    pub deplsnb: Option<f32>,

    #[doc = r#"Title: DEP THRU LIST SVC NOT BROKERED RATIO"#]
    #[doc = r#"Description: "#]
    pub deplsnbr: Option<f32>,

    #[doc = r#"Title: NONINTEREST-BEARING DEP-DOM"#]
    #[doc = r#"Description: "#]
    pub depnidom: Option<f32>,

    #[doc = r#"Title: NONINTEREST-BEARING DEP-DOM RATIO"#]
    #[doc = r#"Description: "#]
    pub depnidomr: Option<f32>,

    #[doc = r#"Title: AMT DEP ACC AT $250,000 OR LESS"#]
    #[doc = r#"Description: "#]
    pub depsmamt: Option<f32>,

    #[doc = r#"Title: AMT DEP ACC AT $250,000 OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub depsmamtr: Option<f32>,

    #[doc = r#"Title: NUM DEP ACC EQUAL OR LESS THAN EQUAL TO $250,000"#]
    #[doc = r#"Description: "#]
    pub depsmb: Option<f32>,

    #[doc = r#"Title: AMT RETIREMENT DEP ACC OF $250,000 OR LESS"#]
    #[doc = r#"Description: "#]
    pub depsmra: Option<f32>,

    #[doc = r#"Title: AMT RETIREMENT DEP ACC OF $250,000 OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub depsmrar: Option<f32>,

    #[doc = r#"Title: NUM RETIREMENT DEP ACC OF $250,000"#]
    #[doc = r#"Description: "#]
    pub depsmrn: Option<f32>,

    #[doc = r#"Title: TOTAL ALLOWABLE EXCLUSIONS (INCLUDING FOREIGN DEPOSITS)"#]
    #[doc = r#"Description: "#]
    pub depallex: Option<f32>,

    #[doc = r#"Title: EST UNINSURED DEP IN DOM-OFF IN INSURED BRANCHES IN US TERR AND POSSESSIONS"#]
    #[doc = r#"Description: "#]
    pub depuna: Option<f32>,

    #[doc = r#"Title: EST UNINSURED DEP IN DOM-OFF IN INSURED BRANCHES IN US TERR AND POSSESSIONS"#]
    #[doc = r#"Description: "#]
    pub depunar: Option<f32>,

    #[doc = r#"Title: ESTIMATED UNINSURED DEPOSITS IN DOMESTIC OFFICES AND IN INSURED BRANCHES IN US TERRITORIES AND POSSESSIONS"#]
    #[doc = r#"Description: "#]
    pub depunins: Option<f32>,

    #[doc = r#"Title: U.S. BANKS&OTH.US INST-FOR"#]
    #[doc = r#"Description: "#]
    pub depusbkf: Option<f32>,

    #[doc = r#"Title: U.S. BANKS&OTH.US INST-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depusbkfr: Option<f32>,

    #[doc = r#"Title: U.S.GOVT & ST & POL SUBS-FOR"#]
    #[doc = r#"Description: "#]
    pub depusmf: Option<f32>,

    #[doc = r#"Title: U.S.GOVT & ST & POL SUBS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub depusmfr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drag: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub dragr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub dragq: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub dragqr: Option<f32>,

    #[doc = r#"Title: AG LOAN CHARGE-OFFS*SMALL BKS"#]
    #[doc = r#"Description: "#]
    pub dragsm: Option<f32>,

    #[doc = r#"Title: AG LOAN CHARGE-OFFS*SMALL BKS RATIO"#]
    #[doc = r#"Description: "#]
    pub dragsmr: Option<f32>,

    #[doc = r#"Title: AG LOAN CHARGE-OFFS*SMALL BKS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub dragsmq: Option<f32>,

    #[doc = r#"Title: AG LOAN CHARGE-OFFS*SMALL BKS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub dragsmqr: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drauto: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drautor: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drautoq: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drautoqr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drci: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drcir: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drciq: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drciqr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S."#]
    #[doc = r#"Description: "#]
    pub drcinus: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub drcinusr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S. QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drcinusq: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN CHARGE-OFFS NON-U.S. QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drcinusqr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drcon: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drconr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drconq: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drconqr: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drconoth: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drconothr: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drconotq: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drconotqr: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drcrcd: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drcrcdr: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drcrcdq: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drcrcdqr: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drdep: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drdepr: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drdepq: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drdepqr: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub drdepnus: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN CHG-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drdepnusr: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN CHG-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drdepnuq: Option<f32>,

    #[doc = r#"Title: FOREIGN DEPS INST LN CHG-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drdepnuqr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub drforgv: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN CHG-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drforgvr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN CHG-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drforgvq: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVERNMENT LN CHG-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drforgvqr: Option<f32>,

    #[doc = r#"Title: LEASE CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drls: Option<f32>,

    #[doc = r#"Title: LEASE CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drlsr: Option<f32>,

    #[doc = r#"Title: LEASE CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drlsq: Option<f32>,

    #[doc = r#"Title: LEASE CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drlsqr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drother: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drotherr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drothq: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drothqr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drre: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drrer: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drreq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drreqr: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drreag: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drreagr: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN CHG-OFFS-QTR"#]
    #[doc = r#"Description: "#]
    pub drreagq: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drreagqr: Option<f32>,

    #[doc = r#"Title: 1-4 FAM CONSTRUCT LN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drrecnfm: Option<f32>,

    #[doc = r#"Title: OTHER CONSTRUCT LN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drrecnot: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN CHG-OFFS-QTR"#]
    #[doc = r#"Description: "#]
    pub drreconq: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drreconqr: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drrecons: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drreconsr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHRG-OFFS-FOR"#]
    #[doc = r#"Description: "#]
    pub drrefor: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHRG-OFFS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub drreforr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHRG-OFFS-FOR QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drreforq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHRG-OFFS-FOR QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drreforqr: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drreloc: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drrelocr: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drrelocq: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drrelocqr: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RE LN CHG-OFFS-QTR"#]
    #[doc = r#"Description: "#]
    pub drremulq: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RES RE LN CHARGE-OFF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drremulqr: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RES RE LN CHARGE-OFF"#]
    #[doc = r#"Description: "#]
    pub drremult: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RES RE LN CHARGE-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub drremultr: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drrenres: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drrenresr: Option<f32>,

    #[doc = r#"Title: OTHER NONFARM NONRES RE CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub drrenrot: Option<f32>,

    #[doc = r#"Title: OWN-OCCUP NONFARM NONRES CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub drrenrow: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN CHG-OFF-QTR"#]
    #[doc = r#"Description: "#]
    pub drrenrsq: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drrenrsqr: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drrenus: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drrenusr: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drrenusq: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drrenusqr: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub drreres: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub drreresr: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY CHG-OFFS-QTR"#]
    #[doc = r#"Description: "#]
    pub drreresq: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drreresqr: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub drrersf2: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-CHG-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub drrersf2r: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-CHG-OFF QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drrers2q: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-CHG-OFF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drrers2qr: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub drrersfm: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub drrersfmr: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drrersfq: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM FIRST LIEN-CHG-OFF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drrersfqr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES"#]
    #[doc = r#"Description: "#]
    pub drreoffdom: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES RATIO"#]
    #[doc = r#"Description: "#]
    pub drreoffdomr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub drreoffdomq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN CHARGE-OFFS DOMESTIC OFFICES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub drreoffdomqr: Option<f32>,

    #[doc = r#"Title: EQUITY"#]
    #[doc = r#"Description: "#]
    pub edcm: Option<f32>,

    #[doc = r#"Title: EFFICIENCY RATIO EXPENSE"#]
    #[doc = r#"Description: "#]
    pub eeff: Option<f32>,

    #[doc = r#"Title: EFFICIENCY RATIO EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eeffq: Option<f32>,

    #[doc = r#"Title: EFFICIENCY RATIO"#]
    #[doc = r#"Description: "#]
    pub eeffr: Option<f32>,

    #[doc = r#"Title: EFFICIENCY QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eeffqr: Option<f32>,

    #[doc = r#"Title: EFFECTIVE DATE"#]
    #[doc = r#"Description: "#]
    pub effdate: Option<f32>,

    #[doc = r#"Title: GOODWILL IMPAIRMENT LOSSES"#]
    #[doc = r#"Description: "#]
    pub eintgw: Option<f32>,

    #[doc = r#"Title: GOODWILL IMPAIRMENT LOSSES RATIO"#]
    #[doc = r#"Description: "#]
    pub eintgwr: Option<f32>,

    #[doc = r#"Title: GOODWILL IMPAIRMENT LOSSES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eintgwq: Option<f32>,

    #[doc = r#"Title: GOODWILL IMPAIRMENT LOSSES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eintgwqr: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSSES OTH INTAN"#]
    #[doc = r#"Description: "#]
    pub eintoth: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSSES OTH INTAN RATIO"#]
    #[doc = r#"Description: "#]
    pub eintothr: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSSES OTH INTAN QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eintothq: Option<f32>,

    #[doc = r#"Title: AMORT & IMPAIR LOSSES OTH INTAN QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eintothqr: Option<f32>,

    #[doc = r#"Title: LOAN LOSS PROV/NT CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub elnantr: Option<f32>,

    #[doc = r#"Title: ELNATRA"#]
    #[doc = r#"Description: "#]
    pub elnatra: Option<f32>,

    #[doc = r#"Title: CREDIT LOSS PROV/AVE ASSETS"#]
    #[doc = r#"Description: "#]
    pub elnatry: Option<f32>,

    #[doc = r#"Title: CREDIT LOSS PROV/AVE ASSETS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub elnatryq: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE-ENHANCEMENTS - AUTO"#]
    #[doc = r#"Description: "#]
    pub enceauto: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE-ENHANCEMENTS - AUTO RATIO"#]
    #[doc = r#"Description: "#]
    pub enceautor: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - CI"#]
    #[doc = r#"Description: "#]
    pub enceci: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - CI RATIO"#]
    #[doc = r#"Description: "#]
    pub encecir: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - CON"#]
    #[doc = r#"Description: "#]
    pub encecon: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - CON RATIO"#]
    #[doc = r#"Description: "#]
    pub enceconr: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - OTH"#]
    #[doc = r#"Description: "#]
    pub enceoth: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub enceothr: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - RES"#]
    #[doc = r#"Description: "#]
    pub enceres: Option<f32>,

    #[doc = r#"Title: CR EXPOSURE - ENHANCEMENTS - RES RATIO"#]
    #[doc = r#"Description: "#]
    pub enceresr: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST EXPENSE"#]
    #[doc = r#"Description: "#]
    pub eothint: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST EXPENSE RATIO"#]
    #[doc = r#"Description: "#]
    pub eothintr: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST EXPENSE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eothintq: Option<f32>,

    #[doc = r#"Title: OTHER INTEREST EXPENSE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eothintqr: Option<f32>,

    #[doc = r#"Title: TOTAL BANK EQUITY CAPITAL-CAVG5"#]
    #[doc = r#"Description: "#]
    pub eq5: Option<f32>,

    #[doc = r#"Title: TRANSACTIONS WITH BHC"#]
    #[doc = r#"Description: "#]
    pub eqcbhctr: Option<f32>,

    #[doc = r#"Title: TRANSACTIONS WITH BHC RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcbhctrr: Option<f32>,

    #[doc = r#"Title: OTHER COMPREHENSIVE INCOME"#]
    #[doc = r#"Description: "#]
    pub eqccompi: Option<f32>,

    #[doc = r#"Title: OTHER COMPREHENSIVE INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub eqccompir: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS ON COMM & PFD-ANN"#]
    #[doc = r#"Description: "#]
    pub eqcdiva: Option<f32>,

    #[doc = r#"Title: CHANGES DUE TO MERGERS"#]
    #[doc = r#"Description: "#]
    pub eqcmrg: Option<f32>,

    #[doc = r#"Title: CHANGES DUE TO MERGERS RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcmrgr: Option<f32>,

    #[doc = r#"Title: BK EQ CAP MOST RECENTLY REPORTED"#]
    #[doc = r#"Description: "#]
    pub eqcprev: Option<f32>,

    #[doc = r#"Title: BK EQ CAP MOST RECENTLY REPORTED RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcprevr: Option<f32>,

    #[doc = r#"Title: ACCOUNTING CHANGES & CORRECTIONS"#]
    #[doc = r#"Description: "#]
    pub eqcrest: Option<f32>,

    #[doc = r#"Title: ACCOUNTING CHANGES & CORRECTIONS RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcrestr: Option<f32>,

    #[doc = r#"Title: SALE OF CAPITAL STOCK"#]
    #[doc = r#"Description: "#]
    pub eqcstkrx: Option<f32>,

    #[doc = r#"Title: SALE OF CAPITAL STOCK RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcstkrxr: Option<f32>,

    #[doc = r#"Title: SALE OF CAPITAL STOCK QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub eqcsxq: Option<f32>,

    #[doc = r#"Title: SALE OF CAPITAL STOCK QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub eqcsxqr: Option<f32>,

    #[doc = r#"Title: TREASURY STOCK TRANSACTIONS"#]
    #[doc = r#"Description: "#]
    pub eqctrstx: Option<f32>,

    #[doc = r#"Title: TREASURY STOCK TRANSACTIONS RATIO"#]
    #[doc = r#"Description: "#]
    pub eqctrstxr: Option<f32>,

    #[doc = r#"Title: TOTAL EQUITY CAPITAL"#]
    #[doc = r#"Description: "#]
    pub eqtot: Option<f32>,

    #[doc = r#"Title: TOTAL EQUITY CAPITAL RATIO"#]
    #[doc = r#"Description: "#]
    pub eqtotr: Option<f32>,

    #[doc = r#"Title: BANK EQUITY CAPITAL/ASSETS"#]
    #[doc = r#"Description: "#]
    pub eqv: Option<f32>,

    #[doc = r#"Title: TOTAL EARNING ASSETS"#]
    #[doc = r#"Description: "#]
    pub ernast: Option<f32>,

    #[doc = r#"Title: TOTAL EARNING ASSETS"#]
    #[doc = r#"Description: "#]
    pub ernast2: Option<f32>,

    #[doc = r#"Title: TOTAL EARNING ASSETS-CAVG5I"#]
    #[doc = r#"Description: "#]
    pub ernast5: Option<f32>,

    #[doc = r#"Title: EARNING ASSETS / TOTAL ASSETS"#]
    #[doc = r#"Description: "#]
    pub ernastr: Option<f32>,

    #[doc = r#"Title: ESTABLISHED DATE"#]
    #[doc = r#"Description: "#]
    pub estymd: Option<f32>,

    #[doc = r#"Title: INACTIVE DATE"#]
    #[doc = r#"Description: "#]
    pub endefymd: Option<f32>,

    #[doc = r#"Title: INACTIVE DATE"#]
    #[doc = r#"Description: "#]
    pub org_end_num_dte: Option<f32>,

    #[doc = r#"Title: TT&L"#]
    #[doc = r#"Description: "#]
    pub ettlotmg: Option<f32>,

    #[doc = r#"Title: THRIFT FINANCIAL REPORT FLAG"#]
    #[doc = r#"Description: "#]
    pub formtfr: Option<f32>,

    #[doc = r#"Title: FOREIGN EXCHANGE-TOTAL CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub fx: Option<f32>,

    #[doc = r#"Title: FOR EXCH-FUTURES & FORWARD CONTR"#]
    #[doc = r#"Description: "#]
    pub fxffc: Option<f32>,

    #[doc = r#"Title: FOR EXCHANGE-SWAPS"#]
    #[doc = r#"Description: "#]
    pub fxnvs: Option<f32>,

    #[doc = r#"Title: FOR EXCH-PUR OPTION CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub fxpoc: Option<f32>,

    #[doc = r#"Title: SPOT FOREIGN EXCHANGE CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub fxspot: Option<f32>,

    #[doc = r#"Title: FOR EXCH-WRITTEN OPTION CONTRACT"#]
    #[doc = r#"Description: "#]
    pub fxwoc: Option<f32>,

    #[doc = r#"Title: INC BEFORE INC TAXS & DISC-QTR"#]
    #[doc = r#"Description: "#]
    pub ibeftxq: Option<f32>,

    #[doc = r#"Title: INCOME BEFORE DISC OPR"#]
    #[doc = r#"Description: "#]
    pub ibefxtr: Option<f32>,

    #[doc = r#"Title: INCOME BEFORE DISC OPR RATIO"#]
    #[doc = r#"Description: "#]
    pub ibefxtrr: Option<f32>,

    #[doc = r#"Title: INCOME BEFORE DISC OPR QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ibefxtrq: Option<f32>,

    #[doc = r#"Title: EFFICIENCY RATIO INCOME"#]
    #[doc = r#"Description: "#]
    pub ieff: Option<f32>,

    #[doc = r#"Title: EFFICIENCY RATIO INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ieffq: Option<f32>,

    #[doc = r#"Title: INCOME BEFORE DISC OPR QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ibefxtrqr: Option<f32>,

    #[doc = r#"Title: FIDUCIARY ACTIVITIES INCOME"#]
    #[doc = r#"Description: "#]
    pub ifiduc: Option<f32>,

    #[doc = r#"Title: FIDUCIARY ACTIVITIES INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub ifiducr: Option<f32>,

    #[doc = r#"Title: FIDUCIARY ACTIVITIES INCOME-QTR"#]
    #[doc = r#"Description: "#]
    pub ifiducq: Option<f32>,

    #[doc = r#"Title: FIDUCIARY ACTIVITIES INCOME-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ifiducqr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-COMMODITY"#]
    #[doc = r#"Description: "#]
    pub iglcmex: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-COMMODITY RATIO"#]
    #[doc = r#"Description: "#]
    pub iglcmexr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-COMMODITY QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iglcmexq: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-COMMODITY RATIO QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iglcmexqr: Option<f32>,

    #[doc = r#"Title: TRADING REVENUE- CREDIT EXPOSURE"#]
    #[doc = r#"Description: "#]
    pub iglcrex: Option<f32>,

    #[doc = r#"Title: TRADING REVENUE- CREDIT EXPOSURE RATIO"#]
    #[doc = r#"Description: "#]
    pub iglcrexr: Option<f32>,

    #[doc = r#"Title: TRADING REVENUE- CREDIT EXPOSURE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iglcrexq: Option<f32>,

    #[doc = r#"Title: TRADING REVENUE- CREDIT EXPOSURE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iglcrexqr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-EQ DERIVATIVE"#]
    #[doc = r#"Description: "#]
    pub igledex: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-EQ DERIVATIVE RATIO"#]
    #[doc = r#"Description: "#]
    pub igledexr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-EQ DERIVATIVE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub igledexq: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-EQ DERIVATIVE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub igledexqr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-FOREIGN EXCHANGE"#]
    #[doc = r#"Description: "#]
    pub iglfxex: Option<f32>,

    #[doc = r#"Title: RADING ACCOUNT-FOREIGN EXCHANGE RATIO"#]
    #[doc = r#"Description: "#]
    pub iglfxexr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-FOREIGN EXCHANGE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iglfxexq: Option<f32>,

    #[doc = r#"Title: RADING ACCOUNT-FOREIGN EXCHANGE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iglfxexqr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-INTEREST RATE"#]
    #[doc = r#"Description: "#]
    pub iglrtex: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-INTEREST RATE RATIO"#]
    #[doc = r#"Description: "#]
    pub iglrtexr: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-INTEREST RATE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iglrtexq: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNT-INTEREST RATE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iglrtexqr: Option<f32>,

    #[doc = r#"Title: SECURITIES GAINS AND LOSSES-QTR"#]
    #[doc = r#"Description: "#]
    pub iglsecq: Option<f32>,

    #[doc = r#"Title: TRADING REVENUES-TOTAL"#]
    #[doc = r#"Description: "#]
    pub igltrad: Option<f32>,

    #[doc = r#"Title: TRADING REVENUES-TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub igltradr: Option<f32>,

    #[doc = r#"Title: TRADING REVENUE-QTR"#]
    #[doc = r#"Description: "#]
    pub igltrdq: Option<f32>,

    #[doc = r#"Title: TRADING REVENUE-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub igltrdqr: Option<f32>,

    #[doc = r#"Title: INSURANCE COMMISSIONS & FEES"#]
    #[doc = r#"Description: "#]
    pub iinscom: Option<f32>,

    #[doc = r#"Title: INSURANCE COMMISSIONS & FEES RATIO"#]
    #[doc = r#"Description: "#]
    pub iinscomr: Option<f32>,

    #[doc = r#"Title: INSURANCE COMMISSIONS & FEES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iinscomq: Option<f32>,

    #[doc = r#"Title: INSURANCE COMMISSIONS & FEES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iinscomqr: Option<f32>,

    #[doc = r#"Title: INSURANCE COM+FEES-OTHER"#]
    #[doc = r#"Description: "#]
    pub iinsoth: Option<f32>,

    #[doc = r#"Title: INSURANCE COM+FEES-OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub iinsothr: Option<f32>,

    #[doc = r#"Title: INSURANCE COM+FEES-OTHER QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iinsothq: Option<f32>,

    #[doc = r#"Title: INSURANCE COM+FEES-OTHER QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iinsothqr: Option<f32>,

    #[doc = r#"Title: INSURANCE UNDERWRITNG INCOME"#]
    #[doc = r#"Description: "#]
    pub iinsund: Option<f32>,

    #[doc = r#"Title: INSURANCE UNDERWRITNG INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub iinsundr: Option<f32>,

    #[doc = r#"Title: INSURANCE UNDERWRITNG INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iinsundq: Option<f32>,

    #[doc = r#"Title: INSURANCE UNDERWRITNG INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iinsundqr: Option<f32>,

    #[doc = r#"Title: INVEST BANK"#]
    #[doc = r#"Description: "#]
    pub iinvfee: Option<f32>,

    #[doc = r#"Title: INVEST BANK RATIO"#]
    #[doc = r#"Description: "#]
    pub iinvfeer: Option<f32>,

    #[doc = r#"Title: INVEST BANK QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iinvfeeq: Option<f32>,

    #[doc = r#"Title: INVEST BANK QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iinvfeeqr: Option<f32>,

    #[doc = r#"Title: PRIMARY INSURER (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub insagnt1: Option<String>,

    #[doc = r#"Title: PURCH CC REL & NONMTG SER ASTS"#]
    #[doc = r#"Description: "#]
    pub intangcc: Option<f32>,

    #[doc = r#"Title: GOODWILL"#]
    #[doc = r#"Description: "#]
    pub intangw: Option<f32>,

    #[doc = r#"Title: GOODWILL RATIO"#]
    #[doc = r#"Description: "#]
    pub intangwr: Option<f32>,

    #[doc = r#"Title: MORTGAGE SERVICING ASSETS"#]
    #[doc = r#"Description: "#]
    pub intanmsr: Option<f32>,

    #[doc = r#"Title: MORTGAGE SERVICING ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub intanmsrr: Option<f32>,

    #[doc = r#"Title: OTHER IDENTIFIABLE INTANG ASSETS"#]
    #[doc = r#"Description: "#]
    pub intanoth: Option<f32>,

    #[doc = r#"Title: OTHER IDENTIFIABLE INTANG ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub intanothr: Option<f32>,

    #[doc = r#"Title: INTEREST INCOME/EARNING ASSETS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub intincyq: Option<f32>,

    #[doc = r#"Title: TOTAL INTEREST INCOME ANNUAL"#]
    #[doc = r#"Description: "#]
    pub intinca: Option<f32>,

    #[doc = r#"Title: OTHER NONINTEREST INCOME"#]
    #[doc = r#"Description: "#]
    pub iotnii: Option<f32>,

    #[doc = r#"Title: OTHER NONINTEREST INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub iotniir: Option<f32>,

    #[doc = r#"Title: OTHER NONINTEREST INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iotniiq: Option<f32>,

    #[doc = r#"Title: OTHER NONINTEREST INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iotniiqr: Option<f32>,

    #[doc = r#"Title: SECURITIZATION INCOME"#]
    #[doc = r#"Description: "#]
    pub isecz: Option<f32>,

    #[doc = r#"Title: SECURITIZATION INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub iseczr: Option<f32>,

    #[doc = r#"Title: SECURITIZATION INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iseczq: Option<f32>,

    #[doc = r#"Title: SECURITIZATION INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iseczqr: Option<f32>,

    #[doc = r#"Title: SERVICE CHARGE ON DEP ACCTS-QTR"#]
    #[doc = r#"Description: "#]
    pub iserchgq: Option<f32>,

    #[doc = r#"Title: SERVICE CHARGE ON DEPOSIT ACCTS-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub iserchgqr: Option<f32>,

    #[doc = r#"Title: SERVICING FEES"#]
    #[doc = r#"Description: "#]
    pub iserfee: Option<f32>,

    #[doc = r#"Title: SERVICING FEES RATIO"#]
    #[doc = r#"Description: "#]
    pub iserfeer: Option<f32>,

    #[doc = r#"Title: SERVICING FEES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub iserfeeq: Option<f32>,

    #[doc = r#"Title: SERVICING FEES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub iserfeeqr: Option<f32>,

    #[doc = r#"Title: VENTURE CAPITAL REVENUE"#]
    #[doc = r#"Description: "#]
    pub ivencap: Option<f32>,

    #[doc = r#"Title: VENTURE CAPITAL REVENUE RATIO"#]
    #[doc = r#"Description: "#]
    pub ivencapr: Option<f32>,

    #[doc = r#"Title: VENTURE CAPITAL REVENUE QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ivencapq: Option<f32>,

    #[doc = r#"Title: VENTURE CAPITAL REVENUE QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ivencapqr: Option<f32>,

    #[doc = r#"Title: AG LOANS - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub lag: Option<f32>,

    #[doc = r#"Title: AG LOANS - LOSS SHARE RATIO"#]
    #[doc = r#"Description: "#]
    pub lagr: Option<f32>,

    #[doc = r#"Title: C&I LOANS - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub lci: Option<f32>,

    #[doc = r#"Title: C&I LOANS - LOSS SHARE RATIO"#]
    #[doc = r#"Description: "#]
    pub lcir: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub lcon: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS - LOSS SHARE RATIO"#]
    #[doc = r#"Description: "#]
    pub lconr: Option<f32>,

    #[doc = r#"Title: TOTAL LIABILITIES-FOR"#]
    #[doc = r#"Description: "#]
    pub liabfor: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 100-$"#]
    #[doc = r#"Description: "#]
    pub lnag1: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 100-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag1r: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-100-250-$"#]
    #[doc = r#"Description: "#]
    pub lnag2: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-100-250-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag2r: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-250-500-$"#]
    #[doc = r#"Description: "#]
    pub lnag3: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-250-500-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag3r: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 500-$"#]
    #[doc = r#"Description: "#]
    pub lnag4: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 500-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag4r: Option<f32>,

    #[doc = r#"Title: AG LOANS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnag5: Option<f32>,

    #[doc = r#"Title: AG LOANS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub lnag22: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 100-NUM"#]
    #[doc = r#"Description: "#]
    pub lnag1n: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 100-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag1nr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-100-250-NUM"#]
    #[doc = r#"Description: "#]
    pub lnag2n: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-100-250-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag2nr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-250-500-NUM"#]
    #[doc = r#"Description: "#]
    pub lnag3n: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-250-500-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag3nr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 500-NUM"#]
    #[doc = r#"Description: "#]
    pub lnag4n: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-UNDER 500-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnag4nr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-FOR"#]
    #[doc = r#"Description: "#]
    pub lnagfor: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LOANS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnagforr: Option<f32>,

    #[doc = r#"Title: LOAN LOSS RESERVE/GROSS LN&LS"#]
    #[doc = r#"Description: "#]
    pub lnatresr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS - AUTO - CAVG2"#]
    #[doc = r#"Description: "#]
    pub lnauto2: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS - AUTO - CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnauto5: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-100-$"#]
    #[doc = r#"Description: "#]
    pub lnci1: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-100-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci1r: Option<f32>,

    #[doc = r#"Title: C&I LOANS-100-250-$"#]
    #[doc = r#"Description: "#]
    pub lnci2: Option<f32>,

    #[doc = r#"Title: C&I LOANS-100-250-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci2r: Option<f32>,

    #[doc = r#"Title: C&I LOANS-250-1M-$"#]
    #[doc = r#"Description: "#]
    pub lnci3: Option<f32>,

    #[doc = r#"Title: C&I LOANS-250-1M-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci3r: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-1M-$"#]
    #[doc = r#"Description: "#]
    pub lnci4: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-1M-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci4r: Option<f32>,

    #[doc = r#"Title: C&I LOANS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnci5: Option<f32>,

    #[doc = r#"Title: C&I LOANS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub lnci22: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-100-NUM"#]
    #[doc = r#"Description: "#]
    pub lnci1n: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-100-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci1nr: Option<f32>,

    #[doc = r#"Title: C&I LOANS-100-250-NUM"#]
    #[doc = r#"Description: "#]
    pub lnci2n: Option<f32>,

    #[doc = r#"Title: C&I LOANS-250-1M-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci2nr: Option<f32>,

    #[doc = r#"Title: C&I LOANS-250-1M-NUM"#]
    #[doc = r#"Description: "#]
    pub lnci3n: Option<f32>,

    #[doc = r#"Title: C&I LOANS-250-1M-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci3nr: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-1M-NUM"#]
    #[doc = r#"Description: "#]
    pub lnci4n: Option<f32>,

    #[doc = r#"Title: C&I LOANS-UNDER-1M-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnci4nr: Option<f32>,

    #[doc = r#"Title: C&I LOANS-FOR"#]
    #[doc = r#"Description: "#]
    pub lncifor: Option<f32>,

    #[doc = r#"Title: C&I LOANS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnciforr: Option<f32>,

    #[doc = r#"Title: C&I LOANS-NON-U.S. DOMICILE"#]
    #[doc = r#"Description: "#]
    pub lncinus: Option<f32>,

    #[doc = r#"Title: C&I LOANS-NON-U.S. DOMICILE-FOR"#]
    #[doc = r#"Description: "#]
    pub lncinusf: Option<f32>,

    #[doc = r#"Title: C&I LOANS-NON-U.S. DOMICILE-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lncinusfr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE LOANS"#]
    #[doc = r#"Description: "#]
    pub lncomre: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lncomrer: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE LOANS2"#]
    #[doc = r#"Description: "#]
    pub lncomre2: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE LOANS CAVG5"#]
    #[doc = r#"Description: "#]
    pub lncomre5: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub lncon2: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lncon5: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-FOR"#]
    #[doc = r#"Description: "#]
    pub lnconfor: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnconforr: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER & RELATED PLANS"#]
    #[doc = r#"Description: "#]
    pub lnconorp: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOANS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub lnconot2: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOANS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnconot5: Option<f32>,

    #[doc = r#"Title: CONSUMER LNS-RELATED PLANS"#]
    #[doc = r#"Description: "#]
    pub lnconrp: Option<f32>,

    #[doc = r#"Title: CONSUMER LNS-RELATED PLANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnconrpr: Option<f32>,

    #[doc = r#"Title: OTHER CONTRA ACCOUNTS"#]
    #[doc = r#"Description: "#]
    pub lncontra: Option<f32>,

    #[doc = r#"Title: OTHER CONTRA ACCOUNTS RATIO"#]
    #[doc = r#"Description: "#]
    pub lncontrar: Option<f32>,

    #[doc = r#"Title: CREDIT CARD PLANS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub lncrcd2: Option<f32>,

    #[doc = r#"Title: CREDIT CARD PLANS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lncrcd5: Option<f32>,

    #[doc = r#"Title: TOTAL DEP INST LNS & ACCEPT"#]
    #[doc = r#"Description: "#]
    pub lndepac: Option<f32>,

    #[doc = r#"Title: TOTAL DEP INST LNS & ACCEPT-DOM"#]
    #[doc = r#"Description: "#]
    pub lndepacd: Option<f32>,

    #[doc = r#"Title: LOANS TO DEPOSITORY INSTITUTIONS AND ACCEPTANCE OF OTHER BANKS"#]
    #[doc = r#"Description: "#]
    pub lndepaobk: Option<f32>,

    #[doc = r#"Title: LOANS TO DEPOSITORY INSTITUTIONS AND ACCEPTANCE OF OTHER BANKS RATIO"#]
    #[doc = r#"Description: "#]
    pub lndepaobkr: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-COMMERCIAL BANKS"#]
    #[doc = r#"Description: "#]
    pub lndepcb: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-COMMERCIAL BK-FOR"#]
    #[doc = r#"Description: "#]
    pub lndepcbf: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-COMMERCIAL BK-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lndepcbfr: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-FOR COUNTRY"#]
    #[doc = r#"Description: "#]
    pub lndepfc: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-FOR COUNTRY-FOR"#]
    #[doc = r#"Description: "#]
    pub lndepfcf: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-FOR COUNTRY-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lndepfcfr: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-FOR COUNTRY-U.S. BR"#]
    #[doc = r#"Description: "#]
    pub lndepfus: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-OTH U.S. INST"#]
    #[doc = r#"Description: "#]
    pub lndepus: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-COM BKS-U.S.BRANCH"#]
    #[doc = r#"Description: "#]
    pub lndepusb: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-OTH U.S. INST-FOR"#]
    #[doc = r#"Description: "#]
    pub lndepusf: Option<f32>,

    #[doc = r#"Title: DEP INST LNS-OTH U.S. INST-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lndepusfr: Option<f32>,

    #[doc = r#"Title: EXECUTIVE OFFICER LOANS-AMOUNT"#]
    #[doc = r#"Description: "#]
    pub lnexamt: Option<f32>,

    #[doc = r#"Title: EXECUTIVE OFFICER LOANS-AMOUNT RATIO"#]
    #[doc = r#"Description: "#]
    pub lnexamtr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVT LOANS-FOR"#]
    #[doc = r#"Description: "#]
    pub lnfgfor: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVT LOANS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnfgforr: Option<f32>,

    #[doc = r#"Title: NET LOANS & LEASES/DEPOSITS"#]
    #[doc = r#"Description: "#]
    pub lnlsdepr: Option<f32>,

    #[doc = r#"Title: LN&LS + UNEARNED INC-FOR"#]
    #[doc = r#"Description: "#]
    pub lnlsfor: Option<f32>,

    #[doc = r#"Title: LN&LS + UNEARNED INC-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnlsforr: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-TOTAL-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnlsgr5: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-TOTAL-FOR"#]
    #[doc = r#"Description: "#]
    pub lnlsgrf: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES-TOTAL-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnlsgrfr: Option<f32>,

    #[doc = r#"Title: NET LOANS & LEASES/ASSETS"#]
    #[doc = r#"Description: "#]
    pub lnlsntv: Option<f32>,

    #[doc = r#"Title: NET LOANS & LEASES/ASSETS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub lnlsnqr: Option<f32>,

    #[doc = r#"Title: LOANS & LEASES HELD FOR RESALE"#]
    #[doc = r#"Description: "#]
    pub lnlssale: Option<f32>,

    #[doc = r#"Title: LOANS & LEASES HELD FOR RESALE RATIO"#]
    #[doc = r#"Description: "#]
    pub lnlssaler: Option<f32>,

    #[doc = r#"Title: PLEDGED LOANS AND LEASES"#]
    #[doc = r#"Description: "#]
    pub lnpledge: Option<f32>,

    #[doc = r#"Title: PLEDGED LOANS AND LEASES RATIO"#]
    #[doc = r#"Description: "#]
    pub lnpledger: Option<f32>,

    #[doc = r#"Title: MUNI LOANS-FOR"#]
    #[doc = r#"Description: "#]
    pub lnmunif: Option<f32>,

    #[doc = r#"Title: MUNI LOANS-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lnmunifr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 1-3 YEARS"#]
    #[doc = r#"Description: "#]
    pub lnot1t3: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 1-3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnot1t3r: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS*3 MO OR LESS"#]
    #[doc = r#"Description: "#]
    pub lnot3les: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS*3 MO OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnot3lesr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 3-5 YEARS"#]
    #[doc = r#"Description: "#]
    pub lnot3t5: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 3-5 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnot3t5r: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 3-12 MONS"#]
    #[doc = r#"Description: "#]
    pub lnot3t12: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 3-12 MONS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnot3t12r: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 5-15 YEARS"#]
    #[doc = r#"Description: "#]
    pub lnot5t15: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * 5-15 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnot5t15r: Option<f32>,

    #[doc = r#"Title: OTHER LOANS & LEASES-QBP-CAVG2"#]
    #[doc = r#"Description: "#]
    pub lnotci2: Option<f32>,

    #[doc = r#"Title: OTHER LOANS & LEASES-QBP-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnotci5: Option<f32>,

    #[doc = r#"Title: LN TO NONDEP FIN INST & OTH-FGN"#]
    #[doc = r#"Description: "#]
    pub lnotherf: Option<f32>,

    #[doc = r#"Title: LN TO NONDEP FIN INST & OTH-FGN RATIO"#]
    #[doc = r#"Description: "#]
    pub lnotherfr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * OVER 15 YRS"#]
    #[doc = r#"Description: "#]
    pub lnotov15: Option<f32>,

    #[doc = r#"Title: ALL OTHER LNS & LS * OVER 15 YRS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnotov15r: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 100-$"#]
    #[doc = r#"Description: "#]
    pub lnreag1: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 100-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag1r: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-100-250-$"#]
    #[doc = r#"Description: "#]
    pub lnreag2: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-100-250-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag2r: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-250-500-$"#]
    #[doc = r#"Description: "#]
    pub lnreag3: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-250-500-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag3r: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 500-$"#]
    #[doc = r#"Description: "#]
    pub lnreag4: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 500-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag4r: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 100-NUM"#]
    #[doc = r#"Description: "#]
    pub lnreag1n: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 100-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag1nr: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-100-250-NUM"#]
    #[doc = r#"Description: "#]
    pub lnreag2n: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-100-250-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag2nr: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-250-500-NUM"#]
    #[doc = r#"Description: "#]
    pub lnreag3n: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-250-500-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag3nr: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 500-NUM"#]
    #[doc = r#"Description: "#]
    pub lnreag4n: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-UNDER 500-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnreag4nr: Option<f32>,

    #[doc = r#"Title: 1-4 FAM RE CONSTRUCTION LOANS"#]
    #[doc = r#"Description: "#]
    pub lnrecnfm: Option<f32>,

    #[doc = r#"Title: 1-4 FAM RE CONSTRUCTION LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrecnfmr: Option<f32>,

    #[doc = r#"Title: OTHER RE CONSTRUCTION & LAND LN"#]
    #[doc = r#"Description: "#]
    pub lnrecnot: Option<f32>,

    #[doc = r#"Title: OTHER RE CONSTRUCTION & LAND LN"#]
    #[doc = r#"Description: "#]
    pub lnrecnotr: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub lnreoth: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-1-4 FAMILY2"#]
    #[doc = r#"Description: "#]
    pub lnreoth2: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY OTHER LOANS CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnreoth5: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 100-$"#]
    #[doc = r#"Description: "#]
    pub lnrenr1: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 100-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr1r: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-100-250-$"#]
    #[doc = r#"Description: "#]
    pub lnrenr2: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-100-250-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr2r: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-250-1M-$"#]
    #[doc = r#"Description: "#]
    pub lnrenr3: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-250-1M-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr3r: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 1M-$"#]
    #[doc = r#"Description: "#]
    pub lnrenr4: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 1M-$ RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr4r: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 100-NUM"#]
    #[doc = r#"Description: "#]
    pub lnrenr1n: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 100-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr1nr: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-100-250-NUM"#]
    #[doc = r#"Description: "#]
    pub lnrenr2n: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-100-250-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr2nr: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-250-1M-NUM"#]
    #[doc = r#"Description: "#]
    pub lnrenr3n: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-250-1M-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr3nr: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 1M-NUM"#]
    #[doc = r#"Description: "#]
    pub lnrenr4n: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES-UNDER 1M-NUM RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenr4nr: Option<f32>,

    #[doc = r#"Title: OTHER NONFARM NONRES RE LNS"#]
    #[doc = r#"Description: "#]
    pub lnrenrot: Option<f32>,

    #[doc = r#"Title: OTHER NONFARM NONRES RE LNS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenrotr: Option<f32>,

    #[doc = r#"Title: OWNER-OCC NONFARM NONRES RE LNS"#]
    #[doc = r#"Description: "#]
    pub lnrenrow: Option<f32>,

    #[doc = r#"Title: OWNER-OCC NONFARM NONRES RE LNS"#]
    #[doc = r#"Description: "#]
    pub lnrenrowr: Option<f32>,

    #[doc = r#"Title: RE LNS-NON US ADDRESSEES"#]
    #[doc = r#"Description: "#]
    pub lnrenus: Option<f32>,

    #[doc = r#"Title: RE LNS-NON US ADDRESSEES RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrenusr: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-FIRST LIENS-ADJUST"#]
    #[doc = r#"Description: "#]
    pub lnrersf1: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-FIRST LIENS-ADJUST RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrersf1r: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-SECOND LIENS"#]
    #[doc = r#"Description: "#]
    pub lnrersf2: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-SECOND LIENS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrersf2r: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-FIRST LIENS"#]
    #[doc = r#"Description: "#]
    pub lnrersfm: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY-FIRST LIENS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrersfmr: Option<f32>,

    #[doc = r#"Title: LOAN LOSS RESERVE/N/C LOANS"#]
    #[doc = r#"Description: "#]
    pub lnresncr: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 1-3 YEARS"#]
    #[doc = r#"Description: "#]
    pub lnrs1t3: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 1-3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrs1t3r: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 3 MONS OR LESS"#]
    #[doc = r#"Description: "#]
    pub lnrs3les: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 3 MONS OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrs3lesr: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 3-5 YEARS"#]
    #[doc = r#"Description: "#]
    pub lnrs3t5: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 3-5 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrs3t5r: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 3-12 MONTHS"#]
    #[doc = r#"Description: "#]
    pub lnrs3t12: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 3-12 MONTHS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrs3t12r: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 5-15 YEARS"#]
    #[doc = r#"Description: "#]
    pub lnrs5t15: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * 5-15 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrs5t15r: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * OVER 15 YEARS"#]
    #[doc = r#"Description: "#]
    pub lnrsov15: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY * OVER 15 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnrsov15r: Option<f32>,

    #[doc = r#"Title: SMALL BUSINESS LNS SOLD-AMT"#]
    #[doc = r#"Description: "#]
    pub lnsb: Option<f32>,

    #[doc = r#"Title: SMALL BUSINESS LNS SOLD"#]
    #[doc = r#"Description: "#]
    pub lnsbr: Option<f32>,

    #[doc = r#"Title: PRIN BAL- LNS SERVICE FOR OTHERS"#]
    #[doc = r#"Description: "#]
    pub lnserv: Option<f32>,

    #[doc = r#"Title: PRIN BAL- LNS SERVICE FOR OTHERS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnservr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LETTERS OF CREDIT"#]
    #[doc = r#"Description: "#]
    pub loccom: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LETTERS OF CREDIT RATIO"#]
    #[doc = r#"Description: "#]
    pub loccomr: Option<f32>,

    #[doc = r#"Title: FIN & PERFORM STANDBY LOC"#]
    #[doc = r#"Description: "#]
    pub locfpsb: Option<f32>,

    #[doc = r#"Title: FIN & PERFORM STANDBY LOC RATIO"#]
    #[doc = r#"Description: "#]
    pub locfpsbr: Option<f32>,

    #[doc = r#"Title: FIN & PERFORM STANDBY LOC-CONVEY"#]
    #[doc = r#"Description: "#]
    pub locfpsbk: Option<f32>,

    #[doc = r#"Title: FIN & PERFORM STANDBY LOC-CONVEY RATIO"#]
    #[doc = r#"Description: "#]
    pub locfpsbkr: Option<f32>,

    #[doc = r#"Title: FINANCIAL STANDBY LOC"#]
    #[doc = r#"Description: "#]
    pub locfsb: Option<f32>,

    #[doc = r#"Title: FINANCIAL STANDBY LOC RATIO"#]
    #[doc = r#"Description: "#]
    pub locfsbr: Option<f32>,

    #[doc = r#"Title: FINANCIAL STANDBY LOC-CONVEYED"#]
    #[doc = r#"Description: "#]
    pub locfsbk: Option<f32>,

    #[doc = r#"Title: FINANCIAL STANDBY LOC-CONVEYED RATIO"#]
    #[doc = r#"Description: "#]
    pub locfsbkr: Option<f32>,

    #[doc = r#"Title: PERFORMANCE STANDBY LOC"#]
    #[doc = r#"Description: "#]
    pub locpsb: Option<f32>,

    #[doc = r#"Title: PERFORMANCE STANDBY LOC RATIO"#]
    #[doc = r#"Description: "#]
    pub locpsbr: Option<f32>,

    #[doc = r#"Title: PERFORMANCE STANDBY LOC-CONVEYED"#]
    #[doc = r#"Description: "#]
    pub locpsbk: Option<f32>,

    #[doc = r#"Title: PERFORMANCE STANDBY LOC-CONVEYED RATIO"#]
    #[doc = r#"Description: "#]
    pub locpsbkr: Option<f32>,

    #[doc = r#"Title: ORE PROTECTED - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub loregty: Option<f32>,

    #[doc = r#"Title: ORE PROTECTED - LOSS SHARE RATIO"#]
    #[doc = r#"Description: "#]
    pub loregtyr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LN & LS - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub loth: Option<f32>,

    #[doc = r#"Title: ALL OTHER LN & LS - LOSS SHARE RATIO"#]
    #[doc = r#"Description: "#]
    pub lothr: Option<f32>,

    #[doc = r#"Title: RE FARMLAND LN - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub lreag: Option<f32>,

    #[doc = r#"Title: RE FARMLAND LN - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub lreagr: Option<f32>,

    #[doc = r#"Title: RE CONSTRUCT LN - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub lrecons: Option<f32>,

    #[doc = r#"Title: RE CONSTRUCT LN - LOSS SHARE RATIO"#]
    #[doc = r#"Description: "#]
    pub lreconsr: Option<f32>,

    #[doc = r#"Title: RE MULTIFAMILY LN-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub lremult: Option<f32>,

    #[doc = r#"Title: RE MULTIFAMILY LN-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub lremultr: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES LN - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub lrenres: Option<f32>,

    #[doc = r#"Title: RE NONFARM NONRES LN - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub lrenresr: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY LNS - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub lreres: Option<f32>,

    #[doc = r#"Title: RE 1-4 FAMILY LNS - LOSS SHARE RATIO"#]
    #[doc = r#"Description: "#]
    pub lreresr: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE-LNLS"#]
    #[doc = r#"Description: "#]
    pub lsalnls: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE-LNLS RATIO"#]
    #[doc = r#"Description: "#]
    pub lsalnlsr: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE -OTH ASSET"#]
    #[doc = r#"Description: "#]
    pub lsaoa: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE -OTH ASSET RATIO"#]
    #[doc = r#"Description: "#]
    pub lsaoar: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE- ORE"#]
    #[doc = r#"Description: "#]
    pub lsaore: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE- ORE RATIO"#]
    #[doc = r#"Description: "#]
    pub lsaorer: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE -DEBT SEC"#]
    #[doc = r#"Description: "#]
    pub lsascdbt: Option<f32>,

    #[doc = r#"Title: CARRY AMT LOSS SHARE -DEBT SEC RATIO"#]
    #[doc = r#"Description: "#]
    pub lsascdbtr: Option<f32>,

    #[doc = r#"Title: LEASES-FOR"#]
    #[doc = r#"Description: "#]
    pub lsfor: Option<f32>,

    #[doc = r#"Title: LEASES-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub lsforr: Option<f32>,

    #[doc = r#"Title: FIPS MSA CODE"#]
    #[doc = r#"Description: "#]
    pub msa: Option<f32>,

    #[doc = r#"Title: OUT PRIN BAL MORT W/ RECOURSE"#]
    #[doc = r#"Description: "#]
    pub msrece: Option<f32>,

    #[doc = r#"Title: OUT PRIN BAL MORT W/ RECOURSE RATIO"#]
    #[doc = r#"Description: "#]
    pub msrecer: Option<f32>,

    #[doc = r#"Title: 1-4 FM SERVICED IN FORECLOSURE"#]
    #[doc = r#"Description: "#]
    pub msresfcl: Option<f32>,

    #[doc = r#"Title: 1-4 FM SERVICED IN FORECLOSURE RATIO"#]
    #[doc = r#"Description: "#]
    pub msresfclr: Option<f32>,

    #[doc = r#"Title: OUT PRIN BAL MORT W/ NO RECOURSE"#]
    #[doc = r#"Description: "#]
    pub msrnrece: Option<f32>,

    #[doc = r#"Title: OUT PRIN BAL MORT W/ NO RECOURSE RATIO"#]
    #[doc = r#"Description: "#]
    pub msrnrecer: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-AGRICULTURAL LNS"#]
    #[doc = r#"Description: "#]
    pub naag: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-AGRICULTURAL LNS RATIO"#]
    #[doc = r#"Description: "#]
    pub naagr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-AG LNS*SMALL BKS"#]
    #[doc = r#"Description: "#]
    pub naagsm: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-AG LNS*SMALL BKS RATIO"#]
    #[doc = r#"Description: "#]
    pub naagsmr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-TOTAL ASSETS"#]
    #[doc = r#"Description: "#]
    pub naasset: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-AG LNS*SMALL BKS RATIO"#]
    #[doc = r#"Description: "#]
    pub naassetr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL AUTO LOANS"#]
    #[doc = r#"Description: "#]
    pub naauto: Option<f32>,

    #[doc = r#"Title: NONACCRUAL AUTO LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub naautor: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-C&I LOANS"#]
    #[doc = r#"Description: "#]
    pub naci: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-C&I LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub nacir: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-C&I*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub nacinus: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-C&I*NON-U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub nacinusr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-CONSUMER LOANS"#]
    #[doc = r#"Description: "#]
    pub nacon: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-CONSUMER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub naconr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-OTHER CONSUMER"#]
    #[doc = r#"Description: "#]
    pub naconoth: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-OTHER CONSUMER RATIO"#]
    #[doc = r#"Description: "#]
    pub naconothr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-CREDIT CARD PLANS"#]
    #[doc = r#"Description: "#]
    pub nacrcd: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-CREDIT CARD PLANS RATIO"#]
    #[doc = r#"Description: "#]
    pub nacrcdr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-DEP INST LOANS"#]
    #[doc = r#"Description: "#]
    pub nadep: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-DEP INST LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub nadepr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-DEP INST*NON U.S."#]
    #[doc = r#"Description: "#]
    pub nadepnus: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-DEP INST*NON U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub nadepnusr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-FOREIGN GOVT"#]
    #[doc = r#"Description: "#]
    pub nafg: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-FOREIGN GOVT RATIO"#]
    #[doc = r#"Description: "#]
    pub nafgr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub nagty: Option<f32>,

    #[doc = r#"Title: NONACCRUAL -GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub nagtyr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL REBOOKED GNMA LOANS"#]
    #[doc = r#"Description: "#]
    pub nagtygnm: Option<f32>,

    #[doc = r#"Title: NONACCRUAL REBOOKED GNMA LNS"#]
    #[doc = r#"Description: "#]
    pub nagtygnmr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-PART GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub nagtypar: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-PART GTY LN&LS RATIO"#]
    #[doc = r#"Description: "#]
    pub nagtyparr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL AG LOANS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalag: Option<f32>,

    #[doc = r#"Title: NONACCRUAL AG LOANS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalagr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL C&I LNS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalci: Option<f32>,

    #[doc = r#"Title: NONACCRUAL C&I LNS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalcir: Option<f32>,

    #[doc = r#"Title: NONACCRUAL CONSUMER LN -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalcon: Option<f32>,

    #[doc = r#"Title: NONACCRUAL CONSUMER LN -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalconr: Option<f32>,

    #[doc = r#"Title: NONACCR PROTECT (GTY)-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalgty: Option<f32>,

    #[doc = r#"Title: NONACCRUAL PROTECT (GTY)-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalgtyr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-L&L HELD FOR SALE"#]
    #[doc = r#"Description: "#]
    pub nalnsale: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-L&L HELD FOR SALE RATIO"#]
    #[doc = r#"Description: "#]
    pub nalnsaler: Option<f32>,

    #[doc = r#"Title: NONACCRUAL OTHER LNS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub naloth: Option<f32>,

    #[doc = r#"Title: NONACCRUAL OTHER LNS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalothr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RE FARM-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalreag: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RE FARM LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalreagr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL CONSTR LN -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalrecon: Option<f32>,

    #[doc = r#"Title: NONACCRUAL CONSTR LN -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalreconr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL MULTIFAM - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalremul: Option<f32>,

    #[doc = r#"Title: NONACCRUAL MULTIFAM - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalremulr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL NFNR LN - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalrenrs: Option<f32>,

    #[doc = r#"Title: NONACCRUAL NFNR LN - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalrenrsr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL 1-4 FM LN-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub nalreres: Option<f32>,

    #[doc = r#"Title: NONACCRUAL 1-4 FM LN-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub nalreresr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-LEASES"#]
    #[doc = r#"Description: "#]
    pub nals: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-LEASES RATIO"#]
    #[doc = r#"Description: "#]
    pub nalsr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL TOTAL LOANS - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub naltot: Option<f32>,

    #[doc = r#"Title: NONACCRUAL TOTAL LOANS - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub naltotr: Option<f32>,

    #[doc = r#"Title: INSTITUTION NAME (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub name: Option<String>,

    #[doc = r#"Title: INSTITUTION FULL NAME (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub namefull: Option<String>,

    #[doc = r#"Title: NONACCRUAL-ALL OTHER LOANS"#]
    #[doc = r#"Description: "#]
    pub naothln: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-ALL OTHER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub naothlnr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-REAL ESTATE LOANS"#]
    #[doc = r#"Description: "#]
    pub nare: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-REAL ESTATE LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub narer: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*FARMLAND"#]
    #[doc = r#"Description: "#]
    pub nareag: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*FARMLAND RATIO"#]
    #[doc = r#"Description: "#]
    pub nareagr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL 1-4 FAM CONSTRUCT LN"#]
    #[doc = r#"Description: "#]
    pub narecnfm: Option<f32>,

    #[doc = r#"Title: NONACCRUAL 1-4 FAM CONSTRUCT LN RATIO"#]
    #[doc = r#"Description: "#]
    pub narecnfmr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL OTHER CONSTR & LAND"#]
    #[doc = r#"Description: "#]
    pub narecnot: Option<f32>,

    #[doc = r#"Title: NONACCRUAL OTHER CONSTR & LAND RATIO"#]
    #[doc = r#"Description: "#]
    pub narecnotr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub narecons: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*CONSTRUCTION RATIO"#]
    #[doc = r#"Description: "#]
    pub nareconsr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*FOREIGN"#]
    #[doc = r#"Description: "#]
    pub narefor: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*FOREIGN RATIO"#]
    #[doc = r#"Description: "#]
    pub nareforr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 FAM LINES"#]
    #[doc = r#"Description: "#]
    pub nareloc: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 FAM LINES RATIO"#]
    #[doc = r#"Description: "#]
    pub narelocr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*MULTIFAMILY"#]
    #[doc = r#"Description: "#]
    pub naremult: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*MULTIFAMILY RATIO"#]
    #[doc = r#"Description: "#]
    pub naremultr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*NONFARM NONRES"#]
    #[doc = r#"Description: "#]
    pub narenres: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*NONFARM NONRES RATIO"#]
    #[doc = r#"Description: "#]
    pub narenresr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL OTHER NONFARM NONRES"#]
    #[doc = r#"Description: "#]
    pub narenrot: Option<f32>,

    #[doc = r#"Title: NONACCRUAL OTHER NONFARM NONRES RATIO"#]
    #[doc = r#"Description: "#]
    pub narenrotr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL 0WN-OCC NONFRM NONRS"#]
    #[doc = r#"Description: "#]
    pub narenrow: Option<f32>,

    #[doc = r#"Title: NONACCRUAL OWN-OCC NONFRM NONRS RATIO"#]
    #[doc = r#"Description: "#]
    pub narenrowr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub narenus: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*NON-U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub narenusr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub nareres: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 FAMILY RATIO"#]
    #[doc = r#"Description: "#]
    pub nareresr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 JUNIOR LIEN"#]
    #[doc = r#"Description: "#]
    pub narersf2: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 JN LIEN RATIO"#]
    #[doc = r#"Description: "#]
    pub narersf2r: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 IST LIEN"#]
    #[doc = r#"Description: "#]
    pub narersfm: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-RE*1-4 IST LIEN RATIO"#]
    #[doc = r#"Description: "#]
    pub narersfmr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRUCT C&I LN"#]
    #[doc = r#"Description: "#]
    pub narsci: Option<f32>,

    #[doc = r#"Title: NONACCR RESTRUCT CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub narscons: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRU LN- 1-4 FAM"#]
    #[doc = r#"Description: "#]
    pub narslnfm: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRU LN- 1-4 FAM RATIO"#]
    #[doc = r#"Description: "#]
    pub narslnfmr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRU LN EXCL 1-4 FM"#]
    #[doc = r#"Description: "#]
    pub narslnls: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRU LN EXCL 1-4 FM RATIO"#]
    #[doc = r#"Description: "#]
    pub narslnlsr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRUCT LN- TOTAL"#]
    #[doc = r#"Description: "#]
    pub narslnlt: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRUCT LN- TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub narslnltr: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRUCT MULTIFAMILY"#]
    #[doc = r#"Description: "#]
    pub narsmult: Option<f32>,

    #[doc = r#"Title: NONACCR RESTRUCTURED NFNR LN"#]
    #[doc = r#"Description: "#]
    pub narsnres: Option<f32>,

    #[doc = r#"Title: NONACCRUAL RESTRUCT ALL OTH LN"#]
    #[doc = r#"Description: "#]
    pub narsoth: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-DEBT SECURITIES"#]
    #[doc = r#"Description: "#]
    pub nascdebt: Option<f32>,

    #[doc = r#"Title: NONACCRUAL-DEBT SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub nascdebtr: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-AGRICULTURAL LNS"#]
    #[doc = r#"Description: "#]
    pub ncag: Option<f32>,

    #[doc = r#"Title: N/C AUTO LOANS"#]
    #[doc = r#"Description: "#]
    pub ncauto: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-C&I LOANS"#]
    #[doc = r#"Description: "#]
    pub ncci: Option<f32>,

    #[doc = r#"Title: NC COMMERCIAL RE/COMMERCIAL RE"#]
    #[doc = r#"Description: "#]
    pub nccomrer: Option<f32>,

    #[doc = r#"Title: NC COMMERCIAL RE/COMMERCIAL RE"#]
    #[doc = r#"Description: "#]
    pub nccomre: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-CONSUMER LOANS"#]
    #[doc = r#"Description: "#]
    pub nccon: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-OTHER CONSUMER"#]
    #[doc = r#"Description: "#]
    pub ncconoth: Option<f32>,

    #[doc = r#"Title: TOTAL N/C CREDIT CARD PLANS"#]
    #[doc = r#"Description: "#]
    pub nccrcd: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-DEP INST LOANS"#]
    #[doc = r#"Description: "#]
    pub ncdep: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-FOREIGN GOVT"#]
    #[doc = r#"Description: "#]
    pub ncfg: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-PART GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub ncgtypar: Option<f32>,

    #[doc = r#"Title: N/C LNS & LS/GROSS LNS & LS"#]
    #[doc = r#"Description: "#]
    pub nclnlsr: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-LEASES"#]
    #[doc = r#"Description: "#]
    pub ncls: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-ALL OTHER LOANS"#]
    #[doc = r#"Description: "#]
    pub ncothln: Option<f32>,

    #[doc = r#"Title: TOTAL N/C REAL ESTATE LOANS"#]
    #[doc = r#"Description: "#]
    pub ncre: Option<f32>,

    #[doc = r#"Title: N/C CONST REAL ESTATE/CONST RE"#]
    #[doc = r#"Description: "#]
    pub ncreconr: Option<f32>,

    #[doc = r#"Title: TOTAL N/C CONST REAL ESTATE CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub ncrecons: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-RE 1-4 FAMILY LINES"#]
    #[doc = r#"Description: "#]
    pub ncreloc: Option<f32>,

    #[doc = r#"Title: N/C HOME EQUITY/HOME EQUITY"#]
    #[doc = r#"Description: "#]
    pub ncrelocr: Option<f32>,

    #[doc = r#"Title: N/C MULTIFAMLY RE/MULTIFAMLY RE"#]
    #[doc = r#"Description: "#]
    pub ncremulr: Option<f32>,

    #[doc = r#"Title: TOTAL N/C MULTIFAMLY RE"#]
    #[doc = r#"Description: "#]
    pub ncremult: Option<f32>,

    #[doc = r#"Title: N/C NONFARM NONRES RE/NONRES RE"#]
    #[doc = r#"Description: "#]
    pub ncrenrer: Option<f32>,

    #[doc = r#"Title: TOTAL N/C NONFARM NONRES RE"#]
    #[doc = r#"Description: "#]
    pub ncrenres: Option<f32>,

    #[doc = r#"Title: N/C REAL ESTATE LNS/REAL ESTATE"#]
    #[doc = r#"Description: "#]
    pub ncrer: Option<f32>,

    #[doc = r#"Title: N/C 1-4 OTHER RE/1-4 OTHER RE"#]
    #[doc = r#"Description: "#]
    pub ncrereso: Option<f32>,

    #[doc = r#"Title: N/C 1-4 OTHER RE/1-4 OTHER RE"#]
    #[doc = r#"Description: "#]
    pub ncrereor: Option<f32>,

    #[doc = r#"Title: N/C 1-4 FAMILY RE"#]
    #[doc = r#"Description: "#]
    pub ncreres: Option<f32>,

    #[doc = r#"Title: N/C 1-4 FAMILY RE/1-4 FAMILY RE"#]
    #[doc = r#"Description: "#]
    pub ncreresr: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF FIX ASSETS"#]
    #[doc = r#"Description: "#]
    pub netgnast: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF FIX ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub netgnastr: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF FIX ASSETS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntglfxaq: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF FIX ASSETS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntglfxaqr: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF LOANS"#]
    #[doc = r#"Description: "#]
    pub netgnsln: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub netgnslnr: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF LOANS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntgllnq: Option<f32>,

    #[doc = r#"Title: NET G/L ON SALES OF LOANS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntgllnqr: Option<f32>,

    #[doc = r#"Title: NET G/L ON OTHER RE OWNED"#]
    #[doc = r#"Description: "#]
    pub netgnsre: Option<f32>,

    #[doc = r#"Title: NET G/L ON OTHER RE OWNED RATIO"#]
    #[doc = r#"Description: "#]
    pub netgnsrer: Option<f32>,

    #[doc = r#"Title: NET G/L ON OTHER RE OWNED QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntglreq: Option<f32>,

    #[doc = r#"Title: NET G/L ON OTHER RE OWNED QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntglreqr: Option<f32>,

    #[doc = r#"Title: NET INCOME- BANK- ANN"#]
    #[doc = r#"Description: "#]
    pub netinca: Option<f32>,

    #[doc = r#"Title: NET INTEREST MARGIN"#]
    #[doc = r#"Description: "#]
    pub nimy: Option<f32>,

    #[doc = r#"Title: NET INTEREST MARGIN QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub nimyq: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ"#]
    #[doc = r#"Description: "#]
    pub noij: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ RATIO"#]
    #[doc = r#"Description: "#]
    pub noijr: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ/ASSETS"#]
    #[doc = r#"Description: "#]
    pub noijy: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ/ASSETS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub noijyq: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ ANNUALLY"#]
    #[doc = r#"Description: "#]
    pub noija: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub noijq: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub noijqa: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-ADJ QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub noijqr: Option<f32>,

    #[doc = r#"Title: NONINTEREST INC/AVERAGE ASSETS"#]
    #[doc = r#"Description: "#]
    pub noniiay: Option<f32>,

    #[doc = r#"Title: NONINTEREST INC/AVERAGE ASSETS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub noniiayq: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST INCOME ANNUALLY"#]
    #[doc = r#"Description: "#]
    pub noniia: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST INCOME-QTR"#]
    #[doc = r#"Description: "#]
    pub noniiq: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST INCOME-QTR"#]
    #[doc = r#"Description: "#]
    pub noniiqa: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST INCOME-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub noniiqr: Option<f32>,

    #[doc = r#"Title: NONINTEREST EXP/AVERAGE ASSETS"#]
    #[doc = r#"Description: "#]
    pub nonixay: Option<f32>,

    #[doc = r#"Title: NONINTEREST EXP/AVERAGE ASSETS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub nonixayq: Option<f32>,

    #[doc = r#"Title: TOTAL NONINTEREST EXPENSE ANNUALLY"#]
    #[doc = r#"Description: "#]
    pub nonixa: Option<f32>,

    #[doc = r#"Title: NONPERF ASSETS/TOTAL ASSETS"#]
    #[doc = r#"Description: "#]
    pub nperf: Option<f32>,

    #[doc = r#"Title: NONPERF ASSETS/TOTAL ASSETS"#]
    #[doc = r#"Description: "#]
    pub nperfv: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntag: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntagr: Option<f32>,

    #[doc = r#"Title: AGRICULTURAL LN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntaga: Option<f32>,

    #[doc = r#"Title: AG LOAN NET CHARGE-OFFS-QTR"#]
    #[doc = r#"Description: "#]
    pub ntagq: Option<f32>,

    #[doc = r#"Title: AG LOAN NET CHARGE-OFFS-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntagqr: Option<f32>,

    #[doc = r#"Title: AG LN NET CHARGE-OFFS*SMALL BKS"#]
    #[doc = r#"Description: "#]
    pub ntagsm: Option<f32>,

    #[doc = r#"Title: AG LN NET CHARGE-OFFS*SMALL BKS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntagsmr: Option<f32>,

    #[doc = r#"Title: AG LN NET CHARGE-OFFS*SMALL BKS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntagsmq: Option<f32>,

    #[doc = r#"Title: AG LN NET CHARGE-OFFS*SMALL BKS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntagsmqr: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntauto: Option<f32>,

    #[doc = r#"Title: AUTO LOANS - NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntautor: Option<f32>,

    #[doc = r#"Title: AUTO LNS - NET CHG-OFFS - ANN"#]
    #[doc = r#"Description: "#]
    pub ntautoa: Option<f32>,

    #[doc = r#"Title: AUTO LNS - NET CHG-OFFS - QTR"#]
    #[doc = r#"Description: "#]
    pub ntautoq: Option<f32>,

    #[doc = r#"Title: AUTO LNS - NET CHG-OFFS - QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntautolnqr: Option<f32>,

    #[doc = r#"Title: AUTO LN-CHG-OFF- QTR/AUTO LN"#]
    #[doc = r#"Description: "#]
    pub ntautoqr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntci: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntcir: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntcia: Option<f32>,

    #[doc = r#"Title: NON-U.S.COMMERCIAL LN NET CHG-OF"#]
    #[doc = r#"Description: "#]
    pub ntcinus: Option<f32>,

    #[doc = r#"Title: NON-U.S.COMMERCIAL LN NET CHG-OF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntcinusr: Option<f32>,

    #[doc = r#"Title: NON-U.S.COMMERCIAL LN NET CHG-OF QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntcinusq: Option<f32>,

    #[doc = r#"Title: NON-U.S.COMMERCIAL LN NET CHG-OF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntcinusqr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntciq: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntciqr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE CHG-OFF/COMM RE LN"#]
    #[doc = r#"Description: "#]
    pub ntcomrer: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE CHG-OFF/COMM RE LN QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntcomreq: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE LN CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntcomrea: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntcon: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntconr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntcona: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOAN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntconota: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LN NET CHARGE-OFF"#]
    #[doc = r#"Description: "#]
    pub ntconoth: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LN NET CHARGE-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntconothr: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntconotq: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntconotqr: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntconq: Option<f32>,

    #[doc = r#"Title: CONSUMER LOAN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntconqr: Option<f32>,

    #[doc = r#"Title: OTH.CONSUMER CHGOFF-QTR/OTH.CONS"#]
    #[doc = r#"Description: "#]
    pub ntcontqr: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntcrcd: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntcrcdr: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LOAN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntcrcda: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntcrcdq: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntcrcdqr: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN NET CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub ntdep: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN NET CHG-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntdepr: Option<f32>,

    #[doc = r#"Title: FOREIGN DEP INST LN NET CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntdepnus: Option<f32>,

    #[doc = r#"Title: FOREIGN DEP INST LN NET CHG-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntdepnusr: Option<f32>,

    #[doc = r#"Title: FOREIGN DEP INST LN NET CHG-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntdepnuq: Option<f32>,

    #[doc = r#"Title: FOREIGN DEP INST LN NET CHG-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntdepnuqr: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntdepq: Option<f32>,

    #[doc = r#"Title: DEPOSITORY INST LOAN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntdepqr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVT LN NET CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntforgv: Option<f32>,

    #[doc = r#"Title: FOREIGN GOVT LN NET CHG-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntforgvr: Option<f32>,

    #[doc = r#"Title: FOREIGN GOV LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntforgvq: Option<f32>,

    #[doc = r#"Title: FOREIGN GOV LN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntforgvqr: Option<f32>,

    #[doc = r#"Title: NET INCOME-BK-HIGHER-PP"#]
    #[doc = r#"Description: "#]
    pub ntinchpp: Option<f32>,

    #[doc = r#"Title: NET INCOME-BANK- LOSERS"#]
    #[doc = r#"Description: "#]
    pub ntincl: Option<f32>,

    #[doc = r#"Title: NET INCOME-BK-LOSER-QTR"#]
    #[doc = r#"Description: "#]
    pub ntinclq: Option<f32>,

    #[doc = r#"Title: TOTAL LN&LS NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntlnlsa: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub ntinqhpp: Option<f32>,

    #[doc = r#"Title: NET CHARGE-OFFS/LOANS & LEASES"#]
    #[doc = r#"Description: "#]
    pub ntlnlsr: Option<f32>,

    #[doc = r#"Title: NET CHARGE-OFFS/LOANS & LEASES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntlnlsqr: Option<f32>,

    #[doc = r#"Title: LEASE NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntls: Option<f32>,

    #[doc = r#"Title: LEASE NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntlsr: Option<f32>,

    #[doc = r#"Title: LEASE NET CHARGE-OFFS-QTR"#]
    #[doc = r#"Description: "#]
    pub ntlsq: Option<f32>,

    #[doc = r#"Title: LEASE NET CHARGE-OFFS-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntlsqr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntother: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOAN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntotherr: Option<f32>,

    #[doc = r#"Title: ALL OTHER LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntothq: Option<f32>,

    #[doc = r#"Title: ALL OTHER LN NET-CHG-QTRS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntothqr: Option<f32>,

    #[doc = r#"Title: AMT TIME DEP OF $100,000 OR LESS"#]
    #[doc = r#"Description: "#]
    pub ntrcdsm: Option<f32>,

    #[doc = r#"Title: AMT TIME DEP OF $100,000 OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrcdsmr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTN-COM BKS & OTH U.S."#]
    #[doc = r#"Description: "#]
    pub ntrcomot: Option<f32>,

    #[doc = r#"Title: NONTRANSACTN-COM BKS & OTH U.S RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrcomotr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntre: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub ntremuqa: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub ntrecoqa: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrelnr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntreq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntreqa: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrerq: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntreag: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreagr: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntreagq: Option<f32>,

    #[doc = r#"Title: RE LN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntrea: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreagqr: Option<f32>,

    #[doc = r#"Title: 1-4 FAM CONST LN NET-OFF"#]
    #[doc = r#"Description: "#]
    pub ntrecnfm: Option<f32>,

    #[doc = r#"Title: OTHER CONSTRUCT NET CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub ntrecnot: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntreconq: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreconqr: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN NET CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntrecons: Option<f32>,

    #[doc = r#"Title: CONST RE LOANS NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntrecosa: Option<f32>,

    #[doc = r#"Title: CONSTRUCTION RE LN NET CHG-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreconsr: Option<f32>,

    #[doc = r#"Title: CONST RE CHG-OFF/CONST RE LOANS"#]
    #[doc = r#"Description: "#]
    pub ntrecosr: Option<f32>,

    #[doc = r#"Title: CONST RE CHG-OFF/CONST RE LOANS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntrecoqr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN NET CHG-OFF-FOR"#]
    #[doc = r#"Description: "#]
    pub ntrefor: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN NET CHG-OFF-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreforr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN NET CHG-OFF-FOR QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntreforq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LN NET CHG-OFF-FOR QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreforqr: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN NET CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub ntreloc: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN NET CHG-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreloclnr: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN NET CHG-OFF QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntrelocq: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN NET CHG-OFF ANNUALLY"#]
    #[doc = r#"Description: "#]
    pub ntreloca: Option<f32>,

    #[doc = r#"Title: LINE OF CREDIT RE LN NET CHG-OFF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrelocqr: Option<f32>,

    #[doc = r#"Title: HOME EQUITY CHG-OFF/HOME EQ LNS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrelocrq: Option<f32>,

    #[doc = r#"Title: HOME EQUITY CHG-OFF/HOME EQ LNS"#]
    #[doc = r#"Description: "#]
    pub ntrelocr: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RE LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntremulq: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RES RE LN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntremula: Option<f32>,

    #[doc = r#"Title: MULTIFAMILY RE LN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntremulqr: Option<f32>,

    #[doc = r#"Title: MULTIFAM RE CHG-OFF/MULTI RE LN"#]
    #[doc = r#"Description: "#]
    pub ntremulr: Option<f32>,

    #[doc = r#"Title: MULTIFAM RE CHG-OFF/MULTI RE LN QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntremuqr: Option<f32>,

    #[doc = r#"Title: MULTIFAMLY RES RE LN NET CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub ntremult: Option<f32>,

    #[doc = r#"Title: MULTIFAMLY RES RE LN NET CHG-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntremultr: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN NET CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub ntrenres: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN NET CHG-OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrenresr: Option<f32>,

    #[doc = r#"Title: OTHER NONFARM NONRS NET CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub ntrenrot: Option<f32>,

    #[doc = r#"Title: OWN OCC NONFRM NONRS NET CHG-OFF"#]
    #[doc = r#"Description: "#]
    pub ntrenrow: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntrenrsa: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntrenrsq: Option<f32>,

    #[doc = r#"Title: NONFARM NONRES RE LN NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrenrsqr: Option<f32>,

    #[doc = r#"Title: NONRES CHG-OFF/NONRES LOANS"#]
    #[doc = r#"Description: "#]
    pub ntrenrsr: Option<f32>,

    #[doc = r#"Title: NONRES CHG-OFF/NONRES LOANS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntrenrqr: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntrenus: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN NET CHARGE-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrenusr: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN NET CHARGE-OFFS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntrenusq: Option<f32>,

    #[doc = r#"Title: OTHER 1-4 FAM RE OTHER LN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntreotha: Option<f32>,

    #[doc = r#"Title: NON-U.S. RE LN NET CHARGE-OFFS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrenusqr: Option<f32>,

    #[doc = r#"Title: OTHER 1-4 FAM RE CHG-OFF/OTH 1-4"#]
    #[doc = r#"Description: "#]
    pub ntreothr: Option<f32>,

    #[doc = r#"Title: OTHER 1-4 FAM RE CHG-OFF/OTH 1-4 QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreothrqr: Option<f32>,

    #[doc = r#"Title: OTHER 1-4 FAM RE CHG-OFF/OTH 1-4 QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntreotqa: Option<f32>,

    #[doc = r#"Title: RE CHARGE-OFF/RE LOANS"#]
    #[doc = r#"Description: "#]
    pub ntrer: Option<f32>,

    #[doc = r#"Title: RE CHARGE-OFF/RE LOANS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntreqr: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY NET CHG-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntreres: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY NET CHG-OFFS RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrereslnr: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY NET-CHG-QTR"#]
    #[doc = r#"Description: "#]
    pub ntreresq: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntreresa: Option<f32>,

    #[doc = r#"Title: RE LOANS 1-4 FAMILY NET-CHG-QTR RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreresqr: Option<f32>,

    #[doc = r#"Title: 1-4 FAM RE CHG-OFF/1-4 FAM LOANS"#]
    #[doc = r#"Description: "#]
    pub ntreresr: Option<f32>,

    #[doc = r#"Title: 1-4 FAM RE CHG-OFF/1-4 FAM LOANS QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreresrq: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-NET C/OFF"#]
    #[doc = r#"Description: "#]
    pub ntrersf2: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-NET C/OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrersf2r: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-NET C/OFF QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntrers2q: Option<f32>,

    #[doc = r#"Title: RE LN 1-4 FAM JR LIEN-NET C/OFF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrers2qr: Option<f32>,

    #[doc = r#"Title: RE LN 1-4FAM IST LIEN-NET C/OFF"#]
    #[doc = r#"Description: "#]
    pub ntrersfm: Option<f32>,

    #[doc = r#"Title: RE LN 1-4FAM IST LIEN-NET C/OFF RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrersfmr: Option<f32>,

    #[doc = r#"Title: RE LN 1-4FAM IST LIEN-NET C/OFF QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntrersfq: Option<f32>,

    #[doc = r#"Title: RE LN 1-4FAM IST LIEN-NET C/OFF QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrersfqr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES"#]
    #[doc = r#"Description: "#]
    pub ntreoffdom: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreoffdomr: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ntreoffdomq: Option<f32>,

    #[doc = r#"Title: REAL ESTATE LOAN NET CHARGE-OFFS DOMESTIC OFFICES QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntreoffdomqr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-FOR COUNTRY"#]
    #[doc = r#"Description: "#]
    pub ntrfc: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-FOR CNTRY & GOVT"#]
    #[doc = r#"Description: "#]
    pub ntrfcfg: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-FOR CNTRY & GOVT RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrfcfgr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION-FOR GOVERNMENT"#]
    #[doc = r#"Description: "#]
    pub ntrfg: Option<f32>,

    #[doc = r#"Title: SAVINGS DEP-MMDA"#]
    #[doc = r#"Description: "#]
    pub ntrsmmda: Option<f32>,

    #[doc = r#"Title: SAVINGS DEP-MMDA RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrsmmdar: Option<f32>,

    #[doc = r#"Title: SAVINGS DEP-OTHER"#]
    #[doc = r#"Description: "#]
    pub ntrsoth: Option<f32>,

    #[doc = r#"Title: SAVINGS DEP-OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub ntrsothr: Option<f32>,

    #[doc = r#"Title: INCOME EARNED NOT COLLECTED"#]
    #[doc = r#"Description: "#]
    pub oaienc: Option<f32>,

    #[doc = r#"Title: LIFE INS ASSETS - GENERAL ACC"#]
    #[doc = r#"Description: "#]
    pub oalifgen: Option<f32>,

    #[doc = r#"Title: LIFE INS ASSETS - GENERAL ACC RATIO"#]
    #[doc = r#"Description: "#]
    pub oalifgenr: Option<f32>,

    #[doc = r#"Title: LIFE INS ASSETS - HYBRID ACC"#]
    #[doc = r#"Description: "#]
    pub oalifhyb: Option<f32>,

    #[doc = r#"Title: LIFE INS ASSETS - HYBRID ACC RATIO"#]
    #[doc = r#"Description: "#]
    pub oalifhybr: Option<f32>,

    #[doc = r#"Title: LIFE INSURANCE ASSETS"#]
    #[doc = r#"Description: "#]
    pub oalifins: Option<f32>,

    #[doc = r#"Title: LIFE INSURANCE RATIO"#]
    #[doc = r#"Description: "#]
    pub oalifinsr: Option<f32>,

    #[doc = r#"Title: LIFE INS ASSETS - SEPARATE ACC"#]
    #[doc = r#"Description: "#]
    pub oalifsep: Option<f32>,

    #[doc = r#"Title: LIFE INS ASSETS - SEPARATE ACC RATIO"#]
    #[doc = r#"Description: "#]
    pub oalifsepr: Option<f32>,

    #[doc = r#"Title: OFF-BALANCE SHEET DERIVATIVES"#]
    #[doc = r#"Description: "#]
    pub obsdir: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-FARMLAND"#]
    #[doc = r#"Description: "#]
    pub oreag: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-FARMLAND RATIO"#]
    #[doc = r#"Description: "#]
    pub oreagr: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-CONST"#]
    #[doc = r#"Description: "#]
    pub orecons: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-CONST RATIO"#]
    #[doc = r#"Description: "#]
    pub oreconsr: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-GNMA LOANS"#]
    #[doc = r#"Description: "#]
    pub oregnma: Option<f32>,

    #[doc = r#"Title: DIRECT & INDIRECT INVEST IN ORE"#]
    #[doc = r#"Description: "#]
    pub oreinv: Option<f32>,

    #[doc = r#"Title: DIRECT & INDIRECT INVEST IN ORE RATIO"#]
    #[doc = r#"Description: "#]
    pub oreinvr: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-MULTI"#]
    #[doc = r#"Description: "#]
    pub oremult: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-MULTI RATIO"#]
    #[doc = r#"Description: "#]
    pub oremultr: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-NONFARM"#]
    #[doc = r#"Description: "#]
    pub orenres: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-NONFARM RATIO"#]
    #[doc = r#"Description: "#]
    pub orenresr: Option<f32>,

    #[doc = r#"Title: OTHER REAL ESTATE OWNED"#]
    #[doc = r#"Description: "#]
    pub oreoth: Option<f32>,

    #[doc = r#"Title: OTHER REAL ESTATE OWNED RATIO"#]
    #[doc = r#"Description: "#]
    pub oreothr: Option<f32>,

    #[doc = r#"Title: OTHER REAL ESTATE OWNED-FOR"#]
    #[doc = r#"Description: "#]
    pub oreothf: Option<f32>,

    #[doc = r#"Title: OTHER REAL ESTATE OWNED-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub oreothfr: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED-1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub oreres: Option<f32>,

    #[doc = r#"Title: ALL OTHER RE OWNED 1-4 FAMILIY RATIO"#]
    #[doc = r#"Description: "#]
    pub oreresr: Option<f32>,

    #[doc = r#"Title: OTHER BORROWED MONEY-FOR"#]
    #[doc = r#"Description: "#]
    pub othborf: Option<f32>,

    #[doc = r#"Title: OTHER-FUTURES & FORWARD CONTRACT"#]
    #[doc = r#"Description: "#]
    pub othffc: Option<f32>,

    #[doc = r#"Title: OTHER-FUTURES & FORWARD CONTRACT RATIO"#]
    #[doc = r#"Description: "#]
    pub othffcr: Option<f32>,

    #[doc = r#"Title: OTHER-NOTIONAL VALUE SWAPS"#]
    #[doc = r#"Description: "#]
    pub othnvs: Option<f32>,

    #[doc = r#"Title: ALL OTH OFF-BALANCE SHEET LIAB"#]
    #[doc = r#"Description: "#]
    pub othoffbs: Option<f32>,

    #[doc = r#"Title: ALL OTH OFF-BALANCE SHEET LIAB RATIO"#]
    #[doc = r#"Description: "#]
    pub othoffbsr: Option<f32>,

    #[doc = r#"Title: OTHER-PURCHASED OPTION CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub othpoc: Option<f32>,

    #[doc = r#"Title: OTHER-WRITTEN OPTION CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub othwoc: Option<f32>,

    #[doc = r#"Title: OTS REGION NAME"#]
    #[doc = r#"Description: "#]
    pub otsregnm: Option<String>,

    #[doc = r#"Title: REC OWN INTEREST SEC - CI"#]
    #[doc = r#"Description: "#]
    pub owncrci: Option<f32>,

    #[doc = r#"Title: REC OWN INTEREST SEC - CRCD"#]
    #[doc = r#"Description: "#]
    pub owncrcrd: Option<f32>,

    #[doc = r#"Title: REC OWN INTEREST SEC - HEL"#]
    #[doc = r#"Description: "#]
    pub owncrhel: Option<f32>,

    #[doc = r#"Title: C/O OWN INTEREST SEC - CI"#]
    #[doc = r#"Description: "#]
    pub owndrci: Option<f32>,

    #[doc = r#"Title: C/O OWN INTEREST SEC - CRCD"#]
    #[doc = r#"Description: "#]
    pub owndrcrd: Option<f32>,

    #[doc = r#"Title: C/O OWN INTEREST SEC - HEL"#]
    #[doc = r#"Description: "#]
    pub owndrhel: Option<f32>,

    #[doc = r#"Title: LN SECURE HELD IN SEC - CI"#]
    #[doc = r#"Description: "#]
    pub ownlnci: Option<f32>,

    #[doc = r#"Title: LN SECURE HELD IN SEC - CRCD"#]
    #[doc = r#"Description: "#]
    pub ownlncrd: Option<f32>,

    #[doc = r#"Title: LN SECURE HELD IN SEC - HEL"#]
    #[doc = r#"Description: "#]
    pub ownlnhel: Option<f32>,

    #[doc = r#"Title: PD 30-89 OWN INTEREST SEC - CI"#]
    #[doc = r#"Description: "#]
    pub ownp3ci: Option<f32>,

    #[doc = r#"Title: PD 30-89 OWN INTEREST SEC - CRCD"#]
    #[doc = r#"Description: "#]
    pub ownp3crd: Option<f32>,

    #[doc = r#"Title: PD30-89 OWN INTEREST SEC - HEL"#]
    #[doc = r#"Description: "#]
    pub ownp3hel: Option<f32>,

    #[doc = r#"Title: PD 90 + OWN INTEREST SEC - CI"#]
    #[doc = r#"Description: "#]
    pub ownp9ci: Option<f32>,

    #[doc = r#"Title: PD 90 + OWN INTEREST SEC - CRCD"#]
    #[doc = r#"Description: "#]
    pub ownp9crd: Option<f32>,

    #[doc = r#"Title: PD 90 + OWN INTEREST SEC - HEL"#]
    #[doc = r#"Description: "#]
    pub ownp9hel: Option<f32>,

    #[doc = r#"Title: SEC. SECURE HELD IN RC-B - CI"#]
    #[doc = r#"Description: "#]
    pub ownscci: Option<f32>,

    #[doc = r#"Title: SEC. SECURE HELD IN RC-B - CRCD"#]
    #[doc = r#"Description: "#]
    pub ownsccrd: Option<f32>,

    #[doc = r#"Title: SEC. SECURE HELD IN RC-B - HEL"#]
    #[doc = r#"Description: "#]
    pub ownschel: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-AGRICULTURAL LNS"#]
    #[doc = r#"Description: "#]
    pub p3ag: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-AGRICULTURAL LNS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3agr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-AG LNS*SMALL BKS"#]
    #[doc = r#"Description: "#]
    pub p3agsm: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-AG LNS*SMALL BKS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3agsmr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-TOTAL ASSETS"#]
    #[doc = r#"Description: "#]
    pub p3asset: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D TOTAL ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3assetr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D AUTO LOANS"#]
    #[doc = r#"Description: "#]
    pub p3auto: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D AUTO LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3autor: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-C&I LOANS"#]
    #[doc = r#"Description: "#]
    pub p3ci: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-C&I LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3cir: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-C&I*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub p3cinus: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-C&I*NON-U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub p3cinusr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-CONSUMER LOANS"#]
    #[doc = r#"Description: "#]
    pub p3con: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-CONSUMER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3conr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-OTHER CONSUMER"#]
    #[doc = r#"Description: "#]
    pub p3conoth: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-OTHER CONSUMER RATIO"#]
    #[doc = r#"Description: "#]
    pub p3conothr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-CREDIT CARD PLANS"#]
    #[doc = r#"Description: "#]
    pub p3crcd: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-CREDIT CARD PLANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3crcdr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-DEP INST LOANS"#]
    #[doc = r#"Description: "#]
    pub p3dep: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-DEP INST LOANS"#]
    #[doc = r#"Description: "#]
    pub p3depr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-DEP INST*NON U.S."#]
    #[doc = r#"Description: "#]
    pub p3depnus: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-DEP INST*NON U.S."#]
    #[doc = r#"Description: "#]
    pub p3depnusr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-FOREIGN GOVT"#]
    #[doc = r#"Description: "#]
    pub p3fg: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-FOREIGN GOVT RATIO"#]
    #[doc = r#"Description: "#]
    pub p3fgr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub p3gty: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub p3gtyr: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D-REBOOKED GNMA LNS"#]
    #[doc = r#"Description: "#]
    pub p3gtygnm: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D-REBOOKED GNMA LNS"#]
    #[doc = r#"Description: "#]
    pub p3gtygnmr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-PART GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub p3gtypar: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-PART GTY LN&LS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3gtyparr: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D AG LOANS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lag: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D AG LOANS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lagr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D C&I LNS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lci: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D C&I LNS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lcir: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D CONSUMER -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lcon: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D CONSUMER -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lconr: Option<f32>,

    #[doc = r#"Title: 30-89 P/D PROTECT (GTY)-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lgty: Option<f32>,

    #[doc = r#"Title: 30-89 P/D PROTECT (GTY)-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lgtyr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-L&L HELD FOR SALE"#]
    #[doc = r#"Description: "#]
    pub p3lnsale: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-L&L HELD FOR SALE RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lnsaler: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D OTH LNS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3loth: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D OTH LNS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lothr: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RE FARM-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lreag: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RE FARM-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lreagr: Option<f32>,

    #[doc = r#"Title: 30-89 P/D CONSTRUCTION -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lrecon: Option<f32>,

    #[doc = r#"Title: 30-89 P/D CONSTRUCTION -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lreconr: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D MULTIFAM -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lremul: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D MULTIFAM -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lremulr: Option<f32>,

    #[doc = r#"Title: 30-89 P/D NONFRM NONRS -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lrenrs: Option<f32>,

    #[doc = r#"Title: 30-89 P/D NONFRM NONRS -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lrenrsr: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D 1-4 FAMILY -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3lreres: Option<f32>,

    #[doc = r#"Title: 30-89 P/D 1-4 FAMILY -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lreresr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-LEASES"#]
    #[doc = r#"Description: "#]
    pub p3ls: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-LEASES RATIO"#]
    #[doc = r#"Description: "#]
    pub p3lsr: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D TOTAL LOANS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p3ltot: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-TOTAL LOANS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p3ltotr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-ALL OTHER LOANS"#]
    #[doc = r#"Description: "#]
    pub p3othln: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-ALL OTHER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3othlnr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-REAL ESTATE LOANS"#]
    #[doc = r#"Description: "#]
    pub p3re: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-REAL ESTATE LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3rer: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*FARMLAND"#]
    #[doc = r#"Description: "#]
    pub p3reag: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*FARMLAND"#]
    #[doc = r#"Description: "#]
    pub p3reagr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D 1-4 FAM CONSTR LN"#]
    #[doc = r#"Description: "#]
    pub p3recnfm: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D 1-4 FAM CONSTR LN"#]
    #[doc = r#"Description: "#]
    pub p3recnfmr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D OTH CONSTR & LAND"#]
    #[doc = r#"Description: "#]
    pub p3recnot: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D OTH CONSTR & LAND"#]
    #[doc = r#"Description: "#]
    pub p3recnotr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub p3recons: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub p3reconsr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*FOREIGN"#]
    #[doc = r#"Description: "#]
    pub p3refor: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*FOREIGN RATIO"#]
    #[doc = r#"Description: "#]
    pub p3reforr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 FAM LINES"#]
    #[doc = r#"Description: "#]
    pub p3reloc: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 FAM LINES RATIO"#]
    #[doc = r#"Description: "#]
    pub p3relocr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*MULTIFAMILY"#]
    #[doc = r#"Description: "#]
    pub p3remult: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*MULTIFAMILY"#]
    #[doc = r#"Description: "#]
    pub p3remultr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*NONFARM NONRES"#]
    #[doc = r#"Description: "#]
    pub p3renres: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*NONFARM NONRES"#]
    #[doc = r#"Description: "#]
    pub p3renresr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D OTH NONFRM NONRES"#]
    #[doc = r#"Description: "#]
    pub p3renrot: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D OTH NONFRM NONRES"#]
    #[doc = r#"Description: "#]
    pub p3renrotr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D 0WN-OCC NONF NONRS"#]
    #[doc = r#"Description: "#]
    pub p3renrow: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D OWN-OCC NONF NONRS RATIO"#]
    #[doc = r#"Description: "#]
    pub p3renrowr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub p3renus: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub p3renusr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub p3reres: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub p3reresr: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 JN LIEN"#]
    #[doc = r#"Description: "#]
    pub p3rersf2: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 JN LIEN RATIO"#]
    #[doc = r#"Description: "#]
    pub p3rersf2r: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 IST LIEN"#]
    #[doc = r#"Description: "#]
    pub p3rersfm: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-RE*1-4 IST LIEN RATIO"#]
    #[doc = r#"Description: "#]
    pub p3rersfmr: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RESTRUCT C&I LN"#]
    #[doc = r#"Description: "#]
    pub p3rsci: Option<f32>,

    #[doc = r#"Title: 30-89 P/D RESTRUCT CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub p3rscons: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RESTR LN- 1-4 FAM"#]
    #[doc = r#"Description: "#]
    pub p3rslnfm: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RESTR LN- 1-4 FAM RATIO"#]
    #[doc = r#"Description: "#]
    pub p3rslnfmr: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D RESTR LN EXCL1-4 FM"#]
    #[doc = r#"Description: "#]
    pub p3rslnls: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D RESTR LN EXCL1-4 FM RATIO"#]
    #[doc = r#"Description: "#]
    pub p3rslnlsr: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RESTR LN- TOTAL"#]
    #[doc = r#"Description: "#]
    pub p3rslnlt: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RESTR LN- TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub p3rslnltr: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D RESTRUCT MULTIFAM"#]
    #[doc = r#"Description: "#]
    pub p3rsmult: Option<f32>,

    #[doc = r#"Title: 30-89 DAY P/D RESTRUCT NFNR LN"#]
    #[doc = r#"Description: "#]
    pub p3rsnres: Option<f32>,

    #[doc = r#"Title: 30-89 D P/D RESTRUCT ALL OTH LN"#]
    #[doc = r#"Description: "#]
    pub p3rsoth: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-DEBT SECURITIES"#]
    #[doc = r#"Description: "#]
    pub p3scdebt: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-DEBT SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub p3scdebtr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-AGRICULTURAL LNS"#]
    #[doc = r#"Description: "#]
    pub p9ag: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-AGRICULTURAL LNS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9agr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-AG LNS*SMALL BKS"#]
    #[doc = r#"Description: "#]
    pub p9agsm: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-AG LNS*SMALL BKS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9agsmr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-TOTAL ASSETS"#]
    #[doc = r#"Description: "#]
    pub p9asset: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-TOTAL ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9assetr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D AUTO LOANS"#]
    #[doc = r#"Description: "#]
    pub p9auto: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D AUTO LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9autor: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-C&I LOANS"#]
    #[doc = r#"Description: "#]
    pub p9ci: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-C&I LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9cir: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-C&I*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub p9cinus: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-C&I*NON-U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub p9cinusr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-CONSUMER LOANS"#]
    #[doc = r#"Description: "#]
    pub p9con: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-CONSUMER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9conr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-OTHER CONSUMER"#]
    #[doc = r#"Description: "#]
    pub p9conoth: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-OTHER CONSUMER RATIO"#]
    #[doc = r#"Description: "#]
    pub p9conothr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-CREDIT CARD PLANS"#]
    #[doc = r#"Description: "#]
    pub p9crcd: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-CREDIT CARD PLANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9crcdr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-DEP INST LOANS"#]
    #[doc = r#"Description: "#]
    pub p9dep: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-DEP INST LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9depr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-DEP INST*NON U.S."#]
    #[doc = r#"Description: "#]
    pub p9depnus: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-DEP INST*NON U.S. RATIO"#]
    #[doc = r#"Description: "#]
    pub p9depnusr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-FOREIGN GOVT"#]
    #[doc = r#"Description: "#]
    pub p9fg: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-FOREIGN GOVT RATIO"#]
    #[doc = r#"Description: "#]
    pub p9fgr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub p9gty: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub p9gtyr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-REBOOKED GNMA LNS"#]
    #[doc = r#"Description: "#]
    pub p9gtygnm: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D-REBOOKED GNMA LNS"#]
    #[doc = r#"Description: "#]
    pub p9gtygnmr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-PART GTY LN&LS"#]
    #[doc = r#"Description: "#]
    pub p9gtypar: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-PART GTY LN&LS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9gtyparr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D AG LOANS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lag: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D AG LOANS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lagr: Option<f32>,

    #[doc = r#"Title: 90+DAYS P/D C&I LNS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lci: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D C&I LNS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lcir: Option<f32>,

    #[doc = r#"Title: 90+ D P/D CONSUMER LN - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lcon: Option<f32>,

    #[doc = r#"Title: 90+ D P/D CONSUMER LN - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lconr: Option<f32>,

    #[doc = r#"Title: 90+ D P/D PROTECT (GTY)-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lgty: Option<f32>,

    #[doc = r#"Title: 90+ D P/D PROTECT (GTY)-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lgtyr: Option<f32>,

    #[doc = r#"Title: 90 DAYS P/D-L&L HELD FOR SALE"#]
    #[doc = r#"Description: "#]
    pub p9lnsale: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-L&L HELD FOR SALE RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lnsaler: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D OTHER LNS-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9loth: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D OTHER LNS-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lothr: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RE FARM-LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lreag: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RE FARM-LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lreagr: Option<f32>,

    #[doc = r#"Title: 90+ D P/D CONSTRUCTION -LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lrecon: Option<f32>,

    #[doc = r#"Title: 90+ D P/D CONSTRUCTION -LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lreconr: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D MULTIFAM - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lremul: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D MULTIFAM - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lremulr: Option<f32>,

    #[doc = r#"Title: 90+ D P/D NFNR - LOSS SHARE"#]
    #[doc = r#"Description: "#]
    pub p9lrenrs: Option<f32>,

    #[doc = r#"Title: 90+ D P/D NFNR - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lrenrsr: Option<f32>,

    #[doc = r#"Title: 90+ D P/D 1-4 FAMILY - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9lreres: Option<f32>,

    #[doc = r#"Title: 90+ D P/D 1-4 FAMILY - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lreresr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-LEASES"#]
    #[doc = r#"Description: "#]
    pub p9ls: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-LEASES RATIO"#]
    #[doc = r#"Description: "#]
    pub p9lsr: Option<f32>,

    #[doc = r#"Title: 90+ D P/D TOTAL LOANS - LOSS SH"#]
    #[doc = r#"Description: "#]
    pub p9ltot: Option<f32>,

    #[doc = r#"Title: 90+ D P/D TOTAL LOANS - LOSS SH RATIO"#]
    #[doc = r#"Description: "#]
    pub p9ltotr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-ALL OTHER LOANS"#]
    #[doc = r#"Description: "#]
    pub p9othln: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-ALL OTHER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9othlnr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-REAL ESTATE LOANS"#]
    #[doc = r#"Description: "#]
    pub p9re: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-REAL ESTATE RATIO"#]
    #[doc = r#"Description: "#]
    pub p9rer: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*FARMLAND"#]
    #[doc = r#"Description: "#]
    pub p9reag: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*FARMLAND"#]
    #[doc = r#"Description: "#]
    pub p9reagr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D 1-4 FAM CONSTRUC LN"#]
    #[doc = r#"Description: "#]
    pub p9recnfm: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D 1-4 FAM CONSTRUC LN RATIO"#]
    #[doc = r#"Description: "#]
    pub p9recnfmr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D OTHER CONSTR & LAND"#]
    #[doc = r#"Description: "#]
    pub p9recnot: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D OTHER CONSTR & LAND RATIO"#]
    #[doc = r#"Description: "#]
    pub p9recnotr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub p9recons: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*CONSTRUCTION RATIO"#]
    #[doc = r#"Description: "#]
    pub p9reconsr: Option<f32>,

    #[doc = r#"Title: 90 + DAYS P/D-RE*FOREIGN"#]
    #[doc = r#"Description: "#]
    pub p9refor: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*FOREIGN RATIO"#]
    #[doc = r#"Description: "#]
    pub p9reforr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 FAM LINES"#]
    #[doc = r#"Description: "#]
    pub p9reloc: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 FAM LINES RATIO"#]
    #[doc = r#"Description: "#]
    pub p9relocr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*MULTIFAMILY"#]
    #[doc = r#"Description: "#]
    pub p9remult: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*MULTIFAMILY RATIO"#]
    #[doc = r#"Description: "#]
    pub p9remultr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*NONFARM NONRES"#]
    #[doc = r#"Description: "#]
    pub p9renres: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*NONFARM NONRES RATIO"#]
    #[doc = r#"Description: "#]
    pub p9renresr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D OTHER NONFRM NONRES"#]
    #[doc = r#"Description: "#]
    pub p9renrot: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D OTHER NONFRM NONRES RATIO"#]
    #[doc = r#"Description: "#]
    pub p9renrotr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D 0WN-OCC NONFR NONRS"#]
    #[doc = r#"Description: "#]
    pub p9renrow: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D OWN-OCC NONFR NONRS RATIO"#]
    #[doc = r#"Description: "#]
    pub p9renrowr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub p9renus: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*NON-U.S."#]
    #[doc = r#"Description: "#]
    pub p9renusr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub p9reres: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 FAMILY RATIO"#]
    #[doc = r#"Description: "#]
    pub p9reresr: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 JN LIEN"#]
    #[doc = r#"Description: "#]
    pub p9rersf2: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 JN LIEN RATIO"#]
    #[doc = r#"Description: "#]
    pub p9rersf2r: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 IST LIEN"#]
    #[doc = r#"Description: "#]
    pub p9rersfm: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-RE*1-4 IST LIEN RATIO"#]
    #[doc = r#"Description: "#]
    pub p9rersfmr: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RESTRUCT C&I LN"#]
    #[doc = r#"Description: "#]
    pub p9rsci: Option<f32>,

    #[doc = r#"Title: 90+ D P/D RESTRUCT CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub p9rscons: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D RESTR LN- 1-4 FAM"#]
    #[doc = r#"Description: "#]
    pub p9rslnfm: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D RESTR LN- 1-4 FAM RATIO"#]
    #[doc = r#"Description: "#]
    pub p9rslnfmr: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RESTRU LN EXCL 1-4 FM"#]
    #[doc = r#"Description: "#]
    pub p9rslnls: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RESTRU LN EXCL 1-4 FM RATIO"#]
    #[doc = r#"Description: "#]
    pub p9rslnlsr: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RESTR LN- TOTAL"#]
    #[doc = r#"Description: "#]
    pub p9rslnlt: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RESTR LN- TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub p9rslnltr: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RESTRUCT MULTIFAM"#]
    #[doc = r#"Description: "#]
    pub p9rsmult: Option<f32>,

    #[doc = r#"Title: 90+ DAY P/D RESTRUCT NFNR LN"#]
    #[doc = r#"Description: "#]
    pub p9rsnres: Option<f32>,

    #[doc = r#"Title: 90+ D P/D RESTRUCT ALL OTH LN"#]
    #[doc = r#"Description: "#]
    pub p9rsoth: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-DEBT SECURITIES"#]
    #[doc = r#"Description: "#]
    pub p9scdebt: Option<f32>,

    #[doc = r#"Title: 90+ DAYS P/D-DEBT SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub p9scdebtr: Option<f32>,

    #[doc = r#"Title: PARTICIPATIONS ACQUIRED"#]
    #[doc = r#"Description: "#]
    pub partacqu: Option<f32>,

    #[doc = r#"Title: PARTICIPATIONS CONVEYED"#]
    #[doc = r#"Description: "#]
    pub partconv: Option<f32>,

    #[doc = r#"Title: PARTICIPATIONS CONVEYED RATIO"#]
    #[doc = r#"Description: "#]
    pub partconvr: Option<f32>,

    #[doc = r#"Title: ALLOWANCE FOR L&L IN TIER 2"#]
    #[doc = r#"Description: "#]
    pub rb2lnres: Option<f32>,

    #[doc = r#"Title: ALLOWANCE FOR L&L IN TIER 2 RATIO"#]
    #[doc = r#"Description: "#]
    pub rb2lnresr: Option<f32>,

    #[doc = r#"Title: RBC-TOTAL-PCA"#]
    #[doc = r#"Description: "#]
    pub rbc: Option<f32>,

    #[doc = r#"Title: TIER 1 RBC-PCA"#]
    #[doc = r#"Description: "#]
    pub rbct1: Option<f32>,

    #[doc = r#"Title: RBC-TIER2-PCA"#]
    #[doc = r#"Description: "#]
    pub rbct2: Option<f32>,

    #[doc = r#"Title: RBC-TIER2-PCA RATIO"#]
    #[doc = r#"Description: "#]
    pub rbct2r: Option<f32>,

    #[doc = r#"Title: RC-R COMMON EQ TIER 1 CAPITAL"#]
    #[doc = r#"Description: "#]
    pub rbct1c: Option<f32>,

    #[doc = r#"Title: COMMON EQUITY TIER 1 RATIO"#]
    #[doc = r#"Description: "#]
    pub rbct1cer: Option<f32>,

    #[doc = r#"Title: TIER 1 RBC ADJUSTED LLR - PCA"#]
    #[doc = r#"Description: "#]
    pub rbct1j: Option<f32>,

    #[doc = r#"Title: TIER 1 RBC ADJUSTED LLR - PCA RATIO"#]
    #[doc = r#"Description: "#]
    pub rbct1jr: Option<f32>,

    #[doc = r#"Title: LEVERAGE RATIO-PCA"#]
    #[doc = r#"Description: "#]
    pub rbc1aaj: Option<f32>,

    #[doc = r#"Title: TIER 1 RBC RATIO-PCA"#]
    #[doc = r#"Description: "#]
    pub rbc1rwaj: Option<f32>,

    #[doc = r#"Title: TOTAL RBC RATIO-PCA"#]
    #[doc = r#"Description: "#]
    pub rbcrwaj: Option<f32>,

    #[doc = r#"Title: REPURCHASE AGREEMENT-FOR"#]
    #[doc = r#"Description: "#]
    pub repopurf: Option<f32>,

    #[doc = r#"Title: REVERSE REPURCHASE AGREEMENT-FOR"#]
    #[doc = r#"Description: "#]
    pub reposldf: Option<f32>,

    #[doc = r#"Title: RETAINED EARNINGS/AVG BK EQUITY"#]
    #[doc = r#"Description: "#]
    pub roeinjr: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LN - C&I"#]
    #[doc = r#"Description: "#]
    pub rsci: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LN - CONSTRUCTION"#]
    #[doc = r#"Description: "#]
    pub rscons: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LN EXCL 1-4 FM"#]
    #[doc = r#"Description: "#]
    pub rslnls: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LN EXCL 1-4 FM RATIO"#]
    #[doc = r#"Description: "#]
    pub rslnlsr: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LOANS - TOTAL"#]
    #[doc = r#"Description: "#]
    pub rslnltot: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LOANS - TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub rslnltotr: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LOANS - 1-4 FAMILY"#]
    #[doc = r#"Description: "#]
    pub rslnrefm: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LOANS - 1-4 FAMILY RATIO"#]
    #[doc = r#"Description: "#]
    pub rslnrefmr: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LN - MULTIFAMILY"#]
    #[doc = r#"Description: "#]
    pub rsmult: Option<f32>,

    #[doc = r#"Title: RESTRUCT LN - NONFARM NONRES"#]
    #[doc = r#"Description: "#]
    pub rsnres: Option<f32>,

    #[doc = r#"Title: RESTRUCTURED LN - ALL OTHER"#]
    #[doc = r#"Description: "#]
    pub rsother: Option<f32>,

    #[doc = r#"Title: FEDERAL RESERVE ID NUMBER"#]
    #[doc = r#"Description: "#]
    pub rssdid: Option<f32>,

    #[doc = r#"Title: INTEREST RATE-TOTAL CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub rt: Option<f32>,

    #[doc = r#"Title: INT RATE-FUTURES & FORWARD CONTR"#]
    #[doc = r#"Description: "#]
    pub rtffc: Option<f32>,

    #[doc = r#"Title: INT RATE-SWAPS"#]
    #[doc = r#"Description: "#]
    pub rtnvs: Option<f32>,

    #[doc = r#"Title: INT RATE-PUR OPTION CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub rtpoc: Option<f32>,

    #[doc = r#"Title: INT RATE-WRITTEN OPTION CONTRACT"#]
    #[doc = r#"Description: "#]
    pub rtwoc: Option<f32>,

    #[doc = r#"Title: RWA-ADJUST-PCA-T1 & CET1 RATIO"#]
    #[doc = r#"Description: "#]
    pub rwaj: Option<f32>,

    #[doc = r#"Title: RWA-ADJUSTED-PCA-TOTAL RBC RAT"#]
    #[doc = r#"Description: "#]
    pub rwajt: Option<f32>,

    #[doc = r#"Title: RWA-ADJUSTED-PCA-TOTAL RBC RAT RATIO"#]
    #[doc = r#"Description: "#]
    pub rwajtr: Option<f32>,

    #[doc = r#"Title: ABS-TOTAL-B/S"#]
    #[doc = r#"Description: "#]
    pub scabs: Option<f32>,

    #[doc = r#"Title: ABS-TOTAL-B/S RATIO"#]
    #[doc = r#"Description: "#]
    pub scabsr: Option<f32>,

    #[doc = r#"Title: SECURITIES-AF"#]
    #[doc = r#"Description: "#]
    pub scaf: Option<f32>,

    #[doc = r#"Title: SECURITIES-AF RATIO"#]
    #[doc = r#"Description: "#]
    pub scafr: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY ALL OTH"#]
    #[doc = r#"Description: "#]
    pub scaot: Option<f32>,

    #[doc = r#"Title: COMMERCIAL MBS - TOTAL"#]
    #[doc = r#"Description: "#]
    pub sccmmb: Option<f32>,

    #[doc = r#"Title: OTHER COMMERCIAL MBS-GOVT"#]
    #[doc = r#"Description: "#]
    pub sccmog: Option<f32>,

    #[doc = r#"Title: OTHER COMMERCIAL MBS-GOVT RATIO"#]
    #[doc = r#"Description: "#]
    pub sccmogr: Option<f32>,

    #[doc = r#"Title: OTHER COMMERCIAL MBS"#]
    #[doc = r#"Description: "#]
    pub sccmot: Option<f32>,

    #[doc = r#"Title: OTHER COMMERCIAL MBS RATIO"#]
    #[doc = r#"Description: "#]
    pub sccmotr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL MBS PASS-THROUGH"#]
    #[doc = r#"Description: "#]
    pub sccmpt: Option<f32>,

    #[doc = r#"Title: COMMERCIAL MBS PASS-THROUGH RATIO"#]
    #[doc = r#"Description: "#]
    pub sccmptr: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY COLLATERAL MTG-RES"#]
    #[doc = r#"Description: "#]
    pub sccol: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY COLLATERAL MTG-RES RATIO"#]
    #[doc = r#"Description: "#]
    pub sccolr: Option<f32>,

    #[doc = r#"Title: COMM MBS PASS-THRU-GOVT"#]
    #[doc = r#"Description: "#]
    pub sccptg: Option<f32>,

    #[doc = r#"Title: COMM MBS PASS-THRU-GOVT RATIO"#]
    #[doc = r#"Description: "#]
    pub sccptgr: Option<f32>,

    #[doc = r#"Title: EQ SEC READILY DET FV"#]
    #[doc = r#"Description: "#]
    pub sceqfv: Option<f32>,

    #[doc = r#"Title: EQ SEC READILY DET FV RATIO"#]
    #[doc = r#"Description: "#]
    pub sceqfvr: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY ISSUED*FNMA-RES"#]
    #[doc = r#"Description: "#]
    pub scfmn: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY ISSUED*FNMA-RES RATIO"#]
    #[doc = r#"Description: "#]
    pub scfmnr: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY GTY BY GNMA"#]
    #[doc = r#"Description: "#]
    pub scgnm: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY GTY BY GNMA RATIO"#]
    #[doc = r#"Description: "#]
    pub scgnmr: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY ISSUED OR GTY-RES"#]
    #[doc = r#"Description: "#]
    pub scgty: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY ISSUED OR GTY-RES RATIO"#]
    #[doc = r#"Description: "#]
    pub scgtyr: Option<f32>,

    #[doc = r#"Title: SECURITIES-HA"#]
    #[doc = r#"Description: "#]
    pub scha: Option<f32>,

    #[doc = r#"Title: SECURITIES-HA RATIO"#]
    #[doc = r#"Description: "#]
    pub schar: Option<f32>,

    #[doc = r#"Title: LESS ALLOW FOR CREDIT LOSSES ON HELD TO MATURITY DEBT SECURITIES"#]
    #[doc = r#"Description: "#]
    pub schtmres: Option<f32>,

    #[doc = r#"Title: LESS ALLOW FOR CREDIT LOSSES ON HELD TO MATURITY DEBT SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub schtmresr: Option<f32>,

    #[doc = r#"Title: SECURITIES LENT"#]
    #[doc = r#"Description: "#]
    pub sclent: Option<f32>,

    #[doc = r#"Title: SECURITIES LENT RATIO"#]
    #[doc = r#"Description: "#]
    pub sclentr: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 1-3 YEARS"#]
    #[doc = r#"Description: "#]
    pub scnm1t3: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 1-3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub scnm1t3r: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC*3 MONS OR LESS"#]
    #[doc = r#"Description: "#]
    pub scnm3les: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC*3 MONS OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub scnm3lesr: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 3-5 YEARS"#]
    #[doc = r#"Description: "#]
    pub scnm3t5: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 3-5 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub scnm3t5r: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 3-12 MONTHS"#]
    #[doc = r#"Description: "#]
    pub scnm3t12: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 3-12 MONTHS RATIO"#]
    #[doc = r#"Description: "#]
    pub scnm3t12r: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 5-15 YEARS"#]
    #[doc = r#"Description: "#]
    pub scnm5t15: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * 5-15 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub scnm5t15r: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * OVER 15 YEARS"#]
    #[doc = r#"Description: "#]
    pub scnmov15: Option<f32>,

    #[doc = r#"Title: NONMTG DEBT SEC * OVER 15 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub scnmov15r: Option<f32>,

    #[doc = r#"Title: OTH MORTGAGE SEC * 3 YR OR LESS"#]
    #[doc = r#"Description: "#]
    pub sco3yles: Option<f32>,

    #[doc = r#"Title: OTH MORTGAGE SEC * 3 YR OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub sco3ylesr: Option<f32>,

    #[doc = r#"Title: Fixed and floating rate debt securities (included above) with remaining maturity of one year or less"#]
    #[doc = r#"Description: "#]
    pub sc1les: Option<f32>,

    #[doc = r#"Title: Fixed and floating rate debt securities (included above) with remaining maturity of one year or less ratio"#]
    #[doc = r#"Description: "#]
    pub sc1lesr: Option<f32>,

    #[doc = r#"Title: OTH DOM DEBT*ALL OTHER"#]
    #[doc = r#"Description: "#]
    pub scodot: Option<f32>,

    #[doc = r#"Title: OTH DOM DEBT*ALL OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub scodotr: Option<f32>,

    #[doc = r#"Title: CMO PRIV ISSUED"#]
    #[doc = r#"Description: "#]
    pub scodpi: Option<f32>,

    #[doc = r#"Title: CMO PRIV ISSUED RATIO"#]
    #[doc = r#"Description: "#]
    pub scodpir: Option<f32>,

    #[doc = r#"Title: OTH MORTGAGE SEC * OVER 3 YRS"#]
    #[doc = r#"Description: "#]
    pub scoov3y: Option<f32>,

    #[doc = r#"Title: OTH MORTGAGE SEC * OVER 3 YRS RATIO"#]
    #[doc = r#"Description: "#]
    pub scoov3yr: Option<f32>,

    #[doc = r#"Title: PLEDGED SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scpledge: Option<f32>,

    #[doc = r#"Title: PLEDGED SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scpledger: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 1-3 YEARS"#]
    #[doc = r#"Description: "#]
    pub scpt1t3: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 1-3 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub scpt1t3r: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC*3 MON OR LESS"#]
    #[doc = r#"Description: "#]
    pub scpt3les: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC*3 MON OR LESS RATIO"#]
    #[doc = r#"Description: "#]
    pub scpt3lesr: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 3-5 YEARS"#]
    #[doc = r#"Description: "#]
    pub scpt3t5: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 3-5 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub scpt3t5r: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 3-12 MONTHS"#]
    #[doc = r#"Description: "#]
    pub scpt3t12: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 3-12 MONTHS RATIO"#]
    #[doc = r#"Description: "#]
    pub scpt3t12r: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 5-15 YEARS"#]
    #[doc = r#"Description: "#]
    pub scpt5t15: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * 5-15 YEARS RATIO"#]
    #[doc = r#"Description: "#]
    pub scpt5t15r: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * OVER 15 YRS"#]
    #[doc = r#"Description: "#]
    pub scptov15: Option<f32>,

    #[doc = r#"Title: MTG PASS-THRU SEC * OVER 15 YRS RATIO"#]
    #[doc = r#"Description: "#]
    pub scptov15r: Option<f32>,

    #[doc = r#"Title: DEBT SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scrdebt: Option<f32>,

    #[doc = r#"Title: DEBT SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scrdebtr: Option<f32>,

    #[doc = r#"Title: STRUCTURED FIN PROD - TOTAL"#]
    #[doc = r#"Description: "#]
    pub scsfp: Option<f32>,

    #[doc = r#"Title: STRUCTURED FIN PROD - TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub scsfpr: Option<f32>,

    #[doc = r#"Title: STRUCTURED NOTES AMORTIZED COST"#]
    #[doc = r#"Description: "#]
    pub scsnhaa: Option<f32>,

    #[doc = r#"Title: STRUCTURED NOTES AMORTIZED COST RATIO"#]
    #[doc = r#"Description: "#]
    pub scsnhaar: Option<f32>,

    #[doc = r#"Title: STRUCTURED NOTES-FAIR VALUE"#]
    #[doc = r#"Description: "#]
    pub scsnhaf: Option<f32>,

    #[doc = r#"Title: STRUCTURED NOTES-FAIR VALUE RATIO"#]
    #[doc = r#"Description: "#]
    pub scsnhafr: Option<f32>,

    #[doc = r#"Title: U.S. AGENCY GOVT SPONSORED"#]
    #[doc = r#"Description: "#]
    pub scspn: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-AUTO"#]
    #[doc = r#"Description: "#]
    pub sz30auto: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-AUTO RATIO"#]
    #[doc = r#"Description: "#]
    pub sz30autor: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-CI"#]
    #[doc = r#"Description: "#]
    pub sz30ci: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-CI RATIO"#]
    #[doc = r#"Description: "#]
    pub sz30cir: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-CON"#]
    #[doc = r#"Description: "#]
    pub sz30con: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-CON RATIO"#]
    #[doc = r#"Description: "#]
    pub sz30conr: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-CRCD"#]
    #[doc = r#"Description: "#]
    pub sz30crcd: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-CRCD RATIO"#]
    #[doc = r#"Description: "#]
    pub sz30crcdr: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-HEL"#]
    #[doc = r#"Description: "#]
    pub sz30hel: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-HEL RATIO"#]
    #[doc = r#"Description: "#]
    pub sz30helr: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-OTH"#]
    #[doc = r#"Description: "#]
    pub sz30oth: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION-OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub sz30othr: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION -RES"#]
    #[doc = r#"Description: "#]
    pub sz30res: Option<f32>,

    #[doc = r#"Title: 30-89 PD LN-SECURITIZATION -RES RATIO"#]
    #[doc = r#"Description: "#]
    pub sz30resr: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-AUTO"#]
    #[doc = r#"Description: "#]
    pub sz90auto: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-AUTO RATIO"#]
    #[doc = r#"Description: "#]
    pub sz90autor: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-CI"#]
    #[doc = r#"Description: "#]
    pub sz90ci: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-CI RATIO"#]
    #[doc = r#"Description: "#]
    pub sz90cir: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-CON"#]
    #[doc = r#"Description: "#]
    pub sz90con: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-CON RATIO"#]
    #[doc = r#"Description: "#]
    pub sz90conr: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-CRCD"#]
    #[doc = r#"Description: "#]
    pub sz90crcd: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-CRCD RATIO"#]
    #[doc = r#"Description: "#]
    pub sz90crcdr: Option<f32>,

    #[doc = r#"Title: 90+ PD LN-SECURITIZATION-HEL"#]
    #[doc = r#"Description: "#]
    pub sz90hel: Option<f32>,

    #[doc = r#"Title: 90+ PD LN-SECURITIZATION-HEL RATIO"#]
    #[doc = r#"Description: "#]
    pub sz90helr: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-OTH"#]
    #[doc = r#"Description: "#]
    pub sz90oth: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub sz90othr: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-RES"#]
    #[doc = r#"Description: "#]
    pub sz90res: Option<f32>,

    #[doc = r#"Title: 90 + PD LN-SECURITIZATION-RES RATION"#]
    #[doc = r#"Description: "#]
    pub sz90resr: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-AUTO"#]
    #[doc = r#"Description: "#]
    pub szcrauto: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-AUTO"#]
    #[doc = r#"Description: "#]
    pub szcrautor: Option<f32>,

    #[doc = r#"Title: OUTSTDG CC FEES IN SECURITZD CC"#]
    #[doc = r#"Description: "#]
    pub szcrcdfe: Option<f32>,

    #[doc = r#"Title: OUTSTDG CC FEES IN SECURITZD CC RATIO"#]
    #[doc = r#"Description: "#]
    pub szcrcdfer: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-CI"#]
    #[doc = r#"Description: "#]
    pub szcrci: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-CI RATIO"#]
    #[doc = r#"Description: "#]
    pub szcrcir: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-CON"#]
    #[doc = r#"Description: "#]
    pub szcrcon: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-CON RATIO"#]
    #[doc = r#"Description: "#]
    pub szcrconr: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION - CRCD"#]
    #[doc = r#"Description: "#]
    pub szcrcrcd: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION - CRCD RATIO"#]
    #[doc = r#"Description: "#]
    pub szcrcrcdr: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD-HEL"#]
    #[doc = r#"Description: "#]
    pub szcrhel: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD-HEL RATIO"#]
    #[doc = r#"Description: "#]
    pub szcrhelr: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-"#]
    #[doc = r#"Description: "#]
    pub szcroth: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION- RATIO"#]
    #[doc = r#"Description: "#]
    pub szcrothr: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-RES"#]
    #[doc = r#"Description: "#]
    pub szcrres: Option<f32>,

    #[doc = r#"Title: REC ASSET SECURITIZATION-RES RATIO"#]
    #[doc = r#"Description: "#]
    pub szcrresr: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-AUTO"#]
    #[doc = r#"Description: "#]
    pub szdrauto: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-AUTO RATIO"#]
    #[doc = r#"Description: "#]
    pub szdrautor: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-CI"#]
    #[doc = r#"Description: "#]
    pub szdrci: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-CI RATIO"#]
    #[doc = r#"Description: "#]
    pub szdrcir: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-CON"#]
    #[doc = r#"Description: "#]
    pub szdrcon: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-CON RATIO"#]
    #[doc = r#"Description: "#]
    pub szdrconr: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-CRCD"#]
    #[doc = r#"Description: "#]
    pub szdrcrcd: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-CRCD RATIO"#]
    #[doc = r#"Description: "#]
    pub szdrcrcdr: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-HEL"#]
    #[doc = r#"Description: "#]
    pub szdrhel: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-HEL RATIO"#]
    #[doc = r#"Description: "#]
    pub szdrhelr: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-OTH"#]
    #[doc = r#"Description: "#]
    pub szdroth: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub szdrothr: Option<f32>,

    #[doc = r#"Title: C/O ON ASSET SECURITIZATION-RES"#]
    #[doc = r#"Description: "#]
    pub szdrres: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - AUTO"#]
    #[doc = r#"Description: "#]
    pub szislaut: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - AUTO RATIO"#]
    #[doc = r#"Description: "#]
    pub szislautr: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - CRCD"#]
    #[doc = r#"Description: "#]
    pub szislccd: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - CRCD RATIO"#]
    #[doc = r#"Description: "#]
    pub szislccdr: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN -CI"#]
    #[doc = r#"Description: "#]
    pub szislci: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN -CI RATIO"#]
    #[doc = r#"Description: "#]
    pub szislcir: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - CON"#]
    #[doc = r#"Description: "#]
    pub szislcon: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - CON RATIO"#]
    #[doc = r#"Description: "#]
    pub szislconr: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - HEL"#]
    #[doc = r#"Description: "#]
    pub szislhel: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN - HEL RATIO"#]
    #[doc = r#"Description: "#]
    pub szislhelr: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN -OTH"#]
    #[doc = r#"Description: "#]
    pub szisloth: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATN -OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub szislothr: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATION RES"#]
    #[doc = r#"Description: "#]
    pub szislres: Option<f32>,

    #[doc = r#"Title: CR EXP ON SECURITIZATION RES RATIO"#]
    #[doc = r#"Description: "#]
    pub szislresr: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - AUTO"#]
    #[doc = r#"Description: "#]
    pub szlauto: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - AUTO RATIO"#]
    #[doc = r#"Description: "#]
    pub szlautor: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - CI"#]
    #[doc = r#"Description: "#]
    pub szlnci: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - CI RATIO"#]
    #[doc = r#"Description: "#]
    pub szlncir: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - CONS"#]
    #[doc = r#"Description: "#]
    pub szlncon: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - CONS RATIO"#]
    #[doc = r#"Description: "#]
    pub szlnconr: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - CRCD"#]
    #[doc = r#"Description: "#]
    pub szlncrcd: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - CRCD RATIO"#]
    #[doc = r#"Description: "#]
    pub szlncrcdr: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - HEL"#]
    #[doc = r#"Description: "#]
    pub szlnhel: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - HEL RATIO"#]
    #[doc = r#"Description: "#]
    pub szlnhelr: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - OTH"#]
    #[doc = r#"Description: "#]
    pub szlnoth: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD - OTH RATIO"#]
    #[doc = r#"Description: "#]
    pub szlnothr: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD-RES"#]
    #[doc = r#"Description: "#]
    pub szlnres: Option<f32>,

    #[doc = r#"Title: RE PRIN SEC ASSET SOLD-RES RATIO"#]
    #[doc = r#"Description: "#]
    pub szlnresr: Option<f32>,

    #[doc = r#"Title: COMMITS FOR LIQUIDITY  - AUTO"#]
    #[doc = r#"Description: "#]
    pub szucauto: Option<f32>,

    #[doc = r#"Title: COMMITS FOR LIQUIDITY  - CI"#]
    #[doc = r#"Description: "#]
    pub szucci: Option<f32>,

    #[doc = r#"Title: COMMITS FOR LIQUIDITY  - CON"#]
    #[doc = r#"Description: "#]
    pub szuccon: Option<f32>,

    #[doc = r#"Title: COMMITS FOR LIQUIDITY  - CRCD"#]
    #[doc = r#"Description: "#]
    pub szuccrcd: Option<f32>,

    #[doc = r#"Title: COMMITS FOR LIQUIDITY - HEL"#]
    #[doc = r#"Description: "#]
    pub szuchel: Option<f32>,

    #[doc = r#"Title: COMMITS FOR LIQUIDITY  - OTH"#]
    #[doc = r#"Description: "#]
    pub szucoth: Option<f32>,

    #[doc = r#"Title: COMMITS FOR LIQUIDITY  - RES"#]
    #[doc = r#"Description: "#]
    pub szucres: Option<f32>,

    #[doc = r#"Title: CORP TRUST-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tcama: Option<f32>,

    #[doc = r#"Title: CORP TRUST-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tcamanum: Option<f32>,

    #[doc = r#"Title: CORP TRUST-NON-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tcanma: Option<f32>,

    #[doc = r#"Title: CORP TRUST-NON-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tcanmnum: Option<f32>,

    #[doc = r#"Title: CORP TRUST-TRUSTEESHIPS-NUM"#]
    #[doc = r#"Description: "#]
    pub tcanum: Option<f32>,

    #[doc = r#"Title: CORP & MUNI-TRUSTEE-DEFAULT-NUM"#]
    #[doc = r#"Description: "#]
    pub tcanumd: Option<f32>,

    #[doc = r#"Title: CORP TRUST-TRUSTEESHIPS-AMT"#]
    #[doc = r#"Description: "#]
    pub tcapao: Option<f32>,

    #[doc = r#"Title: CORP & MUNI-TRUSTEE-DEFAULT-AMT"#]
    #[doc = r#"Description: "#]
    pub tcapaod: Option<f32>,

    #[doc = r#"Title: CORP TRUST-TRANSFER-NUM"#]
    #[doc = r#"Description: "#]
    pub tcatnum: Option<f32>,

    #[doc = r#"Title: CIFS -DOM EQUITY-AMT"#]
    #[doc = r#"Description: "#]
    pub tcdemv: Option<f32>,

    #[doc = r#"Title: CIFS -DOM EQUITY-NUM"#]
    #[doc = r#"Description: "#]
    pub tcdenum: Option<f32>,

    #[doc = r#"Title: CIFS -INTL/GLOBAL-EQ-AMT"#]
    #[doc = r#"Description: "#]
    pub tciemv: Option<f32>,

    #[doc = r#"Title: CIFS -INTL/GLOBAL-EQ-NUM"#]
    #[doc = r#"Description: "#]
    pub tcienum: Option<f32>,

    #[doc = r#"Title: CIFS-MUNICIPAL BOND-AMT"#]
    #[doc = r#"Description: "#]
    pub tcmbmv: Option<f32>,

    #[doc = r#"Title: CIFS-MUNICIPAL BOND-NUM"#]
    #[doc = r#"Description: "#]
    pub tcmbnum: Option<f32>,

    #[doc = r#"Title: CIFS -STOCK/BOND-AMT"#]
    #[doc = r#"Description: "#]
    pub tcsbmv: Option<f32>,

    #[doc = r#"Title: CIFS -STOCK/BOND-NUM"#]
    #[doc = r#"Description: "#]
    pub tcsbnum: Option<f32>,

    #[doc = r#"Title: CUST AND SAFE ACCT-NON-MAN-AMT"#]
    #[doc = r#"Description: "#]
    pub tcsnma: Option<f32>,

    #[doc = r#"Title: CUST AND SAFE ACCT-NON-MAN-NUM"#]
    #[doc = r#"Description: "#]
    pub tcsnmnum: Option<f32>,

    #[doc = r#"Title: CIFS-SPECIALTY/OTHER-AMT"#]
    #[doc = r#"Description: "#]
    pub tcsomv: Option<f32>,

    #[doc = r#"Title: CIFS-SPECIALTY/OTHER-NUM"#]
    #[doc = r#"Description: "#]
    pub tcsonum: Option<f32>,

    #[doc = r#"Title: CIFS-SHORT TERM INV-AMT"#]
    #[doc = r#"Description: "#]
    pub tcstmv: Option<f32>,

    #[doc = r#"Title: CIFS-SHORT TERM INV-NUM"#]
    #[doc = r#"Description: "#]
    pub tcstnum: Option<f32>,

    #[doc = r#"Title: CIFS - TAXABLE BOND-AMT"#]
    #[doc = r#"Description: "#]
    pub tctbmv: Option<f32>,

    #[doc = r#"Title: CIFS - TAXABLE BOND-NUM"#]
    #[doc = r#"Description: "#]
    pub tctbnum: Option<f32>,

    #[doc = r#"Title: CIFS-TOTAL-AMT"#]
    #[doc = r#"Description: "#]
    pub tctotmv: Option<f32>,

    #[doc = r#"Title: CIFS-TOTAL-NUM"#]
    #[doc = r#"Description: "#]
    pub tctotnum: Option<f32>,

    #[doc = r#"Title: EMP BENE-DEF BENE-MANAGE-AMT"#]
    #[doc = r#"Description: "#]
    pub tebma: Option<f32>,

    #[doc = r#"Title: EMP BENE-DEF BENE-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tebmanum: Option<f32>,

    #[doc = r#"Title: EMP BENE-DEF BENE-NON-MAN-AMT"#]
    #[doc = r#"Description: "#]
    pub tebnma: Option<f32>,

    #[doc = r#"Title: EMP BENE-DEF BENE-NON-MAN-NUM"#]
    #[doc = r#"Description: "#]
    pub tebnmnum: Option<f32>,

    #[doc = r#"Title: EMP BENE-CONTRIB-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tecma: Option<f32>,

    #[doc = r#"Title: EMP BENE-CONTRI-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tecmanum: Option<f32>,

    #[doc = r#"Title: EMP BENE-CONTRI-NON-MAN-AMT"#]
    #[doc = r#"Description: "#]
    pub tecnma: Option<f32>,

    #[doc = r#"Title: EMP BENE-CONTRI-NON-MANAGE-NUM"#]
    #[doc = r#"Description: "#]
    pub tecnmnum: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - COM & PF STK"#]
    #[doc = r#"Description: "#]
    pub tecps: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - EQ MUT FUND"#]
    #[doc = r#"Description: "#]
    pub teeqf: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - INT BEARING"#]
    #[doc = r#"Description: "#]
    pub tei: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR-TOT MANAGE AST"#]
    #[doc = r#"Description: "#]
    pub tematot: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - MISC ASSET"#]
    #[doc = r#"Description: "#]
    pub temisc: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - MONEY MKT"#]
    #[doc = r#"Description: "#]
    pub temmf: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - NONINT BEAR"#]
    #[doc = r#"Description: "#]
    pub teni: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR-OTH NOTE & BND"#]
    #[doc = r#"Description: "#]
    pub teothb: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - OTH MUT FUND"#]
    #[doc = r#"Description: "#]
    pub teothf: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - REAL ESTATE"#]
    #[doc = r#"Description: "#]
    pub tere: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - RE MTG"#]
    #[doc = r#"Description: "#]
    pub teremtg: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - MUNI"#]
    #[doc = r#"Description: "#]
    pub tescmun: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR -U.S TREAS & OB"#]
    #[doc = r#"Description: "#]
    pub tescus: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - SHRT TERM OB"#]
    #[doc = r#"Description: "#]
    pub testo: Option<f32>,

    #[doc = r#"Title: EXPENSE FIDUCIARY - YTD"#]
    #[doc = r#"Description: "#]
    pub tetot: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - TRUST FUND"#]
    #[doc = r#"Description: "#]
    pub tetrf: Option<f32>,

    #[doc = r#"Title: EMP BEN & RET TR - UNREG FUNDS"#]
    #[doc = r#"Description: "#]
    pub teuf: Option<f32>,

    #[doc = r#"Title: FOUNDATION & ENDOW-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tfema: Option<f32>,

    #[doc = r#"Title: FOUNDATION & ENDOW-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tfemanum: Option<f32>,

    #[doc = r#"Title: FOUNDATION & END-NON-MANAGE-AMT"#]
    #[doc = r#"Description: "#]
    pub tfenma: Option<f32>,

    #[doc = r#"Title: FOUNDATION & END-NON-MANAGE-NUM"#]
    #[doc = r#"Description: "#]
    pub tfenmnum: Option<f32>,

    #[doc = r#"Title: GR.INC-CORP TRUST & AGENCY-YTD"#]
    #[doc = r#"Description: "#]
    pub tica: Option<f32>,

    #[doc = r#"Title: GR.INC-CUSTODY-YTD"#]
    #[doc = r#"Description: "#]
    pub tics: Option<f32>,

    #[doc = r#"Title: GR.INC-EMP. BENEFIT-BENEFIT-YTD"#]
    #[doc = r#"Description: "#]
    pub tieb: Option<f32>,

    #[doc = r#"Title: GR.INC-EMP. BENEFIT- CONTRI-YTD"#]
    #[doc = r#"Description: "#]
    pub tiec: Option<f32>,

    #[doc = r#"Title: GR. INC- FOUNDATION & ENDOW-YTD"#]
    #[doc = r#"Description: "#]
    pub tife: Option<f32>,

    #[doc = r#"Title: GR.INC - INVESTMENT AGCY - YTD"#]
    #[doc = r#"Description: "#]
    pub tima: Option<f32>,

    #[doc = r#"Title: INVESTMENT AGENCY-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub timma: Option<f32>,

    #[doc = r#"Title: INVESTMENT AGENCY-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub timmanum: Option<f32>,

    #[doc = r#"Title: INVESTMENT AGCY NON-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub timnma: Option<f32>,

    #[doc = r#"Title: INVESTMENT AGCY NON-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub timnmnum: Option<f32>,

    #[doc = r#"Title: INTRACOMPANY INC FIDUCIARY-YTD"#]
    #[doc = r#"Description: "#]
    pub tintra: Option<f32>,

    #[doc = r#"Title: GR.INC-OTHER FIDUCIARY-YTD"#]
    #[doc = r#"Description: "#]
    pub tiof: Option<f32>,

    #[doc = r#"Title: GR.INC-OTHER RETIREMENT -YTD"#]
    #[doc = r#"Description: "#]
    pub tior: Option<f32>,

    #[doc = r#"Title: GR.INC-PERSONAL & AG ACCTS-YTD"#]
    #[doc = r#"Description: "#]
    pub tip: Option<f32>,

    #[doc = r#"Title: GR.INC-RELATED SERV-YTD"#]
    #[doc = r#"Description: "#]
    pub tir: Option<f32>,

    #[doc = r#"Title: TOT FOREIGN OFF GROSS FIDUC-YTD"#]
    #[doc = r#"Description: "#]
    pub titotf: Option<f32>,

    #[doc = r#"Title: FIDUCIARY FGN OFF-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tmaf: Option<f32>,

    #[doc = r#"Title: FIDUCIARY FGN OFF-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tmafnum: Option<f32>,

    #[doc = r#"Title: ADVISED/SPONSORED MUT FND -AMT"#]
    #[doc = r#"Description: "#]
    pub tmasmf: Option<f32>,

    #[doc = r#"Title: ADVISED/SPONSORED MUTAL FND-NUM"#]
    #[doc = r#"Description: "#]
    pub tmasmfn: Option<f32>,

    #[doc = r#"Title: NET FIDUCIARY INCOME -YTD"#]
    #[doc = r#"Description: "#]
    pub tni: Option<f32>,

    #[doc = r#"Title: NET LOSS FROM FIDUCIARY-YTD"#]
    #[doc = r#"Description: "#]
    pub tnl: Option<f32>,

    #[doc = r#"Title: FIDUCIARY FGN OFF-NON-MAN-AMT"#]
    #[doc = r#"Description: "#]
    pub tnmaf: Option<f32>,

    #[doc = r#"Title: FIDUCIARY FGN OFF-NON-MAN-NUM"#]
    #[doc = r#"Description: "#]
    pub tnmnumf: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN ASSET-COM & PFD STK"#]
    #[doc = r#"Description: "#]
    pub tocps: Option<f32>,

    #[doc = r#"Title: ALL OTH MANAGE AST - EQ MUT FND"#]
    #[doc = r#"Description: "#]
    pub toeqf: Option<f32>,

    #[doc = r#"Title: OTH FIDUCIARY-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tofma: Option<f32>,

    #[doc = r#"Title: OTH FIDUCIARY-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tofmanum: Option<f32>,

    #[doc = r#"Title: OTH FIDUCIARY NON-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub tofnma: Option<f32>,

    #[doc = r#"Title: OTH FIDUCIARY-NON-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tofnmnum: Option<f32>,

    #[doc = r#"Title: ALL OTH MANAGE ASSET - INT BEAR"#]
    #[doc = r#"Description: "#]
    pub toi: Option<f32>,

    #[doc = r#"Title: ALL OTHER MANAGED ASSET- TOTAL"#]
    #[doc = r#"Description: "#]
    pub tomatot: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN ASSET - MISC ASSET"#]
    #[doc = r#"Description: "#]
    pub tomisc: Option<f32>,

    #[doc = r#"Title: ALL OTH MANAGE AST - MONEY MKT"#]
    #[doc = r#"Description: "#]
    pub tommf: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN ASSET - NONINT BEAR"#]
    #[doc = r#"Description: "#]
    pub toni: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN AST -OTH NOTE & BND"#]
    #[doc = r#"Description: "#]
    pub toothb: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN ASSET - OTH MUT FND"#]
    #[doc = r#"Description: "#]
    pub toothf: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN ASSET - REAL ESTATE"#]
    #[doc = r#"Description: "#]
    pub tore: Option<f32>,

    #[doc = r#"Title: ALL OTHER MANAGE ASSET - RE MTG"#]
    #[doc = r#"Description: "#]
    pub toremtg: Option<f32>,

    #[doc = r#"Title: OTH RETIREMENT-MANAGED-AMT"#]
    #[doc = r#"Description: "#]
    pub torma: Option<f32>,

    #[doc = r#"Title: OTH RETIREMENT-MANAGED-NUM"#]
    #[doc = r#"Description: "#]
    pub tormanum: Option<f32>,

    #[doc = r#"Title: OTH RETIREMENT-NON-MAN-AMT"#]
    #[doc = r#"Description: "#]
    pub tornma: Option<f32>,

    #[doc = r#"Title: OTH RETIREMENT-NON-MAN-NUM"#]
    #[doc = r#"Description: "#]
    pub tornmnum: Option<f32>,

    #[doc = r#"Title: ALL OTHER MANAGED ASSET - MUNI"#]
    #[doc = r#"Description: "#]
    pub toscmun: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN AST-U.S. TREAS & OB"#]
    #[doc = r#"Description: "#]
    pub toscus: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN AST - SHRT TERM OBL"#]
    #[doc = r#"Description: "#]
    pub tosto: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN ASSET - TRUST FUND"#]
    #[doc = r#"Description: "#]
    pub totrf: Option<f32>,

    #[doc = r#"Title: ALL OTH MAN ASSET - UNREG FUNDS"#]
    #[doc = r#"Description: "#]
    pub touf: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY- COM & PRF STK"#]
    #[doc = r#"Description: "#]
    pub tpicps: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - EQ MUT FUND"#]
    #[doc = r#"Description: "#]
    pub tpieqf: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - INT BEARING"#]
    #[doc = r#"Description: "#]
    pub tpii: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY-TOT MANAGE AST"#]
    #[doc = r#"Description: "#]
    pub tpimatot: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - MISC"#]
    #[doc = r#"Description: "#]
    pub tpimisc: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - MONEY MKT"#]
    #[doc = r#"Description: "#]
    pub tpimmf: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY-NONINT BEARING"#]
    #[doc = r#"Description: "#]
    pub tpini: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY-OTH NOTE & BND"#]
    #[doc = r#"Description: "#]
    pub tpiothb: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - OTH MUT FUND"#]
    #[doc = r#"Description: "#]
    pub tpiothf: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - REAL ESTATE"#]
    #[doc = r#"Description: "#]
    pub tpire: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - RE MTG"#]
    #[doc = r#"Description: "#]
    pub tpiremtg: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - MUNI"#]
    #[doc = r#"Description: "#]
    pub tpiscmun: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY-U.S TREAS & OB"#]
    #[doc = r#"Description: "#]
    pub tpiscus: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - SHRT TERM OB"#]
    #[doc = r#"Description: "#]
    pub tpisto: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY - TRUST FUND"#]
    #[doc = r#"Description: "#]
    pub tpitrf: Option<f32>,

    #[doc = r#"Title: PER TR & INV AGY- UNREG FUNDS"#]
    #[doc = r#"Description: "#]
    pub tpiuf: Option<f32>,

    #[doc = r#"Title: MANAGED ASSET-PER & AGEN-AMT"#]
    #[doc = r#"Description: "#]
    pub tpma: Option<f32>,

    #[doc = r#"Title: MANAGED ASSET - PER&AGEN-NUM"#]
    #[doc = r#"Description: "#]
    pub tpmanum: Option<f32>,

    #[doc = r#"Title: NON-MANAGED - PER&AGEN-AMT"#]
    #[doc = r#"Description: "#]
    pub tpnma: Option<f32>,

    #[doc = r#"Title: NON-MANAGED ASSET-PER&AGEN-NUM"#]
    #[doc = r#"Description: "#]
    pub tpnmnum: Option<f32>,

    #[doc = r#"Title: TRUST POWERS EXERCISED"#]
    #[doc = r#"Description: "#]
    pub trexer: Option<f32>,

    #[doc = r#"Title: TRADING ACCOUNTS-FOR"#]
    #[doc = r#"Description: "#]
    pub trfor: Option<f32>,

    #[doc = r#"Title: IRA"#]
    #[doc = r#"Description: "#]
    pub trhma: Option<f32>,

    #[doc = r#"Title: IRA"#]
    #[doc = r#"Description: "#]
    pub trhmanum: Option<f32>,

    #[doc = r#"Title: IRA"#]
    #[doc = r#"Description: "#]
    pub trhnma: Option<f32>,

    #[doc = r#"Title: IRA"#]
    #[doc = r#"Description: "#]
    pub trhnmnum: Option<f32>,

    #[doc = r#"Title: TRADE-DERIVATIVES NEG VAL"#]
    #[doc = r#"Description: "#]
    pub trlreval: Option<f32>,

    #[doc = r#"Title: TRADE-DERIVATED NEG VAL RATIO"#]
    #[doc = r#"Description: "#]
    pub trlrevalr: Option<f32>,

    #[doc = r#"Title: TRANSACTION-COM BKS& OTHER"#]
    #[doc = r#"Description: "#]
    pub trncbo: Option<f32>,

    #[doc = r#"Title: TRANSACTION-COM BKS& OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub trncbor: Option<f32>,

    #[doc = r#"Title: TRANSACTION-FOR COUNTRY"#]
    #[doc = r#"Description: "#]
    pub trnfc: Option<f32>,

    #[doc = r#"Title: TRANSACTION-FOR COUNTRY & GOVT"#]
    #[doc = r#"Description: "#]
    pub trnfcfg: Option<f32>,

    #[doc = r#"Title: TRANSACTION-FOR COUNTRY & GOVT RATIO"#]
    #[doc = r#"Description: "#]
    pub trnfcfgr: Option<f32>,

    #[doc = r#"Title: TRANSACTION-FOREIGN GOVERNMENT"#]
    #[doc = r#"Description: "#]
    pub trnfg: Option<f32>,

    #[doc = r#"Title: AMT NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub trnnia: Option<f32>,

    #[doc = r#"Title: AMT NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub trnniar: Option<f32>,

    #[doc = r#"Title: NUM NON-INTEREST BEARING TRANSACTION ACC MORE THAN $250,000"#]
    #[doc = r#"Description: "#]
    pub trnnin: Option<f32>,

    #[doc = r#"Title: INSTITUTION HAS TRUST POWER"#]
    #[doc = r#"Description: "#]
    pub trpower: Option<f32>,

    #[doc = r#"Title: TRADE-DERIV POS VAL-DOM"#]
    #[doc = r#"Description: "#]
    pub trrevald: Option<f32>,

    #[doc = r#"Title: TRADE-DERIV POS VALUE-FOR"#]
    #[doc = r#"Description: "#]
    pub trrevalf: Option<f32>,

    #[doc = r#"Title: REVALUATION GAINS ON OFF-BALANCE SHEET CONTRACTS"#]
    #[doc = r#"Description: "#]
    pub trrevalsum: Option<f32>,

    #[doc = r#"Title: REVALUATION GAINS ON OFF-BALANCE SHEET CONTRACTS RATIO"#]
    #[doc = r#"Description: "#]
    pub trrevalsumr: Option<f32>,

    #[doc = r#"Title: TOT FIDUCIARY ACCTS-MAN-AMT"#]
    #[doc = r#"Description: "#]
    pub ttma: Option<f32>,

    #[doc = r#"Title: TOT FIDUCIARY ACCTS-MAN-NUM"#]
    #[doc = r#"Description: "#]
    pub ttnanum: Option<f32>,

    #[doc = r#"Title: TOT FIDUCIARY ACCTS-NON-MAN-AMT"#]
    #[doc = r#"Description: "#]
    pub ttnma: Option<f32>,

    #[doc = r#"Title: TOT FIDUCIARY ACCTS-NON-MAN-NUM"#]
    #[doc = r#"Description: "#]
    pub ttnmnum: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-TOTAL"#]
    #[doc = r#"Description: "#]
    pub uc: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-TOTAL RATIO"#]
    #[doc = r#"Description: "#]
    pub ucr: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-COM RE"#]
    #[doc = r#"Description: "#]
    pub uccomre: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-COM RE RATIO"#]
    #[doc = r#"Description: "#]
    pub uccomrer: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-SECURED COM RE"#]
    #[doc = r#"Description: "#]
    pub uccomres: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-SECURED COM RE RATIO"#]
    #[doc = r#"Description: "#]
    pub uccomresr: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-UNSECURED COM RE"#]
    #[doc = r#"Description: "#]
    pub uccomreu: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-UNSECURED COM RE RATIO"#]
    #[doc = r#"Description: "#]
    pub uccomreur: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-CREDIT CARD LINES"#]
    #[doc = r#"Description: "#]
    pub uccrcd: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-CREDIT CARD LINES RATIO"#]
    #[doc = r#"Description: "#]
    pub uccrcdr: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-TOTAL LOANS"#]
    #[doc = r#"Description: "#]
    pub ucln: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-HOME EQUITY LINES"#]
    #[doc = r#"Description: "#]
    pub ucloc: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-HOME EQUITY LINES RATIO"#]
    #[doc = r#"Description: "#]
    pub uclocr: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-ALL OTHER"#]
    #[doc = r#"Description: "#]
    pub ucother: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-ALL OTHER RATIO"#]
    #[doc = r#"Description: "#]
    pub ucotherr: Option<f32>,

    #[doc = r#"Title: UNUSED COM-OVER 1 YR-RC-R COL A"#]
    #[doc = r#"Description: "#]
    pub ucover1: Option<f32>,

    #[doc = r#"Title: UNUSED COM-OVER 1 YR-RC-R COL A RATIO"#]
    #[doc = r#"Description: "#]
    pub ucover1r: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-SEC UNDERWRITING"#]
    #[doc = r#"Description: "#]
    pub ucsc: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT-SEC UNDERWRITING RATIO"#]
    #[doc = r#"Description: "#]
    pub ucscr: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT FOR SECUR. - AUTO"#]
    #[doc = r#"Description: "#]
    pub ucszauto: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT FOR SECUR. - CI"#]
    #[doc = r#"Description: "#]
    pub ucszci: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT FOR SECUR. - CON"#]
    #[doc = r#"Description: "#]
    pub ucszcon: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT FOR SECUR. - CRCD"#]
    #[doc = r#"Description: "#]
    pub ucszcrcd: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT FOR SECUR. - HEL"#]
    #[doc = r#"Description: "#]
    pub ucszhel: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT FOR SECUR. - OTH"#]
    #[doc = r#"Description: "#]
    pub ucszoth: Option<f32>,

    #[doc = r#"Title: UNUSED COMMIT FOR SECUR. - RES"#]
    #[doc = r#"Description: "#]
    pub ucszres: Option<f32>,

    #[doc = r#"Title: UNEARNED INCOME-FOR"#]
    #[doc = r#"Description: "#]
    pub unincfor: Option<f32>,

    #[doc = r#"Title: UNEARNED INCOME-FOR RATIO"#]
    #[doc = r#"Description: "#]
    pub unincforr: Option<f32>,

    #[doc = r#"Title: VOLATILE LIABILITIES"#]
    #[doc = r#"Description: "#]
    pub voliab: Option<f32>,

    #[doc = r#"Title: VOLATILE LIABILITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub voliabr: Option<f32>,

    #[doc = r#"Title: ZIP CODE"#]
    #[doc = r#"Description: "#]
    pub zip: Option<f32>,

    #[doc = r#"Title: NONMORTGAGE LOANS IN PROCESS"#]
    #[doc = r#"Description: "#]
    pub lipnmtg: Option<f32>,

    #[doc = r#"Title: UNAMORTIZED YIELD ADJ-NONMTG LNS"#]
    #[doc = r#"Description: "#]
    pub uyanmtg: Option<f32>,

    #[doc = r#"Title: LOAN & LEASE INCOME"#]
    #[doc = r#"Description: "#]
    pub ilnls: Option<f32>,

    #[doc = r#"Title: BANKS UNIT"#]
    #[doc = r#"Description: "#]
    pub unit: Option<f32>,

    #[doc = r#"Title: PRE-TAX NET INCOME OPERATING INCOME"#]
    #[doc = r#"Description: "#]
    pub ptaxnetinc: Option<f32>,

    #[doc = r#"Title: PRE-TAX NET INCOME OPERATING INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub ptaxnetincr: Option<f32>,

    #[doc = r#"Title: PRE-TAX NET INCOME OPERATING INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub ptaxnetincq: Option<f32>,

    #[doc = r#"Title: PRE-TAX NET INCOME OPERATING INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ptaxnetincqr: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST INCOME"#]
    #[doc = r#"Description: "#]
    pub addnonii: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST INCOME RATIO"#]
    #[doc = r#"Description: "#]
    pub addnoniir: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST INCOME QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub addnoniiq: Option<f32>,

    #[doc = r#"Title: ADDITIONAL NONINTEREST INCOME QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub addnoniiqr: Option<f32>,

    #[doc = r#"Title: Quarterly average amount of assets purchased under the MMLF and excluded from “Total assets for the leverage ratio.”"#]
    #[doc = r#"Description: "#]
    pub avmmlf: Option<f32>,

    #[doc = r#"Title: Quarterly average amount of assets purchased under the MMLF and excluded from “Total assets for the leverage ratio.” ratio"#]
    #[doc = r#"Description: "#]
    pub avmmlfr: Option<f32>,

    #[doc = r#"Title: Quarterly average amount of PPP loans pledged to the PPPLF and excluded from “Total assets for the leverage ratio.”"#]
    #[doc = r#"Description: "#]
    pub avpppplg: Option<f32>,

    #[doc = r#"Title: Quarterly average amount of PPP loans pledged to the PPPLF and excluded from “Total assets for the leverage ratio.” ratio"#]
    #[doc = r#"Description: "#]
    pub avpppplgr: Option<f32>,

    #[doc = r#"Title: Outstanding balance of assets purchased under the Money Market Mutual Fund Liquidity Facility (MMLF)."#]
    #[doc = r#"Description: "#]
    pub mmlfbal: Option<f32>,

    #[doc = r#"Title: Outstanding balance of assets purchased under the Money Market Mutual Fund Liquidity Facility (MMLF) ratio"#]
    #[doc = r#"Description: "#]
    pub mmlfbalr: Option<f32>,

    #[doc = r#"Title: Outstanding balance under the PPPLF with a remaining maturity of more than one year"#]
    #[doc = r#"Description: "#]
    pub ppplfov1: Option<f32>,

    #[doc = r#"Title: Outstanding balance under the PPPLF with a remaining maturity of more than one year ratio"#]
    #[doc = r#"Description: "#]
    pub ppplfov1r: Option<f32>,

    #[doc = r#"Title: Outstanding balance of PPP loans"#]
    #[doc = r#"Description: "#]
    pub ppplnbal: Option<f32>,

    #[doc = r#"Title: Outstanding balance of PPP loans ratio"#]
    #[doc = r#"Description: "#]
    pub ppplnbalr: Option<f32>,

    #[doc = r#"Title: Number of PPP loans outstanding"#]
    #[doc = r#"Description: "#]
    pub ppplnnum: Option<f32>,

    #[doc = r#"Title: Number of PPP loans outstanding ratio"#]
    #[doc = r#"Description: "#]
    pub ppplnnumr: Option<f32>,

    #[doc = r#"Title: Outstanding balance of PPP loans pledged to the PPPLF"#]
    #[doc = r#"Description: "#]
    pub ppplnplg: Option<f32>,

    #[doc = r#"Title: Outstanding balance of PPP loans pledged to the PPPLF ratio"#]
    #[doc = r#"Description: "#]
    pub ppplnplgr: Option<f32>,

    #[doc = r#"Title: Outstanding balance under the PPPLF with a remaining maturity of one year or less"#]
    #[doc = r#"Description: "#]
    pub ppplf1ls: Option<f32>,

    #[doc = r#"Title: Outstanding balance under the PPPLF with a remaining maturity of one year or less ratio"#]
    #[doc = r#"Description: "#]
    pub ppplf1lsr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL & INDUSTRIAL LOANS"#]
    #[doc = r#"Description: "#]
    pub idntcir: Option<f32>,

    #[doc = r#"Title: COMMERCIAL & INDUSTRIAL LOANS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub idntciqr: Option<f32>,

    #[doc = r#"Title: LOANS TO INDIVIDUALS"#]
    #[doc = r#"Description: "#]
    pub idntconr: Option<f32>,

    #[doc = r#"Title: CREDIT CARDS & RELATED PLANS"#]
    #[doc = r#"Description: "#]
    pub idntcrdr: Option<f32>,

    #[doc = r#"Title: OTHER LOANS TO INDIVIDUALS"#]
    #[doc = r#"Description: "#]
    pub idntcoor: Option<f32>,

    #[doc = r#"Title: OTHER LOANS TO INDIVIDUALS"#]
    #[doc = r#"Description: "#]
    pub idntcooqr: Option<f32>,

    #[doc = r#"Title: CREDIT CARDS & RELATED PLANS QUARTERLY"#]
    #[doc = r#"Description: "#]
    pub idntcrdqr: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub instcnt: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub idntilr: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub idothnii: Option<f32>,

    #[doc = r#"Title: AUTOMOBILE LOANS"#]
    #[doc = r#"Description: "#]
    pub ntautopr: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOANS"#]
    #[doc = r#"Description: "#]
    pub ntconotr: Option<f32>,

    #[doc = r#"Title: EARNINGS COVERAGE OF NET LOAN CHARGE-OFFS (X)"#]
    #[doc = r#"Description: "#]
    pub iderncvr: Option<f32>,

    #[doc = r#"Title: Earnings coverage of net loan charge-offs"#]
    #[doc = r#"Description: "#]
    pub iderncvqr: Option<f32>,

    #[doc = r#"Title: CASH DIVIDENDS TO NET INCOME (YTD ONLY)"#]
    #[doc = r#"Description: "#]
    pub eqcdivntinc: Option<f32>,

    #[doc = r#"Title: NOTIONAL AMOUNT OF CREDIT DERIVATIVES"#]
    #[doc = r#"Description: "#]
    pub nacdir: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE CHG-OFF/COMM RE LN QUARTERLY RATIO"#]
    #[doc = r#"Description: "#]
    pub ntcomreqr: Option<f32>,

    #[doc = r#"Title: Net Charge-offs All other loans & leases (including farm) Numerator"#]
    #[doc = r#"Description: "#]
    pub ntallothnum: Option<f32>,

    #[doc = r#"Title: Net Charge-offs All other loans & leases (including farm) denominator"#]
    #[doc = r#"Description: "#]
    pub ntallothden: Option<f32>,

    #[doc = r#"Title: ALL OTHER LOANS & LEASES (INCLUDING FARM)"#]
    #[doc = r#"Description: "#]
    pub ntallothr: Option<f32>,

    #[doc = r#"Title: Net Charge-offs All other loans & leases (including farm)"#]
    #[doc = r#"Description: "#]
    pub ntallothqr: Option<f32>,

    #[doc = r#"Title: Other loans to individuals"#]
    #[doc = r#"Description: "#]
    pub idnccoor: Option<f32>,

    #[doc = r#"Title: All other loans & leases (including farm )"#]
    #[doc = r#"Description: "#]
    pub idncothr: Option<f32>,

    #[doc = r#"Title: COMMERCIAL & INDUSTRIAL LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub idnccir: Option<f32>,

    #[doc = r#"Title: LOANS TO INDIVIDUALS RATIO"#]
    #[doc = r#"Description: "#]
    pub idncconr: Option<f32>,

    #[doc = r#"Title: CREDIT CARDS & RELATED PLANS RATIO"#]
    #[doc = r#"Description: "#]
    pub idnccrdr: Option<f32>,

    #[doc = r#"Title: AUTOMOBILE LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub idncator: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub idntator: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub idntcotr: Option<f32>,

    #[doc = r#"Title: IDDEPINR"#]
    #[doc = r#"Description: "#]
    pub iddepinr: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub iddivnir: Option<f32>,

    #[doc = r#"Title: OTHER CONSUMER LOANS RATIO"#]
    #[doc = r#"Description: "#]
    pub idnccotr: Option<f32>,

    #[doc = r#"Title: INTEREST INCOME TO EARNING ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub intincy: Option<f32>,

    #[doc = r#"Title: NONCURRENT LOANS WHICH ARE WHOLLY OR PARTIALLY GUARANTEED BY THE U.S. GOVERNMENT RATIO"#]
    #[doc = r#"Description: "#]
    pub idncgtpr: Option<f32>,

    #[doc = r#"Title: NET LOANS AND LEASES TO CORE DEPOSITS RATIO"#]
    #[doc = r#"Description: "#]
    pub idlncorr: Option<f32>,

    #[doc = r#"Title: ID NO CB FLAG"#]
    #[doc = r#"Description: "#]
    pub idt1cnocb: Option<f32>,

    #[doc = r#"Title: ID NO J CB FLAG"#]
    #[doc = r#"Description: "#]
    pub idt1jnocb: Option<f32>,

    #[doc = r#"Title: COMMON EQUITY TIER 1 CAPITAL RATIO"#]
    #[doc = r#"Description: "#]
    pub idt1cer: Option<f32>,

    #[doc = r#"Title: TIER 1 RISK-BASED CAPITAL RATIO"#]
    #[doc = r#"Description: "#]
    pub idt1rwajr: Option<f32>,

    #[doc = r#"Title: EQUITY SECURITIES NOT HELD FOR TRADING"#]
    #[doc = r#"Description: "#]
    pub sceqnft: Option<f32>,

    #[doc = r#"Title: PRIV ISSUED RES MORTGAGE-BACKED SECURITIES"#]
    #[doc = r#"Description: "#]
    pub scrmbpi: Option<f32>,

    #[doc = r#"Title: PRIV ISSUED RES MORTGAGE-BACKED SECURITIES RATIO"#]
    #[doc = r#"Description: "#]
    pub scrmbpir: Option<f32>,

    #[doc = r#"Title: U.S GOVERNMENT OBLIGATIONS"#]
    #[doc = r#"Description: "#]
    pub scuso: Option<f32>,

    #[doc = r#"Title: U.S GOVERNMENT OBLIGATIONS RATIOS"#]
    #[doc = r#"Description: "#]
    pub scusor: Option<f32>,

    #[doc = r#"Title: OTHER COMM MORTGAGE-BACKED SEC"#]
    #[doc = r#"Description: "#]
    pub sccmos: Option<f32>,

    #[doc = r#"Title: OTHER COMM MORTGAGE-BACKED SEC"#]
    #[doc = r#"Description: "#]
    pub sccmosr: Option<f32>,

    #[doc = r#"Title: ASSETS HELD IN TRADING ACCOUNTS FOR TFR REPORTERS"#]
    #[doc = r#"Description: "#]
    pub sctatfr: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES, GROSS"#]
    #[doc = r#"Description: "#]
    pub lnlsgrs: Option<f32>,

    #[doc = r#"Title: LOANS AND LEASES, GROSS RATIO"#]
    #[doc = r#"Description: "#]
    pub lnlsgrsr: Option<f32>,

    #[doc = r#"Title: ALL OTH ASSETS"#]
    #[doc = r#"Description: "#]
    pub aoa: Option<f32>,

    #[doc = r#"Title: ALL OTH ASSETS RATIO"#]
    #[doc = r#"Description: "#]
    pub aoar: Option<f32>,

    #[doc = r#"Title: PERCENTAGE INSURED ESTIMATED"#]
    #[doc = r#"Description: "#]
    pub estins: Option<f32>,

    #[doc = r#"Title: PERCENTAGE INSURED ESTIMATED RATIO"#]
    #[doc = r#"Description: "#]
    pub estinsr: Option<f32>,

    #[doc = r#"Title: P/D 30-89 REAL ESTATE LOANS IN DOMESTIC OFFICES"#]
    #[doc = r#"Description: "#]
    pub p3relndo: Option<f32>,

    #[doc = r#"Title: P/D 30-89 REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO"#]
    #[doc = r#"Description: "#]
    pub p3relndor: Option<f32>,

    #[doc = r#"Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES"#]
    #[doc = r#"Description: "#]
    pub p9relndo: Option<f32>,

    #[doc = r#"Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO"#]
    #[doc = r#"Description: "#]
    pub p9relndor: Option<f32>,

    #[doc = r#"Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES"#]
    #[doc = r#"Description: "#]
    pub narelndo: Option<f32>,

    #[doc = r#"Title: 90+ REAL ESTATE LOANS IN DOMESTIC OFFICES RATIO"#]
    #[doc = r#"Description: "#]
    pub narelndor: Option<f32>,

    #[doc = r#"Title: State and County Nunber"#]
    #[doc = r#"Description: "#]
    pub stcnty: Option<String>,

    #[doc = r#"Title: Metropolitan Statistical Area"#]
    #[doc = r#"Description: "#]
    pub cbsa: Option<String>,

    #[doc = r#"Title: Date of Deposit Insurance (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub insdate: Option<String>,

    #[doc = r#"Title: Last Structure Change Process Date (Search-Eligible)"#]
    #[doc = r#"Description: This field can be used for search and filtering."#]
    pub upddate: Option<String>,

    #[doc = r#"Title: Total Assets Ratio"#]
    #[doc = r#"Description: "#]
    pub assetr: Option<f32>,

    #[doc = r#"Title: AVG TOTAL ASSETS"#]
    #[doc = r#"Description: "#]
    pub avasset: Option<f32>,

    #[doc = r#"Title: BROKERED DEP-INSURED-LARGE"#]
    #[doc = r#"Description: "#]
    pub broinslg: Option<f32>,

    #[doc = r#"Title: RC-R TOTAL ADJ & DED COM EQ T1"#]
    #[doc = r#"Description: "#]
    pub ct1ajtot: Option<f32>,

    #[doc = r#"Title: RC-R COM EQUITY T1 BEFORE ADJ"#]
    #[doc = r#"Description: "#]
    pub ct1badj: Option<f32>,

    #[doc = r#"Title: TOTAL DEPOSITS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub dep2: Option<f32>,

    #[doc = r#"Title: TOTAL DEPOSITS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub dep5: Option<f32>,

    #[doc = r#"Title: INTEREST-BEARING-DEP-Y1"#]
    #[doc = r#"Description: "#]
    pub depiy1: Option<f32>,

    #[doc = r#"Title: INT EXPENSE TIME CD GT $250"#]
    #[doc = r#"Description: "#]
    pub ecd100: Option<f32>,

    #[doc = r#"Title: INT EXP TIME CD GT $250"#]
    #[doc = r#"Description: "#]
    pub ecd100a: Option<f32>,

    #[doc = r#"Title: INT EXP TIME CD GT $250"#]
    #[doc = r#"Description: "#]
    pub ecd100q: Option<f32>,

    #[doc = r#"Title: FED FUNDS & REPO INT EXPENSE-ANN"#]
    #[doc = r#"Description: "#]
    pub efreppa: Option<f32>,

    #[doc = r#"Title: INT EXP TIME CD LE $250"#]
    #[doc = r#"Description: "#]
    pub eothtima: Option<f32>,

    #[doc = r#"Title: INT EXPENSE TIME CD LE $250"#]
    #[doc = r#"Description: "#]
    pub eothtime: Option<f32>,

    #[doc = r#"Title: INT EXP TIME CD LE $250"#]
    #[doc = r#"Description: "#]
    pub eothtimq: Option<f32>,

    #[doc = r#"Title: UNDIVIDED PROFITS"#]
    #[doc = r#"Description: "#]
    pub equpgr: Option<f32>,

    #[doc = r#"Title: NONTRANSACTION SAV ACCTS INT EXP"#]
    #[doc = r#"Description: "#]
    pub esavdp: Option<f32>,

    #[doc = r#"Title: NONTRANSACT SAV ACCT INT EXT-ANN"#]
    #[doc = r#"Description: "#]
    pub esavdpa: Option<f32>,

    #[doc = r#"Title: NONTRANSACT SAV ACCT INT EXP-QTR"#]
    #[doc = r#"Description: "#]
    pub esavdpq: Option<f32>,

    #[doc = r#"Title: SUBORDINATED NOTES INT EXP-ANN"#]
    #[doc = r#"Description: "#]
    pub esubnda: Option<f32>,

    #[doc = r#"Title: TRANSACTION ACCOUNTS INT EXPENSE"#]
    #[doc = r#"Description: "#]
    pub etrandep: Option<f32>,

    #[doc = r#"Title: TRANSACTION ACCOUNTS INT EXP-ANN"#]
    #[doc = r#"Description: "#]
    pub etrandpa: Option<f32>,

    #[doc = r#"Title: TRANSACTION ACCOUNTS INT EXP-QTR"#]
    #[doc = r#"Description: "#]
    pub etrandpq: Option<f32>,

    #[doc = r#"Title: TT&L & OTHER BORROW INT EXP-ANN"#]
    #[doc = r#"Description: "#]
    pub ettlotba: Option<f32>,

    #[doc = r#"Title: TT&L & OTHER BORROW INT EXP-QTR"#]
    #[doc = r#"Description: "#]
    pub ettlotbq: Option<f32>,

    #[doc = r#"Title: FEDERAL FUNDS PURCHASED"#]
    #[doc = r#"Description: "#]
    pub ffpur: Option<f32>,

    #[doc = r#"Title: INC BEFORE INC TAXS & DISC-ANN"#]
    #[doc = r#"Description: "#]
    pub ibeftxa: Option<f32>,

    #[doc = r#"Title: AVAILABLE-FOR-SALE SECS G/L"#]
    #[doc = r#"Description: "#]
    pub iglsca: Option<f32>,

    #[doc = r#"Title: AVAILABLE-FOR-SALE SEC G/L-QTR"#]
    #[doc = r#"Description: "#]
    pub iglscaq: Option<f32>,

    #[doc = r#"Title: HELD-TO-MATURITY SECS G/L"#]
    #[doc = r#"Description: "#]
    pub iglsch: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-ANN"#]
    #[doc = r#"Description: "#]
    pub ilna: Option<f32>,

    #[doc = r#"Title: LOAN & LEASE INCOME-ANN"#]
    #[doc = r#"Description: "#]
    pub ilnlsa: Option<f32>,

    #[doc = r#"Title: LOAN & LEASE INCOME-QTR"#]
    #[doc = r#"Description: "#]
    pub ilnlsq: Option<f32>,

    #[doc = r#"Title: TAX-EXEMPT LN & LS INT INC-ANN"#]
    #[doc = r#"Description: "#]
    pub ilnlsxa: Option<f32>,

    #[doc = r#"Title: TAX-EXEMPT LN & LS INT INC-QTR"#]
    #[doc = r#"Description: "#]
    pub ilnlsxq: Option<f32>,

    #[doc = r#"Title: MUNICIPAL LOAN INCOME-QTR"#]
    #[doc = r#"Description: "#]
    pub ilnmuniq: Option<f32>,

    #[doc = r#"Title: LOAN INCOME-QTR"#]
    #[doc = r#"Description: "#]
    pub ilnq: Option<f32>,

    #[doc = r#"Title: TOTAL SECURITY INCOME-ANN"#]
    #[doc = r#"Description: "#]
    pub isca: Option<f32>,

    #[doc = r#"Title: SERVICE CHARGE ON DEP ACCTS-ANN"#]
    #[doc = r#"Description: "#]
    pub iserchga: Option<f32>,

    #[doc = r#"Title: APPLICABLE INCOME TAXES-ANN"#]
    #[doc = r#"Description: "#]
    pub itaxa: Option<f32>,

    #[doc = r#"Title: APPLICABLE INCOME TAXES-QTR-ANN"#]
    #[doc = r#"Description: "#]
    pub itaxqa: Option<f32>,

    #[doc = r#"Title: CONSTR & LAND DEV LNS/TIER 1"#]
    #[doc = r#"Description: "#]
    pub lncdt1r: Option<f32>,

    #[doc = r#"Title: C&I LOANS/TIER 1"#]
    #[doc = r#"Description: "#]
    pub lncit1r: Option<f32>,

    #[doc = r#"Title: CONSUMER LOANS/TIER 1"#]
    #[doc = r#"Description: "#]
    pub lncont1r: Option<f32>,

    #[doc = r#"Title: ALLOWANCE FOR LOAN AND LEASES"#]
    #[doc = r#"Description: "#]
    pub lnlsres: Option<f32>,

    #[doc = r#"Title: RE AGRICULTURAL-CAVG5"#]
    #[doc = r#"Description: "#]
    pub lnreag5: Option<f32>,

    #[doc = r#"Title: RE LOANS/TIER 1"#]
    #[doc = r#"Description: "#]
    pub lnrert1r: Option<f32>,

    #[doc = r#"Title: TOTAL N/C-RE*FARMLAND"#]
    #[doc = r#"Description: "#]
    pub ncreag: Option<f32>,

    #[doc = r#"Title: N/C 1-4 FAMILY CONSTRUCTION LOAN"#]
    #[doc = r#"Description: "#]
    pub ncrecnfm: Option<f32>,

    #[doc = r#"Title: N/C OTHER CONSTRUCT & LAND DEV"#]
    #[doc = r#"Description: "#]
    pub ncrecnot: Option<f32>,

    #[doc = r#"Title: N/C OTHER NONFARM NONRES RE LN"#]
    #[doc = r#"Description: "#]
    pub ncrenrot: Option<f32>,

    #[doc = r#"Title: N/C OWN-OCCUPIED NONFARM NONRES"#]
    #[doc = r#"Description: "#]
    pub ncrenrow: Option<f32>,

    #[doc = r#"Title: N/C 1-4 FAM JR LN/1-4 FAM JR LN"#]
    #[doc = r#"Description: "#]
    pub ncrers2r: Option<f32>,

    #[doc = r#"Title: N/C RE 1-4 FAM JUNIOR LIEN"#]
    #[doc = r#"Description: "#]
    pub ncrersf2: Option<f32>,

    #[doc = r#"Title: N/C RE 1-4 FAM FIRST LIEN"#]
    #[doc = r#"Description: "#]
    pub ncrersfm: Option<f32>,

    #[doc = r#"Title: N/C 1-4 FAM 1STLN/1-4 FAM IST LN"#]
    #[doc = r#"Description: "#]
    pub ncrersfr: Option<f32>,

    #[doc = r#"Title: NC RESTRUCT LOANS EXCL 1-4 FM"#]
    #[doc = r#"Description: "#]
    pub ncrslnls: Option<f32>,

    #[doc = r#"Title: NET OPERATING INCOME-QTR"#]
    #[doc = r#"Description: "#]
    pub noiq: Option<f32>,

    #[doc = r#"Title: AG LOAN NET CHARGE-OFFS-QTR-ANN"#]
    #[doc = r#"Description: "#]
    pub ntagqa: Option<f32>,

    #[doc = r#"Title: AG LN NET CHARGE-OFFS ANN*SM BKS"#]
    #[doc = r#"Description: "#]
    pub ntagsma: Option<f32>,

    #[doc = r#"Title: AG LOAN NET-CHG-QTR-ANN*SMALL BK"#]
    #[doc = r#"Description: "#]
    pub ntagsmqa: Option<f32>,

    #[doc = r#"Title: COMMERCIAL LOAN NET-CHG-QTR-ANN"#]
    #[doc = r#"Description: "#]
    pub ntciqa: Option<f32>,

    #[doc = r#"Title: COMMERCIAL RE LN NET CHARGE-OFFS"#]
    #[doc = r#"Description: "#]
    pub ntcomre: Option<f32>,

    #[doc = r#"Title: COMML RE NET-CHARGE-OFF-QTR-ANN"#]
    #[doc = r#"Description: "#]
    pub ntcomrqa: Option<f32>,

    #[doc = r#"Title: CONSUMER LN NET-CHG-QTR-ANN"#]
    #[doc = r#"Description: "#]
    pub ntconqa: Option<f32>,

    #[doc = r#"Title: CREDIT CARD LN NET-CHG-QTR-ANN"#]
    #[doc = r#"Description: "#]
    pub ntcrcdqa: Option<f32>,

    #[doc = r#"Title: RETAINED EARNINGS- BANK- QTR"#]
    #[doc = r#"Description: "#]
    pub ntirtq: Option<f32>,

    #[doc = r#"Title: Time Deposits Less Than Or Equal To insurance Limit"#]
    #[doc = r#"Description: "#]
    pub ntrcdsmj: Option<f32>,

    #[doc = r#"Title: FARMLAND RE LN NET-CHG-ANN"#]
    #[doc = r#"Description: "#]
    pub ntreaga: Option<f32>,

    #[doc = r#"Title: FARM RE LN NET CHRG-OFF-QTR-ANN"#]
    #[doc = r#"Description: "#]
    pub ntreagqa: Option<f32>,

    #[doc = r#"Title: OTHER BORROWED FUNDS"#]
    #[doc = r#"Description: "#]
    pub obor: Option<f32>,

    #[doc = r#"Title: OTHER BORROWED FUNDS-CAVG2"#]
    #[doc = r#"Description: "#]
    pub obor2: Option<f32>,

    #[doc = r#"Title: OTHER BORROWED FUNDS-CAVG5"#]
    #[doc = r#"Description: "#]
    pub obor5: Option<f32>,

    #[doc = r#"Title: OTH BOR FHLB-OVER 3 YRS"#]
    #[doc = r#"Description: "#]
    pub othbfh03: Option<f32>,

    #[doc = r#"Title: OTH BOR. FHLB-1 TO 3 YRS"#]
    #[doc = r#"Description: "#]
    pub othbfh13: Option<f32>,

    #[doc = r#"Title: 30-89 DAYS P/D-COMMERCIAL RE"#]
    #[doc = r#"Description: "#]
    pub p3comre: Option<f32>,

    #[doc = r#"Title: 30-89 PAST DUE CONST RE/CONST RE"#]
    #[doc = r#"Description: "#]
    pub p3reconr: Option<f32>,

    #[doc = r#"Title: 30-89 P/D 1-4FAM JR/1-4 FAM JR"#]
    #[doc = r#"Description: "#]
    pub p3rers2r: Option<f32>,

    #[doc = r#"Title: RETAINED EARNINGS - RBC"#]
    #[doc = r#"Description: "#]
    pub rbcequp: Option<f32>,

    #[doc = r#"Title: TIER 1 CAPITAL - REPORTED"#]
    #[doc = r#"Description: "#]
    pub rbct1w: Option<f32>,

    #[doc = r#"Title: REPURCHASE AGREEMENTS"#]
    #[doc = r#"Description: "#]
    pub repopur: Option<f32>,

    #[doc = r#"Title: SECURITIES-CAVG2"#]
    #[doc = r#"Description: "#]
    pub sc2: Option<f32>,

    #[doc = r#"Title: SECURITIES-CAVG5"#]
    #[doc = r#"Description: "#]
    pub sc5: Option<f32>,

    #[doc = r#"Title: MUNICIPAL SECURITIES -AA"#]
    #[doc = r#"Description: "#]
    pub scmuniaa: Option<f32>,

    #[doc = r#"Title: MUNICIPAL SECURITIES -AF"#]
    #[doc = r#"Description: "#]
    pub scmuniaf: Option<f32>,

    #[doc = r#"Title: MUNICIPAL SECURITIES -HA"#]
    #[doc = r#"Description: "#]
    pub scmuniha: Option<f32>,

    #[doc = r#"Title: MUNICIPAL SECURITIES -HF"#]
    #[doc = r#"Description: "#]
    pub scmunihf: Option<f32>,

}

/// Auto-generated response envelope struct for `/financials` endpoint.
/// Spec: risview_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancialsResponse {
    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub data: Option<String>,

}

/// FDIC BankFind API `/financials` endpoint handler
/// Get Financial Information for FDIC Insured Institutions
/// Returns financial information for financial institutions
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
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
pub async fn financials_handler(
    config: &FdicApiConfig,
    params: &FinancialsParameters,
) -> Result<CallToolResult, rmcp::Error> {
   
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "financials",
        method = "GET",
        path = "/financials",
        params = serde_json::to_string(params).unwrap()
    );

    let resp = get_fdic_bank_find_mcp_response(config, params).await;

    // Log outgoing FDIC API request as structured JSON
    resp
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
