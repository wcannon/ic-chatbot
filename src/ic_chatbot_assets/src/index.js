import { ic_chatbot } from "../../declarations/ic_chatbot";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with ic_chatbot actor, calling the greet method
  const greeting = await ic_chatbot.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
