// Tailwind/DaisyUI
import "./SCSS/input.scss";

// Whatever logic is involved
import "./templates";

// Pug templates
const requireAll = (requirement: any) => {
    requirement.keys().forEach(requirement);
};
requireAll(require.context("./templates/", true, /\.pug$/));

// Themes
const { themeChange } = require("theme-change");
themeChange();
