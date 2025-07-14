// Get all required elements
const profileUserName = document.getElementById("profileUserName");

// Set username text
profileUserName.textContent = localStorage.getItem("username");

// On logout just redirect to /
const Exit = async () => 
    { 
        localStorage.setItem("username", "Login to see username!");
        window.location.href = "/";
    }

// Bind the function to the button
document.getElementById("exit")?.addEventListener("click", Exit);