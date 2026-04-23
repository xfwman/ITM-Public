type MantineColorScheme = "light" | "dark" | "auto";

const LOCAL_STORAGE_KEY = "mantine-color-scheme-value";

export function getMantineColorSchemeScript(
  defaultColorScheme: MantineColorScheme,
) {
  return `try {
  var _colorScheme = window.localStorage.getItem("${LOCAL_STORAGE_KEY}");
  var colorScheme = _colorScheme === "light" || _colorScheme === "dark" || _colorScheme === "auto" ? _colorScheme : "${defaultColorScheme}";
  var computedColorScheme = colorScheme !== "auto" ? colorScheme : window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  document.documentElement.setAttribute("data-mantine-color-scheme", computedColorScheme);
} catch (e) {}`;
}
