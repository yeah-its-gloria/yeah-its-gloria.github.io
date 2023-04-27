"use strict";

function header_button_switch_page(page_name) // string -> void
{
    let elements = document.getElementsByTagName("head");

    console.assert(elements.length == 1);
    console.assert(elements[0].hasAttribute("data-current-page"));

    if (elements[0].getAttribute("data-current-page") == page_name)
    {
        header_button_toggle_dropdown_menu();
        return;
    }

    switch (page_name)
    {
    case "main":
        window.location.href = "/";
        break;

    case "projects":
        window.location.href = "/projects";
        break;

    case "cell":
        window.location.href = "/cell";
        break;

    case "random":
        window.location.href = "/random";
        break;
    }
}

function header_button_serenaaaa()
{
    let elements = document.getElementsByTagName("head");

    console.assert(elements.length == 1);
    console.assert(elements[0].hasAttribute("data-current-page"));

    if (elements[0].getAttribute("data-current-page") == "main")
    {
        return;
    }

    window.location.href = "/";
}

function header_button_main()
{
    header_button_switch_page("main");
}

function header_button_projects()
{
    header_button_switch_page("projects");
}

function header_button_cell()
{
    header_button_switch_page("cell");
}

function header_button_random()
{
    header_button_switch_page("random");
}

function header_button_toggle_dropdown_menu()
{
    let menu = document.getElementById("header_small_dropdown_menu");

    if (menu.style.visibility !== "visible")
    {
        menu.style.visibility = "visible";
        menu.style.top        = "4rem"; // Number of buttons
        return;
    }

    menu.style.visibility = "hidden";
    menu.style.top        = "-16rem"; // Number of buttons, squared
}
