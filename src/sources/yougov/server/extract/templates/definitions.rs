use super::PanelTemplate;

pub(super) const PRIMARY_PANEL_HEADER: &str = "Sex Race Age Education";
const SECONDARY_PANEL_HEADER: &str = "2024 Vote Reg Ideology MAGA Party ID";

const PRIMARY_PANEL_COLUMNS: &[&str] = &[
    "Total",
    "Male",
    "Female",
    "White",
    "Black",
    "Hispanic",
    "18-29",
    "30-44",
    "45-64",
    "65+",
    "No degree",
    "College grad",
];

const SECONDARY_PANEL_COLUMNS: &[&str] = &[
    "Total",
    "Harris",
    "Trump",
    "Voters",
    "Lib",
    "Mod",
    "Con",
    "Supporter",
    "Dem",
    "Ind",
    "Rep",
];

const PRIMARY_PANEL_GROUPS: &[(&str, &[&str])] = &[
    ("Sex", &["Male", "Female"]),
    ("Race", &["White", "Black", "Hispanic"]),
    ("Age", &["18-29", "30-44", "45-64", "65+"]),
    ("Education", &["No degree", "College grad"]),
];

const SECONDARY_PANEL_GROUPS: &[(&str, &[&str])] = &[
    ("2024 Vote", &["Harris", "Trump"]),
    ("Reg", &["Voters"]),
    ("Ideology", &["Lib", "Mod", "Con"]),
    ("MAGA", &["Supporter"]),
    ("Party ID", &["Dem", "Ind", "Rep"]),
];

pub(super) const PANEL_TEMPLATES: &[PanelTemplate] = &[
    PanelTemplate {
        header: PRIMARY_PANEL_HEADER,
        columns: PRIMARY_PANEL_COLUMNS,
        groups: PRIMARY_PANEL_GROUPS,
    },
    PanelTemplate {
        header: SECONDARY_PANEL_HEADER,
        columns: SECONDARY_PANEL_COLUMNS,
        groups: SECONDARY_PANEL_GROUPS,
    },
];
