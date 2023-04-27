"use strict";

function projects_load()
{
    let list = document.getElementById("project_list");

    let request = new XMLHttpRequest();
    request.onreadystatechange = function()
    {
        if (this.readyState != 4 || this.status != 200) return;

        let response = JSON.parse(request.response);

        response.forEach(element =>
        {
            let entry                             = document.createElement("tr");

            let projects_element                  = document.createElement("td");
            let projects_element_link             = document.createElement("a");
            let projects_element_link_image       = document.createElement("img");

            projects_element_link.innerText       = " " + element.name;
            projects_element_link.href            = element.html_url;
            projects_element_link.target          = "_blank";

            projects_element_link_image.src       = "/Content/GitHub.png";
            projects_element_link_image.alt       = "";
            projects_element_link_image.className = "utilities_icon";

            if (element.fork)
            {
                // TODO: use https://api.github.com/repos/<element.full_name> to retrieve the "source" repo

                let projects_element_link_fork_text             = document.createElement("text");

                projects_element_link_fork_text.innerText       += " - fork";
                projects_element_link_fork_text.style.fontStyle = "italic";

                projects_element_link.appendChild(projects_element_link_fork_text);
            }

            projects_element.appendChild(projects_element_link_image);
            projects_element.appendChild(projects_element_link);
            entry.appendChild(projects_element);

            let descriptionElement       = document.createElement("td");
            descriptionElement.innerText = element.description;
            entry.appendChild(descriptionElement);

            let languageElement       = document.createElement("td");
            languageElement.innerText = element.language;
            entry.appendChild(languageElement);

            list.appendChild(entry);
        });
    };

    request.open("GET", "https://api.github.com/users/yeah-its-gloria/repos");
    request.setRequestHeader("Accept", "application/vnd.github.v3+json");
    request.send();
}
