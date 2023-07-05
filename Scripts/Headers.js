import init, { header_button_main, header_button_projects, header_button_cell, header_button_random, header_button_toggle_drop_down_menu } from "/Scripts/Module.js";

init().then(() => {
    // yeah uhh mhm it's totally okay

    document.getElementById("header_button_header")    .addEventListener("click", header_button_main);
    document.getElementById("header_button_main")      .addEventListener("click", header_button_main);
    document.getElementById("header_button_projects")  .addEventListener("click", header_button_projects);
    document.getElementById("header_button_cell")      .addEventListener("click", header_button_cell);
    document.getElementById("header_button_random")    .addEventListener("click", header_button_random);

    document.getElementById("header_button_header_2")  .addEventListener("click", header_button_main);
    document.getElementById("header_button_main_2")    .addEventListener("click", header_button_main);
    document.getElementById("header_button_projects_2").addEventListener("click", header_button_projects);
    document.getElementById("header_button_cell_2")    .addEventListener("click", header_button_cell);
    document.getElementById("header_button_random_2")  .addEventListener("click", header_button_random);

    document.getElementById("header_button_toggle_drop_down_menu").addEventListener("click", header_button_toggle_drop_down_menu);
});
