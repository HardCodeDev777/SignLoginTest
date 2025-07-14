// Получаем все нужные данные
const profileUserName = document.getElementById("profileUserName");

// Ставим юзернейм тексту
profileUserName.textContent = localStorage.getItem("username");

// При выходе просто на / перекидывает
const Exit = async () => 
    { 
        localStorage.setItem("username", "Login to see username!");
        window.location.href = "/";
    }

// Привязываем функцию к кнопке
document.getElementById("exit")?.addEventListener("click", Exit);