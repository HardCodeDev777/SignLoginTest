// Получаем все нужные данные
const signupUserName = document.getElementById("signupUserName");
const signupPassword = document.getElementById("signupPassword");
const output = document.getElementById("output");

// Привязываем функцию к кнопке
document.getElementById("signup")?.addEventListener("click", SignUp);

async function SignUp() {
    // Получаем значения
    const userName = signupUserName.value;
    const password = signupPassword.value;

    // Делаем POST запрос на сервер
    const response = await fetch("http://localhost:4444/signup", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name: userName, password: password })
    });

    // Это нужно чтоб показать что неверные данные
    const text = await response.text();
    output.textContent = text;
}