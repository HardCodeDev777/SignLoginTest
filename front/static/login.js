// Get all required elements
const loginUserName = document.getElementById("loginUserName");
const loginPassword = document.getElementById("loginPassword");
const output = document.getElementById("output");

// Bind the function to the button
document.getElementById("login")?.addEventListener("click", Login);

async function Login() {
    // Get values
    const userName = loginUserName.value;
    const password = loginPassword.value;

    // Make POST request to server
    const response = await fetch("http://localhost:4444/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name: userName, password: password })
    });

    // If successful, redirect to /profile and set the username in localStorage
    if (response.status === 200) {
        localStorage.setItem("username", userName);
        window.location.href = "/profile";
    }

    // Display server response
    const text = await response.text();
    output.textContent = text;
}