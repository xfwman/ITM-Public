import { createTheme, rem } from "@mantine/core";

export const theme = createTheme({
  primaryColor: "brand",
  primaryShade: {
    light: 5,
    dark: 4,
  },
  defaultRadius: "lg",
  fontFamily: "var(--font-body), sans-serif",
  headings: {
    fontFamily: "var(--font-heading), serif",
    fontWeight: "700",
    sizes: {
      h1: { fontSize: rem(52), lineHeight: "1.04" },
      h2: { fontSize: rem(40), lineHeight: "1.08" },
      h3: { fontSize: rem(28), lineHeight: "1.15" },
    },
  },
  colors: {
    brand: [
      "#e7f6f4",
      "#cfeee8",
      "#9dded6",
      "#68cdc2",
      "#3bbeb2",
      "#1da69a",
      "#11857d",
      "#0d6962",
      "#0d534e",
      "#0d4541",
    ],
    ink: [
      "#edf2f7",
      "#d6dee8",
      "#a9b6c8",
      "#7b8ca6",
      "#526786",
      "#384d6c",
      "#273b58",
      "#1b2b43",
      "#111d2f",
      "#09111b",
    ],
    sand: [
      "#fff8ee",
      "#f8eddc",
      "#ecd9b7",
      "#dfc18f",
      "#d4ac6d",
      "#cc9c56",
      "#b58041",
      "#906433",
      "#764f2d",
      "#634226",
    ],
  },
  other: {
    borderColor: "rgba(27, 43, 67, 0.12)",
    borderColorDark: "rgba(214, 222, 232, 0.12)",
  },
  components: {
    Button: {
      defaultProps: {
        radius: "xl",
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
