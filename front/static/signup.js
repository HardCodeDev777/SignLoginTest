// Get all required elements
const signupUserName = document.getElementById("signupUserName");
const signupPassword = document.getElementById("signupPassword");
const output = document.getElementById("output");

// Attach the function to the button
document.getElementById("signup")?.addEventListener("click", SignUp);

async function SignUp() {
    // Get values
    const userName = signupUserName.value;
    const password = signupPassword.value;

    // Make POST request to server
    const response = await fetch("http://localhost:4444/signup", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name: userName, password: password })
    });

    // Display server response
    const text = await response.text();
    output.textContent = text;
}
