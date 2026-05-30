async function loadNavbar() {
  const response = await fetch("/app/navbar.html");
  const html = await response.text();
  document.getElementById("navbar").innerHTML = html;
}
loadNavbar();
