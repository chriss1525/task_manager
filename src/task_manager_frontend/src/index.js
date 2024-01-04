import { task_manager_backend } from "../../declarations/task_manager_backend";

document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const name = document.getElementById("name").value.toString();

  if (!name) {
    alert('Name cannot be empty');
    return;
  }

  button.setAttribute("disabled", true);

  try {
    // Interact with foo actor, calling the greet method
    const greeting = await task_manager_backend.greet(name);

    button.removeAttribute("disabled");

    document.getElementById("greeting").innerText = greeting;
  } catch (error) {
    console.error('An error occurred:', error);
    button.removeAttribute("disabled");
    alert('An error occurred. Please try again later.');
  }

  return false;
});
