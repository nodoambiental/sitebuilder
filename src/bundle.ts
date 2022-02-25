import "./SCSS/input.scss";
import "./templates";

const requireAll = (requirement: any) => {
    requirement.keys().forEach(requirement);
};

requireAll(require.context("./templates/", true, /\.pug$/));
