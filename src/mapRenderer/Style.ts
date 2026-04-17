import { TagTagData } from "@/types/osmData";

export const LIGHT_VALUES = {
  "50": "50",
  "100": "100",
  "200": "200",
  "300": "300",
  "400": "400",
  "500": "500",
  "600": "600",
  "700": "700",
  "800": "800",
  "900": "900",
  "950": "950",
} as const;

export const COLOR_NAMES = {
  red: "red",
  orange: "orange",
  amber: "amber",
  yellow: "yellow",
  lime: "lime",
  green: "green",
  emerald: "emerald",
  teal: "teal",
  cyan: "cyan",
  sky: "sky",
  blue: "blue",
  indigo: "indigo",
  violet: "violet",
  purple: "purple",
  fuchsia: "fuchsia",
  pink: "pink",
  rose: "rose",
  slate: "slate",
  gray: "gray",
  zinc: "zinc",
  neutral: "neutral",
  stone: "stone",
  taupe: "taupe",
  mauve: "mauve",
  mist: "mist",
  olive: "olive",
} as const;

export const getColor = (
  name: keyof typeof COLOR_NAMES,
  lightness: keyof typeof LIGHT_VALUES,
) => {
  return window
    .getComputedStyle(document.body)
    .getPropertyValue(`--color-${name}-${lightness}`);
};

export const strokeConditions: [
  (t: TagTagData) => boolean,
  { width: number; color: string },
][] = [
  [
    (t) => t["place"] === "island",
    { width: 1, color: getColor(COLOR_NAMES["amber"], LIGHT_VALUES[400]) },
  ],
  [
    (t) => t["highway"] === "unclassified",
    { width: 10, color: getColor(COLOR_NAMES["neutral"], LIGHT_VALUES[50]) },
  ],
  [
    () => true,
    { width: 1, color: getColor(COLOR_NAMES["neutral"], LIGHT_VALUES[400]) },
  ],
];

export const colorConditions: [(t: TagTagData) => boolean, string][] = [
  [
    (t) => t["place"] === "island",
    getColor(COLOR_NAMES["amber"], LIGHT_VALUES[100]),
  ],
  [
    (t) => t.landuse === "meadow",
    getColor(COLOR_NAMES["lime"], LIGHT_VALUES[200]),
  ],

  [
    (t) => t.natural === "wood",
    getColor(COLOR_NAMES["green"], LIGHT_VALUES[300]),
  ],
  [
    (t) => t.natural === "water",
    getColor(COLOR_NAMES["blue"], LIGHT_VALUES[200]),
  ],
  [
    (t) => t.landuse === "residential",
    getColor(COLOR_NAMES["neutral"], LIGHT_VALUES[200]),
  ],
  [
    (t) => t.building === "yes",
    getColor(COLOR_NAMES["neutral"], LIGHT_VALUES[400]),
  ],
  [
    (t) => t.landuse === "basin",
    getColor(COLOR_NAMES["blue"], LIGHT_VALUES[200]),
  ],
  [
    (t) => t.landuse === "farmland",
    getColor(COLOR_NAMES["orange"], LIGHT_VALUES[200]),
  ],
];
