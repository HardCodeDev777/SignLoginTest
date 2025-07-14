// Получаем все нужные данные
const loginUserName = document.getElementById("loginUserName");
const loginPassword = document.getElementById("loginPassword");
const output = document.getElementById("output");

// Привязываем функцию к кнопке
document.getElementById("login")?.addEventListener("click", Login);

async function Login() {
    // Получаем значения
    const userName = loginUserName.value;
    const password = loginPassword.value;

    // Делаем POST запрос на сервер
    const response = await fetch("http://localhost:4444/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name: userName, password: password })
    });

    // Если успешно то идем в /profile и ставим экспортному юзернейму текущий юзернейм
    if (response.status === 200) {
        localStorage.setItem("username", userName);
        window.location.href = "/profile";
    }
    
    // Это нужно чтоб показать что неверные данные
    const text = await response.text();
    output.textContent = text;
}