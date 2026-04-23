import { createTheme, rem } from "@mantine/core";

export const theme = createTheme({
  primaryColor: "brand",
  primaryShade: {
    light: 7,
    dark: 5,
  },
  defaultRadius: "sm",
  fontFamily: "var(--font-body), sans-serif",
  headings: {
    fontFamily: "var(--font-heading), sans-serif",
    fontWeight: "700",
    sizes: {
      h1: { fontSize: rem(60), lineHeight: "1.02" },
      h2: { fontSize: rem(42), lineHeight: "1.06" },
      h3: { fontSize: rem(28), lineHeight: "1.12" },
    },
  },
  colors: {
    brand: [
      "#eef4ff",
      "#d9e6ff",
      "#aec7ef",
      "#80a7df",
      "#5b8acb",
      "#3e6fb0",
      "#2d5892",
      "#1f4372",
      "#143051",
      "#0a1d34",
    ],
    ink: [
      "#eff3f7",
      "#d5dde7",
      "#abb8c8",
      "#8193aa",
      "#607690",
      "#485d76",
      "#35485f",
      "#243447",
      "#141e2a",
      "#091019",
    ],
    sand: [
      "#fff4e8",
      "#ffe1c1",
      "#ffc88c",
      "#ffad4f",
      "#ff931b",
      "#ff7800",
      "#e45f00",
      "#bd4a00",
      "#973d04",
      "#7a3308",
    ],
    ember: [
      "#fff0e6",
      "#ffd1bc",
      "#ffae84",
      "#ff8747",
      "#ff6519",
      "#f24b00",
      "#cc3900",
      "#a42d04",
      "#82250a",
      "#681f0d",
    ],
  },
  other: {
    borderColor: "rgba(20, 28, 38, 0.12)",
    borderColorDark: "rgba(239, 243, 247, 0.12)",
    colorRoles: {
      headerBackground: "#02114b",
      headerForeground: "#ffffff",
      bodyAmbientStart: "#ff931b",
      bodyAmbientMiddle: "#ff5f00",
      bodyAmbientEnd: "#fff0df",
      surfaceDefault: "rgba(255, 250, 245, 0.88)",
      surfaceStrong: "rgba(255, 253, 249, 0.96)",
    },
  },
  components: {
    Button: {
      defaultProps: {
        radius: "sm",
        fw: 600,
      },
    },
    Paper: {
      defaultProps: {
        radius: "md",
      },
    },
    Card: {
      defaultProps: {
        radius: "md",
        padding: "lg",
      },
    },
    Badge: {
      defaultProps: {
        radius: "sm",
        variant: "light",
      },
    },
  },
});
