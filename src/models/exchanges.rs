use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Exchange {
    #[default]
    SMART,
    AMEX,
    NYSE,
    CBOE,
    PHLX,
    CHX,
    ARCA,
    ISLAND,
    ISE,
    IDEAL,
    NASDAQQ,
    NASDAQ,
    DRCTEDGE,
    BEX,
    BATS,
    NITEECN,
    EDGEA,
    CSFBALGO,
    JEFFALGO,
    CSFBCROSS,
    NYSENASD,
    PSX,
    BYX,
    ITG,
    PDQ,
    IBKRATS,
    CITADEL,
    MIAX,
    IBDARK,
    CITADELDP,
    NASDDARK,
    IEX,
    WEDBUSH,
    SUMMER,
    WINSLOW,
    FINRA,
    LIQITG,
    UBSDARK,
    BTIG,
    VIRTU,
    LEERINK,
    JEFF,
    OPCO,
    COWEN,
    DBK,
    JPMC,
    EDGX,
    JANE,
    NEEDHAM,
    FRACSHARE,
    RBCALGO,
    VIRTUDP,
    BAYCREST,
    FOXRIVER,
    MND,
    NITEEXST,
    PEARL,
    GSDARK,
    NITERTL,
    NYSENAT,
    IEXMID,
    HRT,
    FLOWTRADE,
    HRTDP,
    JANELP,
    PEAK6,
    IMCDP,
    CTDLZERO,
    HRTMID,
    JANEZERO,
    HRTEXST,
    IMCLP,
    LTSE,
    SOCGENDP,
    MEMX,
    INTELCROS,
    VIRTUBYIN,
    JUMPTRADE,
    NITEZERO,
    TPLUS1,
    XTXEXST,
    XTXDP,
    XTXMID,
    COWENLP,
    BARCDP,
    JUMPLP,
    OLDMCLP,
    RBCCMALP,
    WALLBETH,
    JONES,
    GSLP,
    CME,
    QBALGO,
    IBUSOPT,
    NYSEDARK,
    IBEOS,
    BLUEOCEAN,
}
impl FromStr for Exchange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SMART" => Ok(Exchange::SMART),
            "AMEX" => Ok(Exchange::AMEX),
            "NYSE" => Ok(Exchange::NYSE),
            "CBOE" => Ok(Exchange::CBOE),
            "PHLX" => Ok(Exchange::PHLX),
            "CHX" => Ok(Exchange::CHX),
            "ARCA" => Ok(Exchange::ARCA),
            "ISLAND" => Ok(Exchange::ISLAND),
            "ISE" => Ok(Exchange::ISE),
            "IDEAL" => Ok(Exchange::IDEAL),
            "NASDAQQ" => Ok(Exchange::NASDAQQ),
            "NASDAQ" => Ok(Exchange::NASDAQ),
            "DRCTEDGE" => Ok(Exchange::DRCTEDGE),
            "BEX" => Ok(Exchange::BEX),
            "BATS" => Ok(Exchange::BATS),
            "NITEECN" => Ok(Exchange::NITEECN),
            "EDGEA" => Ok(Exchange::EDGEA),
            "CSFBALGO" => Ok(Exchange::CSFBALGO),
            "JEFFALGO" => Ok(Exchange::JEFFALGO),
            "CSFBCROSS" => Ok(Exchange::CSFBCROSS),
            "NYSENASD" => Ok(Exchange::NYSENASD),
            "PSX" => Ok(Exchange::PSX),
            "BYX" => Ok(Exchange::BYX),
            "ITG" => Ok(Exchange::ITG),
            "PDQ" => Ok(Exchange::PDQ),
            "IBKRATS" => Ok(Exchange::IBKRATS),
            "CITADEL" => Ok(Exchange::CITADEL),
            "MIAX" => Ok(Exchange::MIAX),
            "IBDARK" => Ok(Exchange::IBDARK),
            "CITADELDP" => Ok(Exchange::CITADELDP),
            "NASDDARK" => Ok(Exchange::NASDDARK),
            "IEX" => Ok(Exchange::IEX),
            "WEDBUSH" => Ok(Exchange::WEDBUSH),
            "SUMMER" => Ok(Exchange::SUMMER),
            "WINSLOW" => Ok(Exchange::WINSLOW),
            "FINRA" => Ok(Exchange::FINRA),
            "LIQITG" => Ok(Exchange::LIQITG),
            "UBSDARK" => Ok(Exchange::UBSDARK),
            "BTIG" => Ok(Exchange::BTIG),
            "VIRTU" => Ok(Exchange::VIRTU),
            "LEERINK" => Ok(Exchange::LEERINK),
            "JEFF" => Ok(Exchange::JEFF),
            "OPCO" => Ok(Exchange::OPCO),
            "COWEN" => Ok(Exchange::COWEN),
            "DBK" => Ok(Exchange::DBK),
            "JPMC" => Ok(Exchange::JPMC),
            "EDGX" => Ok(Exchange::EDGX),
            "JANE" => Ok(Exchange::JANE),
            "NEEDHAM" => Ok(Exchange::NEEDHAM),
            "FRACSHARE" => Ok(Exchange::FRACSHARE),
            "RBCALGO" => Ok(Exchange::RBCALGO),
            "VIRTUDP" => Ok(Exchange::VIRTUDP),
            "BAYCREST" => Ok(Exchange::BAYCREST),
            "FOXRIVER" => Ok(Exchange::FOXRIVER),
            "MND" => Ok(Exchange::MND),
            "NITEEXST" => Ok(Exchange::NITEEXST),
            "PEARL" => Ok(Exchange::PEARL),
            "GSDARK" => Ok(Exchange::GSDARK),
            "NITERTL" => Ok(Exchange::NITERTL),
            "NYSENAT" => Ok(Exchange::NYSENAT),
            "IEXMID" => Ok(Exchange::IEXMID),
            "HRT" => Ok(Exchange::HRT),
            "FLOWTRADE" => Ok(Exchange::FLOWTRADE),
            "HRTDP" => Ok(Exchange::HRTDP),
            "JANELP" => Ok(Exchange::JANELP),
            "PEAK6" => Ok(Exchange::PEAK6),
            "IMCDP" => Ok(Exchange::IMCDP),
            "CTDLZERO" => Ok(Exchange::CTDLZERO),
            "HRTMID" => Ok(Exchange::HRTMID),
            "JANEZERO" => Ok(Exchange::JANEZERO),
            "HRTEXST" => Ok(Exchange::HRTEXST),
            "IMCLP" => Ok(Exchange::IMCLP),
            "LTSE" => Ok(Exchange::LTSE),
            "SOCGENDP" => Ok(Exchange::SOCGENDP),
            "MEMX" => Ok(Exchange::MEMX),
            "INTELCROS" => Ok(Exchange::INTELCROS),
            "VIRTUBYIN" => Ok(Exchange::VIRTUBYIN),
            "JUMPTRADE" => Ok(Exchange::JUMPTRADE),
            "NITEZERO" => Ok(Exchange::NITEZERO),
            "TPLUS1" => Ok(Exchange::TPLUS1),
            "XTXEXST" => Ok(Exchange::XTXEXST),
            "XTXDP" => Ok(Exchange::XTXDP),
            "XTXMID" => Ok(Exchange::XTXMID),
            "COWENLP" => Ok(Exchange::COWENLP),
            "BARCDP" => Ok(Exchange::BARCDP),
            "JUMPLP" => Ok(Exchange::JUMPLP),
            "OLDMCLP" => Ok(Exchange::OLDMCLP),
            "RBCCMALP" => Ok(Exchange::RBCCMALP),
            "WALLBETH" => Ok(Exchange::WALLBETH),
            "JONES" => Ok(Exchange::JONES),
            "GSLP" => Ok(Exchange::GSLP),
            "CME" => Ok(Exchange::CME),
            "QBALGO" => Ok(Exchange::QBALGO),
            "IBUSOPT" => Ok(Exchange::IBUSOPT),
            "NYSEDARK" => Ok(Exchange::NYSEDARK),
            "IBEOS" => Ok(Exchange::IBEOS),
            "BLUEOCEAN" => Ok(Exchange::BLUEOCEAN),
            _ => Err(()),
        }
    }
}
impl Display for Exchange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
