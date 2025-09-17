// SPDX-FileCopyrightText: 2025 HalfSweet
// SPDX-License-Identifier: Apache-2.0

#[allow(non_snake_case)]
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct FileName {
    pub EDA_Name: &'static str,

    pub NPTH_Through: &'static str,
    pub PTH_Through: &'static str,
    pub PTH_Through_Via: &'static str,

    pub Bottom_Silkscreen: &'static str,
    pub Bottom_Soldermask: &'static str,
    pub Bottom_PasteMask: &'static str,
    pub Bottom_Layer: &'static str,

    pub Top_Silkscreen: &'static str,
    pub Top_Soldermask: &'static str,
    pub Top_PasteMask: &'static str,
    pub Top_Layer: &'static str,

    pub Board_Outline: &'static str,

    pub InnerLayer: &'static str,

    pub InnerLayer_Templete: &'static str,

    pub Other: Vec<&'static str>,
}

impl IntoIterator for FileName {
    type Item = (&'static str, Vec<&'static str>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("NPTH_Through", vec![self.NPTH_Through]),
            ("PTH_Through", vec![self.PTH_Through]),
            ("PTH_Through_Via", vec![self.PTH_Through_Via]),
            ("Bottom_Silkscreen", vec![self.Bottom_Silkscreen]),
            ("Bottom_Soldermask", vec![self.Bottom_Soldermask]),
            ("Bottom_PasteMask", vec![self.Bottom_PasteMask]),
            ("Bottom_Layer", vec![self.Bottom_Layer]),
            ("Top_Silkscreen", vec![self.Top_Silkscreen]),
            ("Top_Soldermask", vec![self.Top_Soldermask]),
            ("Top_PasteMask", vec![self.Top_PasteMask]),
            ("Top_Layer", vec![self.Top_Layer]),
            ("Board_Outline", vec![self.Board_Outline]),
            ("InnerLayer", vec![self.InnerLayer]),
            ("Other", self.Other),
        ]
        .into_iter()
    }
}

impl FileName {
    pub fn get(&self, key: &str) -> Option<&'static str> {
        match key {
            "NPTH_Through" => Some(self.NPTH_Through),
            "PTH_Through" => Some(self.PTH_Through),
            "PTH_Through_Via" => Some(self.PTH_Through_Via),
            "Bottom_Silkscreen" => Some(self.Bottom_Silkscreen),
            "Bottom_Soldermask" => Some(self.Bottom_Soldermask),
            "Bottom_PasteMask" => Some(self.Bottom_PasteMask),
            "Bottom_Layer" => Some(self.Bottom_Layer),
            "Top_Silkscreen" => Some(self.Top_Silkscreen),
            "Top_Soldermask" => Some(self.Top_Soldermask),
            "Top_PasteMask" => Some(self.Top_PasteMask),
            "Top_Layer" => Some(self.Top_Layer),
            "Board_Outline" => Some(self.Board_Outline),
            "InnerLayer" => Some(self.InnerLayer),
            _ => None,
        }
    }
}

lazy_static! {
    pub static ref JLC_STYLE: FileName = FileName {
        EDA_Name: "JLC",

        NPTH_Through: "Drill_NPTH_Through.DRL",
        PTH_Through: "Drill_PTH_Through.DRL",
        PTH_Through_Via: "Drill_PTH_Through_Via.DRL",

        Bottom_Silkscreen: "Gerber_BottomSilkscreenLayer.GBO",
        Bottom_Soldermask: "Gerber_BottomSolderMaskLayer.GBS",
        Bottom_PasteMask: "Gerber_BottomPasteMaskLayer.GBP",
        Bottom_Layer: "Gerber_BottomLayer.GBL",

        Top_Silkscreen: "Gerber_TopSilkscreenLayer.GTO",
        Top_Soldermask: "Gerber_TopSolderMaskLayer.GTS",
        Top_PasteMask: "Gerber_TopPasteMaskLayer.GTP",
        Top_Layer: "Gerber_TopLayer.GTL",

        Board_Outline: "Gerber_BoardOutlineLayer.GKO",

        InnerLayer: "^Gerber_InnerLayer(\\d+)\\.G(\\d+)$",

        InnerLayer_Templete: "Gerber_InnerLayer{0}.G{1}",

        Other: vec!["null"],
    };
}

lazy_static! {
    pub static ref KICAD_STYLE: FileName = FileName {
        EDA_Name: "KiCAD",

        NPTH_Through: "-NPTH.drl",
        PTH_Through: "-PTH.drl",
        PTH_Through_Via: "null",

        Bottom_Silkscreen: "-B_Silkscreen",
        Bottom_Soldermask: "-B_Mask",
        Bottom_PasteMask: "-B_Paste",
        Bottom_Layer: "-B_Cu",

        Top_Silkscreen: "-F_Silkscreen",
        Top_Soldermask: "-F_Mask",
        Top_PasteMask: "-F_Paste",
        Top_Layer: "-F_Cu",

        Board_Outline: "-Edge.Cuts",

        InnerLayer: "In\\d+_Cu\\.g\\d+$",

        InnerLayer_Templete: "In{0}_Cu.g{1}",

        Other: vec!["null"],
    };
}

lazy_static! {
    pub static ref PROTEL_STYLE: FileName = FileName {
        EDA_Name: "Protel",

        NPTH_Through: "",
        PTH_Through: "",
        PTH_Through_Via: "",

        Bottom_Silkscreen: "(?i)\\.gbo$",
        Bottom_Soldermask: "(?i)\\.gbs$",
        Bottom_PasteMask: "(?i)\\.gbp$",
        Bottom_Layer: "(?i)\\.gbl$",

        Top_Silkscreen: "(?i)\\.gto$",
        Top_Soldermask: "(?i)\\.gts$",
        Top_PasteMask: "(?i)\\.gtp$",
        Top_Layer: "(?i)\\.gtl$",

        Board_Outline: "null",

        InnerLayer: "null",

        InnerLayer_Templete: "null",

        Other: vec!["(?i)\\.txt$", "(?i)\\.drl$", "(?i)\\.drr$"],
    };
}

lazy_static! {
    pub static ref ALL_STYLE: [&'static FileName; 3] = [&JLC_STYLE, &KICAD_STYLE, &PROTEL_STYLE];
}
