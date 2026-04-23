import { createTheme, rem } from "@mantine/core";

export const theme = createTheme({
  primaryColor: "brand",
  primaryShade: {
    light: 5,
    dark: 6,
  },
  defaultRadius: "md",
  fontFamily: "var(--font-body), sans-serif",
  headings: {
    fontFamily: "var(--font-heading), serif",
    fontWeight: "700",
    sizes: {
      h1: { fontSize: rem(60), lineHeight: "1.02" },
      h2: { fontSize: rem(42), lineHeight: "1.06" },
      h3: { fontSize: rem(28), lineHeight: "1.12" },
    },
  },
  colors: {
    brand: [
      "#efffe6",
      "#defec8",
      "#bcfb95",
      "#98f55d",
      "#7fe93a",
      "#70d02a",
      "#59a91f",
      "#437f1a",
      "#356417",
      "#2c5316",
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
      "#fff8ef",
      "#f7eddb",
      "#ead7b5",
      "#dcbc87",
      "#d0a667",
      "#c48e4b",
      "#a67339",
      "#82592e",
      "#6a4727",
      "#583c24",
    ],
  },
  other: {
    borderColor: "rgba(20, 28, 38, 0.12)",
    borderColorDark: "rgba(239, 243, 247, 0.12)",
  },
  components: {
    Button: {
      defaultProps: {
        radius: "xl",
        fw: 600,
      },
    },
    Paper: {
      defaultProps: {
        radius: "xl",
      },
    },
    Card: {
      defaultProps: {
        radius: "xl",
        padding: "lg",
      },
    },
    Badge: {
      defaultProps: {
        radius: "xl",
        variant: "light",
      },
    },
  },
});
